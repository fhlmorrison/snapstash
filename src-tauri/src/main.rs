// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod parameters;
mod server;
mod clip;

use tauri::{AppHandle, Manager};
use std::sync::Arc;
use std::path::{Path, PathBuf};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn clip_auto_tag(app_handle: AppHandle, images: Vec<String>, threshold: f32) -> Result<(), String> {
    let state = app_handle.state::<Arc<database::AppState>>();
    
    // 1. Get all auto-taggable tags
    let tags = app_handle.db(|db| database::get_auto_taggable_tags(db).map_err(|e| e.to_string()))?;
    if tags.is_empty() {
        println!("No auto-taggable tags found in database.");
        return Ok(());
    }

    // 2. Load CLIP model
    let clip_guard = state.clip.get_or_init().map_err(|e| e.to_string())?;
    let clip = clip_guard.as_ref().unwrap();

    // 3. Compute tag embeddings
    println!("Computing embeddings for {} tags", tags.len());
    let tag_embeddings = clip.get_text_embeddings(&tags).map_err(|e| e.to_string())?;

    // 4. For each image, compute embedding and find matches
    let total = images.len();
    for (idx, image_path) in images.iter().enumerate() {
        println!("[{}/{}] Processing CLIP for: {}", idx + 1, total, image_path);
        let path = Path::new(image_path);
        if !path.exists() {
            eprintln!("File not found: {}", image_path);
            continue;
        }

        let image_embedding = match clip.get_image_embeddings(path) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error getting image embeddings for {}: {}", image_path, err);
                continue;
            }
        };

        // Similarity = dot product (since they are normalized)
        let similarities = image_embedding.matmul(&tag_embeddings.t().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?
            .to_vec2::<f32>().map_err(|e| e.to_string())?[0].clone();

        let mut tag_scores: Vec<(&String, f32)> = tags.iter().zip(similarities.iter().copied()).collect();
        tag_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (tag, sim) in tag_scores {
            println!("  Tag: {} | Similarity: {:.4}", tag, sim);
            if sim >= threshold {
                println!("    -> Matched: {} (threshold: {:.4})", tag, threshold);
                let _ = app_handle.db(|db| database::add_tag_to_image(db, image_path, tag));
            }
        }
    }

    println!("CLIP auto-tagging complete for {} images.", total);
    Ok(())
}

#[tauri::command]
fn read_parameters(src: &str) -> Result<String, String> {
    // println!("Reading parameters from {}", src);
    let path = PathBuf::from(src);
    let decoder = png::Decoder::new(std::fs::File::open(path).map_err(|e| e.to_string())?);
    let reader = decoder.read_info().map_err(|e| e.to_string())?;
    let chunks = &reader.info().uncompressed_latin1_text;
    // TODO: add parsing for itxt chunks
    return chunks
        .iter()
        .find(|c| c.keyword == "parameters")
        .map(|c| c.text.clone())
        .ok_or("No parameters found".to_string());
}

#[tauri::command]
fn read_tags(app_handle: AppHandle, src: &str) -> Result<Vec<String>, String> {
    // println!("Reading parameters from {}", src);
    app_handle
        .db(|db| database::get_image_tags(db, src))
        .map_err(|e| e.to_string())
}

// Manually add a tag to an image
#[tauri::command]
fn add_tag_to_image(app_handle: AppHandle, image: &str, tag: &str) -> Result<(), String> {
    app_handle
        .db(|db| database::add_tag_to_image(db, image, tag))
        .map_err(|e| e.to_string())
}

// Manually remove a tag from an image
#[tauri::command]
fn remove_tag_from_image(app_handle: AppHandle, image: &str, tag: &str) -> Result<(), String> {
    println!("Removing tag {} from image {}", tag, image);
    app_handle
        .db(|db| database::remove_tag_from_image(db, image, tag))
        .map_err(|e| e.to_string())
}

// Search for images with tags
#[tauri::command]
fn search_with_tags(app_handle: AppHandle, tags: Vec<&str>) -> Result<Vec<String>, String> {
    println!("Searching with tags: {:?}", tags);
    app_handle
        .db(|db| database::search_with_tags_and(db, tags))
        .map_err(|e| e.to_string())
}

// Search for images with tags advanced
#[tauri::command]
fn search_with_tags_advanced(
    app_handle: AppHandle,
    positive_tags: Vec<&str>,
    negative_tags: Vec<&str>,
) -> Result<Vec<String>, String> {
    println!(
        "Searching with positive tags: {:?} and negative tags: {:?}",
        positive_tags, negative_tags
    );
    app_handle
        .db(|db| database::search_with_tags_advanced(db, positive_tags, negative_tags))
        .map_err(|e| e.to_string())
}

