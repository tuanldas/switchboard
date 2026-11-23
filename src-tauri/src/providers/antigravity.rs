use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde_json::Value;

use super::{DetectedAccount, Provider, ProviderOps};

/// Priority list of candidate configuration and authentication files
/// used by Google Antigravity (across CLI, IDE, and Desktop 2.0 surfaces).
const CANDIDATE_FILES: &[&str] = &[
    "auth.json",
    "credentials.json",
    ".credentials.json",
    "config.json",
    "settings.json",
    "oauth.json",
    "session.json",
    "accounts.json",
    "user.json",
    "state.json",
];

/// Candidate subdirectories inside the config directory that may store credentials.
const CANDIDATE_SUBDIRS: &[&str] = &["auth", "accounts", "credentials"];

/// Directories to skip when scanning the config directory for arbitrary json configs.
const SKIP_SCAN_DIRS: &[&str] = &[
    "brain",
    "logs",
    "cache",
    "scratch",
    "plugins",
    "skills",
    "node_modules",
    ".system_generated",
];

pub struct AntigravityProvider;

impl AntigravityProvider {
    /// Returns the configuration directory for Google Antigravity: ~/.gemini/antigravity
    pub fn get_config_dir() -> PathBuf {
        dirs::home_dir()
            .map(|home| home.join(".gemini").join("antigravity"))
            .unwrap_or_else(|| PathBuf::from(".gemini").join("antigravity"))
    }

    /// Alias for get_config_dir()
    pub fn config_dir() -> PathBuf {
        Self::get_config_dir()
    }

    /// Detects accounts from ~/.gemini/antigravity by inspecting authentication and config files.
    /// Handles missing directory or files gracefully by returning an empty Vec.
    pub fn detect_accounts() -> Result<Vec<DetectedAccount>> {
        Self::detect_accounts_in(&Self::get_config_dir())
    }

