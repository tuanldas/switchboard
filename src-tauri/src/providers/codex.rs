use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde_json::Value;

use super::{DetectedAccount, Provider, ProviderOps};

pub struct CodexProvider;

impl CodexProvider {
    /// Returns the configuration directory for OpenAI Codex CLI: ~/.codex
    pub fn get_config_dir() -> PathBuf {
        dirs::home_dir()
            .map(|home| home.join(".codex"))
            .unwrap_or_else(|| PathBuf::from(".codex"))
    }

    /// Alias for get_config_dir()
    pub fn config_dir() -> PathBuf {
        Self::get_config_dir()
    }

    /// Returns the credentials file path for OpenAI Codex CLI: ~/.codex/auth.json
    pub fn get_credentials_path() -> PathBuf {
        Self::get_config_dir().join("auth.json")
    }

    /// Alias for get_credentials_path()
    pub fn credentials_path() -> PathBuf {
        Self::get_credentials_path()
    }

    /// Detects accounts from ~/.codex by inspecting credential files.
    /// Checks ~/.codex/auth.json (or ~/.codex/.auth.json).
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

        let plain_auth = config_dir.join("auth.json");
        let dot_auth = config_dir.join(".auth.json");

        let auth_path = if plain_auth.exists() {
            Some(plain_auth)
        } else if dot_auth.exists() {
            Some(dot_auth)
        } else {
            None
        };

        let Some(path) = auth_path else {
            return Ok(Vec::new());
        };

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e).with_context(|| format!("Failed to read Codex credentials at {}", path.display())),
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
                        val.get("tokens").and_then(|t| {
                            t.get("email")
                                .or_else(|| t.get("account_id"))
                                .or_else(|| t.get("user_id"))
                        })
                    })
                    .or_else(|| {
                        val.get("oauth").and_then(|o| {
                            o.get("email")
                                .or_else(|| o.get("account").and_then(|a| a.get("email").or_else(|| a.get("name"))))
                        })
                    })
                    .or_else(|| val.get("account_id"))
                    .or_else(|| val.get("user_id"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "default".to_string());

        let account = DetectedAccount {
            name,
            provider: Provider::Codex,
            config_snapshot: content,
        };

        Ok(vec![account])
    }

    /// Switches the active Codex account by atomically writing config_data to ~/.codex/auth.json.
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

        let target_path = config_dir.join("auth.json");
        let temp_path = config_dir.join(format!(".auth.json.tmp.{}", uuid::Uuid::new_v4()));

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

    /// Reads the current ~/.codex/auth.json.
    /// Returns the contents as String, or None if the file is not found.
    pub fn backup_current() -> Result<Option<String>> {
        Self::backup_current_in(&Self::get_config_dir())
    }

    /// Internal helper that reads current credentials from a specific directory (useful for testing).
    pub fn backup_current_in(config_dir: &Path) -> Result<Option<String>> {
        let target_path = config_dir.join("auth.json");

        if !target_path.exists() {
            let alt_path = config_dir.join(".auth.json");
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

impl ProviderOps for CodexProvider {
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
        let dir = CodexProvider::get_config_dir();
        assert!(dir.ends_with(".codex"));
        assert_eq!(dir, CodexProvider::config_dir());
    }

    #[test]
    fn test_get_credentials_path() {
        let creds_path = CodexProvider::get_credentials_path();
        assert!(creds_path.ends_with(".codex/auth.json") || creds_path.ends_with(".codex\\auth.json"));
        assert_eq!(creds_path, CodexProvider::credentials_path());
    }

    #[test]
    fn test_detect_accounts_nonexistent_dir() {
        let temp_dir = std::env::temp_dir().join(format!("test_codex_none_{}", uuid::Uuid::new_v4()));
        let res = CodexProvider::detect_accounts_in(&temp_dir).unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_detect_accounts_auth_json() {
        let temp_dir = std::env::temp_dir().join(format!("test_codex_auth_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join("auth.json");
        let mock_json = r#"{"name": "codex-user", "tokens": {"access_token": "tok-123"}}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = CodexProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "codex-user");
        assert_eq!(accounts[0].provider, Provider::Codex);
        assert_eq!(accounts[0].config_snapshot, mock_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_with_email() {
        let temp_dir = std::env::temp_dir().join(format!("test_codex_email_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join("auth.json");
        let mock_json = r#"{"email": "developer@openai.com", "token": "abc"}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = CodexProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "developer@openai.com");
        assert_eq!(accounts[0].provider, Provider::Codex);
        assert_eq!(accounts[0].config_snapshot, mock_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_switch_to_and_backup() {
        let temp_dir = std::env::temp_dir().join(format!("test_codex_switch_{}", uuid::Uuid::new_v4()));
        let initial_json = r#"{"token": "initial"}"#;
        let new_json = r#"{"token": "switched"}"#;

        // Switch to initial
        CodexProvider::switch_to_in(&temp_dir, initial_json).unwrap();
        let backup = CodexProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, Some(initial_json.to_string()));

        // Switch to new
        CodexProvider::switch_to_in(&temp_dir, new_json).unwrap();
        let target_path = temp_dir.join("auth.json");
        let current_content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(current_content, new_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_backup_current_not_found() {
        let temp_dir = std::env::temp_dir().join(format!("test_codex_notfound_{}", uuid::Uuid::new_v4()));
        let backup = CodexProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, None);
    }

    #[test]
    fn test_provider_ops() {
        let temp_dir = std::env::temp_dir().join(format!("test_codex_ops_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();

        let initial_json = r#"{"email": "ops@openai.com"}"#;
        CodexProvider::switch_to_in(&temp_dir, initial_json).unwrap();

        let accounts = CodexProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "ops@openai.com");

        let backup = CodexProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, Some(initial_json.to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
