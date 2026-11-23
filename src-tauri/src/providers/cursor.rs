use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde_json::Value;

use super::{DetectedAccount, Provider, ProviderOps};

/// The credentials file used by Cursor in its globalStorage directory.
pub const CREDENTIALS_FILE: &str = "cursor-login.json";

/// The SQLite state database file stored in Cursor's globalStorage directory.
pub const SQLITE_STATE_FILE: &str = "state.vscdb";

/// Helper function that returns the platform-specific configuration directory for Cursor globalStorage.
///
/// - macOS: `~/Library/Application Support/Cursor/User/globalStorage/`
/// - Windows: `%APPDATA%/Cursor/User/globalStorage/`
/// - Linux: `~/.config/Cursor/User/globalStorage/`
pub fn get_config_dir() -> Option<PathBuf> {
    if cfg!(target_os = "macos") {
        dirs::home_dir().map(|h| {
            h.join("Library")
                .join("Application Support")
                .join("Cursor")
                .join("User")
                .join("globalStorage")
        })
    } else if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(dirs::config_dir)
            .map(|p| {
                p.join("Cursor")
                    .join("User")
                    .join("globalStorage")
            })
    } else if cfg!(target_os = "linux") {
        dirs::home_dir()
            .map(|h| {
                h.join(".config")
                    .join("Cursor")
                    .join("User")
                    .join("globalStorage")
            })
            .or_else(|| {
                dirs::config_dir().map(|p| {
                    p.join("Cursor")
                        .join("User")
                        .join("globalStorage")
                })
            })
    } else {
        dirs::config_dir().map(|p| {
            p.join("Cursor")
                .join("User")
                .join("globalStorage")
        })
    }
}

/// Helper function that returns the full path to `cursor-login.json` in globalStorage.
pub fn get_credentials_path() -> Option<PathBuf> {
    get_config_dir().map(|dir| dir.join(CREDENTIALS_FILE))
}

/// Cursor account provider.
///
/// Cursor stores credentials and state in a SQLite database (`state.vscdb`)
/// and JSON files (`cursor-login.json`) within its platform-specific `globalStorage` directory.
pub struct CursorProvider;

impl CursorProvider {
    /// Returns the platform-specific configuration directory for Cursor's globalStorage.
    pub fn get_config_dir() -> Option<PathBuf> {
        get_config_dir()
    }

    /// Returns the path to the credentials file (`cursor-login.json`).
    pub fn get_credentials_path() -> Option<PathBuf> {
        get_credentials_path()
    }

    /// Detects accounts from Cursor's globalStorage directory by inspecting `cursor-login.json`.
    /// Reads the email from it and creates a `DetectedAccount` with `Provider::Cursor`.
    /// Handles missing directory or file gracefully (returns empty `Vec`).
    pub fn detect_accounts() -> Result<Vec<DetectedAccount>> {
        match Self::get_config_dir() {
            Some(dir) => Self::detect_accounts_in(&dir),
            None => Ok(Vec::new()),
        }
    }

    /// Internal helper that detects accounts within a specific directory (useful for testing).
    pub fn detect_accounts_in(config_dir: &Path) -> Result<Vec<DetectedAccount>> {
        if !config_dir.exists() {
            return Ok(Vec::new());
        }

        let creds_path = config_dir.join(CREDENTIALS_FILE);
        if !creds_path.exists() {
            return Ok(Vec::new());
        }

        let content = match fs::read_to_string(&creds_path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => {
                return Err(e).with_context(|| {
                    format!("Failed to read Cursor credentials at {}", creds_path.display())
                })
            }
        };

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        // Extract email from cursor-login.json
        let email = serde_json::from_str::<Value>(&content)
            .ok()
            .and_then(|val| {
                val.get("email")
                    .or_else(|| val.get("userEmail"))
                    .or_else(|| val.get("user_email"))
                    .or_else(|| val.get("user").and_then(|u| u.get("email").or_else(|| u.get("name"))))
                    .or_else(|| val.get("account").and_then(|a| a.get("email").or_else(|| a.get("name"))))
                    .or_else(|| val.get("profile").and_then(|p| p.get("email").or_else(|| p.get("name"))))
                    .or_else(|| val.get("auth").and_then(|a| a.get("email").or_else(|| a.get("name"))))
                    .or_else(|| val.get("name"))
                    .or_else(|| val.get("account"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "default".to_string());

        let account = DetectedAccount {
            name: email,
            provider: Provider::Cursor,
            config_snapshot: content,
        };

        Ok(vec![account])
    }

    /// Switches the active Cursor account by atomically writing `config_data` to `cursor-login.json`.
    /// Uses the atomic write + rollback pattern (writes to temp file first, then renames; restores backup on failure).
    pub fn switch_to(config_data: &str) -> Result<()> {
        let config_dir = Self::get_config_dir()
            .context("Failed to determine Cursor configuration directory on this platform")?;
        Self::switch_to_in(&config_dir, config_data)
    }

    /// Internal helper that writes credentials atomically within a specific directory (useful for testing).
    pub fn switch_to_in(config_dir: &Path, config_data: &str) -> Result<()> {
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .with_context(|| format!("Failed to create directory: {}", config_dir.display()))?;
        }

        let target_path = config_dir.join(CREDENTIALS_FILE);
        let temp_path = config_dir.join(format!(
            ".{}.tmp.{}",
            CREDENTIALS_FILE,
            uuid::Uuid::new_v4()
        ));

        // Backup current file content for rollback if it exists
        let backup_data = if target_path.exists() {
            match fs::read_to_string(&target_path) {
                Ok(content) => Some(content),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(e) => return Err(e).with_context(|| "Failed to backup current credentials for rollback"),
            }
        } else {
            None
        };

        // Write to temporary file first, then atomically rename to target
        let write_op = (|| -> Result<()> {
            fs::write(&temp_path, config_data)
                .with_context(|| format!("Failed to write temporary credentials file: {}", temp_path.display()))?;
            fs::rename(&temp_path, &target_path)
                .with_context(|| format!("Failed to atomically rename {} to {}", temp_path.display(), target_path.display()))?;
            Ok(())
        })();

        if let Err(err) = write_op {
            // Clean up temporary file if it still exists
            if temp_path.exists() {
                let _ = fs::remove_file(&temp_path);
            }
            // Rollback: restore backup if target was removed or corrupted
            if let Some(backup_content) = backup_data {
                if !target_path.exists() {
                    let _ = fs::write(&target_path, backup_content);
                }
            }
            return Err(err);
        }

        Ok(())
    }

    /// Reads the current `cursor-login.json` in globalStorage.
    /// Returns the contents as String, or None if the file is not found.
    pub fn backup_current() -> Result<Option<String>> {
        match Self::get_config_dir() {
            Some(dir) => Self::backup_current_in(&dir),
            None => Ok(None),
        }
    }

    /// Internal helper that reads current credentials from a specific directory (useful for testing).
    pub fn backup_current_in(config_dir: &Path) -> Result<Option<String>> {
        let target_path = config_dir.join(CREDENTIALS_FILE);

        if !target_path.exists() {
            return Ok(None);
        }

        match fs::read_to_string(&target_path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e).with_context(|| format!("Failed to read {}", target_path.display())),
        }
    }
}

impl ProviderOps for CursorProvider {
    fn detect_accounts() -> Result<Vec<DetectedAccount>> {
        Self::detect_accounts()
    }

