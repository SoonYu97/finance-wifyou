use crate::backend::db::get_connection;
use rusqlite::Result;

#[derive(serde::Serialize)]
pub struct Ledger {
    pub id: i32,
    pub name: String,
    pub base_currency: String,
    pub base_account: i32,
    pub archived: bool,
}

// Function to create the ledger table
pub fn create_ledgers_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS ledgers (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        archived BOOLEAN NOT NULL DEFAULT 0,
        base_account INTEGER,
        base_currency TEXT NOT NULL,
        FOREIGN KEY (base_account) REFERENCES accounts(id) ON DELETE SET NULL,
        FOREIGN KEY (base_currency) REFERENCES currencies(code) ON DELETE RESTRICT
    );";

    conn.execute(create_table_sql, [])?;
    Ok(())
}

// Add more ledger-related functions here as needed
