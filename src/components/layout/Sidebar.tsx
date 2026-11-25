import React from "react";
import Logo from "../common/Logo";
import useAccountStore from "../../stores/accountStore";

export interface NavItem {
  id: string;
  label: string;
  icon: string;
  badge?: string;
}

export const navItems: NavItem[] = [
  { id: "accounts", label: "Profiles", icon: "🎛️" },
  { id: "dashboard", label: "Cockpit HUD", icon: "📊" },
  { id: "settings", label: "System Config", icon: "⚙️" },
];

export interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ activeTab, onTabChange }) => {
  const { accounts, activeAccount } = useAccountStore();

  return (
    <aside className="w-60 h-full bg-[#0d121f] border-r border-white/5 flex flex-col justify-between select-none relative z-10">
      <div className="flex flex-col">
        {/* Brand Header */}
        <div
          data-tauri-drag-region=""
          className="flex items-center justify-between px-4 py-4 border-b border-white/5 cursor-default bg-white/[0.02]"
        >
          <Logo size={32} showText={true} />
          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
            v0.1
          </span>
        </div>

        {/* Live Active Profile Pill */}
        <div className="px-3 pt-3.5 pb-1">
          <div className="px-3 py-2 rounded-xl bg-gradient-to-r from-emerald-500/10 via-teal-500/5 to-transparent border border-emerald-500/20 flex items-center justify-between">
            <div className="flex items-center gap-2 min-w-0">
              <span className="relative flex h-2 w-2 shrink-0">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
              <div className="flex flex-col min-w-0">
                <span className="text-[10px] text-emerald-400/80 font-medium uppercase tracking-wider">
                  Active Link
                </span>
                <span className="text-xs font-semibold text-zinc-200 truncate">
                  {activeAccount ? activeAccount.name : "Standby (No active)"}
                </span>
              </div>
            </div>
            <span className="text-[10px] text-zinc-500 font-mono">⌘K</span>
          </div>
        </div>

        {/* Navigation Items */}
        <nav className="p-3 space-y-1">
          <div className="text-[10px] font-semibold uppercase tracking-wider text-zinc-500 px-3 py-1.5">
            Operations
          </div>
          {navItems.map((item, idx) => {
            const isActive = activeTab === item.id;
            return (
              <button
                key={item.id}
                type="button"
                onClick={() => onTabChange(item.id)}
                className={`w-full flex items-center justify-between px-3 py-2.5 rounded-xl text-sm font-medium transition-all duration-200 cursor-pointer ${
                  isActive
                    ? "bg-gradient-to-r from-indigo-600/30 via-indigo-600/20 to-transparent text-white border border-indigo-500/30 shadow-[0_0_12px_rgba(99,102,241,0.15)]"
                    : "text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04]"
                }`}
              >
                <div className="flex items-center gap-3">
                  <span className="text-base leading-none">{item.icon}</span>
                  <span>{item.label}</span>
                </div>
                {item.id === "accounts" && accounts.length > 0 && (
                  <span className="text-[10px] font-mono px-1.5 py-0.5 rounded-full bg-white/10 text-zinc-300">
                    {accounts.length}
                  </span>
                )}
                {item.id !== "accounts" && (
                  <span className="text-[10px] font-mono text-zinc-600">
                    ⌘{idx + 1}
                  </span>
                )}
              </button>
            );
          })}
        </nav>
      </div>

      {/* Footer / System Telemetry */}
      <div className="p-3.5 border-t border-white/5 bg-black/20">
        <div className="flex items-center justify-between text-[11px] text-zinc-400">
          <div className="flex items-center gap-1.5">
            <span className="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
            <span className="font-mono text-zinc-400">Local Daemon</span>
          </div>
          <span className="text-[10px] font-mono text-zinc-500">AES-256</span>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
