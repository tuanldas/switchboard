import React, { useState, useEffect, useMemo } from "react";
import useAccountStore from "../../stores/accountStore";
import PROVIDERS from "../../lib/constants";

export interface AddAccountModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AddAccountModal: React.FC<AddAccountModalProps> = ({
  isOpen,
  onClose,
}) => {
  // Provider list derived from constants
  const providerEntries = useMemo(() => {
    if (!PROVIDERS) return [];
    if (Array.isArray(PROVIDERS)) {
      return PROVIDERS.map((p) => ({
        id: p.id || p.key || p.provider || p.name,
        name: p.name,
        icon: p.icon,
      }));
    }
    return Object.entries(PROVIDERS).map(([key, info]) => ({
      id: key,
      name: (info as { name?: string; icon?: string })?.name ?? key,
      icon: (info as { name?: string; icon?: string })?.icon ?? "🤖",
    }));
  }, []);

  const defaultProvider = providerEntries[0]?.id || "claude_code";

  // Component state
  const [provider, setProvider] = useState<string>(defaultProvider);
  const [name, setName] = useState<string>("");
  const [label, setLabel] = useState<string>("");
  const [submitting, setSubmitting] = useState<boolean>(false);

  const { addNewAccount } = useAccountStore();

  // Reset form fields when modal opens
  useEffect(() => {
    if (isOpen) {
      setProvider(defaultProvider);
      setName("");
      setLabel("");
      setSubmitting(false);
    }
  }, [isOpen, defaultProvider]);

  // If !isOpen, return null
  if (!isOpen) {
    return null;
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (submitting) return;

    setSubmitting(true);
    try {
      if (addNewAccount) {
        await addNewAccount(provider, name, label);
      }
      onClose();
    } catch (error) {
      console.error("Failed to add account:", error);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4"
      onClick={onClose}
    >
      <div
        className="bg-zinc-800 rounded-2xl p-6 w-96 max-w-full text-zinc-100 shadow-xl border border-zinc-700/50"
        onClick={(e) => e.stopPropagation()}
      >
        <h2 className="text-xl font-bold text-zinc-100 mb-5">Add Account</h2>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* Provider Selection: 4 buttons in a 2x2 grid */}
          <div>
            <label className="block text-xs font-medium text-zinc-300 mb-2">
              Select Provider
            </label>
            <div className="grid grid-cols-2 gap-2.5">
              {providerEntries.map((p) => {
                const isSelected = provider === p.id;
                return (
                  <button
                    key={p.id}
                    type="button"
                    onClick={() => setProvider(p.id)}
                    className={`flex items-center gap-2.5 p-3 rounded-lg border text-left cursor-pointer transition-all ${
                      isSelected
                        ? "border-blue-500 bg-zinc-700/80 ring-2 ring-blue-500 text-zinc-100"
                        : "border-zinc-700 bg-zinc-700/40 hover:bg-zinc-700/70 text-zinc-300"
                    }`}
                  >
                    <span className="text-xl leading-none">{p.icon}</span>
                    <span className="text-sm font-medium truncate">
                      {p.name}
                    </span>
                  </button>
                );
              })}
            </div>
          </div>

          {/* Name input field */}
          <div>
            <label
              htmlFor="account-name"
              className="block text-xs font-medium text-zinc-300 mb-1.5"
            >
              Name
            </label>
            <input
              id="account-name"
              type="text"
              placeholder="Account name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
              className="w-full px-3 py-2 bg-zinc-700 border border-zinc-600 rounded-lg text-zinc-100 placeholder-zinc-400 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
            />
          </div>

          {/* Label input field (optional) */}
          <div>
            <label
              htmlFor="account-label"
              className="block text-xs font-medium text-zinc-300 mb-1.5"
            >
              Label{" "}
              <span className="text-zinc-400 text-xs font-normal">
                (optional)
              </span>
            </label>
            <input
              id="account-label"
              type="text"
              placeholder="e.g. Work, Personal"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="w-full px-3 py-2 bg-zinc-700 border border-zinc-600 rounded-lg text-zinc-100 placeholder-zinc-400 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
            />
          </div>

          {/* Bottom buttons */}
          <div className="flex items-center justify-end gap-3 pt-3">
            <button
              type="button"
              onClick={onClose}
              disabled={submitting}
              className="px-4 py-2 text-sm font-medium text-zinc-400 hover:text-zinc-100 hover:bg-zinc-700/50 rounded-lg transition-colors cursor-pointer disabled:opacity-50"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={submitting}
              className="px-4 py-2 text-sm font-medium bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Add Account
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default AddAccountModal;
