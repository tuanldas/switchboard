import { create } from "zustand";
import { AppSettings, getSettings, updateSettings } from "../lib/tauri";

export interface SettingsState {
  settings: AppSettings;
  loading: boolean;
  fetchSettings: () => Promise<void>;
  saveSettings: (settings: AppSettings) => Promise<void>;
}

export const defaultSettings: AppSettings = {
  theme: "system",
  auto_start: true,
  auto_switch_on_quota: false,
  language: "en",
  budget_limit: null,
  notifications_enabled: true,
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: defaultSettings,
  loading: false,
  fetchSettings: async () => {
    set({ loading: true });
    try {
      const settings = await getSettings();
      set({ settings, loading: false });
    } catch (error) {
      console.error("Failed to fetch settings:", error);
      set({ loading: false });
    }
  },
  saveSettings: async (settings: AppSettings) => {
    set({ loading: true });
    try {
      await updateSettings(settings);
      await get().fetchSettings();
    } catch (error) {
      console.error("Failed to save settings:", error);
      set({ loading: false });
      throw error;
    }
  },
}));

export const settingsStore = useSettingsStore;
export default useSettingsStore;
