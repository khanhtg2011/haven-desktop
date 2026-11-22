use tauri::{AppHandle, Emitter, Manager, State};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutsConfig {
    pub ptt_key: Option<String>,
    pub ptm_key: Option<String>,
    pub toggle_mute: Option<String>,
    pub toggle_deafen: Option<String>,
    pub overlay_toggle: Option<String>,
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        Self {
            ptt_key: None,
            ptm_key: None,
            toggle_mute: Some("Ctrl+Shift+M".into()),
            toggle_deafen: Some("Ctrl+Shift+D".into()),
            overlay_toggle: Some("Ctrl+Shift+O".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceOverlaySettings {
    pub enabled: bool,
    pub only_speaking: bool,
    pub show_messages: bool,
    pub scale: f64,
    pub opacity: f64,
    pub editing: bool,
    pub corner: String,
}

impl Default for VoiceOverlaySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            only_speaking: false,
            show_messages: true,
            scale: 1.0,
            opacity: 1.0,
            editing: false,
            corner: "top-left".into(),
        }
    }
}

pub struct AppState {
    pub shortcuts: Mutex<ShortcutsConfig>,
    pub server_history: Mutex<Vec<String>>,
    pub voice_state: Mutex<serde_json::Value>,
    pub overlay_settings: Mutex<VoiceOverlaySettings>,
    pub overlay_last_report: Mutex<Option<serde_json::Value>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            shortcuts: Mutex::new(ShortcutsConfig::default()),
            server_history: Mutex::new(vec!["https://haven.khanhs.io.vn".to_string()]),
            voice_state: Mutex::new(serde_json::json!({})),
            overlay_settings: Mutex::new(VoiceOverlaySettings::default()),
            overlay_last_report: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub fn get_app_version() -> String {
    "2.0.0-alpha.1".to_string()
}

#[tauri::command]
pub fn window_minimize(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn window_maximize(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_maximized().unwrap_or(false) {
            window.unmaximize().map_err(|e| e.to_string())?;
        } else {
            window.maximize().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn window_close(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_shortcuts_config(state: State<AppState>) -> ShortcutsConfig {
    state.shortcuts.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_shortcuts_config(state: State<AppState>, config: ShortcutsConfig) -> Result<(), String> {
    let mut lock = state.shortcuts.lock().unwrap();
    *lock = config;
    log::info!("Updated global shortcuts: {:?}", *lock);
    Ok(())
}

#[tauri::command]
pub fn update_voice_state(state: State<AppState>, voice_state: serde_json::Value) -> Result<(), String> {
    let mut lock = state.voice_state.lock().unwrap();
    *lock = voice_state;
    Ok(())
}

// ── Voice Overlay Commands ──────────────────────────────────────────────────

#[tauri::command]
pub fn is_voice_overlay_enabled(state: State<AppState>) -> bool {
    state.overlay_settings.lock().unwrap().enabled
}

#[tauri::command]
pub fn set_voice_overlay_enabled(app: AppHandle, state: State<AppState>, enabled: bool) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().enabled = enabled;
    if !enabled {
        if let Some(w) = app.get_webview_window("overlay") {
            let _ = w.hide();
        }
    } else {
        let last = state.overlay_last_report.lock().unwrap().clone();
        if let Some(r) = last {
            if r.get("inVoice").and_then(|v| v.as_bool()).unwrap_or(false) {
                if let Some(w) = app.get_webview_window("overlay") {
                    let _ = w.show();
                    let _ = app.emit_to("overlay", "voice-overlay:state", &r);
                }
            }
        }
    }
    send_overlay_view(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn toggle_voice_overlay_command(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    let current = state.overlay_settings.lock().unwrap().enabled;
    set_voice_overlay_enabled(app, state, !current)
}

#[tauri::command]
pub fn report_voice_overlay(app: AppHandle, state: State<AppState>, report: serde_json::Value) -> Result<(), String> {
    let enabled = state.overlay_settings.lock().unwrap().enabled;
    *state.overlay_last_report.lock().unwrap() = Some(report.clone());

    if let Some(overlay_win) = app.get_webview_window("overlay") {
        let in_voice = report.get("inVoice").and_then(|v| v.as_bool()).unwrap_or(false);
        if enabled && in_voice {
            if !overlay_win.is_visible().unwrap_or(false) {
                let _ = overlay_win.show();
                let _ = overlay_win.set_ignore_cursor_events(true);
            }
            let _ = app.emit_to("overlay", "voice-overlay:state", &report);
        } else if !in_voice {
            let editing = state.overlay_settings.lock().unwrap().editing;
            if !editing && overlay_win.is_visible().unwrap_or(false) {
                let _ = overlay_win.hide();
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn voice_overlay_done(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().editing = false;
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(true);
    }
    send_overlay_view(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn voice_overlay_set_editing(app: AppHandle, state: State<AppState>, editing: bool) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().editing = editing;
    if let Some(w) = app.get_webview_window("overlay") {
        if editing {
            let _ = w.show();
            let _ = w.set_ignore_cursor_events(false);
            let _ = w.set_focus();
        } else {
            let _ = w.set_ignore_cursor_events(true);
        }
    }
    send_overlay_view(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn voice_overlay_snap(app: AppHandle, state: State<AppState>, corner: String) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().corner = corner.clone();
    if let Some(w) = app.get_webview_window("overlay") {
        let pos = match corner.as_str() {
            "top-right" => (1920 - 360, 20),
            "bottom-left" => (20, 1080 - 640),
            "bottom-right" => (1920 - 360, 1080 - 640),
            _ => (20, 20), // top-left
        };
        let _ = w.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(pos.0, pos.1)));
    }
    send_overlay_view(&app, &state);
    Ok(())
}

pub fn send_overlay_view(app: &AppHandle, state: &State<AppState>) {
    let settings = state.overlay_settings.lock().unwrap().clone();
    let align = if settings.corner.contains("right") { "right" } else { "left" };
    let view_payload = serde_json::json!({
        "onlySpeaking": settings.only_speaking,
        "showMessages": settings.show_messages,
        "scale": settings.scale,
        "opacity": settings.opacity,
        "messageSeconds": 12,
        "editing": settings.editing,
        "align": align,
        "labels": {
            "move": "Kéo để di chuyển vị trí overlay",
            "done": "Xong",
            "idle": "Tham gia phòng thoại để thấy thành viên tại đây."
        }
    });
    let _ = app.emit_to("overlay", "voice-overlay:view", &view_payload);
}

// ────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn is_audio_capture_supported() -> bool {
    true
}

#[tauri::command]
pub fn stop_audio_capture() -> Result<(), String> {
    log::info!("Audio loopback capture stopped");
    Ok(())
}

#[tauri::command]
pub fn get_server_history(state: State<AppState>) -> Vec<String> {
    state.server_history.lock().unwrap().clone()
}

#[tauri::command]
pub fn switch_server(app: AppHandle, url: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(parsed_url) = url.parse() {
            window.navigate(parsed_url).map_err(|e| e.to_string())?;
            log::info!("Switched server to {}", url);
        }
    }
    Ok(())
}