    /// Detects accounts within a specific directory.
    ///
    /// Flexible implementation:
    /// 1. Inspects known candidate files (auth.json, credentials.json, config.json, settings.json, etc.).
    /// 2. Inspects candidate subdirectories (auth/, accounts/, credentials/).
    /// 3. Scans root of config directory for any remaining .json files (excluding cache/log directories).
    /// 4. Parses single-account objects, multi-account arrays, and wrapper objects ({ "accounts": [...] }).
    /// 5. Gracefully falls back to ~/.gemini/antigravity-cli/settings.json if no primary accounts are found.
    pub fn detect_accounts_in(config_dir: &Path) -> Result<Vec<DetectedAccount>> {
        if !config_dir.exists() {
            // Check fallback CLI directory if standard directory does not exist
            return Self::check_cli_fallback();
        }

        let mut discovered_files: Vec<PathBuf> = Vec::new();

        // 1. Check known candidate files in priority order
        for file_name in CANDIDATE_FILES {
            let candidate = config_dir.join(file_name);
            if candidate.is_file() {
                discovered_files.push(candidate);
            }
        }

        // 2. Check candidate subdirectories
        for subdir_name in CANDIDATE_SUBDIRS {
            let subdir = config_dir.join(subdir_name);
            if subdir.is_dir() {
                if let Ok(entries) = fs::read_dir(&subdir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                            discovered_files.push(path);
                        }
                    }
                }
            }
        }

        // 3. Fallback scan for any other .json files directly in config_dir
        if let Ok(entries) = fs::read_dir(config_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && path.extension().and_then(|s| s.to_str()) == Some("json")
                    && !discovered_files.contains(&path)
                {
                    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    if !SKIP_SCAN_DIRS.contains(&filename) {
                        discovered_files.push(path);
                    }
                }
            }
        }

        let mut accounts: Vec<DetectedAccount> = Vec::new();
        let mut seen_names: std::collections::HashSet<String> = std::collections::HashSet::new();

        for file_path in &discovered_files {
            let content = match fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => {
                    log::warn!(
                        "Failed to read Antigravity config file at {}: {}",
                        file_path.display(),
                        e
                    );
                    continue;
                }
            };

            let trimmed = content.trim();
            if trimmed.is_empty() {
                continue;
            }

            let file_stem = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("default");

            // Try parsing as JSON
            if let Ok(json_val) = serde_json::from_str::<Value>(trimmed) {
                let parsed_accounts = Self::extract_accounts_from_json(&json_val, trimmed, file_stem);
                for account in parsed_accounts {
                    if !seen_names.contains(&account.name) {
                        seen_names.insert(account.name.clone());
                        accounts.push(account);
                    }
                }
            } else {
                // Non-JSON config/credential file (e.g. token or raw key)
                let name = file_stem.to_string();
                if !seen_names.contains(&name) {
                    seen_names.insert(name.clone());
                    accounts.push(DetectedAccount {
                        name,
                        provider: Provider::Antigravity,
                        config_snapshot: trimmed.to_string(),
                    });
                }
            }
        }

        // 5. If no accounts detected in ~/.gemini/antigravity, check ~/.gemini/antigravity-cli/
        if accounts.is_empty() {
            return Self::check_cli_fallback();
        }

        Ok(accounts)
    }

    /// Extracts one or more DetectedAccount records from a parsed JSON value.
    fn extract_accounts_from_json(val: &Value, raw_content: &str, file_stem: &str) -> Vec<DetectedAccount> {
        let mut accounts = Vec::new();

        // Case A: JSON is an array of account objects
        if let Some(arr) = val.as_array() {
            for item in arr {
                if let Some(name) = Self::extract_name_from_value(item) {
                    let snapshot = serde_json::to_string(item).unwrap_or_else(|_| raw_content.to_string());
                    accounts.push(DetectedAccount {
                        name,
                        provider: Provider::Antigravity,
                        config_snapshot: snapshot,
                    });
                }
            }
            if !accounts.is_empty() {
                return accounts;
            }
        }

        // Case B: JSON object has an "accounts", "profiles", or "users" array
        for list_key in &["accounts", "profiles", "users"] {
            if let Some(arr) = val.get(list_key).and_then(|v| v.as_array()) {
                for item in arr {
                    if let Some(name) = Self::extract_name_from_value(item) {
                        let snapshot = serde_json::to_string(item).unwrap_or_else(|_| raw_content.to_string());
                        accounts.push(DetectedAccount {
                            name,
                            provider: Provider::Antigravity,
                            config_snapshot: snapshot,
                        });
                    }
                }
                if !accounts.is_empty() {
                    return accounts;
                }
            }
        }

        // Case C: Single account object
        let name = Self::extract_name_from_value(val).unwrap_or_else(|| file_stem.to_string());
        accounts.push(DetectedAccount {
            name,
            provider: Provider::Antigravity,
            config_snapshot: raw_content.to_string(),
        });

        accounts
    }

    /// Flexibly extracts a user-identifying name or email from a JSON object.
    fn extract_name_from_value(val: &Value) -> Option<String> {
        // Direct string fields
        for key in &[
            "email",
            "name",
            "username",
            "user",
            "account",
            "display_name",
            "displayName",
            "google_account",
            "identity",
            "account_id",
            "accountId",
            "sub",
            "id",
        ] {
            if let Some(s) = val.get(*key).and_then(|v| v.as_str()) {
                if !s.trim().is_empty() {
                    return Some(s.trim().to_string());
                }
            }
        }

        // Nested account/user/oauth objects
        for parent_key in &["account", "user", "oauth", "profile", "auth", "session"] {
            if let Some(parent) = val.get(*parent_key).and_then(|v| v.as_object()) {
                for child_key in &["email", "name", "username", "account", "user", "display_name", "id"] {
                    if let Some(s) = parent.get(*child_key).and_then(|v| v.as_str()) {
                        if !s.trim().is_empty() {
                            return Some(s.trim().to_string());
                        }
                    }
                }
            }
        }

        None
    }

    /// Checks the alternative Antigravity CLI directory (~/.gemini/antigravity-cli/) for settings/auth.
    fn check_cli_fallback() -> Result<Vec<DetectedAccount>> {
        let cli_dir = dirs::home_dir()
            .map(|h| h.join(".gemini").join("antigravity-cli"))
            .unwrap_or_else(|| PathBuf::from(".gemini").join("antigravity-cli"));

        if !cli_dir.exists() {
            return Ok(Vec::new());
        }

        let cli_settings = cli_dir.join("settings.json");
        let cli_auth = cli_dir.join("auth.json");

        let target = if cli_settings.is_file() {
            Some(cli_settings)
        } else if cli_auth.is_file() {
            Some(cli_auth)
        } else {
            None
        };

        let Some(path) = target else {
            return Ok(Vec::new());
        };

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Ok(Vec::new()),
        };

        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let name = serde_json::from_str::<Value>(trimmed)
            .ok()
            .and_then(|v| Self::extract_name_from_value(&v))
            .unwrap_or_else(|| "cli-default".to_string());

        Ok(vec![DetectedAccount {
            name,
            provider: Provider::Antigravity,
            config_snapshot: trimmed.to_string(),
        }])
    }

    /// Switches the active Antigravity account by atomically writing config_data to the config directory.
    ///
    /// Atomic write with rollback pattern:
    /// 1. Determines the target file:
    ///    - Uses "target_file" field in config_data if specified in JSON.
    ///    - Overwrites an existing candidate file (auth.json, credentials.json, config.json, etc.).
    ///    - Defaults to auth.json if no candidate file exists yet.
    /// 2. Creates a temporary file in the same directory (.filename.tmp.<uuid>).
    /// 3. Backs up current content for rollback in case of write failure.
    /// 4. Atomically renames temporary file to target file.
    /// 5. Restores original file if rename fails.
    pub fn switch_to(config_data: &str) -> Result<()> {
        Self::switch_to_in(&Self::get_config_dir(), config_data)
    }

    /// Internal helper that writes credentials atomically within a specific directory (useful for testing).
    pub fn switch_to_in(config_dir: &Path, config_data: &str) -> Result<()> {
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .with_context(|| format!("Failed to create directory: {}", config_dir.display()))?;
        }

        // Determine target file name
        let target_filename = Self::resolve_target_filename(config_dir, config_data);
        let target_path = config_dir.join(&target_filename);

        let temp_filename = format!(
            ".{}.tmp.{}",
            target_filename,
            uuid::Uuid::new_v4()
        );
        let temp_path = config_dir.join(temp_filename);

        // Backup current file content for rollback if it exists
        let backup_data = if target_path.exists() {
            match fs::read_to_string(&target_path) {
                Ok(content) => Some(content),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(e) => {
                    return Err(e).with_context(|| {
                        format!("Failed to backup current config at {} for rollback", target_path.display())
                    })
                }
            }
        } else {
            None
        };

        // Write to temporary file first, then atomically rename to target
        let write_op = (|| -> Result<()> {
            fs::write(&temp_path, config_data).with_context(|| {
                format!("Failed to write temporary config file: {}", temp_path.display())
            })?;
            fs::rename(&temp_path, &target_path).with_context(|| {
                format!(
                    "Failed to atomically rename {} to {}",
                    temp_path.display(),
                    target_path.display()
                )
            })?;
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

    /// Resolves which target file to write to:
    /// 1. If config_data contains a "target_file" JSON field, use it.
    /// 2. If an existing candidate file is found in config_dir, reuse it.
    /// 3. Otherwise default to "auth.json".
    fn resolve_target_filename(config_dir: &Path, config_data: &str) -> String {
        if let Ok(val) = serde_json::from_str::<Value>(config_data) {
            if let Some(target) = val.get("target_file").and_then(|v| v.as_str()) {
                if !target.trim().is_empty() {
                    return target.trim().to_string();
                }
            }
        }

        // Reuse an existing candidate file if present
        for file_name in CANDIDATE_FILES {
            let path = config_dir.join(file_name);
            if path.is_file() {
                return file_name.to_string();
            }
        }

        "auth.json".to_string()
    }

    /// Reads the current Antigravity authentication / configuration.
    /// Returns the contents as String, or None if no configuration file is found.
    pub fn backup_current() -> Result<Option<String>> {
        Self::backup_current_in(&Self::get_config_dir())
    }

    /// Internal helper that reads current configuration from a specific directory (useful for testing).
    pub fn backup_current_in(config_dir: &Path) -> Result<Option<String>> {
        if !config_dir.exists() {
            return Ok(None);
        }

        // 1. Check known candidate files in priority order
        for file_name in CANDIDATE_FILES {
            let path = config_dir.join(file_name);
            if path.is_file() {
                match fs::read_to_string(&path) {
                    Ok(content) => return Ok(Some(content)),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                    Err(e) => {
                        return Err(e).with_context(|| format!("Failed to read {}", path.display()))
                    }
                }
            }
        }

        // 2. Check candidate subdirectories
        for subdir_name in CANDIDATE_SUBDIRS {
            let subdir = config_dir.join(subdir_name);
            if subdir.is_dir() {
                if let Ok(entries) = fs::read_dir(&subdir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                            match fs::read_to_string(&path) {
                                Ok(content) => return Ok(Some(content)),
                                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                                Err(e) => {
                                    return Err(e).with_context(|| {
                                        format!("Failed to read {}", path.display())
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Check any remaining .json files directly in config_dir
        if let Ok(entries) = fs::read_dir(config_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    if !SKIP_SCAN_DIRS.contains(&filename) {
                        match fs::read_to_string(&path) {
                            Ok(content) => return Ok(Some(content)),
                            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                            Err(e) => {
                                return Err(e).with_context(|| format!("Failed to read {}", path.display()))
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

impl ProviderOps for AntigravityProvider {
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
        let dir = AntigravityProvider::get_config_dir();
        assert!(
            dir.ends_with(".gemini/antigravity")
                || dir.ends_with(".gemini\\antigravity")
                || dir.to_string_lossy().contains(".gemini")
        );
        assert_eq!(AntigravityProvider::config_dir(), dir);
    }

    #[test]
    fn test_detect_accounts_nonexistent_dir() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_none_{}", uuid::Uuid::new_v4()));
        let res = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_detect_accounts_auth_json() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_auth_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let auth_file = temp_dir.join("auth.json");
        let mock_json = r#"{"name": "antigravity-user", "token": "sec-123"}"#;
        fs::write(&auth_file, mock_json).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "antigravity-user");
        assert_eq!(accounts[0].provider, Provider::Antigravity);
        assert_eq!(accounts[0].config_snapshot, mock_json);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_credentials_json() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_creds_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_file = temp_dir.join("credentials.json");
        let mock_json = r#"{"email": "engineer@google.com", "apiKey": "AIzaSy..."}"#;
        fs::write(&creds_file, mock_json).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "engineer@google.com");
        assert_eq!(accounts[0].provider, Provider::Antigravity);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_config_json() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_config_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let config_file = temp_dir.join("config.json");
        let mock_json = r#"{"account": {"email": "dev@antigravity.dev", "name": "Dev User"}}"#;
        fs::write(&config_file, mock_json).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "dev@antigravity.dev");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_multiple_in_array() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_multi_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let auth_file = temp_dir.join("auth.json");
        let mock_json = r#"[
            {"email": "alice@antigravity.dev", "token": "tok1"},
            {"email": "bob@antigravity.dev", "token": "tok2"}
        ]"#;
        fs::write(&auth_file, mock_json).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].name, "alice@antigravity.dev");
        assert_eq!(accounts[1].name, "bob@antigravity.dev");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_wrapper_object() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_wrap_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let accounts_file = temp_dir.join("accounts.json");
        let mock_json = r#"{
            "accounts": [
                {"email": "primary@google.com", "active": true},
                {"email": "secondary@google.com", "active": false}
            ]
        }"#;
        fs::write(&accounts_file, mock_json).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].name, "primary@google.com");
        assert_eq!(accounts[1].name, "secondary@google.com");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_accounts_in_subdir() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_subdir_{}", uuid::Uuid::new_v4()));
        let auth_subdir = temp_dir.join("auth");
        fs::create_dir_all(&auth_subdir).unwrap();
        let session_file = auth_subdir.join("session.json");
        let mock_json = r#"{"oauth": {"email": "oauth_user@example.com", "access_token": "ya29..."}}"#;
        fs::write(&session_file, mock_json).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name, "oauth_user@example.com");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_switch_to_and_backup() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_switch_{}", uuid::Uuid::new_v4()));
        let initial_json = r#"{"email": "initial@google.com", "token": "init_tok"}"#;
        let switched_json = r#"{"email": "switched@google.com", "token": "switched_tok"}"#;

        // 1. Initial switch
        AntigravityProvider::switch_to_in(&temp_dir, initial_json).unwrap();
        let backup = AntigravityProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, Some(initial_json.to_string()));

        // 2. Switch to new account
        AntigravityProvider::switch_to_in(&temp_dir, switched_json).unwrap();
        let target_path = temp_dir.join("auth.json");
        let current_content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(current_content, switched_json);

        // 3. Backup reflects switched content
        let backup2 = AntigravityProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup2, Some(switched_json.to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_switch_to_preserves_existing_target() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_reuse_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let creds_path = temp_dir.join("credentials.json");
        fs::write(&creds_path, r#"{"existing": true}"#).unwrap();

        let new_content = r#"{"email": "new@google.com"}"#;
        AntigravityProvider::switch_to_in(&temp_dir, new_content).unwrap();

        // Should have updated credentials.json instead of creating auth.json
        assert!(creds_path.exists());
        assert_eq!(fs::read_to_string(&creds_path).unwrap(), new_content);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_switch_to_with_custom_target_file() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_target_{}", uuid::Uuid::new_v4()));
        let custom_config = r#"{"target_file": "custom_auth.json", "token": "xyz"}"#;

        AntigravityProvider::switch_to_in(&temp_dir, custom_config).unwrap();
        let custom_path = temp_dir.join("custom_auth.json");
        assert!(custom_path.exists());
        assert_eq!(fs::read_to_string(&custom_path).unwrap(), custom_config);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_backup_current_not_found() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_notfound_{}", uuid::Uuid::new_v4()));
        let backup = AntigravityProvider::backup_current_in(&temp_dir).unwrap();
        assert_eq!(backup, None);
    }

    #[test]
    fn test_provider_ops_implementation() {
        let temp_dir = std::env::temp_dir().join(format!("test_antigravity_ops_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let auth_file = temp_dir.join("auth.json");
        fs::write(&auth_file, r#"{"email": "ops@example.com"}"#).unwrap();

        let accounts = AntigravityProvider::detect_accounts_in(&temp_dir).unwrap();
        assert_eq!(accounts.len(), 1);

        <AntigravityProvider as ProviderOps>::switch_to("dummy_id", r#"{"email": "updated@example.com"}"#).ok();

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
