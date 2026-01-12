# Le Chat Desktop

A lightweight cross-platform desktop app for Mistral AI's Le Chat (https://chat.mistral.ai/chat) built with Tauri v2.

## Features
- Native app for Windows, macOS, and Linux
- Keyboard shortcuts: `Ctrl/Cmd+R` to reload, `Ctrl/Cmd+Shift+I` for DevTools
- Automatic cross-platform builds via GitHub Actions

## Installation

Download the latest release from [Releases](https://github.com/Chri5thian/Le_Chat_App/releases):
- **Windows**: `.msi` or `.exe`
- **macOS**: `.dmg` (Intel & Apple Silicon)
- **Linux**: `.deb` or `.AppImage`

## Development

**Prerequisites:**
- [Rust](https://rustup.rs/) toolchain
- Tauri CLI: `cargo install tauri-cli --version ^2 --locked`
- Linux: `libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

**Run:**
```bash
cargo tauri dev
```

**Build:**
```bash
cargo tauri build
```

## License
MIT - See [LICENSE](LICENSE) file for details.

## Attribution & Trademark Notice
- **Le Chat & Mistral AI**: This is an unofficial third-party desktop wrapper for [Mistral AI's Le Chat](https://chat.mistral.ai).
- **Mistral Logo**: The Mistral AI logo used in this application is the property of Mistral AI. This project is not affiliated with, endorsed by, or connected to Mistral AI.
- **Disclaimer**: This application is a wrapper that loads Mistral's web service. All AI responses and services are provided by Mistral AI.

If you are a Mistral AI representative and have concerns about this project, please open an issue.
