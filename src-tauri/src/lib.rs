pub mod commands;
pub mod tray;

use commands::*;
use tauri::{Manager, WebviewWindowBuilder};

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

            // Construct main window from config and bind system-level initialization script
            if let Some(win_cfg) = app.config().app.windows.first() {
                let mut builder = WebviewWindowBuilder::from_config(app_handle, win_cfg)?;
                builder = builder.initialization_script(BRIDGE_SCRIPT);
                let window = builder.build()?;

                // Close-to-Tray: Hide to tray instead of terminating when user clicks Close (✕)
                let win_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win_clone.hide();
                    }
                });
            }

            log::info!("Haven Desktop 2.0 (Tauri v2 + Rust) loaded with full desktop parity!");
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
