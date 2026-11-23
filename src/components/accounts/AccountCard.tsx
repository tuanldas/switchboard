import React, { useState } from "react";
import { PROVIDERS } from "../../lib/constants";
import useAccountStore from "../../stores/accountStore";
import { AccountInfo } from "../../lib/tauri";

export interface AccountCardProps {
  account: AccountInfo;
}

export const AccountCard: React.FC<AccountCardProps> = ({ account }) => {
  const { switchTo, deleteAccount } = useAccountStore();
  const [isSwitching, setIsSwitching] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  // Look up provider info from PROVIDERS constant
  const providerInfo = PROVIDERS[account.provider as keyof typeof PROVIDERS] || {
    name: account.provider,
    icon: "🔑",
    color: "#3F3F46",
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
      `Are you sure you want to delete ${account.name}?`
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
      className={`bg-zinc-800 rounded-xl p-4 border border-zinc-700 flex items-center justify-between gap-4 transition-colors ${
        account.is_active ? "border-l-4 border-green-500" : ""
      }`}
    >
      {/* Left side: Provider icon (emoji), provider name + account name */}
      <div className="flex items-center gap-3.5 min-w-0">
        <div
          className="w-10 h-10 rounded-lg flex items-center justify-center text-xl shrink-0 select-none shadow-sm"
          style={{ backgroundColor: providerInfo.color }}
        >
          <span role="img" aria-label={providerInfo.name}>
            {providerInfo.icon}
          </span>
        </div>

        <div className="flex flex-col min-w-0">
          <div className="flex items-center gap-2 flex-wrap">
            <span className="font-semibold text-zinc-100 text-sm truncate">
              {account.name}
            </span>
            {account.label && (
              <span className="px-2 py-0.5 text-xs font-medium rounded-full bg-zinc-700 text-zinc-300 shrink-0">
                {account.label}
              </span>
            )}
          </div>
          <span className="text-xs text-zinc-400 truncate">
            {providerInfo.name}
          </span>
        </div>
      </div>

      {/* Right side: Active badge / Switch button, and Delete button */}
      <div className="flex items-center gap-2.5 shrink-0">
        {account.is_active ? (
          <span className="px-2.5 py-1 text-xs font-semibold rounded-full bg-green-500/10 text-green-400 border border-green-500/20 flex items-center gap-1.5 select-none">
            <span className="w-1.5 h-1.5 rounded-full bg-green-400" />
            Active
          </span>
        ) : (
          <button
            type="button"
            onClick={handleSwitch}
            disabled={isSwitching}
            className="px-3 py-1.5 text-xs font-medium rounded-lg bg-zinc-700 hover:bg-zinc-600 text-zinc-200 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1.5"
          >
            {isSwitching && (
              <svg
                className="animate-spin h-3.5 w-3.5 text-zinc-200"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                />
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
            )}
            <span>{isSwitching ? "Switching..." : "Switch"}</span>
          </button>
        )}

        <button
          type="button"
          onClick={handleDelete}
          disabled={isDeleting}
          aria-label="Delete"
          title="Delete account"
          className="px-2.5 py-1.5 text-xs font-medium rounded-lg text-red-400 hover:text-red-300 hover:bg-red-500/10 border border-transparent hover:border-red-500/20 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-3.5 w-3.5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
            />
          </svg>
          <span>Delete</span>
        </button>
      </div>
    </div>
  );
};

export default AccountCard;
