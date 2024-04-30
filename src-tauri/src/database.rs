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
            name TEXT NOT NULL UNIQUE,
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
    static UNKNOWN: &str = "unknown";
    let name = std::path::Path::new(path)
        .file_name()
        .map(|file_name| file_name.to_str().unwrap_or(UNKNOWN))
        .unwrap_or(UNKNOWN);
    conn.execute(
        "INSERT INTO images (path, name) values (?1, ?2) ON CONFLICT(path) DO NOTHING",
        [path, name],
    )?;
    Ok(())
}

pub fn add_image_with_params(conn: &Connection, path: &str, params: &str) -> Result<()> {
    static UNKNOWN: &str = "unknown";
    let name = std::path::Path::new(path)
        .file_name()
        .map(|file_name| file_name.to_str().unwrap_or(UNKNOWN))
        .unwrap_or(UNKNOWN);
    conn.execute(
        "INSERT INTO images (path, name, params) values (?1, ?2, ?3) ON CONFLICT(path) DO NOTHING",
        [path, name, params],
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

pub fn get_params(conn: &Connection, path: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT params FROM images WHERE path = ?1")?;
    let mut rows = stmt.query_map([path], |row| row.get(0))?;
    let params = rows.by_ref().flatten().next().unwrap_or(None);
    Ok(params)
}

pub fn create_tag(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tags (name) values (?1) ON CONFLICT(name) DO NOTHING",
        [name],
    )?;
    Ok(())
}

pub fn add_tag_to_image(conn: &Connection, image: &str, tag: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO image_tags (image_id, tag_id) values
        ((SELECT id FROM images WHERE path = ?1),
        (SELECT id FROM tags WHERE name = ?2))
        ON CONFLICT(image_id, tag_id) DO NOTHING",
        [image, tag],
    )?;
    Ok(())
}

pub fn add_tag_to_image_by_id(conn: &Connection, image_id: i32, tag_id: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO image_tags (image_id, tag_id) values (?1, ?2) ON CONFLICT(image_id, tag_id) DO NOTHING",
        [image_id, tag_id],
    )?;
    Ok(())
}

pub fn get_tags(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM tags ORDER BY name ASC")?;
    let mut rows = stmt.query_map([], |row| row.get(0))?;
    let tags: Vec<String> = rows.by_ref().flatten().collect();
    Ok(tags)
}

pub fn get_tag_counts(conn: &Connection) -> Result<Vec<(String, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT tags.name, COUNT(image_tags.tag_id) FROM tags 
    LEFT JOIN image_tags ON tags.id = image_tags.tag_id 
    GROUP BY tags.name ORDER BY tags.name ASC",
    )?;
    let mut rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    let tags: Vec<(String, i32)> = rows.by_ref().flatten().collect();
    Ok(tags)
}

pub fn get_image_tags(conn: &Connection, image: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT tags.name FROM tags 
            INNER JOIN image_tags ON tags.id = image_tags.tag_id 
            WHERE image_tags.image_id = (SELECT id FROM images WHERE path = ?1) 
            ORDER BY tags.name ASC",
    )?;
    let mut rows = stmt.query_map([image], |row| row.get(0))?;
    let tags: Vec<String> = rows.by_ref().flatten().collect();
    Ok(tags)
}

pub fn get_image_tags_by_id(conn: &Connection, image_id: i32) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT tags.name FROM tags 
        INNER JOIN image_tags ON tags.id = image_tags.tag_id 
        WHERE image_tags.image_id = ?1 
        ORDER BY tags.name ASC",
    )?;
    let mut rows = stmt.query_map([image_id], |row| row.get(0))?;
    let tags: Vec<String> = rows.by_ref().flatten().collect();
    Ok(tags)
}

pub fn search_with_tags_or(conn: &Connection, tags: &[String]) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT path FROM images 
    WHERE id IN (SELECT image_id FROM image_tags WHERE tag_id IN (SELECT id FROM tags WHERE name IN (?1))) 
    ORDER BY name DESC")?;
    let tags = tags.join(",");
    let mut rows = stmt.query_map([tags], |row| row.get(0))?;
    let images: Vec<String> = rows.by_ref().flatten().collect();
    Ok(images)
}

pub fn search_with_tags_and(conn: &Connection, tags: &[String]) -> Result<Vec<String>> {
    !todo!("Implement search with tags and");
    let mut stmt = conn.prepare("SELECT path FROM images 
    WHERE id IN (SELECT image_id FROM image_tags WHERE tag_id IN (SELECT id FROM tags WHERE name IN (?1))) 
    ORDER BY name DESC")?;
    let tags = tags.join(",");
    let mut rows = stmt.query_map([tags], |row| row.get(0))?;
    let images: Vec<String> = rows.by_ref().flatten().collect();
    Ok(images)
}

pub fn search_params(conn: &Connection, query_text: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT path FROM images 
        WHERE params LIKE ?1 
        ORDER BY name DESC",
    )?;
    let params = format!("%{}%", query_text);
    let mut rows = stmt.query_map([params], |row| row.get(0))?;
    let images: Vec<String> = rows.by_ref().flatten().collect();
    Ok(images)
}
