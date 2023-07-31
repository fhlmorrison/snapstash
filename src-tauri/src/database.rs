use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub path: String,
    pub params: Option<String>,
}
pub struct DbImage {
    pub id: i32,
    pub path: String,
    pub params: Option<String>,
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
        "CREATE TABLE IF NOT EXISTS images (
            id INTEGER NOT NULL PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            params TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER NOT NULL PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS image_tags (
            id INTEGER NOT NULL PRIMARY KEY,
            image_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            UNIQUE (image_id, tag_id),
            FOREIGN KEY (image_id) REFERENCES images(id),
            FOREIGN KEY (tag_id) REFERENCES tags(id)
        )",
        [],
    )?;

    Ok(conn)
}

pub fn add_image(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO images (path) values (?1) ON CONFLICT(path) DO NOTHING",
        [path],
    )?;
    Ok(())
}

pub fn add_image_with_params(conn: &Connection, path: &str, params: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO images (path, params) values (?1, ?2) ON CONFLICT(path) DO NOTHING",
        [path, params],
    )?;
    Ok(())
}

pub fn move_image(conn: &Connection, old_path: &str, new_path: &str) -> Result<()> {
    conn.execute(
        "UPDATE images SET path=?1 WHERE path=?2",
        [old_path, new_path],
    )?;
    Ok(())
}

pub fn remove_image(conn: &Connection, path: &str) -> Result<()> {
    conn.execute("DELETE FROM images WHERE path=?1", [path])?;
    Ok(())
}

pub fn add_params(conn: &Connection, path: &str, params: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO images (path, params) values (?1, ?2) ON CONFLICT(path) DO UPDATE SET params=?2",
        [path, params],
    )?;
    Ok(())
}

pub fn create_tag(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tags (name) values (?1) ON CONFLICT(name) DO NOTHING",
        [name],
    )?;
    Ok(())
}

pub fn add_tag_to_image(conn: &Connection, image_id: i32, tag_id: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO image_tags (image_id, tag_id) values (?1, ?2) ON CONFLICT(image_id, tag_id) DO NOTHING",
        [image_id, tag_id],
    )?;
    Ok(())
}
