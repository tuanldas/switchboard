use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub mod antigravity;
pub mod claude;
pub mod codex;
pub mod cursor;

pub use antigravity::AntigravityProvider;
pub use claude::ClaudeProvider;
pub use codex::CodexProvider;
pub use cursor::CursorProvider;

/// Supported AI coding tool providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(alias = "claude", alias = "claude_code", alias = "claudeCode")]
    ClaudeCode,
    #[serde(alias = "codex")]
    Codex,
    #[serde(alias = "cursor")]
    Cursor,
    #[serde(alias = "antigravity")]
    Antigravity,
}

impl Provider {
    #[allow(non_upper_case_globals)]
    pub const Claude: Provider = Provider::ClaudeCode;

    pub const ALL: [Provider; 4] = [
        Provider::ClaudeCode,
        Provider::Codex,
        Provider::Cursor,
        Provider::Antigravity,
    ];

    /// Returns a slice containing all supported providers.
    pub fn all() -> &'static [Provider] {
        &Self::ALL
    }

    /// Returns the configuration directory for each provider:
    /// - ClaudeCode -> ~/.claude
    /// - Codex -> ~/.codex
    /// - Cursor -> platform-specific (~/Library/Application Support/Cursor on macOS, %APPDATA%/Cursor on Windows, ~/.config/Cursor on Linux)
    /// - Antigravity -> ~/.gemini/antigravity
    pub fn config_dir(&self) -> Option<PathBuf> {
        match self {
            Provider::ClaudeCode => dirs::home_dir().map(|h| h.join(".claude")),
            Provider::Codex => dirs::home_dir().map(|h| h.join(".codex")),
            Provider::Cursor => {
                #[cfg(target_os = "macos")]
                {
                    dirs::home_dir()
                        .map(|h| h.join("Library").join("Application Support").join("Cursor"))
                        .or_else(|| dirs::config_dir().map(|c| c.join("Cursor")))
                }
                #[cfg(target_os = "windows")]
                {
                    std::env::var_os("APPDATA")
                        .map(PathBuf::from)
                        .map(|p| p.join("Cursor"))
                        .or_else(|| dirs::config_dir().map(|c| c.join("Cursor")))
                }
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                {
                    dirs::config_dir()
                        .map(|c| c.join("Cursor"))
                        .or_else(|| dirs::home_dir().map(|h| h.join(".config").join("Cursor")))
                }
            }
            Provider::Antigravity => dirs::home_dir().map(|h| h.join(".gemini").join("antigravity")),
        }
    }

    /// Returns the human-readable display name for this provider.
    pub fn display_name(&self) -> &str {
        match self {
            Provider::ClaudeCode => "Claude Code",
            Provider::Codex => "Codex",
            Provider::Cursor => "Cursor",
            Provider::Antigravity => "Antigravity",
        }
    }

    /// Returns the directory where usage logs are stored for this provider.
    pub fn log_dir(&self) -> Option<PathBuf> {
        self.config_dir().map(|p| p.join("logs"))
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(['-', '_'], "").as_str() {
            "claudecode" | "claude" => Ok(Provider::ClaudeCode),
            "codex" => Ok(Provider::Codex),
            "cursor" => Ok(Provider::Cursor),
            "antigravity" => Ok(Provider::Antigravity),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

/// Represents a configured provider account stored in the application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAccount {
    pub id: String,
    pub provider: Provider,
    pub name: String,
    pub label: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl ProviderAccount {
    pub fn new(
        id: impl Into<String>,
        provider: Provider,
        name: impl Into<String>,
        label: Option<String>,
        is_active: bool,
        created_at: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            provider,
            name: name.into(),
            label,
            is_active,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        }
    }
}

/// Represents an account detected from the provider's configuration on the local machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedAccount {
    pub name: String,
    pub provider: Provider,
    pub config_snapshot: String,
}

