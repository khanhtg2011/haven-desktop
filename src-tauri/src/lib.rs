pub mod commands;
pub mod tray;

use commands::*;
use tauri::{
    webview::{PermissionResponse, WebviewWindowBuilder},
    AppHandle, Manager, WebviewUrl,
};

const BRIDGE_SCRIPT: &str = include_str!("../../src/tauri-bridge/haven-bridge.js");
const OVERLAY_BRIDGE_SCRIPT: &str = r#"
(function() {
  if (window.__TAURI__ && window.__TAURI__.event) {
    window.voiceOverlay = {
      onView: function(fn) {
        window.__TAURI__.event.listen('voice-overlay:view', function(e) { fn(e.payload); });
      },
      onState: function(fn) {
        window.__TAURI__.event.listen('voice-overlay:state', function(e) { fn(e.payload); });
      },
      done: function() {
        if (window.__TAURI__.core) {
          window.__TAURI__.core.invoke('voice_overlay_done');
        }
      }
    };
  }
})();
"#;

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

            // 1. Construct Main Window from config and bind system-level initialization script + auto permissions
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
                            tray::trim_working_set();
                        }
                        tauri::WindowEvent::Focused(false) => {
                            // Deep Working Set Trim when user is gaming or switching windows
                            tray::trim_working_set();
                        }
                        _ => {}
                    }
                });
            }

            // 2. Construct In-Game Voice Overlay Window (Always-on-top, Click-Through)
            let overlay_builder = WebviewWindowBuilder::new(
                app_handle,
                "overlay",
                WebviewUrl::App("voice-overlay.html".into()),
            )
            .title("Haven Voice Overlay")
            .inner_size(340.0, 620.0)
            .position(20.0, 20.0)
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .visible(false)
            .initialization_script(OVERLAY_BRIDGE_SCRIPT);

            if let Ok(overlay_win) = overlay_builder.build() {
                let _ = overlay_win.set_ignore_cursor_events(true);
            }

            log::info!("Haven Desktop 2.0 (Tauri v2 + Rust) loaded with full parity & Voice Overlay!");
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
            is_voice_overlay_enabled,
            set_voice_overlay_enabled,
            toggle_voice_overlay_command,
            report_voice_overlay,
            voice_overlay_done,
            voice_overlay_set_editing,
            voice_overlay_snap,
            is_audio_capture_supported,
            stop_audio_capture,
            get_server_history,
            switch_server
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
