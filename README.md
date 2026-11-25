# 🎛️ Switchboard

> **All-in-one status bar switcher & token tracker for AI coding tools.**  
> Seamlessly switch between multiple accounts and monitor your token quotas, usage, and costs directly from your system tray / menu bar.

---

## ⚡ Features

- 🔄 **1-Click Account Switching**: Instant switching across multiple profiles for:
  - **Claude Code**
  - **OpenAI Codex**
  - **Cursor IDE**
  - **Antigravity**
- 📊 **Usage & Cost Tracking**:
  - Live token counters (Prompt, Completion, Cache Read/Write)
  - Cost estimation computed against real model rates
  - Daily, weekly, and monthly breakdowns
- ⏱️ **Rate Limit & Quota Monitoring**:
  - 5-hour rolling window and weekly limit tracking
  - Pace verdict to warn before quota exhaustion
  - Native desktop notifications
- 🔐 **Privacy-First & Secure**:
  - Local-first architecture (no telemetry, no third-party cloud servers)
  - **AES-256-GCM** encryption for parked credentials
  - Integrated with native OS keychains (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- 🖥️ **Cross-Platform**:
  - macOS (Apple Silicon & Intel)
  - Windows 10/11
  - Linux (Ubuntu / Debian / AppImage)

---

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v20+) & [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/) (1.80+)
- Platform-specific build tools:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `libsecret-1-dev`
  - **Windows**: WebView2 runtime & MSVC build tools

### Development

```bash
# Clone the repository
git clone https://github.com/tuanldas/switchboard.git
cd switchboard

# Install dependencies
pnpm install

# Run development mode
pnpm tauri dev
```

### Build Releases

```bash
# Build native release bundle for your current OS
pnpm tauri build
```

The compiled binary and installers will be generated under:
`src-tauri/target/release/bundle/`

---

## 🛠️ Tech Stack

- **Framework:** [Tauri 2](https://v2.tauri.app/) (Rust Backend + Native Webview)
- **Frontend:** React 19, TypeScript, Tailwind CSS, Zustand, Recharts
- **Security:** AES-256-GCM, OS Keychain via `keyring`
- **Database:** Local SQLite (`rusqlite`)

---

## 📄 License

MIT License © 2026 tuanldas
