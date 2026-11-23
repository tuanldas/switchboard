use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde_json::Value;

use super::{DetectedAccount, Provider, ProviderOps};

pub struct ClaudeProvider;

impl ClaudeProvider {
    /// Returns the configuration directory for Claude Code: ~/.claude
    pub fn get_config_dir() -> PathBuf {
        dirs::home_dir()
            .map(|home| home.join(".claude"))
            .unwrap_or_else(|| PathBuf::from(".claude"))
    }

    /// Detects accounts from ~/.claude by inspecting credential files.
    /// Checks ~/.claude/.credentials.json or ~/.claude/credentials.json.
    /// If found, creates a DetectedAccount with the credential data as config_snapshot.
    /// Handles missing directory or files gracefully (returns empty Vec).
    pub fn detect_accounts() -> Result<Vec<DetectedAccount>> {
        Self::detect_accounts_in(&Self::get_config_dir())
    }

    /// Internal helper that detects accounts within a specific directory (useful for testing).
    pub fn detect_accounts_in(config_dir: &Path) -> Result<Vec<DetectedAccount>> {
        if !config_dir.exists() {
            return Ok(Vec::new());
        }

        let dot_creds = config_dir.join(".credentials.json");
        let plain_creds = config_dir.join("credentials.json");

        let creds_path = if dot_creds.exists() {
            Some(dot_creds)
        } else if plain_creds.exists() {
            Some(plain_creds)
        } else {
            None
        };

        let Some(path) = creds_path else {
            return Ok(Vec::new());
        };

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e).with_context(|| format!("Failed to read Claude credentials at {}", path.display())),
        };

        // Extract a descriptive account name from the JSON if available, defaulting to "default"
        let name = serde_json::from_str::<Value>(&content)
            .ok()
            .and_then(|val| {
                val.get("name")
                    .or_else(|| val.get("email"))
                    .or_else(|| val.get("account").and_then(|a| a.get("email").or_else(|| a.get("name"))))
                    .or_else(|| val.get("user").and_then(|u| u.get("email").or_else(|| u.get("name"))))
                    .or_else(|| {
                        val.get("oauth").and_then(|o| {
                            o.get("email")
                                .or_else(|| o.get("account").and_then(|a| a.get("email").or_else(|| a.get("name"))))
                        })
                    })
                    .or_else(|| val.get("account_id"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "default".to_string());

        let account = DetectedAccount {
            name,
            provider: Provider::Claude,
            config_snapshot: content,
        };

        Ok(vec![account])
    }

    /// Switches the active Claude Code account by atomically writing config_data to ~/.claude/.credentials.json.
    /// Uses the atomic write + rollback pattern (writes to temp file first, then renames; restores backup on failure).
    pub fn switch_to(config_data: &str) -> Result<()> {
        Self::switch_to_in(&Self::get_config_dir(), config_data)
    }

    /// Internal helper that writes credentials atomically within a specific directory (useful for testing).
    pub fn switch_to_in(config_dir: &Path, config_data: &str) -> Result<()> {
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .with_context(|| format!("Failed to create directory: {}", config_dir.display()))?;
        }

        let target_path = config_dir.join(".credentials.json");
        let temp_path = config_dir.join(format!(".credentials.json.tmp.{}", uuid::Uuid::new_v4()));

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

    /// Reads the current ~/.claude/.credentials.json.
    /// Returns the contents as String, or None if the file is not found.
    pub fn backup_current() -> Result<Option<String>> {
        Self::backup_current_in(&Self::get_config_dir())
    }

    /// Internal helper that reads current credentials from a specific directory (useful for testing).
    pub fn backup_current_in(config_dir: &Path) -> Result<Option<String>> {
        let target_path = config_dir.join(".credentials.json");

        if !target_path.exists() {
            let alt_path = config_dir.join("credentials.json");
            if alt_path.exists() {
                match fs::read_to_string(&alt_path) {
                    Ok(content) => return Ok(Some(content)),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                    Err(e) => return Err(e).with_context(|| format!("Failed to read {}", alt_path.display())),
                }
            }
            return Ok(None);
        }

        match fs::read_to_string(&target_path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e).with_context(|| format!("Failed to read {}", target_path.display())),
        }
    }
}

impl ProviderOps for ClaudeProvider {
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
        let dir = ClaudeProvider::get_config_dir();
        assert!(dir.ends_with(".claude"));
    }

    #[test]
    fn test_detect_accounts_nonexistent_dir() {
        let temp_dir = std::env::temp_dir().join(format!("test_claude_none_{}", uuid::Uuid::new_v4()));
        let res = ClaudeProvider::detect_accounts_in(&temp_dir).unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_detect_accounts_dot_credentials() {
        let temp_dir = std::env::temp_dir().join(format!("test_claude_dot_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join(".credentials.json");
        let mock_json = r#"{"name": "test-user", "sessionKey": "sk-123"}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = ClaudeProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "test-user");
        assert_eq!(accounts[0].config_snapshot, mock_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_plain_credentials() {
        let temp_dir = std::env::temp_dir().join(format!("test_claude_plain_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join("credentials.json");
        let mock_json = r#"{"email": "user@example.com", "token": "abc"}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = ClaudeProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "user@example.com");
        assert_eq!(accounts[0].config_snapshot, mock_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_switch_to_and_backup() {
        let temp_dir = std::env::temp_dir().join(format!("test_claude_switch_{}", uuid::Uuid::new_v4()));
        let initial_json = r#"{"token": "initial"}"#;
        let new_json = r#"{"token": "switched"}"#;

        // Switch to initial
        ClaudeProvider::switch_to_in(&temp_dir, initial_json).unwrap();
        let backup = ClaudeProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, Some(initial_json.to_string()));

        // Switch to new
        ClaudeProvider::switch_to_in(&temp_dir, new_json).unwrap();
        let target_path = temp_dir.join(".credentials.json");
        let current_content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(current_content, new_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_backup_current_not_found() {
        let temp_dir = std::env::temp_dir().join(format!("test_claude_notfound_{}", uuid::Uuid::new_v4()));
        let backup = ClaudeProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, None);
    }
}
