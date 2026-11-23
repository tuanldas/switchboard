interface CostChartProps {
  className?: string;
}

export default function CostChart({ className = '' }: CostChartProps) {
  return (
    <div
      className={`flex flex-col items-center justify-center rounded-lg border border-neutral-800 bg-neutral-900/50 p-8 text-center ${className}`}
    >
      <div className="mb-2 text-3xl select-none" role="img" aria-label="chart">
        📊
      </div>
      <p className="text-sm font-medium text-neutral-400">
        Cost Chart - Coming in Phase 2
      </p>
    </div>
  );
}
