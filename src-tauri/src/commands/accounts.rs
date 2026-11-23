use super::super::{db, providers, crypto, keychain};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Account information returned to IPC callers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: String,
    pub provider: String,
    pub name: String,
    pub label: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Helper function to validate allowed provider identifiers.
fn validate_provider(provider: &str) -> Result<(), String> {
    match provider {
        "claude_code" | "codex" | "cursor" | "antigravity" => Ok(()),
        _ => Err(format!(
            "Invalid provider: '{}'. Must be 'claude_code', 'codex', 'cursor', or 'antigravity'",
            provider
        )),
    }
}

/// Dispatches backup_current to the appropriate provider implementation.
fn provider_backup_current(provider: &str) -> anyhow::Result<Option<String>> {
    match provider {
        "claude_code" => providers::ClaudeProvider::backup_current(),
        "codex" => providers::CodexProvider::backup_current(),
        "cursor" => providers::CursorProvider::backup_current(),
        "antigravity" => providers::AntigravityProvider::backup_current(),
        _ => anyhow::bail!("Invalid provider: {}", provider),
    }
}

/// Dispatches switch_to to the appropriate provider implementation.
fn provider_switch_to(provider: &str, _account_id: &str, config_data: &str) -> anyhow::Result<()> {
    match provider {
        "claude_code" => providers::ClaudeProvider::switch_to(config_data),
        "codex" => providers::CodexProvider::switch_to(config_data),
        "cursor" => providers::CursorProvider::switch_to(config_data),
        "antigravity" => providers::AntigravityProvider::switch_to(config_data),
        _ => anyhow::bail!("Invalid provider: {}", provider),
    }
}

