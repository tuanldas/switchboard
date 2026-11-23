import React, { useState, useEffect } from "react";
import useAccountStore from "../../stores/accountStore";
import AccountCard from "./AccountCard";
import AddAccountModal from "./AddAccountModal";

export const AccountList: React.FC = () => {
  const [showAddModal, setShowAddModal] = useState<boolean>(false);
  const { accounts, loading, error, fetchAccounts } = useAccountStore();

  useEffect(() => {
    fetchAccounts();
  }, [fetchAccounts]);

  return (
    <div className="w-full p-6 text-zinc-100">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-white">Accounts</h1>
        <button
          type="button"
          onClick={() => setShowAddModal(true)}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors cursor-pointer"
        >
          Add Account
        </button>
      </div>

      {/* Loading state */}
      {loading && (
        <div className="text-zinc-400 py-4">Loading...</div>
      )}

      {/* Error state */}
      {error && (
        <div className="text-red-500 mb-4">{error}</div>
      )}

      {/* Empty state */}
      {!loading && !error && accounts.length === 0 && (
        <div className="text-zinc-400 py-8 text-center">
          No accounts yet. Add your first account to get started.
        </div>
      )}

      {/* Accounts list */}
      {!loading && accounts.length > 0 && (
        <div className="grid grid-cols-1 gap-4">
          {accounts.map((account) => (
            <AccountCard key={account.id} account={account} />
          ))}
        </div>
      )}

      {/* Add Account Modal */}
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
