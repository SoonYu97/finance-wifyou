use crate::backend::db::get_connection;
use rusqlite::Result;

#[derive(serde::Serialize)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub amount: f64,
    pub transaction_type: String,
    pub date_time: String,
    pub note: Option<String>,
    pub tag: Option<String>,
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
