import { create } from "zustand";
import { getUsageSummary } from "../lib/tauri";

export interface TrackingState {
  summary: { total_tokens: number; total_cost: number; today_tokens: number; today_cost: number } | null;
  loading: boolean;
  fetchSummary: () => Promise<void>;
}

export const useTrackingStore = create<TrackingState>((set) => ({
  summary: null,
  loading: false,
  fetchSummary: async () => {
    set({ loading: true });
    try {
      const summary = await getUsageSummary();
      set({ summary, loading: false });
    } catch (error) {
      console.error("Failed to fetch usage summary:", error);
      set({ loading: false });
    }
  },
}));

export default useTrackingStore;
