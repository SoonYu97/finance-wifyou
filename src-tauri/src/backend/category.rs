use crate::backend::db::get_connection;
use rusqlite::Result;

#[derive(serde::Serialize)]
pub struct Category {
    pub id: i32,
    pub ledger_id: i32,
    pub name: String,
    pub category_type: String,
    pub subcategories: String,
}

// Function to create the category table
pub fn create_categories_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS categories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        ledger_id INTEGER,
        type TEXT CHECK( type IN ('expense', 'income') ),
        FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE
    );";

    conn.execute(create_table_sql, [])?;
    
    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS subcategories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        category_id INTEGER,
        FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
    );";
    conn.execute(create_table_sql, [])?;

    Ok(())
}

// Add more ledger-related functions here as needed
