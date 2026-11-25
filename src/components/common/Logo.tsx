import React from "react";

interface LogoProps {
  size?: number;
  className?: string;
  showText?: boolean;
}

export const Logo: React.FC<LogoProps> = ({
  size = 28,
  className = "",
  showText = false,
}) => {
  return (
    <div className={`flex items-center gap-2.5 select-none ${className}`}>
      <div
        className="relative flex items-center justify-center shrink-0 rounded-xl p-1 bg-gradient-to-br from-cyan-500/20 via-indigo-500/20 to-purple-500/20 border border-indigo-500/30 shadow-[0_0_15px_rgba(99,102,241,0.25)]"
        style={{ width: size, height: size }}
      >
        <svg
          viewBox="0 0 100 100"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className="w-full h-full"
        >
          <defs>
            <linearGradient id="sw-cyan" x1="0" y1="0" x2="100" y2="100" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stopColor="#06b6d4" />
              <stop offset="100%" stopColor="#3b82f6" />
            </linearGradient>
            <linearGradient id="sw-indigo" x1="0" y1="0" x2="100" y2="100" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stopColor="#6366f1" />
              <stop offset="100%" stopColor="#8b5cf6" />
            </linearGradient>
            <linearGradient id="sw-purple" x1="0" y1="0" x2="100" y2="100" gradientUnits="userSpaceOnUse">
              <stop offset="0%" stopColor="#a855f7" />
              <stop offset="100%" stopColor="#ec4899" />
            </linearGradient>
            <filter id="sw-glow" x="-20%" y="-20%" width="140%" height="140%">
              <feGaussianBlur stdDeviation="3" result="blur" />
              <feComposite in="SourceGraphic" in2="blur" operator="over" />
            </filter>
          </defs>

          {/* Background Matrix Grid */}
          <rect x="12" y="12" width="76" height="76" rx="20" fill="#090d16" stroke="url(#sw-indigo)" strokeWidth="3" strokeOpacity="0.4" />

          {/* Channel Tracks */}
          <rect x="24" y="30" width="52" height="8" rx="4" fill="#1e293b" />
          <rect x="24" y="46" width="52" height="8" rx="4" fill="#1e293b" />
          <rect x="24" y="62" width="52" height="8" rx="4" fill="#1e293b" />

          {/* Active Rails */}
          <rect x="24" y="30" width="34" height="8" rx="4" fill="url(#sw-purple)" />
          <rect x="36" y="46" width="40" height="8" rx="4" fill="url(#sw-cyan)" />
          <rect x="24" y="62" width="28" height="8" rx="4" fill="url(#sw-indigo)" />

          {/* Switch Knobs with Core Glow */}
          <circle cx="58" cy="34" r="8" fill="#ffffff" filter="url(#sw-glow)" />
          <circle cx="58" cy="34" r="5" fill="#ec4899" />

          <circle cx="36" cy="50" r="8" fill="#ffffff" filter="url(#sw-glow)" />
          <circle cx="36" cy="50" r="5" fill="#06b6d4" />

          <circle cx="52" cy="66" r="8" fill="#ffffff" filter="url(#sw-glow)" />
          <circle cx="52" cy="66" r="5" fill="#6366f1" />
        </svg>
      </div>

      {showText && (
        <div className="flex flex-col">
          <span className="font-extrabold tracking-tight text-white bg-gradient-to-r from-cyan-400 via-indigo-300 to-purple-400 bg-clip-text text-transparent text-base">
            Switchboard
          </span>
          <span className="text-[10px] text-zinc-400 font-medium tracking-wider uppercase -mt-0.5">
            AI Cockpit
          </span>
        </div>
      )}
    </div>
  );
};

export default Logo;
