pub mod commands;
pub mod tray;

use commands::*;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

const BRIDGE_SCRIPT: &str = include_str!("../../src/tauri-bridge/haven-bridge.js");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            let app_handle = app.handle();
            let _ = tray::setup_tray(app_handle);

            // Configure main window with bridge initialization script
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(BRIDGE_SCRIPT);
            }

            log::info!("Haven Desktop (Tauri v2) initialized successfully!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            window_minimize,
            window_maximize,
            window_close,
            get_shortcuts_config,
            set_shortcuts_config,
            update_voice_state,
            is_audio_capture_supported,
            stop_audio_capture,
            get_server_history,
            switch_server
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
