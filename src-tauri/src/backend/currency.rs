use crate::backend::db::get_connection;
use rusqlite::Result;

#[derive(serde::Serialize)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

// Function to create the currency table
pub fn create_currencies_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS currencies (
        code TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        symbol TEXT NOT NULL
    );";

    conn.execute(create_table_sql, [])?;

    insert_into_currencies_table(conn)?;
    Ok(())
}

fn insert_into_currencies_table(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM currencies",
        [],
        |row| row.get(0),
    )?;

    if count == 0 {
        let insert_table_sql = "
        INSERT OR IGNORE INTO currencies (code, name, symbol)
        VALUES
            ('USD', 'United States Dollar', '$'),
            ('EUR', 'Euro', '€'),
            ('GBP', 'British Pound', '£'),
            ('MYR', 'Malaysian Ringgit', 'RM'),
            ('JPY', 'Japanese Yen', '¥'),
            ('AUD', 'Australian Dollar', 'A$'),
            ('CAD', 'Canadian Dollar', 'C$'),
            ('SGD', 'Singapore Dollar', 'S$'),
            ('CNY', 'Chinese Yuan', '¥'),
            ('INR', 'Indian Rupee', '₹');
        ";
        conn.execute(insert_table_sql, [])?;
    }
    Ok(())
}

// Add more currency-related functions here as needed
