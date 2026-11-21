# ⬡ Haven Desktop (Rust Edition)

> High-performance client companion engine for **Haven**, rewritten from scratch in idiomatic Rust.

---

## Features
- **Cross-Platform Loopback Audio Capture**: WASAPI on Windows, ALSA/PipeWire on Linux, CoreAudio on macOS powered by `cpal`.
- **Global Push-To-Talk Hotkey Listener**: Instant response with zero input lag.
- **In-Game Voice Overlay**: Transparent HUD displaying active speaking participants.
- **Native Memory Footprint**: Less than 3 MB RAM.

---

## Quick Start

### 1. Run with Default Config
```bash
cargo run --release -- --server-url https://haven.khanhs.io.vn
```

### 2. Build Multi-Platform
```bash
cargo build --release
```

---

## License
AGPL-3.0
