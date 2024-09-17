use crate::backend::db::get_connection;
use rusqlite::{Connection, Result, params};

#[derive(serde::Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
}

// Function to create the rag table
pub fn create_tags_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );";

    conn.execute(create_table_sql, [])?;

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS transaction_tags (
        transaction_id INTEGER,
        tag_id INTEGER,
        PRIMARY KEY (transaction_id, tag_id),
        FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE,
        FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
    );";

    conn.execute(create_table_sql, [])?;

    Ok(())
}

// Add more ledger-related functions here as needed
#[tauri::command]
pub fn create_tag(name: &str, color: Option<&str>) -> Result<i64, String> {
    let conn = get_connection();
    insert_tag(&conn, name, color)
}

#[tauri::command]
pub fn get_tags() -> Result<Vec<Tag>, String> {
    let conn = get_connection();
    get_all_tags(&conn)
}

#[tauri::command]
pub fn update_tag(tag_id: i64, name: &str, color: Option<&str>) -> Result<(), String> {
    let conn = get_connection();
    update_tag_row(&conn, tag_id, name, color)
}

#[tauri::command]
pub fn delete_tag(tag_id: i64) -> Result<(), String> {
    let conn = get_connection();
    delete_tag_row(&conn, tag_id)
}

fn insert_tag(conn: &Connection, name: &str, color: Option<&str>) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO tags (name, color) VALUES (?1, ?2)",
        params![name, color],
    )
    .map_err(|err| format!("Failed to insert tag: {}", err))?;

    Ok(conn.last_insert_rowid())
}

fn get_all_tags(conn: &Connection) -> Result<Vec<Tag>, String> {
    let mut stmt = conn.prepare("SELECT id, name, color FROM tags")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let tag_iter = stmt
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|err| format!("Failed to get tags: {}", err))?;

    let mut tags = Vec::new();
    for tag in tag_iter {
        tags.push(tag.map_err(|e| format!("Failed to parse tag row: {}", e))?);
    }

    Ok(tags)
}

// Update tag
fn update_tag_row(conn: &Connection, tag_id: i64, name: &str, color: Option<&str>) -> Result<(), String> {
    conn.execute(
        "UPDATE tags SET name = ?1, color = ?2 WHERE id = ?3",
        params![name, color, tag_id],
    )
    .map_err(|err| format!("Failed to update tag: {}", err))?;

    Ok(())
}

// Delete tag
fn delete_tag_row(conn: &Connection, tag_id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM tags WHERE id = ?1", params![tag_id])
        .map_err(|err| format!("Failed to delete tag: {}", err))?;

    Ok(())
}