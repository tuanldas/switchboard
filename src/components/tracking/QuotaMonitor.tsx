interface QuotaMonitorProps {
  className?: string;
}

export default function QuotaMonitor({ className = '' }: QuotaMonitorProps) {
  return (
    <div
      className={`flex min-h-[160px] flex-col items-center justify-center rounded-xl border border-neutral-800 bg-neutral-900/60 p-6 text-center ${className}`}
    >
      <div className="flex flex-col items-center gap-2">
        <svg
          className="h-8 w-8 text-neutral-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="m12 14 4-4"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M3.34 19a10 10 0 1 1 17.32 0"
          />
        </svg>
        <p className="text-sm font-medium text-neutral-400">
          Quota Monitor - Coming in Phase 3
        </p>
      </div>
    </div>
  );
}
