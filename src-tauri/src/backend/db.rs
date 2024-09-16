use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use std::sync::Once;

const DB_PATH: &str = ".data/finance.db";
const DIR_PATH: &str = ".data";
static INIT: Once = Once::new();
static mut DB_CONNECTION: Option<Connection> = None;

pub fn init_db() -> Result<()> {
    INIT.call_once(|| unsafe {
        if !Path::new(DIR_PATH).exists() {
            match fs::create_dir(DIR_PATH) {
                Ok(_) => println!("Directory {} created", DIR_PATH),
                Err(e) => println!("Failed to create directory: {}", e),
            }
        }
        if !Path::new(DB_PATH).exists() {
            match fs::File::create(DB_PATH) {
                Ok(_) => println!("File {} created", DB_PATH),
                Err(e) => println!("Failed to create file: {}", e),
            }
        } else {
            println!("File {} already exists", DB_PATH);
        }
        DB_CONNECTION = Some(Connection::open(DB_PATH).unwrap());
    });
    Ok(())
}

pub fn get_connection() -> &'static Connection {
    unsafe {
        DB_CONNECTION
            .as_ref()
            .expect("Database connection not initialized")
    }
}
