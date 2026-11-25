import React from "react";

export interface NavItem {
  id: string;
  label: string;
  icon: string;
}

export const navItems: NavItem[] = [
  { id: "accounts", label: "Accounts", icon: "👥" },
  { id: "dashboard", label: "Dashboard", icon: "📊" },
  { id: "settings", label: "Settings", icon: "⚙️" },
];

export interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ activeTab, onTabChange }) => {
  return (
    <aside className="w-56 h-full bg-zinc-900 border-r border-zinc-700 flex flex-col justify-between select-none">
      <div className="flex flex-col">
        {/* Header / Brand Area (drag region for Tauri window) */}
        <div
          data-tauri-drag-region=""
          className="flex items-center gap-2.5 px-4 py-4 border-b border-zinc-700/50 cursor-default"
        >
          <span data-tauri-drag-region="" className="text-base select-none">
            ⚡
          </span>
          <span
            data-tauri-drag-region=""
            className="text-lg font-bold text-zinc-100 tracking-tight"
          >
            Switchboard
          </span>
        </div>

        {/* Navigation Items */}
        <nav className="p-3 space-y-1">
          {navItems.map((item) => {
            const isActive = activeTab === item.id;
            return (
              <button
                key={item.id}
                type="button"
                onClick={() => onTabChange(item.id)}
                className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer text-left ${
                  isActive
                    ? "bg-zinc-700/50 text-blue-400"
                    : "text-zinc-300 hover:bg-zinc-700"
                }`}
              >
                <span className="text-base leading-none">{item.icon}</span>
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Footer / Version */}
      <div className="p-4 border-t border-zinc-700/50 text-xs text-zinc-500">
        v0.1.0
      </div>
    </aside>
  );
};

export default Sidebar;
