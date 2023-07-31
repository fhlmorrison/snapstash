// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
mod database;

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

fn main() {
    tauri::Builder::default()
        .manage(database::AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            read_parameters,
            save_images
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
