use crate::backend::db::get_connection;
use rusqlite::{params, Result};

#[derive(serde::Serialize)]
pub struct Category {
    pub id: i64,
    pub ledger_id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub category_type: String,
    pub subcategories: Vec<String>,
}

pub fn create_categories_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS categories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ledger_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        icon TEXT,
        color TEXT,
        subcategories TEXT,  -- JSON array of subcategories
        type TEXT NOT NULL CHECK( type IN ('expense', 'income', 'transfer') ),
        FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE
    );";

    conn.execute(create_table_sql, [])?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_category(
    ledger_id: i64,
    name: &str,
    icon: Option<&str>,
    color: Option<&str>,
    subcategories: Vec<String>,
    category_type: &str, // 'expense', 'income', 'transfer'
) -> Result<i64, String> {
    let conn = get_connection();

    let subcategories_json = serde_json::to_string(&subcategories).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO categories (ledger_id, name, icon, color, subcategories, type) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            ledger_id,
            name,
            icon,
            color,
            subcategories_json,
            category_type
        ],
    )
    .map_err(|err| format!("Failed to insert category: {}", err))?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_categories_for_ledger(
    ledger_id: i64,
    category_type: &str,
) -> Result<Vec<Category>, String> {
    let conn = get_connection();

    let mut stmt = conn
        .prepare(
            "SELECT id, name, icon, color, subcategories 
         FROM categories WHERE ledger_id = ?1 AND type = ?2",
        )
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let category_iter = stmt
        .query_map(params![ledger_id, category_type], |row| {
            let subcategories_json: String = row.get(4)?;

            Ok(Category {
                id: row.get(0)?,
                ledger_id,
                name: row.get(1)?,
                icon: row.get(2)?,
                color: row.get(3)?,
                subcategories: serde_json::from_str(&subcategories_json).unwrap_or_else(|_| vec![]),
                category_type: category_type.to_string(),
            })
        })
        .map_err(|err| format!("Failed to get category: {}", err))?;

    let mut categories = Vec::new();
    for category in category_iter {
        categories.push(category.map_err(|e| format!("Failed to parse category row: {}", e))?);
    }

    Ok(categories)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_category(
    category_id: i64,
    name: &str,
    icon: Option<&str>,
    color: Option<&str>,
    subcategories: Vec<String>,
    category_type: &str, // 'expense', 'income', 'transfer'
) -> Result<(), String> {
    let conn = get_connection();

    let subcategories_json = serde_json::to_string(&subcategories).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE categories SET name = ?1, icon = ?2, color = ?3, subcategories = ?4, type = ?5 
         WHERE id = ?6",
        params![
            name,
            icon,
            color,
            subcategories_json,
            category_type,
            category_id
        ],
    )
    .map_err(|err| format!("Failed to update category: {}", err))?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_category(category_id: i64) -> Result<(), String> {
    let conn = get_connection();

    conn.execute("DELETE FROM categories WHERE id = ?1", params![category_id])
        .map_err(|err| format!("Failed to delete category: {}", err))?;

    Ok(())
}
