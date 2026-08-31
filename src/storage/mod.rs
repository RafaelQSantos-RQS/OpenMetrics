pub mod models;
pub mod queries;

use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub type SharedDb = Arc<Mutex<Connection>>;

pub fn initialize(db_path: &str) -> Result<SharedDb, Box<dyn std::error::Error>> {
    // Ensure parent directory exists
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrent read performance
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;

    // Create tables
    models::create_tables(&conn)?;

    tracing::info!("Database initialized at {}", db_path);

    Ok(Arc::new(Mutex::new(conn)))
}
