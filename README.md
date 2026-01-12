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

## Releasing

Create and push a version tag to trigger automated builds:
```bash
git tag v1.0.0
git push origin v1.0.0
```

Installers will be created automatically and attached to the GitHub release.

## License
MIT
