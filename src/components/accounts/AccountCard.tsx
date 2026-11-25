import React, { useState } from "react";
import { PROVIDERS } from "../../lib/constants";
import useAccountStore from "../../stores/accountStore";
import { AccountInfo } from "../../lib/tauri";

export interface AccountCardProps {
  account: AccountInfo;
}

const ProviderBrandIcon: React.FC<{ provider: string }> = ({ provider }) => {
  switch (provider) {
    case "claude_code":
      return (
        <div className="w-10 h-10 rounded-xl bg-[#D97757]/15 border border-[#D97757]/30 flex items-center justify-center shrink-0 shadow-[0_0_12px_rgba(217,119,87,0.2)]">
          <span className="text-lg">🤖</span>
        </div>
      );
    case "codex":
      return (
        <div className="w-10 h-10 rounded-xl bg-[#10A37F]/15 border border-[#10A37F]/30 flex items-center justify-center shrink-0 shadow-[0_0_12px_rgba(16,163,127,0.2)]">
          <span className="text-lg">🧠</span>
        </div>
      );
    case "cursor":
      return (
        <div className="w-10 h-10 rounded-xl bg-[#007ACC]/15 border border-[#007ACC]/30 flex items-center justify-center shrink-0 shadow-[0_0_12px_rgba(0,122,204,0.2)]">
          <span className="text-lg">⚡</span>
        </div>
      );
    case "antigravity":
      return (
        <div className="w-10 h-10 rounded-xl bg-[#4285F4]/15 border border-[#4285F4]/30 flex items-center justify-center shrink-0 shadow-[0_0_12px_rgba(66,133,244,0.2)]">
          <span className="text-lg">🚀</span>
        </div>
      );
    default:
      return (
        <div className="w-10 h-10 rounded-xl bg-zinc-700/20 border border-zinc-600/30 flex items-center justify-center shrink-0">
          <span className="text-lg">🔑</span>
        </div>
      );
  }
};

export const AccountCard: React.FC<AccountCardProps> = ({ account }) => {
  const { switchTo, deleteAccount } = useAccountStore();
  const [isSwitching, setIsSwitching] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const providerInfo = PROVIDERS[account.provider as keyof typeof PROVIDERS] || {
    name: account.provider,
    icon: "🔑",
    color: "#64748b",
  };

  const handleSwitch = async () => {
    if (isSwitching || account.is_active) return;
    try {
      setIsSwitching(true);
      await switchTo(account.id);
    } catch (error) {
      console.error("Failed to switch account:", error);
    } finally {
      setIsSwitching(false);
    }
  };

  const handleDelete = async () => {
    if (isDeleting) return;
    const confirmed = window.confirm(
      `Disconnect profile "${account.name}"? Encrypted credentials will be securely wiped.`
    );
    if (!confirmed) return;

    try {
      setIsDeleting(true);
      await deleteAccount(account.id);
    } catch (error) {
      console.error("Failed to delete account:", error);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div
      className={`glass-card rounded-2xl p-4 border transition-all duration-300 relative overflow-hidden group ${
        account.is_active
          ? "border-emerald-500/50 bg-emerald-950/10 shadow-[0_0_20px_rgba(16,185,129,0.15)]"
          : "border-white/5 hover:border-white/15"
      }`}
    >
      {account.is_active && (
        <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-emerald-400 via-teal-300 to-cyan-400" />
      )}

      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-3.5 min-w-0">
          <ProviderBrandIcon provider={account.provider} />

          <div className="flex flex-col min-w-0">
            <div className="flex items-center gap-2 flex-wrap">
              <span className="font-semibold text-white text-sm truncate tracking-tight">
                {account.name}
              </span>
              {account.label && (
                <span className="px-2 py-0.5 text-[10px] font-mono font-medium rounded-full bg-white/5 text-zinc-300 border border-white/10 shrink-0">
                  {account.label}
                </span>
              )}
            </div>
            <div className="flex items-center gap-2 mt-0.5">
              <span className="text-xs text-zinc-400 truncate font-medium">
                {providerInfo.name}
              </span>
              <span className="text-zinc-600">•</span>
              <span className="text-[11px] font-mono text-zinc-400">
                AES-256 Secured
              </span>
            </div>
          </div>
        </div>

        <div className="flex items-center gap-3 shrink-0">
          {account.is_active ? (
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 text-xs font-semibold select-none active-pulse">
              <span className="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
              <span>LINKED ACTIVE</span>
            </div>
          ) : (
            <button
              type="button"
              onClick={handleSwitch}
              disabled={isSwitching}
              className="px-3.5 py-1.5 text-xs font-semibold rounded-xl bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-500 hover:to-indigo-600 text-white transition-all duration-200 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed shadow-[0_0_12px_rgba(99,102,241,0.3)] flex items-center gap-1.5"
            >
              {isSwitching ? (
                <>
                  <span className="animate-spin text-sm">⏳</span>
                  <span>Switching...</span>
                </>
              ) : (
                <>
                  <span>⚡</span>
                  <span>Switch To</span>
                </>
              )}
            </button>
          )}

          <button
            type="button"
            onClick={handleDelete}
            disabled={isDeleting || account.is_active}
            title={account.is_active ? "Cannot remove active account" : "Remove account"}
            className="p-1.5 rounded-lg text-zinc-400 hover:text-red-400 hover:bg-red-500/10 transition-colors disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer"
          >
            🗑️
          </button>
        </div>
      </div>

      <div className="mt-3 pt-3 border-t border-white/5 flex items-center justify-between text-[11px] text-zinc-400">
        <div className="flex items-center gap-2">
          <span className="text-zinc-400">Quota Limit:</span>
          <div className="w-24 h-1.5 bg-black/40 rounded-full overflow-hidden">
            <div
              className={`h-full rounded-full ${
                account.is_active ? "bg-emerald-500" : "bg-zinc-600"
              }`}
              style={{ width: account.is_active ? "64%" : "12%" }}
            />
          </div>
          <span className="font-mono text-zinc-400">
            {account.is_active ? "64%" : "Idle"}
          </span>
        </div>

        <span className="font-mono text-zinc-400">
          Last used: {account.is_active ? "Just now" : "Parked"}
        </span>
      </div>
    </div>
  );
};

export default AccountCard;