// Add tag to images that have the tag word in their prompt parameters
#[tauri::command]
fn auto_tag(
    app_handle: AppHandle,
    tag: &str,
    images: Vec<&str>,
    strict: bool,
) -> Result<(), String> {
    println!("Auto tagging images with tag {}", tag);

    let lower_tag = tag.to_lowercase();
    let lower_tag = lower_tag.as_str();

    for x in images {
        let params = read_parameters(x);
        if let Ok(p) = params {
            let tags = parameters::get_prompts(&p);

            // ASSUMPTION: Prompt tokens are all lowercase

            let contains = if strict {
                // Strict (exact match to prompt token)
                tags.contains(&lower_tag)
            } else {
                // Contains (tag is in prompt token)
                tags.iter().any(|t| t.to_lowercase().contains(lower_tag))
            };

            // Strict contains
            if contains {
                println!("Tagging {}", x);
                // Save in db
                let res = app_handle.db(|db| database::add_tag_to_image(db, x, tag));
                // Match on success/failure
                match res {
                    Ok(_) => println!("Tagged {}", x),
                    Err(e) => println!("Failed to tag {}: {}", x, e),
                }
            }
        } else {
            println!("Failed to read parameters for {}", x);
        }
    }
    println!("Finished auto tagging images with tag {}", tag);

    Ok(())
}

#[tauri::command]
fn save_images(app_handle: AppHandle, images: Vec<&str>) -> Result<(), String> {
    println!("Saving images");
    images.iter().for_each(|x| {
        println!("Saving {}", x);
        let params = read_parameters(x);
        // Save in db
        let res = match params {
            Ok(p) => app_handle.db(|db| database::add_image_with_params(db, x, &p)),
            Err(_) => app_handle.db(|db| database::add_image(db, x)),
        };
        // Match on success/failure
        match res {
            Ok(_) => println!("Saved {}", x),
            Err(e) => println!("Failed to save {}: {}", x, e),
        }
        // TODO: Emit event
        // todo!("Add interupt");
    });
    Ok(())
}

#[tauri::command]
async fn search_images(app: tauri::AppHandle, query_text: &str) -> Result<Vec<String>, String> {
    let images = app
        .db(|db| database::search_params(db, query_text))
        .map_err(|e| e.to_string())?;
    Ok(images)
}

#[tauri::command]
fn get_tags(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let image = app
        .db(|db| database::get_tags(db))
        .map_err(|e| e.to_string())?;
    Ok(image)
}

#[tauri::command]
fn create_tag(app: tauri::AppHandle, tag: &str) -> Result<(), String> {
    let image = app
        .db(|db| database::create_tag(db, tag))
        .map_err(|e| e.to_string())?;
    Ok(image)
}

#[tauri::command]
fn create_tag_group(app: tauri::AppHandle, name: &str, is_auto_taggable: bool) -> Result<(), String> {
    println!("Creating tag group: {} (auto-taggable: {})", name, is_auto_taggable);
    app.db(|db| database::create_tag_group(db, name, is_auto_taggable))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tag_groups(app: tauri::AppHandle) -> Result<Vec<database::TagGroup>, String> {
    println!("Fetching tag groups");
    app.db(|db| database::get_tag_groups(db))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tags_v2(app: tauri::AppHandle) -> Result<Vec<database::Tag>, String> {
    println!("Fetching all tags (v2)");
    app.db(|db| database::get_tags_v2(db))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn create_tag_with_group(app: tauri::AppHandle, name: &str, group_id: i32) -> Result<(), String> {
    println!("Creating tag: {} in group id: {}", name, group_id);
    app.db(|db| database::create_tag_with_group(db, name, group_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn assign_tag_to_group(app: tauri::AppHandle, tag_name: &str, group_id: Option<i32>) -> Result<(), String> {
    println!("Assigning tag: {} to group id: {:?}", tag_name, group_id);
    app.db(|db| database::assign_tag_to_group(db, tag_name, group_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_tag_group_auto_taggable(app: tauri::AppHandle, group_id: i32, is_auto_taggable: bool) -> Result<(), String> {
    println!("Updating tag group id: {} auto-taggable to: {}", group_id, is_auto_taggable);
    app.db(|db| database::update_tag_group_auto_taggable(db, group_id, is_auto_taggable))
        .map_err(|e| e.to_string())
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    let app_state = Arc::new(database::AppState {
        db: Default::default(),
        clip: clip::ClipStore::new(),
    });

    let app_state_clone = app_state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            read_parameters,
            save_images,
            search_images,
            get_tags,
            create_tag,
            auto_tag,
            clip_auto_tag,
            read_tags,
            add_tag_to_image,
            remove_tag_from_image,
            search_with_tags,
            search_with_tags_advanced,
            create_tag_group,
            get_tag_groups,
            get_tags_v2,
            create_tag_with_group,
            assign_tag_to_group,
            update_tag_group_auto_taggable,
        ])
        .setup(move |app| {
            let handle = app.handle();

            let app_state: tauri::State<Arc<database::AppState>> = handle.state();
            let db = database::init_db(&handle).expect("failed to open database");
            *app_state.db.lock().unwrap() = Some(db);

            // Start Actix server
            tauri::async_runtime::spawn(async move {
                match server::create_server(app_state_clone) {
                    Ok(server) => {
                        if let Err(e) = server.await {
                            eprintln!("Actix server error: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to create Actix server: {}", e),
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

trait DatabaseAccess {
    fn db<F, T>(&self, operation: F) -> T
    where
        F: FnOnce(&rusqlite::Connection) -> T;
}

impl DatabaseAccess for tauri::AppHandle {
    fn db<F, T>(&self, operation: F) -> T
    where
        F: FnOnce(&rusqlite::Connection) -> T,
    {
        let state = self.state::<Arc<database::AppState>>();
        let db_guard = state.db.lock().unwrap();
        let db = db_guard.as_ref().unwrap();
        operation(db)
    }
}
