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

    #[cfg(target_os = "linux")]
    let _window = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
        .title("Le Chat")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .decorations(true)
        .minimizable(true)
        .maximizable(true)
        .visible(true)
        .devtools(false)
        .additional_browser_args("--class=le-chat")
        .on_navigation(|url| {
            let url_str = url.as_str();
            // Allow all mistral.ai domains and subdomains
            if url_str.contains("mistral.ai") || url_str.contains("mistral.com") {
                return true;
            }
            // Allow auth/oauth domains
            if url_str.contains("auth0.com")
                || url_str.contains("accounts.google.com")
                || url_str.contains("login.microsoftonline.com")
                || url_str.contains("github.com/login")
            {
                return true;
            }
            // Allow CDNs and API endpoints
            if url_str.contains("cloudflare.com")
                || url_str.contains("cloudfront.net")
                || url_str.starts_with("data:")
                || url_str.starts_with("blob:")
            {
                return true;
            }
            // Open other external links in default browser
            if url_str.starts_with("http://") || url_str.starts_with("https://") {
                let _ = open::that(url_str);
                return false;
            }
            true
        })
        .build()?;

    #[cfg(not(target_os = "linux"))]
    let _window = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
        .title("Le Chat")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .decorations(true)
        .minimizable(true)
        .maximizable(true)
        .visible(true)
        .devtools(false)
        .on_navigation(|url| {
            let url_str = url.as_str();
            // Allow all mistral.ai domains and subdomains
            if url_str.contains("mistral.ai") || url_str.contains("mistral.com") {
                return true;
            }
            // Allow auth/oauth domains
            if url_str.contains("auth0.com")
                || url_str.contains("accounts.google.com")
                || url_str.contains("login.microsoftonline.com")
                || url_str.contains("github.com/login")
            {
                return true;
            }
            // Allow CDNs and API endpoints
            if url_str.contains("cloudflare.com")
                || url_str.contains("cloudfront.net")
                || url_str.starts_with("data:")
                || url_str.starts_with("blob:")
            {
                return true;
            }
            // Open other external links in default browser
            if url_str.starts_with("http://") || url_str.starts_with("https://") {
                let _ = open::that(url_str);
                return false;
            }
            true
        })
        .build()?;

    Ok(())
}
