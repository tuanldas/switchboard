import { useEffect } from "react";
import { useTrackingStore } from "@/stores/trackingStore";

export function UsageDashboard() {
  const { summary, fetchSummary } = useTrackingStore();

  useEffect(() => {
    fetchSummary?.();
  }, [fetchSummary]);

  return (
    <div className="w-full p-6 space-y-6 text-zinc-100 bg-zinc-900 min-h-screen">
      <div>
        <h1 className="text-2xl font-bold text-white">Dashboard</h1>
      </div>

      <div className="grid grid-cols-2 gap-4">
        {/* Card 1: Total Tokens */}
        <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700">
          <div className="text-sm text-zinc-400">Total Tokens</div>
          <div className="text-2xl font-bold text-white mt-1">
            {summary?.total_tokens || 0}
          </div>
        </div>

        {/* Card 2: Total Cost */}
        <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700">
          <div className="text-sm text-zinc-400">Total Cost</div>
          <div className="text-2xl font-bold text-white mt-1">
            {"$" + (summary?.total_cost || 0).toFixed(4)}
          </div>
        </div>

        {/* Card 3: Today Tokens */}
        <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700">
          <div className="text-sm text-zinc-400">Today Tokens</div>
          <div className="text-2xl font-bold text-white mt-1">
            {summary?.today_tokens || 0}
          </div>
        </div>

        {/* Card 4: Today Cost */}
        <div className="bg-zinc-800 rounded-xl p-5 border border-zinc-700">
          <div className="text-sm text-zinc-400">Today Cost</div>
          <div className="text-2xl font-bold text-white mt-1">
            {"$" + (summary?.today_cost || 0).toFixed(4)}
          </div>
        </div>
      </div>

      {/* Coming Soon Section for Phase 2 */}
      <div className="bg-zinc-800 rounded-xl p-6 border border-zinc-700">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-lg font-semibold text-white">Coming Soon</h2>
          <span className="text-xs font-medium px-2.5 py-0.5 rounded-full bg-zinc-700 text-zinc-300 border border-zinc-600">
            Phase 2
          </span>
        </div>
        <p className="text-sm text-zinc-400 mb-5">
          Detailed metrics and advanced usage analytics planned for Phase 2:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="p-4 rounded-lg bg-zinc-900/50 border border-zinc-700/60">
            <h3 className="text-sm font-semibold text-zinc-200 mb-1">Activity heatmap</h3>
            <p className="text-xs text-zinc-400">
              Interactive visualization of token consumption and coding activity across days and weeks.
            </p>
          </div>

          <div className="p-4 rounded-lg bg-zinc-900/50 border border-zinc-700/60">
            <h3 className="text-sm font-semibold text-zinc-200 mb-1">Cost breakdown by model</h3>
            <p className="text-xs text-zinc-400">
              Detailed cost and token analysis partitioned by individual AI models and providers.
            </p>
          </div>

          <div className="p-4 rounded-lg bg-zinc-900/50 border border-zinc-700/60">
            <h3 className="text-sm font-semibold text-zinc-200 mb-1">Session timeline</h3>
            <p className="text-xs text-zinc-400">
              Chronological log of prompt interactions, context caching metrics, and session costs.
            </p>
          </div>

          <div className="p-4 rounded-lg bg-zinc-900/50 border border-zinc-700/60">
            <h3 className="text-sm font-semibold text-zinc-200 mb-1">Quota monitoring</h3>
            <p className="text-xs text-zinc-400">
              Proactive quota tracking, threshold notifications, and automated fallback switching.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default UsageDashboard;
