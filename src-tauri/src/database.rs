use rusqlite::{Connection, Result};
use tauri::AppHandle;

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
}

pub fn init_db(handle: &AppHandle) -> Result<Connection> {
    let dir = handle
        .path_resolver()
        .app_data_dir()
        .expect("failed to get config dir");

    if !dir.is_dir() {
        std::fs::create_dir_all(dir.clone()).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some(format!("Failed to create directory: {}", e)),
            )
        })?;
    }

    let conn = Connection::open(dir.join("db.sqlite"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pictures (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            params TEXT NOT NULL,
            tags TEXT NOT NULL,
        )",
        [],
    )?;

    Ok(conn)
}

// Add update metadata function

// Add

pub fn add_song(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO songs (title, url, thumbnail, path, downloaded) VALUES (?1, ?2, ?3, ?4, ?5)",
        [],
    )?;
    Ok(())
}
