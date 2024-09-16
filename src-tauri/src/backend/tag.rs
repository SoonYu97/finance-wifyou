use crate::backend::db::get_connection;
use rusqlite::Result;

#[derive(serde::Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
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
