import React from "react";

interface QuotaMonitorProps {
  className?: string;
}

export const QuotaMonitor: React.FC<QuotaMonitorProps> = ({ className = "" }) => {
  const fiveHourUsage = 64;
  const weeklyUsage = 42;

  return (
    <div className={`glass-card rounded-2xl p-5 border border-white/5 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div>
          <div className="flex items-center gap-2">
            <h3 className="text-sm font-semibold text-white tracking-wide">
              Rate Limit & Quota Radar
            </h3>
            <span className="text-[10px] font-mono px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 flex items-center gap-1">
              <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
              Pace Normal
            </span>
          </div>
          <p className="text-xs text-zinc-400 mt-0.5">
            Active rolling window telemetry for current account
          </p>
        </div>

        <div className="text-right">
          <span className="text-[10px] text-zinc-500 uppercase font-mono">Next Reset In</span>
          <p className="text-xs font-mono font-bold text-cyan-400">1h 48m</p>
        </div>
      </div>

      {/* 5-Hour Window Meter */}
      <div className="space-y-2 mb-4">
        <div className="flex items-center justify-between text-xs">
          <span className="text-zinc-300 font-medium flex items-center gap-1.5">
            <span>⏱️</span> 5-Hour Rolling Window
          </span>
          <span className="font-mono text-zinc-200">
            <span className="text-cyan-400 font-bold">{fiveHourUsage}%</span> / 100%
          </span>
        </div>
        <div className="h-2.5 w-full bg-black/40 rounded-full overflow-hidden p-0.5 border border-white/5">
          <div
            className="h-full rounded-full bg-gradient-to-r from-cyan-500 via-indigo-500 to-purple-500 transition-all duration-500 shadow-[0_0_8px_rgba(99,102,241,0.5)]"
            style={{ width: `${fiveHourUsage}%` }}
          />
        </div>
        <div className="flex justify-between text-[10px] text-zinc-500 font-mono">
          <span>0%</span>
          <span>Warning 80%</span>
          <span>100% (Auto-Switch)</span>
        </div>
      </div>

      {/* Weekly Plan Limit */}
      <div className="space-y-2 mb-4">
        <div className="flex items-center justify-between text-xs">
          <span className="text-zinc-300 font-medium flex items-center gap-1.5">
            <span>📅</span> Weekly Allowance Pool
          </span>
          <span className="font-mono text-zinc-200">
            <span className="text-indigo-400 font-bold">{weeklyUsage}%</span> (Safe)
          </span>
        </div>
        <div className="h-2 w-full bg-black/40 rounded-full overflow-hidden p-0.5 border border-white/5">
          <div
            className="h-full rounded-full bg-gradient-to-r from-indigo-500 to-blue-500 transition-all duration-500"
            style={{ width: `${weeklyUsage}%` }}
          />
        </div>
      </div>

      {/* Pace Verdict Banner */}
      <div className="p-3 rounded-xl bg-gradient-to-r from-cyan-950/40 via-indigo-950/30 to-purple-950/20 border border-cyan-500/20 flex items-center justify-between text-xs">
        <div className="flex items-center gap-2">
          <span className="text-base">🛡️</span>
          <div>
            <div className="font-medium text-cyan-200">Auto-Switch Ready</div>
            <div className="text-[11px] text-zinc-400">
              Switches to secondary profile if current token limit hits 95%
            </div>
          </div>
        </div>
        <span className="text-[10px] font-mono px-2 py-1 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 font-semibold shrink-0">
          ARMED
        </span>
      </div>
    </div>
  );
};

export default QuotaMonitor;
