use crate::backend::db::get_connection;
use rusqlite::{params, Result};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ledger {
    pub id: i64,
    pub name: String,
    pub base_currency: String,
    pub base_account: i64,
    pub is_archived: bool,
}

// Function to create the ledger table
pub fn create_ledgers_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS ledgers (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        base_account INTEGER,
        base_currency TEXT NOT NULL,
        is_archived BOOLEAN NOT NULL DEFAULT 0,
        FOREIGN KEY (base_account) REFERENCES accounts(id) ON DELETE SET NULL,
        FOREIGN KEY (base_currency) REFERENCES currencies(code) ON DELETE RESTRICT
    );";

    conn.execute(create_table_sql, [])?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_ledger(
    name: &str,
    base_currency: &str,
    base_account: i64,
    is_archived: bool,
) -> Result<i64, String> {
    let conn = get_connection();
    conn.execute(
        "INSERT INTO ledgers (name, base_currency, base_account, is_archived) 
         VALUES (?1, ?2, ?3, ?4)",
        params![name, base_currency, base_account, is_archived],
    )
    .map_err(|err| format!("Failed to insert ledger: {}", err))?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_ledgers() -> Result<Vec<Ledger>, String> {
    let conn = get_connection();

    let mut stmt = match conn
        .prepare("SELECT id, name, base_currency, base_account, archived, categories FROM ledgers")
    {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let ledger_iter = match stmt.query_map([], |row| {
        Ok(Ledger {
            id: row.get(0)?,
            name: row.get(1)?,
            base_currency: row.get(2)?,
            base_account: row.get(3)?,
            is_archived: row.get(4)?,
        })
    }) {
        Ok(iterator) => iterator,
        Err(e) => return Err(format!("Failed to query accounts: {}", e)),
    };

    let mut ledgers = Vec::new();
    for ledger in ledger_iter {
        match ledger {
            Ok(ledger) => ledgers.push(ledger),
            Err(e) => return Err(format!("Failed to parse account row: {}", e)),
        }
    }

    Ok(ledgers)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_ledger(ledger_id: i64) -> Result<Ledger, String> {
    let conn = get_connection();
    let mut stmt = match conn.prepare(
        "SELECT id, name, base_currency, base_account, is_archived 
         FROM ledgers WHERE id = ?1",
    ) {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let ledger = stmt
        .query_row(params![ledger_id], |row| {
            Ok(Ledger {
                id: row.get(0)?,
                name: row.get(1)?,
                base_currency: row.get(2)?,
                base_account: row.get(3)?,
                is_archived: row.get(4)?,
            })
        })
        .map_err(|err| format!("Failed to get ledger: {}", err))?;

    Ok(ledger)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_ledger(
    ledger_id: i64,
    name: &str,
    base_currency: &str,
    base_account: i64,
    is_archived: bool,
) -> Result<(), String> {
    let conn = get_connection();
    conn.execute(
        "UPDATE ledgers SET name = ?1, base_currency = ?2, base_account = ?3, is_archived = ?4 
         WHERE id = ?5",
        params![name, base_currency, base_account, is_archived, ledger_id],
    )
    .map_err(|err| format!("Failed to update ledger: {}", err))?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_ledger(ledger_id: i64) -> Result<(), String> {
    let conn = get_connection();
    conn.execute("DELETE FROM ledgers WHERE id = ?1", params![ledger_id])
        .map_err(|err| format!("Failed to delete ledger: {}", err))?;

    Ok(())
}
