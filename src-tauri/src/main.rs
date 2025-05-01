// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
mod database;
mod parameters;

use database::get_image_tags;
use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    let length = images.len();
    images.iter().enumerate().for_each(|(i, x)| {
        println!("Saving {}", x);
        let params = read_parameters(x);
        // Save in db
        let res = match params {
            Ok(p) => app_handle.db(|db| database::add_image_with_params(db, x, &p)),
            Err(e) => app_handle.db(|db| database::add_image(db, x)),
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

fn main() {
    tauri::Builder::default()
        .manage(database::AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            read_parameters,
            save_images,
            search_images,
            get_tags,
            create_tag,
            auto_tag,
            read_tags,
            add_tag_to_image,
            remove_tag_from_image,
            search_with_tags,
            search_with_tags_advanced,
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: tauri::State<database::AppState> = handle.state();
            let db = database::init_db(&handle).expect("failed to open database");
            *app_state.db.lock().unwrap() = Some(db);

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
        let state = self.state::<database::AppState>();
        let db_guard = state.db.lock().unwrap();
        let db = db_guard.as_ref().unwrap();
        operation(db)
    }
}
