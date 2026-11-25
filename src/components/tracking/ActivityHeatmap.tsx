import React from "react";

interface ActivityHeatmapProps {
  className?: string;
}

const days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const slots = Array.from({ length: 14 }, (_, i) => `${i + 8}h`);

const heatmapData = [
  [0, 1, 2, 4, 3, 2, 1, 0, 1, 3, 4, 2, 1, 0],
  [1, 2, 3, 4, 4, 3, 2, 1, 2, 4, 3, 1, 0, 0],
  [0, 0, 1, 2, 3, 4, 4, 3, 2, 1, 2, 3, 1, 0],
  [1, 3, 4, 4, 3, 2, 1, 2, 4, 4, 3, 2, 1, 1],
  [2, 4, 4, 3, 2, 1, 0, 1, 3, 4, 4, 3, 2, 0],
  [0, 1, 2, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 0],
  [0, 0, 1, 1, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0],
];

const intensityColors = [
  "bg-white/[0.03] border-white/[0.04]",
  "bg-cyan-500/25 border-cyan-500/30",
  "bg-indigo-500/40 border-indigo-500/50",
  "bg-purple-500/60 border-purple-500/70",
  "bg-pink-500/80 border-pink-500 shadow-[0_0_8px_rgba(236,72,153,0.4)]",
];

export const ActivityHeatmap: React.FC<ActivityHeatmapProps> = ({ className = "" }) => {
  return (
    <div className={`glass-card rounded-2xl p-5 border border-white/5 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-sm font-semibold text-white tracking-wide">
            Agent Interaction Matrix
          </h3>
          <p className="text-xs text-zinc-400 mt-0.5">
            Prompt frequency & CLI session bursts across the week
          </p>
        </div>

        <div className="flex items-center gap-1.5 text-[10px] text-zinc-400 font-mono">
          <span>Less</span>
          {intensityColors.map((cls, idx) => (
            <span key={idx} className={`w-2.5 h-2.5 rounded-sm ${cls}`} />
          ))}
          <span>More</span>
        </div>
      </div>

      {/* Grid */}
      <div className="overflow-x-auto">
        <div className="min-w-[480px]">
          {/* Header hours */}
          <div className="grid grid-cols-[36px_repeat(14,1fr)] gap-1 mb-1.5 text-[9px] font-mono text-zinc-500 text-center">
            <span></span>
            {slots.map((s) => (
              <span key={s}>{s}</span>
            ))}
          </div>

          {/* Days rows */}
          <div className="space-y-1.5">
            {days.map((day, dIdx) => (
              <div key={day} className="grid grid-cols-[36px_repeat(14,1fr)] gap-1 items-center">
                <span className="text-[10px] font-mono text-zinc-400 text-right pr-2">
                  {day}
                </span>
                {heatmapData[dIdx].map((lvl, hIdx) => (
                  <div
                    key={hIdx}
                    title={`${day} @ ${slots[hIdx]}: Level ${lvl} intensity`}
                    className={`h-4 rounded-md border transition-all duration-150 hover:scale-125 hover:z-20 cursor-pointer ${intensityColors[lvl]}`}
                  />
                ))}
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Heatmap summary pill */}
      <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-zinc-400">
        <span className="flex items-center gap-1.5">
          <span className="text-amber-400">🔥</span>
          <span>Peak activity: <strong>Thursday 14:00 - 18:00</strong></span>
        </span>
        <span className="font-mono text-zinc-500 text-[11px]">7.2k tokens / peak hr</span>
      </div>
    </div>
  );
};

export default ActivityHeatmap;
