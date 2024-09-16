pub mod backend {
    pub mod account;
    pub mod category;
    pub mod currency;
    pub mod db;
    pub mod ledger;
    pub mod tag;
    pub mod transaction;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            if let Err(e) = backend::db::init_db() {
                eprintln!("Error setting up the database: {}", e);
            }
            if let Err(e) = backend::currency::create_currencies_table() {
                eprintln!("Error creating currencies table: {}", e);
            }
            if let Err(e) = backend::account::create_accounts_table() {
                eprintln!("Error creating accounts table: {}", e);
            }
            if let Err(e) = backend::ledger::create_ledgers_table() {
                eprintln!("Error creating ledgers table: {}", e);
            }
            if let Err(e) = backend::transaction::create_transactions_table() {
                eprintln!("Error creating transactions table: {}", e);
            }
            if let Err(e) = backend::category::create_categories_table() {
                eprintln!("Error creating categories table: {}", e);
            }
            if let Err(e) = backend::tag::create_tags_table() {
                eprintln!("Error creating tag table: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            backend::account::create_account,
            backend::account::read_accounts,
            backend::account::delete_account,
            backend::account::update_account,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
