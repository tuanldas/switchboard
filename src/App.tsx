import React, { useState } from "react";
import AccountList from "./components/accounts/AccountList";
import UsageDashboard from "./components/tracking/UsageDashboard";
import SettingsPanel from "./components/settings/SettingsPanel";
import Sidebar, { navItems } from "./components/layout/Sidebar";

export type NavTab = "accounts" | "dashboard" | "settings";

export { Sidebar, navItems };

/**
 * Main application component for Status Bar (v0.1.0).
 *
 * Layout:
 * - Left sidebar (w-56, bg-zinc-900) displaying:
 *   - App name "Status Bar" at top
 *   - Navigation tabs with emoji icon + label:
 *     - "👥 Accounts" (id: "accounts")
 *     - "📊 Dashboard" (id: "dashboard")
 *     - "⚙️ Settings" (id: "settings")
 *   - App version "v0.1.0" at the bottom
 * - Main content area taking the rest of the width (flex-1 bg-zinc-800 text-white)
 * - Default active tab is "accounts"
 */
export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<string>("accounts");

  const renderContent = () => {
    switch (activeTab) {
      case "accounts":
        return <AccountList />;
      case "dashboard":
        return <UsageDashboard />;
      case "settings":
        return <SettingsPanel />;
      default:
        return <AccountList />;
    }
  };

  return (
    <div
      className="flex h-screen w-screen overflow-hidden bg-zinc-800 text-white select-none"
      data-testid="app-container"
    >
      {/* Left sidebar (w-56) with dark background (bg-zinc-900) showing app name "Status Bar" at top and nav items */}
      <Sidebar activeTab={activeTab} onTabChange={setActiveTab} />

      {/* Main content area taking the rest of the width */}
      <main
        className="flex-1 h-full overflow-y-auto bg-zinc-800 text-white"
        role="main"
        data-testid="main-content"
      >
        {renderContent()}
      </main>
    </div>
  );
};

export default App;