    fn switch_to(_account_id: &str, config_data: &str) -> Result<()> {
        Self::switch_to(config_data)
    }

    fn backup_current() -> Result<Option<String>> {
        Self::backup_current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_dir() {
        let dir = get_config_dir();
        assert!(dir.is_some());
        let path = dir.unwrap();
        assert_eq!(Some(path.clone()), CursorProvider::get_config_dir());

        #[cfg(target_os = "macos")]
        {
            let home = dirs::home_dir().expect("Home directory should exist");
            assert_eq!(
                path,
                home.join("Library/Application Support/Cursor/User/globalStorage")
            );
        }

        #[cfg(target_os = "windows")]
        {
            assert!(
                path.ends_with(Path::new("Cursor").join("User").join("globalStorage"))
            );
        }

        #[cfg(target_os = "linux")]
        {
            assert!(
                path.ends_with(Path::new(".config").join("Cursor").join("User").join("globalStorage"))
                    || path.ends_with(Path::new("Cursor").join("User").join("globalStorage"))
            );
        }
    }

    #[test]
    fn test_detect_accounts_nonexistent_dir() {
        let temp_dir = std::env::temp_dir().join(format!("test_cursor_none_{}", uuid::Uuid::new_v4()));
        let res = CursorProvider::detect_accounts_in(&temp_dir).unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_detect_accounts_with_email() {
        let temp_dir = std::env::temp_dir().join(format!("test_cursor_email_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join(CREDENTIALS_FILE);
        let mock_json = r#"{"email": "developer@example.com", "accessToken": "tok-123"}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = CursorProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "developer@example.com");
        assert_eq!(accounts[0].provider, Provider::Cursor);
        assert_eq!(accounts[0].config_snapshot, mock_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_with_nested_email() {
        let temp_dir = std::env::temp_dir().join(format!("test_cursor_nested_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join(CREDENTIALS_FILE);
        let mock_json = r#"{"user": {"email": "nested@example.com"}, "session": "s-456"}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = CursorProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "nested@example.com");
        assert_eq!(accounts[0].provider, Provider::Cursor);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_switch_to_and_backup() {
        let temp_dir = std::env::temp_dir().join(format!("test_cursor_switch_{}", uuid::Uuid::new_v4()));
        let initial_json = r#"{"email": "init@example.com", "token": "initial"}"#;
        let new_json = r#"{"email": "switched@example.com", "token": "switched"}"#;

        // Switch to initial
        CursorProvider::switch_to_in(&temp_dir, initial_json).unwrap();
        let backup = CursorProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, Some(initial_json.to_string()));

        // Switch to new
        CursorProvider::switch_to_in(&temp_dir, new_json).unwrap();
        let target_path = temp_dir.join(CREDENTIALS_FILE);
        let current_content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(current_content, new_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_backup_current_not_found() {
        let temp_dir = std::env::temp_dir().join(format!("test_cursor_notfound_{}", uuid::Uuid::new_v4()));
        let backup = CursorProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, None);
    }

    #[test]
    fn test_provider_ops_implementation() {
        let res = <CursorProvider as ProviderOps>::detect_accounts();
        assert!(res.is_ok());
    }
}
