export interface ProviderInfo {
  name: string;
  icon: string;
  color: string;
}

export type ProviderDisplayInfo = ProviderInfo;

export type ProviderKey = 'claude_code' | 'codex' | 'cursor' | 'antigravity';

export const PROVIDERS = {
  claude_code: { name: "Claude Code", icon: "🤖", color: "#D97757" },
  codex: { name: "Codex", icon: "🧠", color: "#10A37F" },
  cursor: { name: "Cursor", icon: "⚡", color: "#007ACC" },
  antigravity: { name: "Antigravity", icon: "🚀", color: "#4285F4" },
} as const;

export const APP_NAME = "Status Bar";
export const APP_VERSION = "0.1.0";
export const REFRESH_INTERVAL = 30000;

export default PROVIDERS;
