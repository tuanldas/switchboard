import React, { useState } from "react";
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer } from "recharts";

interface CostChartProps {
  className?: string;
}

const mockDailyData = [
  { day: "Mon", cost: 0.28, tokens: 18400 },
  { day: "Tue", cost: 0.65, tokens: 42100 },
  { day: "Wed", cost: 0.42, tokens: 31000 },
  { day: "Thu", cost: 0.89, tokens: 59300 },
  { day: "Fri", cost: 1.15, tokens: 78500 },
  { day: "Sat", cost: 0.54, tokens: 36200 },
  { day: "Sun", cost: 0.72, tokens: 49800 },
];

export const CostChart: React.FC<CostChartProps> = ({ className = "" }) => {
  const [timeRange, setTimeRange] = useState<"7D" | "30D">("7D");

  return (
    <div className={`glass-card rounded-2xl p-5 border border-white/5 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div>
          <div className="flex items-center gap-2">
            <h3 className="text-sm font-semibold text-white tracking-wide">
              Token Spend & Volume
            </h3>
            <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
              Live Rate
            </span>
          </div>
          <p className="text-xs text-zinc-400 mt-0.5">
            Estimated USD usage across all active agent runs
          </p>
        </div>

        {/* Filters */}
        <div className="flex items-center gap-1 bg-black/40 p-0.5 rounded-lg border border-white/5 text-xs">
          {(["7D", "30D"] as const).map((range) => (
            <button
              key={range}
              onClick={() => setTimeRange(range)}
              className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors ${
                timeRange === range
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {range}
            </button>
          ))}
        </div>
      </div>

      {/* Chart */}
      <div className="h-48 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={mockDailyData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
            <defs>
              <linearGradient id="costGradient" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#6366f1" stopOpacity={0.4} />
                <stop offset="95%" stopColor="#6366f1" stopOpacity={0.0} />
              </linearGradient>
            </defs>
            <XAxis
              dataKey="day"
              stroke="#64748b"
              fontSize={11}
              tickLine={false}
              axisLine={false}
            />
            <YAxis
              stroke="#64748b"
              fontSize={11}
              tickLine={false}
              axisLine={false}
              tickFormatter={(v) => `$${v}`}
            />
            <Tooltip
              content={({ active, payload }) => {
                if (active && payload && payload.length) {
                  const data = payload[0].payload;
                  return (
                    <div className="bg-[#0f172a] border border-indigo-500/30 rounded-xl p-3 shadow-xl text-xs space-y-1">
                      <div className="font-semibold text-white">{data.day}</div>
                      <div className="text-indigo-400 font-mono">
                        Cost: ${(data.cost).toFixed(2)}
                      </div>
                      <div className="text-cyan-400 font-mono">
                        Tokens: {(data.tokens).toLocaleString()}
                      </div>
                    </div>
                  );
                }
                return null;
              }}
            />
            <Area
              type="monotone"
              dataKey="cost"
              stroke="#6366f1"
              strokeWidth={2}
              fillOpacity={1}
              fill="url(#costGradient)"
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      {/* Footer KPI */}
      <div className="grid grid-cols-3 gap-2 pt-3 mt-2 border-t border-white/5 text-center">
        <div>
          <span className="text-[10px] text-zinc-500 uppercase font-medium">Daily Avg</span>
          <p className="text-xs font-semibold text-zinc-200 mt-0.5">$0.66</p>
        </div>
        <div>
          <span className="text-[10px] text-zinc-500 uppercase font-medium">7D Total</span>
          <p className="text-xs font-semibold text-indigo-400 mt-0.5">$4.65</p>
        </div>
        <div>
          <span className="text-[10px] text-zinc-500 uppercase font-medium">Burn Velocity</span>
          <p className="text-xs font-semibold text-emerald-400 mt-0.5">-14% vs wk</p>
        </div>
      </div>
    </div>
  );
};

export default CostChart;
