pub mod commands;
pub mod tray;
pub mod native_overlay;

use commands::*;
use tauri::{
    webview::{PermissionResponse, WebviewWindowBuilder},
    AppHandle, Manager,
};

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

            // 1. Construct Main Window from config, bind bridge script & auto-permissions
            if let Some(win_cfg) = app.config().app.windows.first() {
                let mut builder = WebviewWindowBuilder::from_config(app_handle, win_cfg)?;
                builder = builder
                    .initialization_script(BRIDGE_SCRIPT)
                    .on_permission_request(|_webview, _kind| PermissionResponse::Allow);
                let window = builder.build()?;

                // Close-to-Tray: Hide to tray instead of terminating when user clicks Close (✕)
                let win_clone = window.clone();
                window.on_window_event(move |event| {
                    match event {
                        tauri::WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            let _ = win_clone.hide();
                            let _ = win_clone.eval("if (window.gc) window.gc();");
                            tray::trim_working_set();
                        }
                        tauri::WindowEvent::Focused(false) => {
                            // Deep Working Set Trim when user is gaming or switching windows
                            let _ = win_clone.eval("if (window.gc) window.gc();");
                            tray::trim_working_set();
                        }
                        _ => {}
                    }
                });
            }

            // 2. Initialize Pure Native Win32 / GDI+ In-Game HUD (Zero WebView2, ~3MB RAM)
            native_overlay::init();

            // 3. Periodic Background RAM Working Set Trimmer
            // Triggers V8 JS Garbage Collection and flushes working set across WebView2 processes every 30s
            let bg_app = app_handle.clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    if let Some(main_win) = bg_app.get_webview_window("main") {
                        let _ = main_win.eval("if (window.gc) window.gc();");
                    }
                    std::thread::sleep(std::time::Duration::from_millis(150));
                    tray::trim_working_set();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            window_minimize,
            window_maximize,
            window_close,
            get_shortcuts_config,
            set_shortcuts_config,
            stop_audio_capture,
            get_server_history,
            switch_server,
            update_voice_state,
            report_voice_overlay,
            is_voice_overlay_enabled,
            set_voice_overlay_enabled,
            voice_overlay_set_editing,
            voice_overlay_snap,
            toggle_voice_overlay_command,
            toggle_voice_overlay_persistent,
            clear_overlay_messages,
            voice_overlay_done
        ])
        .run(tauri::generate_context!())
        .expect("error while running haven desktop application");
}
