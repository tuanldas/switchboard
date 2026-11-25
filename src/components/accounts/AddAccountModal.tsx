import React, { useState, useEffect } from "react";
import useAccountStore from "../../stores/accountStore";

export interface AddAccountModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const providerOptions = [
  { id: "claude_code", name: "Claude Code", icon: "🤖", color: "#D97757", tag: "Anthropic" },
  { id: "codex", name: "Codex CLI", icon: "🧠", color: "#10A37F", tag: "OpenAI" },
  { id: "cursor", name: "Cursor IDE", icon: "⚡", color: "#007ACC", tag: "Anysphere" },
  { id: "antigravity", name: "Antigravity", icon: "🚀", color: "#4285F4", tag: "Google" },
];

export const AddAccountModal: React.FC<AddAccountModalProps> = ({
  isOpen,
  onClose,
}) => {
  const [provider, setProvider] = useState<string>("claude_code");
  const [name, setName] = useState<string>("");
  const [label, setLabel] = useState<string>("");
  const [submitting, setSubmitting] = useState<boolean>(false);

  const { addNewAccount } = useAccountStore();

  useEffect(() => {
    if (isOpen) {
      setProvider("claude_code");
      setName("");
      setLabel("");
      setSubmitting(false);
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (submitting || !name.trim()) return;

    setSubmitting(true);
    try {
      if (addNewAccount) {
        await addNewAccount(provider, name.trim(), label.trim() || undefined);
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
      className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-200"
      onClick={onClose}
    >
      <div
        className="glass-panel rounded-3xl p-6 w-[420px] max-w-full text-zinc-100 shadow-2xl border border-white/10"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-5">
          <div className="flex items-center gap-2.5">
            <span className="text-xl">🎛️</span>
            <div>
              <h2 className="text-base font-bold text-white tracking-tight">
                Add AI Profile
              </h2>
              <p className="text-xs text-zinc-400">
                Snapshots active session credentials into Keychain
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-zinc-400 hover:text-white p-1 rounded-lg text-sm"
          >
            ✕
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-xs font-medium text-zinc-300 mb-2">
              Select Provider
            </label>
            <div className="grid grid-cols-2 gap-2">
              {providerOptions.map((opt) => {
                const isSelected = provider === opt.id;
                return (
                  <button
                    key={opt.id}
                    type="button"
                    onClick={() => setProvider(opt.id)}
                    className={`flex items-center gap-2.5 p-2.5 rounded-xl border text-left transition-all cursor-pointer ${
                      isSelected
                        ? "bg-indigo-600/20 border-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.25)]"
                        : "bg-black/30 border-white/5 hover:border-white/10"
                    }`}
                  >
                    <span className="text-xl">{opt.icon}</span>
                    <div className="min-w-0">
                      <div className="text-xs font-semibold text-white truncate">
                        {opt.name}
                      </div>
                      <div className="text-[10px] text-zinc-400 font-mono">
                        {opt.tag}
                      </div>
                    </div>
                  </button>
                );
              })}
            </div>
          </div>

          <div>
            <label className="block text-xs font-medium text-zinc-300 mb-1.5">
              Profile Name *
            </label>
            <input
              type="text"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Work Claude Pro, Codex Team, Client A"
              className="w-full px-3.5 py-2.5 rounded-xl bg-black/40 border border-white/10 text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-indigo-500 transition-colors"
            />
          </div>

          <div>
            <label className="block text-xs font-medium text-zinc-300 mb-1.5">
              Custom Tag / Label (Optional)
            </label>
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              placeholder="e.g. Primary, Secondary, High-Rate"
              className="w-full px-3.5 py-2.5 rounded-xl bg-black/40 border border-white/10 text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-indigo-500 transition-colors"
            />
          </div>

          <div className="flex items-center justify-end gap-2.5 pt-3 border-t border-white/5">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 rounded-xl text-xs font-medium text-zinc-400 hover:text-white hover:bg-white/5 transition-colors cursor-pointer"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={submitting || !name.trim()}
              className="px-5 py-2 rounded-xl text-xs font-semibold text-white bg-gradient-to-r from-cyan-600 via-indigo-600 to-purple-600 hover:from-cyan-500 hover:to-purple-500 transition-all cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed shadow-[0_0_12px_rgba(99,102,241,0.35)]"
            >
              {submitting ? "Snapshotting..." : "Save Profile"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default AddAccountModal;
