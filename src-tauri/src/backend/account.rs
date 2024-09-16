use crate::backend::db::get_connection;
use rusqlite::{params, Connection, Result};

#[derive(serde::Serialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub account_type: String,
    pub balance: f64,
    pub currency: String,
    pub note: Option<String>,
    pub count_in_asset: bool,
    pub credit_limit: Option<f64>,
    pub owed: Option<f64>,
    pub billing_date: Option<String>,
    pub due_date: Option<String>,
    pub avg_cost: Option<f64>,
    pub quantity: Option<f64>,
    pub total_cap: Option<f64>,
}

// Function to create the accounts table
pub fn create_accounts_table() -> Result<()> {
    let conn = get_connection();

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        type TEXT CHECK( type IN ('debit', 'credit', 'invest', 'member') ),
        balance DECIMAL(10, 4) DEFAULT 0,
        note TEXT,
        currency TEXT NOT NULL,
        FOREIGN KEY (currency) REFERENCES currencies(code) ON DELETE RESTRICT
    );";

    conn.execute(create_table_sql, [])?;

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS credit_accounts (
        account_id INTEGER PRIMARY KEY,
        credit_limit DECIMAL(10, 4) NOT NULL,
        owed DECIMAL(10, 4) DEFAULT 0,
        billing_date TEXT NOT NULL,
        due_date TEXT NOT NULL,
        FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
    );";

    conn.execute(create_table_sql, [])?;

    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS invest_accounts (
        account_id INTEGER PRIMARY KEY,
        average_cost DECIMAL(10, 4),
        quantity DECIMAL(10, 4),
        total_cap DECIMAL(10, 4),
        FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
    );";

    conn.execute(create_table_sql, [])?;
    Ok(())
}

#[tauri::command]
pub fn read_accounts() -> Result<Vec<Account>, String> {
    let conn = get_connection();

    let mut stmt = match conn.prepare(
        "SELECT a.id, a.name, a.type, a.balance, a.currency, a.note, a.count_in_asset,
            c.credit_limit, c.owed, c.billing_date, c.due_date, 
            i.avg_cost, i.quantity, i.total_cap
        FROM accounts a
        LEFT JOIN credit_accounts c ON a.id = c.account_id
        LEFT JOIN invest_accounts i ON a.id = i.account_id",
    ) {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let account_iter = match stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            account_type: row.get(2)?,
            balance: row.get(3)?,
            currency: row.get(4)?,
            note: row.get(5)?,
            count_in_asset: row.get(6)?,
            credit_limit: row.get(7).ok(),
            owed: row.get(8).ok(),
            billing_date: row.get(9).ok(),
            due_date: row.get(10).ok(),
            avg_cost: row.get(11).ok(),
            quantity: row.get(12).ok(),
            total_cap: row.get(13).ok(),
        })
    }) {
        Ok(iterator) => iterator,
        Err(e) => return Err(format!("Failed to query accounts: {}", e)),
    };

    let mut accounts = Vec::new();
    for account in account_iter {
        match account {
            Ok(acc) => accounts.push(acc),
            Err(e) => return Err(format!("Failed to parse account row: {}", e)),
        }
    }

    Ok(accounts)
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_account(
    name: &str,
    account_type: &str,
    balance: f64,
    currency: &str,
    note: Option<&str>,
    credit_limit: Option<f64>,
    owed: Option<f64>,
    billing_date: Option<&str>,
    due_date: Option<&str>,
    avg_cost: Option<f64>,
    quantity: Option<f64>,
    total_cap: Option<f64>,
) -> Result<(), String> {
    let conn = get_connection();

    if account_type == "credit" {
        if let (Some(credit_limit), Some(owed), Some(billing_date), Some(due_date)) =
            (credit_limit, owed, billing_date, due_date)
        {
            create_general_account(&conn, name, account_type, balance, currency, note)
                .map_err(|err| format!("Failed to insert account: {}", err))?;

            let account_id = conn.last_insert_rowid();

            create_credit_account(
                &conn,
                account_id,
                credit_limit,
                owed,
                billing_date,
                due_date,
            )
            .map_err(|err| format!("Failed to insert credit account details: {}", err))?;
        } else {
            return Err(
                "Missing credit account details: credit_limit, owed, billing_date, or due_date"
                    .to_string(),
            );
        }
    } else if account_type == "invest" {
        if let (Some(avg_cost), Some(quantity), Some(total_cap)) = (avg_cost, quantity, total_cap) {
            create_general_account(&conn, name, account_type, balance, currency, note)
                .map_err(|err| format!("Failed to insert account: {}", err))?;

            let account_id = conn.last_insert_rowid();

            insert_invest_account(&conn, account_id, avg_cost, quantity, total_cap)
                .map_err(|err| format!("Failed to insert invest account: {}", err))?;
        } else {
            return Err(
                "Missing invest account details: avg_cost, quantity, or total_cap".to_string(),
            );
        }
    } else {
        create_general_account(&conn, name, account_type, balance, currency, note)
            .map_err(|err| format!("Failed to insert account: {}", err))?;
    }

    Ok(())
}

