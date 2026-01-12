# Le Chat Desktop

A lightweight cross-platform desktop app for Mistral AI's Le Chat (https://chat.mistral.ai/chat) built with Tauri v2.

## Project Goals

Leading AI platforms like ChatGPT and GitHub Copilot offer dedicated desktop applications that provide a more professional, practical user experience with benefits like local inference support, better system integration, and improved workflows. This project aims to demonstrate the value of a native desktop experience for Le Chat and encourage Mistral AI to develop an official desktop client that meets the same standard.

As a European, I'm also concerned about Europe's growing dependence on US and Chinese AI infrastructure. Supporting and elevating European AI companies like Mistral AI is crucial for technological sovereignty. A world-class desktop application would help Mistral compete more effectively on the global stage and provide a viable alternative to US-dominated platforms.

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
MIT - See [LICENSE](LICENSE) file for details.

## Attribution & Trademark Notice
- **Le Chat & Mistral AI**: This is an unofficial third-party desktop wrapper for [Mistral AI's Le Chat](https://chat.mistral.ai).
- **Mistral Logo**: The Mistral AI logo used in this application is the property of Mistral AI. This project is not affiliated with, endorsed by, or connected to Mistral AI.
- **Disclaimer**: This application is a wrapper that loads Mistral's web service. All AI responses and services are provided by Mistral AI.

**Note to Mistral AI:** I'd love to collaborate or discuss this project with your team. If you have feedback, concerns, or would like to work together on improving desktop access to Le Chat, please reach out via GitHub Issues or contact me directly.