/// Standard Base64 encoding.
fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARSET[(triple >> 18) & 0x3F] as char);
        result.push(CHARSET[(triple >> 12) & 0x3F] as char);
        if chunk.len() > 1 {
            result.push(CHARSET[(triple >> 6) & 0x3F] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARSET[triple & 0x3F] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Standard Base64 decoding.
fn base64_decode(input: &str) -> anyhow::Result<Vec<u8>> {
    let clean: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.is_empty() {
        return Ok(Vec::new());
    }
    if clean.len() % 4 != 0 {
        anyhow::bail!("Invalid base64 length");
    }
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    let bytes = clean.as_bytes();
    let decode_char = |b: u8| -> anyhow::Result<u8> {
        match b {
            b'A'..=b'Z' => Ok(b - b'A'),
            b'a'..=b'z' => Ok(b - b'a' + 26),
            b'0'..=b'9' => Ok(b - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => Ok(0),
            _ => anyhow::bail!("Invalid base64 character: {}", b as char),
        }
    };

    for chunk in bytes.chunks(4) {
        let c0 = decode_char(chunk[0])?;
        let c1 = decode_char(chunk[1])?;
        let c2 = decode_char(chunk[2])?;
        let c3 = decode_char(chunk[3])?;

        let triple = ((c0 as u32) << 18) | ((c1 as u32) << 12) | ((c2 as u32) << 6) | (c3 as u32);
        out.push(((triple >> 16) & 0xFF) as u8);
        if chunk[2] != b'=' {
            out.push(((triple >> 8) & 0xFF) as u8);
        }
        if chunk[3] != b'=' {
            out.push((triple & 0xFF) as u8);
        }
    }
    Ok(out)
}

/// Lists all accounts stored in the database.
#[tauri::command]
pub async fn list_accounts() -> Result<Vec<AccountInfo>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, provider, name, label, is_active, created_at, updated_at
             FROM accounts
             ORDER BY created_at ASC;",
        )
        .map_err(|e| e.to_string())?;

    let account_iter = stmt
        .query_map([], |row| {
            let is_active_int: i64 = row.get(4)?;
            Ok(AccountInfo {
                id: row.get(0)?,
                provider: row.get(1)?,
                name: row.get(2)?,
                label: row.get(3)?,
                is_active: is_active_int != 0,
                created_at: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                updated_at: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
            })
        })
        .map_err(|e| e.to_string())?;

    let mut accounts = Vec::new();
    for account in account_iter {
        accounts.push(account.map_err(|e| e.to_string())?);
    }
    Ok(accounts)
}

/// Adds a new account for the specified provider:
/// - Validates provider string ("claude_code", "codex", "cursor", or "antigravity")
/// - Calls provider's backup_current() to snapshot current credentials
/// - Encrypts the snapshot data using crypto::encrypt with the keychain encryption key
/// - Stores encrypted data in accounts table (config_data stores base64-encoded ciphertext)
/// - Generates UUID for id
/// - Returns the new AccountInfo
#[tauri::command]
pub async fn add_account(
    provider: String,
    name: String,
    label: Option<String>,
) -> Result<AccountInfo, String> {
    validate_provider(&provider)?;

    // Snapshot current provider credentials
    let config_snapshot = provider_backup_current(&provider)
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    // Encrypt snapshot using keychain encryption key
    let key = keychain::get_encryption_key().map_err(|e| e.to_string())?;
    let encrypted_bytes =
        crypto::encrypt(config_snapshot.as_bytes(), &key).map_err(|e| e.to_string())?;
    let config_data_base64 = base64_encode(&encrypted_bytes);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let conn = db::get_connection().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO accounts (id, provider, name, label, config_data, is_active, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
        params![
            &id,
            &provider,
            &name,
            &label,
            &config_data_base64,
            0,
            &now,
            &now,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(AccountInfo {
        id,
        provider,
        name,
        label,
        is_active: false,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Removes an account by ID:
/// - Deletes from accounts table
/// - Cleans up any stored encrypted data
#[tauri::command]
pub async fn remove_account(id: String) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM accounts WHERE id = ?1;", params![&id])
        .map_err(|e| e.to_string())?;

    // Clean up stored secret from keychain if present
    let _ = keychain::delete_secret(&id);

    Ok(())
}

/// Switches active account:
/// - Gets the account from DB
/// - Gets encrypted config_data, decrypts it
/// - Calls the appropriate provider's backup_current() to save current account first
/// - Updates the previous active account's config_data with the backup
/// - Calls provider's switch_to() with decrypted config
/// - Updates is_active flags in DB (deactivate all, activate this one)
/// - Returns updated AccountInfo
#[tauri::command]
pub async fn switch_account(id: String) -> Result<AccountInfo, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    // 1. Get the account from DB
    let target = {
        let mut stmt = conn
            .prepare(
                "SELECT id, provider, name, label, config_data, created_at
                 FROM accounts
                 WHERE id = ?1;",
            )
            .map_err(|e| e.to_string())?;

        stmt.query_row(params![&id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            ))
        })
        .map_err(|e| format!("Account with id '{}' not found: {}", id, e))?
    };
    let (target_id, target_provider, target_name, target_label, target_config_base64, target_created_at) = target;

    // 2. Decrypt target config_data
    let key = keychain::get_encryption_key().map_err(|e| e.to_string())?;
    let decrypted_config = if !target_config_base64.is_empty() {
        let encrypted_bytes = base64_decode(&target_config_base64).map_err(|e| e.to_string())?;
        let decrypted_bytes =
            crypto::decrypt(&encrypted_bytes, &key).map_err(|e| e.to_string())?;
        String::from_utf8(decrypted_bytes).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    // 3 & 4. Backup current credentials and update previous active account's config_data
    let prev_active: Option<(String, String)> = {
        let mut stmt = conn
            .prepare("SELECT id, provider FROM accounts WHERE is_active = 1 LIMIT 1;")
            .map_err(|e| e.to_string())?;
        let mut rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| e.to_string())?;
        rows.next().transpose().map_err(|e| e.to_string())?
    };

    if let Some((prev_id, prev_provider)) = prev_active {
        let backup_opt = provider_backup_current(&prev_provider).map_err(|e| e.to_string())?;
        if let Some(backup_content) = backup_opt {
            let encrypted_backup = crypto::encrypt(backup_content.as_bytes(), &key)
                .map_err(|e| e.to_string())?;
            let backup_base64 = base64_encode(&encrypted_backup);
            let now_ts = Utc::now().to_rfc3339();
            conn.execute(
                "UPDATE accounts SET config_data = ?1, updated_at = ?2 WHERE id = ?3;",
                params![&backup_base64, &now_ts, &prev_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    // 5. Switch provider to decrypted config
    provider_switch_to(&target_provider, &target_id, &decrypted_config)
        .map_err(|e| e.to_string())?;

    // 6. Update is_active flags in DB (deactivate all, activate this one)
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE accounts SET is_active = 0, updated_at = ?1 WHERE is_active != 0;",
        params![&now],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE accounts SET is_active = 1, updated_at = ?1 WHERE id = ?2;",
        params![&now, &target_id],
    )
    .map_err(|e| e.to_string())?;

    // 7. Return updated AccountInfo
    Ok(AccountInfo {
        id: target_id,
        provider: target_provider,
        name: target_name,
        label: target_label,
        is_active: true,
        created_at: target_created_at,
        updated_at: now,
    })
}

/// Retrieves the currently active account (is_active = 1), or None if none is active.
#[tauri::command]
pub async fn get_active_account() -> Result<Option<AccountInfo>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, provider, name, label, is_active, created_at, updated_at
             FROM accounts
             WHERE is_active = 1
             LIMIT 1;",
        )
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([], |row| {
            let is_active_int: i64 = row.get(4)?;
            Ok(AccountInfo {
                id: row.get(0)?,
                provider: row.get(1)?,
                name: row.get(2)?,
                label: row.get(3)?,
                is_active: is_active_int != 0,
                created_at: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                updated_at: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
            })
        })
        .map_err(|e| e.to_string())?;

    match rows.next() {
        Some(account_res) => {
            let account = account_res.map_err(|e| e.to_string())?;
            Ok(Some(account))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_provider() {
        assert!(validate_provider("claude_code").is_ok());
        assert!(validate_provider("codex").is_ok());
        assert!(validate_provider("cursor").is_ok());
        assert!(validate_provider("antigravity").is_ok());
        assert!(validate_provider("invalid_provider").is_err());
    }

    #[test]
    fn test_base64_encode_decode_roundtrip() {
        let test_cases: Vec<&[u8]> = vec![
            b"",
            b"f",
            b"fo",
            b"foo",
            b"foob",
            b"fooba",
            b"foobar",
            b"Hello, Tauri 2 World! \x00\x01\xfe\xff",
        ];

        for original in test_cases {
            let encoded = base64_encode(original);
            let decoded = base64_decode(&encoded).expect("Decoded failed");
            assert_eq!(original, decoded.as_slice());
        }
    }

    #[test]
    fn test_account_info_serde() {
        let account = AccountInfo {
            id: "uuid-123".to_string(),
            provider: "claude_code".to_string(),
            name: "Work Claude".to_string(),
            label: Some("Primary".to_string()),
            is_active: true,
            created_at: "2026-11-23T19:00:00Z".to_string(),
            updated_at: "2026-11-23T19:30:00Z".to_string(),
        };

        let json = serde_json::to_string(&account).expect("Serialize AccountInfo");
        let deserialized: AccountInfo =
            serde_json::from_str(&json).expect("Deserialize AccountInfo");
        assert_eq!(account, deserialized);
    }

    #[tokio::test]
    async fn test_accounts_crud_lifecycle() {
        let _ = db::init();
        let provider = "codex".to_string();
        let name = "Codex Dev".to_string();
        let label = Some("Work".to_string());

        let add_res = add_account(provider, name, label).await;
        if let Ok(account) = add_res {
            assert_eq!(account.provider, "codex");
            assert_eq!(account.name, "Codex Dev");
            assert_eq!(account.label, Some("Work".to_string()));
            assert!(!account.is_active);

            let list = list_accounts().await.expect("list_accounts failed");
            assert!(list.iter().any(|a| a.id == account.id));

            let switched = switch_account(account.id.clone())
                .await
                .expect("switch_account failed");
            assert!(switched.is_active);

            let active = get_active_account().await.expect("get_active failed");
            assert!(active.is_some());
            assert_eq!(active.unwrap().id, account.id);

            remove_account(account.id.clone())
                .await
                .expect("remove_account failed");
            let after_remove = list_accounts().await.expect("list_accounts failed");
            assert!(!after_remove.iter().any(|a| a.id == account.id));
        }
    }
}
