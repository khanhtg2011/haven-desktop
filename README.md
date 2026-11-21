# ⬡ Haven Desktop (Ultra-Optimized Edition)

[![Release](https://img.shields.io/github/v/release/khanhtg2011/haven-desktop?style=flat-square&color=blue)](https://github.com/khanhtg2011/haven-desktop/releases/latest)
[![Build Status](https://img.shields.io/github/actions/workflow/status/khanhtg2011/haven-desktop/build.yml?branch=main&style=flat-square)](https://github.com/khanhtg2011/haven-desktop/actions)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-orange.svg?style=flat-square)](LICENSE)

The official multi-platform desktop client for **[Haven](https://github.com/khanhtg2011/haven)**, deeply optimized for gaming, low latency voice reliability, and minimal CPU & memory footprint.

---

## 🚀 Key Highlights & Architectural Optimizations

### 🎙️ 1. Voice-First Decoupled Architecture
- **Unthrottled Background Audio**: WebRTC voice audio operates in a prioritized background loop with OS `powerSaveBlocker`, preventing Windows and Linux from suspending or stuttering audio when playing games or running full-screen applications.
- **Time-Critical Native Capture**: Native C++ audio capture addon (`WASAPI` on Windows, `PulseAudio/PipeWire` on Linux) runs with `THREAD_PRIORITY_TIME_CRITICAL` to eliminate crackling and frame drops under 100% CPU loads.

### 💤 2. Dynamic Chat Hibernation
- **Intelligent Sleep Mode**: When Haven Desktop is minimized or hidden behind a full-screen game, the Chat UI automatically enters deep hibernation:
  - Pauses all CSS animations, video embeds, and GIF decodes.
  - Implements `content-visibility: hidden` on message elements.
  - Flushes Chromium session cache and invokes Garbage Collection to trim memory footprint down to minimum.
- **Zero-Disruption Chat Reload**: Chat messages can be reloaded or refreshed on demand without disconnecting or resetting your active WebRTC voice call.

### ⚡ 3. Hardware-Accelerated Zero-Copy GPU Pipeline
- Direct3D 11/12 and VA-API GPU video frames enabled with Zero-Copy Direct Memory Buffers.
- Hardware video encoding/decoding for HEVC, AV1, and H.264 screen sharing with low latency and near-zero CPU overhead.

### 🎮 4. In-Game Voice Overlay & Global Hotkeys
- **Transparent HUD**: Click-through transparent in-game overlay showing who is talking and recent channel messages while in fullscreen borderless games.
- **Global Hotkeys**: Instant Push-to-Talk (PTT) and Push-to-Mute (PTM) listener powered by native hooks (`uiohook-napi`).

### 🎵 5. Per-Application Audio Sharing & Rich Presence
- Share audio from a **specific window or game** without streaming desktop sounds or background notifications.
- Discord Rich Presence and Steam/Spotify activity status support.

---

## 📥 Download & Installation

Pre-built binaries for all major platforms are available on GitHub Releases:

| Platform | Format | Description | Download |
|:---|:---|:---|:---|
| **Windows** | `.exe` | One-Click NSIS Installer (Windows 10 / 11) | [Download Setup (.exe)](https://github.com/khanhtg2011/haven-desktop/releases/latest) |
| **Linux** | `.AppImage` | Universal portable package (No install needed) | [Download AppImage](https://github.com/khanhtg2011/haven-desktop/releases/latest) |
| **Linux** | `.deb` | Debian / Ubuntu / Mint package | [Download .deb](https://github.com/khanhtg2011/haven-desktop/releases/latest) |
| **macOS** | `.dmg` | Apple Silicon (M1/M2/M3/M4) & Intel | [Download DMG](https://github.com/khanhtg2011/haven-desktop/releases/latest) |

### Quick Run Instructions:
- **Windows**: Run `Haven.Setup.X.X.X.exe` → Installs and launches automatically with start menu & desktop shortcuts.
- **Linux (AppImage)**:
  ```bash
  chmod +x Haven-*.AppImage
  ./Haven-*.AppImage
  ```
- **Linux (.deb)**:
  ```bash
  sudo dpkg -i haven-desktop_*_amd64.deb
  ```

---

## 🛠️ Development & Building from Source

### Prerequisites
- **Node.js**: `v22+`
- **npm**: `v10+`
- **C/C++ Build Tools**:
  - **Windows**: Visual Studio C++ Build Tools (Desktop development with C++)
  - **Linux**: `libpulse-dev`, `libx11-dev`, `libxtst-dev`, `libxinerama-dev`, `libxrandr-dev`

### Installation
```bash
# Clone repository
git clone https://github.com/khanhtg2011/haven-desktop.git
cd haven-desktop

# Install dependencies
npm install
```

### Running in Development
```bash
npm run dev
```

### Building Installers Locally
```bash
# Build native C++ audio addon
npm run build:native

# Build Windows installer (.exe)
npm run build:win

# Build Linux packages (.AppImage, .deb)
npm run build:linux
```

---

## 🤝 Ecosystem
- **Haven Music Bot & Discord Bridge**: Lightweight WebRTC audio companion bots.

---

## 📄 License
This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.
