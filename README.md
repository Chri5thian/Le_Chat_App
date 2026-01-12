# Mistral Chat Tauri Wrapper

A minimal Tauri v2 Rust app that opens the Mistral Chat web experience (https://chat.mistral.ai/chat) in a native shell with quick shortcuts for reload and devtools.

## Prerequisites
- Rust toolchain (stable)
- Tauri CLI v2: `cargo install tauri-cli --version ^2 --locked`
- Linux dependencies for Tauri (WebKitGTK, etc.)

## Run in dev
```bash
cargo tauri dev
```
This will open the window pointed at the hosted Mistral Chat site.

## Build release
```bash
cargo tauri build
```
Output binaries live under `src-tauri/target/release/`.

## Shortcuts
- Reload: `Ctrl/Cmd + R`
- Toggle devtools: `Ctrl/Cmd + Shift + I`

## Notes
- Remote content is loaded directly; CSP is disabled in config to avoid blocking the remote app. Tighten this if you host content yourself.
- If the remote site changes domains/assets, update `APP_URL` in `src-tauri/src/main.rs`.
