#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Url, WebviewUrl, WebviewWindowBuilder};

const APP_URL: &str = "https://chat.mistral.ai/chat";

fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            open_main_window(handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}

fn open_main_window(app: &AppHandle) -> tauri::Result<()> {
    let url = Url::parse(APP_URL).expect("valid Mistral Chat url");

    WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
        .title("Le Chat")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .decorations(true)
        .minimizable(true)
        .maximizable(true)
        .visible(true)
        .devtools(false)
        .build()?;

    Ok(())
}