// Insert the account into the general accounts table
fn create_general_account(
    conn: &Connection,
    name: &str,
    acc_type: &str,
    balance: f64,
    currency: &str,
    note: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO accounts (name, type, balance, currency, note) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, acc_type, balance, currency, note],
    )?;
    Ok(())
}

// Insert into the credit_accounts table
fn create_credit_account(
    conn: &Connection,
    account_id: i64,
    credit_limit: f64,
    owed: f64,
    billing_date: &str,
    due_date: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO credit_accounts (account_id, credit_limit, owed, billing_date, due_date) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![account_id, credit_limit, owed, billing_date, due_date],
    )?;
    Ok(())
}

fn insert_invest_account(
    conn: &Connection,
    account_id: i64,
    avg_cost: f64,
    quantity: f64,
    total_cap: f64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO invest_accounts (account_id, avg_cost, quantity, total_cap) VALUES (?1, ?2, ?3, ?4)",
        params![account_id, avg_cost, quantity, total_cap],
    )?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_account(account_id: i64) -> Result<(), String> {
    let conn = get_connection();

    if let Err(e) = conn.execute(
        "DELETE FROM credit_accounts WHERE account_id = ?1",
        params![account_id],
    ) {
        return Err(format!("Failed to delete from credit accounts: {}", e));
    }

    match conn.execute("DELETE FROM accounts WHERE id = ?1", params![account_id]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete account: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_account(
    account_id: i64,
    name: &str,
    account_type: &str,
    balance: f64,
    currency: &str,
    note: Option<&str>,
    credit_limit: Option<f64>, // Optional fields for credit accounts
    owed: Option<f64>,
    billing_date: Option<&str>,
    due_date: Option<&str>,
    avg_cost: Option<f64>, // Optional fields for invest accounts
    quantity: Option<f64>,
    total_cap: Option<f64>,
) -> Result<(), String> {
    let conn = get_connection();

    // First, update the general accounts table for all types of accounts
    update_general_account(
        &conn,
        account_id,
        name,
        account_type,
        balance,
        currency,
        note,
    )
    .map_err(|err| format!("Failed to update account: {}", err))?;

    // Update credit-specific fields if it's a credit account
    if account_type == "credit" {
        if let (Some(credit_limit), Some(owed), Some(billing_date), Some(due_date)) =
            (credit_limit, owed, billing_date, due_date)
        {
            update_credit_account(
                &conn,
                account_id,
                credit_limit,
                owed,
                billing_date,
                due_date,
            )
            .map_err(|err| format!("Failed to update credit account details: {}", err))?;
        } else {
            return Err(
                "Missing credit account details: credit_limit, owed, billing_date, or due_date"
                    .to_string(),
            );
        }
    }

    // Update invest-specific fields if it's an invest account
    if account_type == "invest" {
        if let (Some(avg_cost), Some(quantity), Some(total_cap)) = (avg_cost, quantity, total_cap) {
            update_invest_account(&conn, account_id, avg_cost, quantity, total_cap)
                .map_err(|err| format!("Failed to update invest account details: {}", err))?;
        } else {
            return Err(
                "Missing invest account details: avg_cost, quantity, or total_cap".to_string(),
            );
        }
    }

    // For debit and member accounts, no additional table updates are needed

    Ok(())
}

// Update the general accounts table
fn update_general_account(
    conn: &Connection,
    account_id: i64,
    name: &str,
    acc_type: &str,
    balance: f64,
    currency: &str,
    note: Option<&str>,
) -> Result<()> {
    conn.execute(
        "UPDATE accounts SET name = ?1, type = ?2, balance = ?3, currency = ?4, note = ?5 WHERE id = ?6",
        params![name, acc_type, balance, currency, note, account_id],
    )?;
    Ok(())
}

// Update the credit-specific fields in the credit_accounts table
fn update_credit_account(
    conn: &Connection,
    account_id: i64,
    credit_limit: f64,
    owed: f64,
    billing_date: &str,
    due_date: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE credit_accounts SET credit_limit = ?1, owed = ?2, billing_date = ?3, due_date = ?4 WHERE account_id = ?5",
        params![credit_limit, owed, billing_date, due_date, account_id],
    )?;
    Ok(())
}

// Update the invest-specific fields in the invest_accounts table
fn update_invest_account(
    conn: &Connection,
    account_id: i64,
    avg_cost: f64,
    quantity: f64,
    total_cap: f64,
) -> Result<()> {
    conn.execute(
        "UPDATE invest_accounts SET avg_cost = ?1, quantity = ?2, total_cap = ?3 WHERE account_id = ?4",
        params![avg_cost, quantity, total_cap, account_id],
    )?;
    Ok(())
}
