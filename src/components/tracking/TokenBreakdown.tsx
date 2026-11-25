import React from "react";
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip } from "recharts";

const tokenData = [
  { name: "Prompt / Input", value: 168000, color: "#3b82f6" },
  { name: "Completion / Output", value: 94000, color: "#a855f7" },
  { name: "Cache Read", value: 242000, color: "#10b981" },
];

export const TokenBreakdown: React.FC = () => {
  const totalTokens = tokenData.reduce((acc, curr) => acc + curr.value, 0);

  return (
    <div className="glass-card rounded-2xl p-5 border border-white/5">
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-sm font-semibold text-white tracking-wide">
          Token Composition
        </h3>
        <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
          Cache Saver ON
        </span>
      </div>
      <p className="text-xs text-zinc-400 mb-4">
        Breakdown of prompt, generation, and prompt caching tokens
      </p>

      {/* Donut Chart with Center KPI */}
      <div className="relative h-44 w-full flex items-center justify-center">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={tokenData}
              innerRadius={50}
              outerRadius={70}
              paddingAngle={4}
              dataKey="value"
            >
              {tokenData.map((entry, index) => (
                <Cell key={`cell-${index}`} fill={entry.color} stroke="#0f172a" strokeWidth={2} />
              ))}
            </Pie>
            <Tooltip
              content={({ active, payload }) => {
                if (active && payload && payload.length) {
                  const data = payload[0].payload;
                  return (
                    <div className="bg-[#0f172a] border border-white/10 rounded-lg p-2 text-xs shadow-xl">
                      <span className="text-zinc-200 font-medium">{data.name}</span>
                      <p className="font-mono text-white mt-0.5">
                        {data.value.toLocaleString()} tokens ({((data.value / totalTokens) * 100).toFixed(1)}%)
                      </p>
                    </div>
                  );
                }
                return null;
              }}
            />
          </PieChart>
        </ResponsiveContainer>

        {/* Center Label */}
        <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
          <span className="text-lg font-bold text-white font-mono leading-none">
            {(totalTokens / 1000).toFixed(0)}k
          </span>
          <span className="text-[10px] text-zinc-400 mt-0.5">Total</span>
        </div>
      </div>

      {/* Legend & Savings Banner */}
      <div className="space-y-2 mt-2">
        <div className="flex items-center justify-between text-xs px-1">
          <div className="flex items-center gap-1.5">
            <span className="w-2.5 h-2.5 rounded-full bg-blue-500"></span>
            <span className="text-zinc-400">Prompt</span>
          </div>
          <span className="font-mono text-zinc-200">168k (33%)</span>
        </div>
        <div className="flex items-center justify-between text-xs px-1">
          <div className="flex items-center gap-1.5">
            <span className="w-2.5 h-2.5 rounded-full bg-purple-500"></span>
            <span className="text-zinc-400">Output</span>
          </div>
          <span className="font-mono text-zinc-200">94k (19%)</span>
        </div>
        <div className="flex items-center justify-between text-xs px-1">
          <div className="flex items-center gap-1.5">
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-500"></span>
            <span className="text-zinc-400">Cache Read</span>
          </div>
          <span className="font-mono text-emerald-400">242k (48%)</span>
        </div>

        <div className="mt-3 p-2.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-between text-[11px]">
          <span className="text-emerald-300 font-medium flex items-center gap-1.5">
            ⚡ Context Cache Savings
          </span>
          <span className="font-mono text-emerald-400 font-bold">~$14.80 saved</span>
        </div>
      </div>
    </div>
  );
};

export default TokenBreakdown;
