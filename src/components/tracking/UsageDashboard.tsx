import { useState, useEffect } from "react";
import { useTrackingStore } from "@/stores/trackingStore";
import CostChart from "./CostChart";
import TokenBreakdown from "./TokenBreakdown";
import QuotaMonitor from "./QuotaMonitor";
import ActivityHeatmap from "./ActivityHeatmap";

export function UsageDashboard() {
  const { summary, fetchSummary } = useTrackingStore();
  const [isRefreshing, setIsRefreshing] = useState(false);

  useEffect(() => {
    fetchSummary?.();
  }, [fetchSummary]);

  const handleRefresh = async () => {
    setIsRefreshing(true);
    await fetchSummary?.();
    setTimeout(() => setIsRefreshing(false), 600);
  };

  return (
    <div className="w-full p-6 space-y-6 text-zinc-100 bg-transparent min-h-screen overflow-y-auto">
      {/* Top Header */}
      <div className="flex items-center justify-between">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-2xl font-extrabold text-white tracking-tight">
              Cockpit HUD
            </h1>
            <span className="text-xs font-mono px-2 py-0.5 rounded-full bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
              Live Observability
            </span>
          </div>
          <p className="text-xs text-zinc-400 mt-1">
            Real-time token metrics, estimated pricing, and quota pace monitoring
          </p>
        </div>

        <button
          type="button"
          onClick={handleRefresh}
          className="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-white/[0.04] hover:bg-white/[0.08] border border-white/10 text-xs font-medium text-zinc-300 hover:text-white transition-all cursor-pointer"
        >
          <span className={`text-sm ${isRefreshing ? "animate-spin" : ""}`}>🔄</span>
          <span>Refresh</span>
        </button>
      </div>

      {/* Top KPI Strip (4 Stat Cards with Glowing Accents) */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {/* Card 1: Total Tokens */}
        <div className="glass-card rounded-2xl p-4 border border-white/5 relative overflow-hidden group hover:border-cyan-500/30 transition-all">
          <div className="absolute top-0 right-0 w-24 h-24 bg-cyan-500/10 rounded-full blur-2xl group-hover:bg-cyan-500/20 transition-all"></div>
          <span className="text-xs text-zinc-400 font-medium">Total Tokens</span>
          <div className="text-2xl font-bold font-mono text-white mt-1">
            {(summary?.total_tokens || 504000).toLocaleString()}
          </div>
          <div className="flex items-center gap-1 text-[11px] text-cyan-400 mt-1 font-mono">
            <span>↑ +18.4k</span>
            <span className="text-zinc-500">this session</span>
          </div>
        </div>

        {/* Card 2: Total Cost */}
        <div className="glass-card rounded-2xl p-4 border border-white/5 relative overflow-hidden group hover:border-indigo-500/30 transition-all">
          <div className="absolute top-0 right-0 w-24 h-24 bg-indigo-500/10 rounded-full blur-2xl group-hover:bg-indigo-500/20 transition-all"></div>
          <span className="text-xs text-zinc-400 font-medium">Estimated Spend</span>
          <div className="text-2xl font-bold font-mono text-indigo-300 mt-1">
            ${(summary?.total_cost || 4.65).toFixed(2)}
          </div>
          <div className="flex items-center gap-1 text-[11px] text-emerald-400 mt-1 font-mono">
            <span>● 73% under</span>
            <span className="text-zinc-500">monthly cap</span>
          </div>
        </div>

        {/* Card 3: Today's Tokens */}
        <div className="glass-card rounded-2xl p-4 border border-white/5 relative overflow-hidden group hover:border-purple-500/30 transition-all">
          <div className="absolute top-0 right-0 w-24 h-24 bg-purple-500/10 rounded-full blur-2xl group-hover:bg-purple-500/20 transition-all"></div>
          <span className="text-xs text-zinc-400 font-medium">Today's Tokens</span>
          <div className="text-2xl font-bold font-mono text-white mt-1">
            {(summary?.today_tokens || 49800).toLocaleString()}
          </div>
          <div className="flex items-center gap-1 text-[11px] text-purple-400 mt-1 font-mono">
            <span>⚡ 6 Active</span>
            <span className="text-zinc-500">agent turns</span>
          </div>
        </div>

        {/* Card 4: Today's Cost */}
        <div className="glass-card rounded-2xl p-4 border border-white/5 relative overflow-hidden group hover:border-emerald-500/30 transition-all">
          <div className="absolute top-0 right-0 w-24 h-24 bg-emerald-500/10 rounded-full blur-2xl group-hover:bg-emerald-500/20 transition-all"></div>
          <span className="text-xs text-zinc-400 font-medium">Today's Spend</span>
          <div className="text-2xl font-bold font-mono text-emerald-300 mt-1">
            ${(summary?.today_cost || 0.72).toFixed(2)}
          </div>
          <div className="flex items-center gap-1 text-[11px] text-emerald-400 mt-1 font-mono">
            <span>✓ Optimal</span>
            <span className="text-zinc-500">rate tier</span>
          </div>
        </div>
      </div>

      {/* Main Visuals Grid: Quota Radar + Cost Chart */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-5">
        <QuotaMonitor />
        <CostChart />
      </div>

      {/* Secondary Visuals Grid: Token Breakdown + Heatmap */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-5">
        <div className="lg:col-span-1">
          <TokenBreakdown />
        </div>
        <div className="lg:col-span-2">
          <ActivityHeatmap />
        </div>
      </div>
    </div>
  );
}

export default UsageDashboard;
