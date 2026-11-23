import { invoke } from "@tauri-apps/api/core";

export interface AccountInfo {
  id: string;
  provider: string;
  name: string;
  label?: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface AppSettings {
  theme: string;
  auto_start: boolean;
  auto_switch_on_quota: boolean;
  language: string;
  budget_limit?: number | null;
  notifications_enabled: boolean;
}

export interface UsageSummary {
  total_tokens: number;
  total_cost: number;
  today_tokens: number;
  today_cost: number;
}

/**
 * Lists all accounts stored in the database.
 */
export async function listAccounts(): Promise<AccountInfo[]> {
  return invoke<AccountInfo[]>("list_accounts");
}

/**
 * Adds a new account for the specified provider.
 */
export async function addAccount(
  provider: string,
  name: string,
  label?: string | null,
): Promise<AccountInfo> {
  return invoke<AccountInfo>("add_account", { provider, name, label });
}

/**
 * Removes an account by ID.
 */
export async function removeAccount(id: string): Promise<void> {
  return invoke<void>("remove_account", { id });
}

/**
 * Switches the active account to the specified account ID.
 */
export async function switchAccount(id: string): Promise<AccountInfo> {
  return invoke<AccountInfo>("switch_account", { id });
}

/**
 * Retrieves the currently active account, or null if none is active.
 */
export async function getActiveAccount(): Promise<AccountInfo | null> {
  return invoke<AccountInfo | null>("get_active_account");
}

/**
 * Reads the application settings.
 */
export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

/**
 * Updates the application settings.
 */
export async function updateSettings(settings: AppSettings): Promise<void> {
  return invoke<void>("update_settings", { settings });
}

/**
 * Retrieves a summary of AI token usage and costs.
 */
export async function getUsageSummary(): Promise<UsageSummary> {
  return invoke<UsageSummary>("get_usage_summary");
}
