use std::path::PathBuf;
use rusqlite::{Connection, params};
use anyhow::{Context, Result};

/// Returns the path to the database: ~/.statusbar/data.db
pub fn db_path() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".statusbar").join("data.db"))
        .unwrap_or_else(|| PathBuf::from(".statusbar").join("data.db"))
}

/// Static DB_PATH function returning the database path (~/.statusbar/data.db)
#[allow(non_snake_case)]
pub fn DB_PATH() -> PathBuf {
    db_path()
}

pub fn get_db_path() -> PathBuf {
    db_path()
}

/// Opens and returns a connection to the SQLite database.
/// Ensures the ~/.statusbar directory exists before opening.
pub fn get_connection() -> Result<Connection> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }
    let conn = Connection::open(&path)
        .with_context(|| format!("Failed to open SQLite database at {}", path.display()))?;
    conn.execute("PRAGMA foreign_keys = ON;", params![])
        .context("Failed to enable foreign keys")?;
    Ok(conn)
}

/// Initializes the database for the Status Bar app:
/// - Creates ~/.statusbar directory if not exists
/// - Opens/creates SQLite connection
/// - Creates tables using CREATE TABLE IF NOT EXISTS:
///   a. accounts
///   b. usage_records
///   c. settings
pub fn init() -> Result<()> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }

    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            provider TEXT NOT NULL,
            name TEXT NOT NULL,
            label TEXT,
            config_data TEXT,
            is_active INTEGER DEFAULT 0,
            created_at TEXT,
            updated_at TEXT
        );",
        params![],
    )
    .context("Failed to create accounts table")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS usage_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id TEXT,
            provider TEXT,
            model TEXT,
            input_tokens INTEGER DEFAULT 0,
            output_tokens INTEGER DEFAULT 0,
            cache_read_tokens INTEGER DEFAULT 0,
            cache_write_tokens INTEGER DEFAULT 0,
            cost_usd REAL DEFAULT 0.0,
            session_id TEXT,
            project TEXT,
            timestamp TEXT,
            FOREIGN KEY(account_id) REFERENCES accounts(id)
        );",
        params![],
    )
    .context("Failed to create usage_records table")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );",
        params![],
    )
    .context("Failed to create settings table")?;

    Ok(())
}

/// Optional struct wrapper providing static method access
pub struct DB;

impl DB {
    #[allow(non_snake_case)]
    pub fn DB_PATH() -> PathBuf {
        db_path()
    }

    pub fn db_path() -> PathBuf {
        db_path()
    }

    pub fn init() -> Result<()> {
        init()
    }

    pub fn get_connection() -> Result<Connection> {
        get_connection()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_path() {
        let path = db_path();
        assert!(path.ends_with(".statusbar/data.db"));
    }

    #[test]
    fn test_db_path_static_fn() {
        assert_eq!(db_path(), DB_PATH());
        assert_eq!(db_path(), DB::DB_PATH());
    }

    #[test]
    fn test_schema_creation_in_memory() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON;", params![])?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                provider TEXT NOT NULL,
                name TEXT NOT NULL,
                label TEXT,
                config_data TEXT,
                is_active INTEGER DEFAULT 0,
                created_at TEXT,
                updated_at TEXT
            );",
            params![],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS usage_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id TEXT,
                provider TEXT,
                model TEXT,
                input_tokens INTEGER DEFAULT 0,
                output_tokens INTEGER DEFAULT 0,
                cache_read_tokens INTEGER DEFAULT 0,
                cache_write_tokens INTEGER DEFAULT 0,
                cost_usd REAL DEFAULT 0.0,
                session_id TEXT,
                project TEXT,
                timestamp TEXT,
                FOREIGN KEY(account_id) REFERENCES accounts(id)
            );",
            params![],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT
            );",
            params![],
        )?;

        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;")?;
        let tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<std::result::Result<_, _>>()?;

        assert!(tables.contains(&"accounts".to_string()));
        assert!(tables.contains(&"usage_records".to_string()));
        assert!(tables.contains(&"settings".to_string()));

        Ok(())
    }
}
