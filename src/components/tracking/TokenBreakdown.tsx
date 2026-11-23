export default function TokenBreakdown() {
  return (
    <div className="flex flex-col items-center justify-center rounded-lg border border-neutral-800 bg-neutral-900/60 p-6 text-center text-neutral-400">
      <div className="mb-3 rounded-full bg-neutral-800/80 p-3 text-neutral-400">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="h-6 w-6"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          strokeWidth={1.5}
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z"
          />
        </svg>
      </div>
      <h3 className="text-sm font-medium text-neutral-200">
        Token Breakdown - Coming in Phase 2
      </h3>
      <p className="mt-1 text-xs text-neutral-500">
        Detailed token usage and cost breakdown will be available in the next phase.
      </p>
    </div>
  );
}
