import React, { useState, useEffect } from "react";
import { useSettingsStore } from "../../stores/settingsStore";
import type { AppSettings } from "../../lib/tauri";

interface ToggleProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  id?: string;
  name?: string;
  "aria-label"?: string;
}

const Toggle: React.FC<ToggleProps> = ({
  checked,
  onChange,
  id,
  name,
  "aria-label": ariaLabel,
}) => {
  return (
    <label className="relative inline-flex items-center cursor-pointer select-none">
      <input
        type="checkbox"
        id={id}
        name={name}
        aria-label={ariaLabel}
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
        className="sr-only peer"
      />
      <div
        className={`w-11 h-6 rounded-full transition-colors duration-200 ease-in-out relative peer-focus:ring-2 peer-focus:ring-blue-500/40 ${
          checked ? "bg-blue-600" : "bg-zinc-600"
        }`}
      >
        <span
          className={`absolute top-0.5 left-0.5 bg-white rounded-full h-5 w-5 transition-transform duration-200 ease-in-out shadow-sm ${
            checked ? "translate-x-5" : "translate-x-0"
          }`}
        />
      </div>
    </label>
  );
};

export const SettingsPanel: React.FC = () => {
  const { settings, fetchSettings, saveSettings } = useSettingsStore();

  const [formState, setFormState] = useState<AppSettings>({
    theme: "system",
    language: "en",
    auto_start: true,
    auto_switch_on_quota: false,
    notifications_enabled: true,
    budget_limit: null,
  });

  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<string | null>(null);

  // Fetch settings on mount
  useEffect(() => {
    fetchSettings();
  }, [fetchSettings]);

  // Sync local form state when store settings change
  useEffect(() => {
    if (settings) {
      setFormState({
        theme: settings.theme ?? "system",
        language: settings.language ?? "en",
        auto_start: settings.auto_start ?? true,
        auto_switch_on_quota: settings.auto_switch_on_quota ?? false,
        notifications_enabled: settings.notifications_enabled ?? true,
        budget_limit: settings.budget_limit ?? null,
      });
    }
  }, [settings]);

  const updateField = <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ) => {
    setFormState((prev) => ({
      ...prev,
      [key]: value,
    }));
    setSaveStatus(null);
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await saveSettings(formState);
      setSaveStatus("Settings saved successfully");
      setTimeout(() => {
        setSaveStatus(null);
      }, 3000);
    } catch (err) {
      console.error("Failed to save settings:", err);
      setSaveStatus("Failed to save settings");
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="w-full max-w-3xl mx-auto p-6 space-y-6 text-zinc-100">
      {/* Header */}
      <div className="flex items-center justify-between pb-2 border-b border-zinc-800">
        <h1 className="text-2xl font-bold text-zinc-100">Settings</h1>
      </div>

      {/* Group 1 - Appearance */}
      <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700 space-y-4">
        <h2 className="text-base font-semibold text-zinc-200">Appearance</h2>

        <div className="flex items-center justify-between py-2 border-b border-zinc-700/50">
          <div>
            <label
              htmlFor="settings-theme"
              className="text-sm font-medium text-zinc-200 block"
            >
              Theme
            </label>
            <span className="text-xs text-zinc-400">
              Select interface color theme
            </span>
          </div>
          <select
            id="settings-theme"
            value={formState.theme}
            onChange={(e) => updateField("theme", e.target.value)}
            className="bg-zinc-700 border border-zinc-600 text-zinc-100 text-sm rounded-lg px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500 cursor-pointer min-w-[140px]"
          >
            <option value="system">System</option>
            <option value="dark">Dark</option>
            <option value="light">Light</option>
          </select>
        </div>

        <div className="flex items-center justify-between py-2">
          <div>
            <label
              htmlFor="settings-language"
              className="text-sm font-medium text-zinc-200 block"
            >
              Language
            </label>
            <span className="text-xs text-zinc-400">
              Select display language
            </span>
          </div>
          <select
            id="settings-language"
            value={formState.language}
            onChange={(e) => updateField("language", e.target.value)}
            className="bg-zinc-700 border border-zinc-600 text-zinc-100 text-sm rounded-lg px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500 cursor-pointer min-w-[140px]"
          >
            <option value="en">English</option>
            <option value="vi">Tiếng Việt</option>
          </select>
        </div>
      </div>

      {/* Group 2 - Behavior */}
      <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700 space-y-4">
        <h2 className="text-base font-semibold text-zinc-200">Behavior</h2>

        <div className="flex items-center justify-between py-2 border-b border-zinc-700/50">
          <div>
            <span className="text-sm font-medium text-zinc-200 block">
              Auto-start on boot
            </span>
            <span className="text-xs text-zinc-400">
              Launch application automatically when computer starts
            </span>
          </div>
          <Toggle
            id="settings-auto-start"
            name="auto_start"
            aria-label="Auto-start on boot"
            checked={formState.auto_start}
            onChange={(checked) => updateField("auto_start", checked)}
          />
        </div>

        <div className="flex items-center justify-between py-2 border-b border-zinc-700/50">
          <div>
            <span className="text-sm font-medium text-zinc-200 block">
              Auto-switch on quota exhaustion
            </span>
            <span className="text-xs text-zinc-400">
              Automatically switch to backup account when limits are hit
            </span>
          </div>
          <Toggle
            id="settings-auto-switch"
            name="auto_switch_on_quota"
            aria-label="Auto-switch on quota exhaustion"
            checked={formState.auto_switch_on_quota}
            onChange={(checked) => updateField("auto_switch_on_quota", checked)}
          />
        </div>

        <div className="flex items-center justify-between py-2">
          <div>
            <span className="text-sm font-medium text-zinc-200 block">
              Enable notifications
            </span>
            <span className="text-xs text-zinc-400">
              Receive desktop alerts for quota warnings and account switches
            </span>
          </div>
          <Toggle
            id="settings-notifications"
            name="notifications_enabled"
            aria-label="Enable notifications"
            checked={formState.notifications_enabled}
            onChange={(checked) => updateField("notifications_enabled", checked)}
          />
        </div>
      </div>

      {/* Group 3 - Budget */}
      <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700 space-y-4">
        <h2 className="text-base font-semibold text-zinc-200">Budget</h2>

        <div className="flex items-center justify-between py-2">
          <div>
            <label
              htmlFor="settings-budget-limit"
              className="text-sm font-medium text-zinc-200 block"
            >
              Monthly budget limit
            </label>
            <span className="text-xs text-zinc-400">
              Set spending ceiling for usage tracking (USD)
            </span>
          </div>
          <div className="relative flex items-center">
            <span className="absolute left-3 text-zinc-400 text-sm pointer-events-none select-none">
              $
            </span>
            <input
              id="settings-budget-limit"
              type="number"
              min="0"
              step="0.01"
              placeholder="No limit"
              value={
                formState.budget_limit !== null &&
                formState.budget_limit !== undefined
                  ? formState.budget_limit
                  : ""
              }
              onChange={(e) => {
                const val = e.target.value.trim();
                updateField(
                  "budget_limit",
                  val === "" ? null : parseFloat(val)
                );
              }}
              className="bg-zinc-700 border border-zinc-600 text-zinc-100 text-sm rounded-lg pl-7 pr-3 py-2 w-36 outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Save Button */}
      <div className="flex items-center justify-between pt-2">
        <div>
          {saveStatus && (
            <span
              className={`text-sm ${
                saveStatus.includes("Failed")
                  ? "text-red-400"
                  : "text-emerald-400"
              }`}
            >
              {saveStatus}
            </span>
          )}
        </div>
        <button
          type="button"
          onClick={handleSave}
          disabled={isSaving}
          className="px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white font-medium text-sm rounded-lg transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isSaving ? "Saving..." : "Save"}
        </button>
      </div>
    </div>
  );
};

export default SettingsPanel;
