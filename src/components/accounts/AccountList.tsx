import React, { useState, useEffect } from "react";
import useAccountStore from "../../stores/accountStore";
import AccountCard from "./AccountCard";
import AddAccountModal from "./AddAccountModal";

export const AccountList: React.FC = () => {
  const [showAddModal, setShowAddModal] = useState<boolean>(false);
  const [filterProvider, setFilterProvider] = useState<string>("all");
  const [searchQuery, setSearchQuery] = useState<string>("");
  const { accounts, loading, error, fetchAccounts } = useAccountStore();

  useEffect(() => {
    fetchAccounts();
  }, [fetchAccounts]);

  const filteredAccounts = accounts.filter((acc) => {
    const matchesProvider =
      filterProvider === "all" || acc.provider === filterProvider;
    const matchesSearch =
      acc.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (acc.label && acc.label.toLowerCase().includes(searchQuery.toLowerCase()));
    return matchesProvider && matchesSearch;
  });

  return (
    <div className="w-full p-6 text-zinc-100 space-y-6 overflow-y-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-2xl font-extrabold text-white tracking-tight">
              AI Profiles & Accounts
            </h1>
            <span className="text-xs font-mono px-2 py-0.5 rounded-full bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
              {accounts.length} Profiles
            </span>
          </div>
          <p className="text-xs text-zinc-400 mt-1">
            Seamlessly switch sessions across Claude Code, Codex, Cursor, and Antigravity
          </p>
        </div>

        <button
          type="button"
          onClick={() => setShowAddModal(true)}
          className="flex items-center gap-2 px-4 py-2 text-sm font-semibold text-white bg-gradient-to-r from-cyan-600 via-indigo-600 to-purple-600 hover:from-cyan-500 hover:to-purple-500 rounded-xl transition-all duration-200 cursor-pointer shadow-[0_0_15px_rgba(99,102,241,0.35)]"
        >
          <span>✨</span>
          <span>Add Profile</span>
        </button>
      </div>

      {/* Filter and Search Bar */}
      <div className="flex items-center justify-between gap-3 flex-wrap">
        <div className="flex items-center gap-1.5 bg-black/40 p-1 rounded-xl border border-white/5 text-xs">
          {[
            { id: "all", label: "All" },
            { id: "claude_code", label: "🤖 Claude" },
            { id: "codex", label: "🧠 Codex" },
            { id: "cursor", label: "⚡ Cursor" },
            { id: "antigravity", label: "🚀 Antigravity" },
          ].map((item) => (
            <button
              key={item.id}
              onClick={() => setFilterProvider(item.id)}
              className={`px-3 py-1.5 rounded-lg text-xs font-medium transition-all ${
                filterProvider === item.id
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {item.label}
            </button>
          ))}
        </div>

        <div className="relative">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search profiles or labels..."
            className="w-56 px-3 py-1.5 pl-8 rounded-xl bg-black/40 border border-white/10 text-xs text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-indigo-500 transition-colors"
          />
          <span className="absolute left-2.5 top-2 text-zinc-500 text-xs">🔍</span>
        </div>
      </div>

      {loading && (
        <div className="text-zinc-400 py-8 text-center text-sm flex items-center justify-center gap-2">
          <span className="animate-spin">⏳</span> Syncing keychain profiles...
        </div>
      )}

      {error && (
        <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
          {error}
        </div>
      )}

      {!loading && !error && filteredAccounts.length === 0 && (
        <div className="glass-card rounded-2xl p-12 text-center border border-white/5 space-y-3">
          <div className="text-4xl">🎛️</div>
          <h3 className="text-base font-semibold text-white">
            {accounts.length === 0 ? "No Accounts Registered" : "No Matching Profiles"}
          </h3>
          <p className="text-xs text-zinc-400 max-w-sm mx-auto">
            {accounts.length === 0
              ? "Add your Claude Code, Codex, or Cursor profiles to enable 1-click account switching."
              : "Try adjusting your search query or provider filter."}
          </p>
          {accounts.length === 0 && (
            <button
              type="button"
              onClick={() => setShowAddModal(true)}
              className="mt-2 px-4 py-2 text-xs font-semibold text-white bg-indigo-600 hover:bg-indigo-500 rounded-xl transition-all cursor-pointer"
            >
              Add First Profile
            </button>
          )}
        </div>
      )}

      {!loading && filteredAccounts.length > 0 && (
        <div className="grid grid-cols-1 gap-3.5">
          {filteredAccounts.map((account) => (
            <AccountCard key={account.id} account={account} />
          ))}
        </div>
      )}

      {showAddModal && (
        <AddAccountModal
          isOpen={showAddModal}
          onClose={() => setShowAddModal(false)}
        />
      )}
    </div>
  );
};

export default AccountList;