impl DetectedAccount {
    pub fn new(
        name: impl Into<String>,
        provider: Provider,
        config_snapshot: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            provider,
            config_snapshot: config_snapshot.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn config_data(&self) -> &str {
        &self.config_snapshot
    }

    pub fn to_provider_account(&self, is_active: bool) -> ProviderAccount {
        let now = chrono::Utc::now().to_rfc3339();
        ProviderAccount {
            id: uuid::Uuid::new_v4().to_string(),
            provider: self.provider,
            name: self.name.clone(),
            label: None,
            is_active,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// Operations that every provider implementation must support.
pub trait ProviderOps {
    fn detect_accounts() -> anyhow::Result<Vec<DetectedAccount>>;
    fn switch_to(account_id: &str, config_data: &str) -> anyhow::Result<()>;
    fn backup_current() -> anyhow::Result<Option<String>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name() {
        assert_eq!(Provider::ClaudeCode.display_name(), "Claude Code");
        assert_eq!(Provider::Codex.display_name(), "Codex");
        assert_eq!(Provider::Cursor.display_name(), "Cursor");
        assert_eq!(Provider::Antigravity.display_name(), "Antigravity");
    }

    #[test]
    fn test_config_dir() {
        let home = dirs::home_dir().expect("Home directory should exist");

        assert_eq!(
            Provider::ClaudeCode.config_dir(),
            Some(home.join(".claude"))
        );
        assert_eq!(
            Provider::Codex.config_dir(),
            Some(home.join(".codex"))
        );
        assert_eq!(
            Provider::Antigravity.config_dir(),
            Some(home.join(".gemini").join("antigravity"))
        );

        let cursor_dir = Provider::Cursor.config_dir().expect("Cursor config dir should be resolved");
        #[cfg(target_os = "macos")]
        assert_eq!(
            cursor_dir,
            home.join("Library").join("Application Support").join("Cursor")
        );
        #[cfg(target_os = "windows")]
        assert!(cursor_dir.ends_with(r"Cursor"));
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        assert!(cursor_dir.ends_with("Cursor"));
    }

    #[test]
    fn test_log_dir() {
        let home = dirs::home_dir().expect("Home directory should exist");

        assert_eq!(
            Provider::ClaudeCode.log_dir(),
            Some(home.join(".claude").join("logs"))
        );
        assert_eq!(
            Provider::Codex.log_dir(),
            Some(home.join(".codex").join("logs"))
        );
        assert_eq!(
            Provider::Antigravity.log_dir(),
            Some(home.join(".gemini").join("antigravity").join("logs"))
        );

        let cursor_log = Provider::Cursor.log_dir().expect("Cursor log dir should be resolved");
        assert!(cursor_log.ends_with(PathBuf::from("Cursor").join("logs")));
    }

    #[test]
    fn test_provider_serde() {
        let providers = vec![
            Provider::ClaudeCode,
            Provider::Codex,
            Provider::Cursor,
            Provider::Antigravity,
        ];

        for p in providers {
            let json = serde_json::to_string(&p).expect("Serialization failed");
            let deserialized: Provider = serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(p, deserialized);
        }

        // Test alias deserialization
        let claude_alias: Provider = serde_json::from_str(r#""claude""#).unwrap();
        assert_eq!(claude_alias, Provider::ClaudeCode);

        let claude_snake: Provider = serde_json::from_str(r#""claude_code""#).unwrap();
        assert_eq!(claude_snake, Provider::ClaudeCode);
    }

    #[test]
    fn test_provider_from_str() {
        use std::str::FromStr;

        assert_eq!(Provider::from_str("ClaudeCode").unwrap(), Provider::ClaudeCode);
        assert_eq!(Provider::from_str("claude-code").unwrap(), Provider::ClaudeCode);
        assert_eq!(Provider::from_str("claude").unwrap(), Provider::ClaudeCode);
        assert_eq!(Provider::from_str("Codex").unwrap(), Provider::Codex);
        assert_eq!(Provider::from_str("cursor").unwrap(), Provider::Cursor);
        assert_eq!(Provider::from_str("Antigravity").unwrap(), Provider::Antigravity);
        assert!(Provider::from_str("unknown").is_err());
    }

    #[test]
    fn test_provider_account_serde() {
        let account = ProviderAccount {
            id: "acc_123".to_string(),
            provider: Provider::ClaudeCode,
            name: "Personal Account".to_string(),
            label: Some("Primary".to_string()),
            is_active: true,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&account).expect("Serialization failed");
        let deserialized: ProviderAccount = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(account, deserialized);
    }

    #[test]
    fn test_detected_account_serde() {
        let detected = DetectedAccount {
            name: "test-user".to_string(),
            provider: Provider::ClaudeCode,
            config_snapshot: r#"{"sessionKey":"sk-123"}"#.to_string(),
        };

        let json = serde_json::to_string(&detected).expect("Serialization failed");
        let deserialized: DetectedAccount = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(detected, deserialized);
        assert_eq!(detected.id(), "test-user");
        assert_eq!(detected.config_data(), r#"{"sessionKey":"sk-123"}"#);
    }

    #[test]
    fn test_provider_ops_implementations() {
        // Verify all 4 provider implementations implement ProviderOps
        let claude_res = ClaudeProvider::detect_accounts();
        assert!(claude_res.is_ok());

        let codex_res = CodexProvider::detect_accounts();
        assert!(codex_res.is_ok());

        let cursor_res = CursorProvider::detect_accounts();
        assert!(cursor_res.is_ok());

        let antigravity_res = AntigravityProvider::detect_accounts();
        assert!(antigravity_res.is_ok());
    }
}
