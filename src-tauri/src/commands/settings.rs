use super::super::db;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json;

/// Application settings data structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,           // "system", "dark", "light"
    pub auto_start: bool,
    pub auto_switch_on_quota: bool,
    pub language: String,        // "en", "vi"
    pub budget_limit: Option<f64>,
    pub notifications_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            auto_start: true,
            auto_switch_on_quota: false,
            language: "en".to_string(),
            budget_limit: None,
            notifications_enabled: true,
        }
    }
}

/// Helper function to load settings from a given SQLite connection.
pub fn get_settings_from_conn(conn: &rusqlite::Connection) -> Result<AppSettings, String> {
    // Ensure the settings table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );",
        params![],
    )
    .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT key, value FROM settings;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![], |row| {
            let key: String = row.get(0)?;
            let value: Option<String> = row.get(1)?;
            Ok((key, value))
        })
        .map_err(|e| e.to_string())?;

    let mut settings = AppSettings::default();

    for row in rows {
        let (key, value) = row.map_err(|e| e.to_string())?;
        if let Some(val) = value {
            match key.as_str() {
                "theme" => {
                    settings.theme = serde_json::from_str::<String>(&val).unwrap_or(val);
                }
                "auto_start" => {
                    settings.auto_start = serde_json::from_str::<bool>(&val).unwrap_or_else(|_| {
                        match val.trim().to_lowercase().as_str() {
                            "1" | "true" => true,
                            "0" | "false" => false,
                            _ => settings.auto_start,
                        }
                    });
                }
                "auto_switch_on_quota" => {
                    settings.auto_switch_on_quota =
                        serde_json::from_str::<bool>(&val).unwrap_or_else(|_| {
                            match val.trim().to_lowercase().as_str() {
                                "1" | "true" => true,
                                "0" | "false" => false,
                                _ => settings.auto_switch_on_quota,
                            }
                        });
                }
                "language" => {
                    settings.language = serde_json::from_str::<String>(&val).unwrap_or(val);
                }
                "budget_limit" => {
                    let trimmed = val.trim();
                    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") {
                        settings.budget_limit = None;
                    } else {
                        settings.budget_limit = serde_json::from_str::<Option<f64>>(trimmed)
                            .ok()
                            .flatten()
                            .or_else(|| trimmed.parse::<f64>().ok());
                    }
                }
                "notifications_enabled" => {
                    settings.notifications_enabled =
                        serde_json::from_str::<bool>(&val).unwrap_or_else(|_| {
                            match val.trim().to_lowercase().as_str() {
                                "1" | "true" => true,
                                "0" | "false" => false,
                                _ => settings.notifications_enabled,
                            }
                        });
                }
                _ => {}
            }
        }
    }

    Ok(settings)
}

/// Helper function to update settings using a given SQLite connection.
pub fn update_settings_with_conn(
    conn: &mut rusqlite::Connection,
    settings: &AppSettings,
) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );",
        params![],
    )
    .map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let budget_limit_val = match settings.budget_limit {
        Some(limit) => limit.to_string(),
        None => "null".to_string(),
    };

    let records: [(&str, String); 6] = [
        ("theme", settings.theme.clone()),
        ("auto_start", settings.auto_start.to_string()),
        ("auto_switch_on_quota", settings.auto_switch_on_quota.to_string()),
        ("language", settings.language.clone()),
        ("budget_limit", budget_limit_val),
        (
            "notifications_enabled",
            settings.notifications_enabled.to_string(),
        ),
    ];

    for (key, val) in &records {
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2);",
            params![key, val],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}

/// Reads the application settings from the SQLite settings table.
/// Returns defaults if not found.
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    get_settings_from_conn(&conn)
}

/// Writes each field of the application settings to the SQLite settings table using INSERT OR REPLACE.
#[tauri::command]
pub async fn update_settings(settings: AppSettings) -> Result<(), String> {
    let mut conn = db::get_connection().map_err(|e| e.to_string())?;
    update_settings_with_conn(&mut conn, &settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_default_settings() {
        let defaults = AppSettings::default();
        assert_eq!(defaults.theme, "system");
        assert_eq!(defaults.auto_start, true);
        assert_eq!(defaults.auto_switch_on_quota, false);
        assert_eq!(defaults.language, "en");
        assert_eq!(defaults.budget_limit, None);
        assert_eq!(defaults.notifications_enabled, true);
    }

    #[test]
    fn test_serde_serialization() {
        let settings = AppSettings {
            theme: "dark".to_string(),
            auto_start: false,
            auto_switch_on_quota: true,
            language: "vi".to_string(),
            budget_limit: Some(49.99),
            notifications_enabled: false,
        };

        let json = serde_json::to_string(&settings).expect("serialization failed");
        let deserialized: AppSettings =
            serde_json::from_str(&json).expect("deserialization failed");

        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_get_settings_defaults_when_empty() {
        let conn = Connection::open_in_memory().unwrap();
        let loaded = get_settings_from_conn(&conn).expect("get_settings_from_conn failed");
        assert_eq!(loaded, AppSettings::default());
    }

    #[test]
    fn test_update_and_get_settings() {
        let mut conn = Connection::open_in_memory().unwrap();
        let custom = AppSettings {
            theme: "light".to_string(),
            auto_start: false,
            auto_switch_on_quota: true,
            language: "vi".to_string(),
            budget_limit: Some(120.5),
            notifications_enabled: false,
        };

        update_settings_with_conn(&mut conn, &custom).expect("update failed");
        let loaded = get_settings_from_conn(&conn).expect("get failed");
        assert_eq!(loaded, custom);
    }

    #[test]
    fn test_partial_settings_and_defaults() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);",
            params![],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('theme', 'dark');",
            params![],
        )
        .unwrap();

        let loaded = get_settings_from_conn(&conn).unwrap();
        assert_eq!(loaded.theme, "dark");
        assert_eq!(loaded.language, "en"); // default
        assert_eq!(loaded.auto_start, true); // default
        assert_eq!(loaded.auto_switch_on_quota, false); // default
        assert_eq!(loaded.budget_limit, None); // default
        assert_eq!(loaded.notifications_enabled, true); // default
    }

    #[test]
    fn test_json_quoted_strings_compatibility() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);",
            params![],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('theme', '\"dark\"');",
            params![],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('language', '\"vi\"');",
            params![],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('budget_limit', 'null');",
            params![],
        )
        .unwrap();

        let loaded = get_settings_from_conn(&conn).unwrap();
        assert_eq!(loaded.theme, "dark");
        assert_eq!(loaded.language, "vi");
        assert_eq!(loaded.budget_limit, None);
    }
}
