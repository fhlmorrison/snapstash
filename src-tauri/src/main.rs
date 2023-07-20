// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

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
    return chunks
        .iter()
        .find(|c| c.keyword == "parameters")
        .map(|c| c.text.clone())
        .ok_or("No parameters found".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read_parameters])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
