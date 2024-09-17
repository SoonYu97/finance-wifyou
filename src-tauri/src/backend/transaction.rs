use crate::backend::db::get_connection;
use rusqlite::{params, Connection, OptionalExtension, Result};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub ledger_id: i64,
    pub account_id: i64,
    pub amount: f64,
    pub currency: String,
    pub date: String,
    pub note: Option<String>,
    pub tags: Vec<String>,
}

// Function to create the transactions table
pub fn create_transactions_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS transactions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ledger_id INTEGER NOT NULL,
        account_id INTEGER NOT NULL,
        amount DECIMAL(10, 4) NOT NULL,
        type TEXT CHECK( type IN ('expense', 'income', 'transfer') ),
        date_time TEXT DEFAULT CURRENT_TIMESTAMP,
        note TEXT,
        FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE,
        FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
    );";

    conn.execute(create_table_sql, [])?;
    Ok(())
}

// Add more transaction-related functions here as needed
#[tauri::command(rename_all = "snake_case")]
pub fn create_transaction(
    ledger_id: i64,
    account_id: i64,
    amount: f64,
    currency: &str,
    date: &str,
    tags: Vec<String>,
    note: Option<&str>,
) -> Result<(), String> {
    let conn = get_connection();

    conn.execute(
        "INSERT INTO transactions (ledger_id, account_id, amount, currency, date, note) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![ledger_id, account_id, amount, currency, date, note],
    )
    .map_err(|err| format!("Failed to insert transaction: {}", err))?;

    let transaction_id = conn.last_insert_rowid();

    // Insert tags and link them to the transaction
    for tag in tags {
        let tag_id = insert_or_get_tag(&conn, &tag)?;
        conn.execute(
            "INSERT INTO transaction_tags (transaction_id, tag_id) VALUES (?1, ?2)",
            params![transaction_id, tag_id],
        )
        .map_err(|err| format!("Failed to link tag to transaction: {}", err))?;
    }

    Ok(())
}

fn insert_or_get_tag(conn: &Connection, tag_name: &str) -> Result<i64, String> {
    // Check if tag exists
    let mut stmt = match conn.prepare("SELECT id FROM tags WHERE name = ?1") {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let tag_id: Option<i64> = stmt
        .query_row([tag_name], |row| row.get(0))
        .optional()
        .map_err(|e| e.to_string())?;

    match tag_id {
        Some(id) => Ok(id), // tag already exists, return id
        None => {
            // Insert new tag if it doesn't exist
            conn.execute("INSERT INTO tags (name) VALUES (?1)", params![tag_name])
                .map_err(|err| format!("Failed to update transaction: {}", err))?;
            Ok(conn.last_insert_rowid()) // return newly inserted tag id
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_transactions() -> Result<Vec<Transaction>, String> {
    let conn = get_connection();

    let mut stmt = match conn
        .prepare("SELECT id, ledger_id, account_id, amount, currency, date, note FROM transactions")
    {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let transaction_iter = match stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            ledger_id: row.get(1)?,
            account_id: row.get(2)?,
            amount: row.get(3)?,
            currency: row.get(4)?,
            date: row.get(5)?,
            note: row.get(6)?,
            tags: get_tags_for_transaction(&conn, row.get(0)?).unwrap_or(vec![]),
        })
    }) {
        Ok(iterator) => iterator,
        Err(e) => return Err(format!("Failed to query accounts: {}", e)),
    };

    let mut transactions = Vec::new();
    for transaction in transaction_iter {
        match transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(e) => return Err(format!("Failed to parse account row: {}", e)),
        }
    }

    Ok(transactions)
}

fn get_tags_for_transaction(conn: &Connection, transaction_id: i64) -> Result<Vec<String>, String> {
    let mut stmt = match conn.prepare(
        "SELECT t.name 
        FROM tags t
        JOIN transaction_tags tt ON t.id = tt.tag_id
        WHERE tt.transaction_id = ?1",
    ) {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let tag_iter = match stmt.query_map([transaction_id], |row| row.get(0)) {
        Ok(iterator) => iterator,
        Err(e) => return Err(format!("Failed to query accounts: {}", e)),
    };
    let mut tags = Vec::new();
    for tag in tag_iter {
        match tag {
            Ok(tag) => tags.push(tag),
            Err(e) => return Err(format!("Failed to parse account row: {}", e)),
        }
    }

    Ok(tags)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_transaction(
    id: i64,
    ledger_id: i64,
    account_id: i64,
    amount: f64,
    currency: &str,
    date: &str,
    tags: Vec<String>,
    note: Option<&str>,
) -> Result<(), String> {
    let conn = get_connection();

    conn.execute(
        "UPDATE transactions SET ledger_id = ?1, account_id = ?2, amount = ?3, currency = ?4, date = ?5, note = ?6 WHERE id = ?7",
        params![ledger_id, account_id, amount, currency, date, note, id],
    ).map_err(|err| format!("Failed to update transaction: {}", err))?;

    conn.execute(
        "DELETE FROM transaction_tags WHERE transaction_id = ?1",
        params![id],
    )
    .map_err(|err| format!("Failed to clear existing tags: {}", err))?;

    for tag in tags {
        let tag_id = insert_or_get_tag(&conn, &tag)?;
        conn.execute(
            "INSERT INTO transaction_tags (transaction_id, tag_id) VALUES (?1, ?2)",
            params![id, tag_id],
        )
        .map_err(|err| format!("Failed to link tag to transaction: {}", err))?;
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_transaction(id: i64) -> Result<(), String> {
    let conn = get_connection();

    conn.execute(
        "DELETE FROM transaction_tags WHERE transaction_id = ?1",
        params![id],
    )
    .map_err(|err| format!("Failed to delete transaction tags: {}", err))?;

    conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])
        .map_err(|err| format!("Failed to delete transaction: {}", err))?;

    Ok(())
}
