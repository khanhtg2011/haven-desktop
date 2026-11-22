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
    pub persistent_chat: bool,
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
            persistent_chat: true,
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
    crate::native_overlay::set_enabled(enabled);
    send_overlay_view(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn toggle_voice_overlay_command(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    let current = state.overlay_settings.lock().unwrap().enabled;
    set_voice_overlay_enabled(app, state, !current)
}

#[tauri::command]
pub fn toggle_voice_overlay_persistent(app: AppHandle, state: State<AppState>) -> Result<bool, String> {
    let new_val = crate::native_overlay::toggle_persistent();
    state.overlay_settings.lock().unwrap().persistent_chat = new_val;
    send_overlay_view(&app, &state);
    Ok(new_val)
}

#[tauri::command]
pub fn clear_overlay_messages(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    if let Some(ref mut rep) = *state.overlay_last_report.lock().unwrap() {
        if let Some(obj) = rep.as_object_mut() {
            obj.insert("messages".to_string(), serde_json::json!([]));
        }
    }
    crate::native_overlay::clear_messages();
    let _ = app.emit("voice-overlay:clear-messages", &serde_json::json!({}));
    Ok(())
}

#[tauri::command]
pub fn report_voice_overlay(app: AppHandle, state: State<AppState>, report: serde_json::Value) -> Result<(), String> {
    *state.overlay_last_report.lock().unwrap() = Some(report.clone());
    crate::native_overlay::update_report(report);
    let _ = app;
    Ok(())
}

#[tauri::command]
pub fn voice_overlay_done(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().editing = false;
    send_overlay_view(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn voice_overlay_set_editing(app: AppHandle, state: State<AppState>, editing: bool) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().editing = editing;
    send_overlay_view(&app, &state);
    Ok(())
}

#[tauri::command]
pub fn voice_overlay_snap(app: AppHandle, state: State<AppState>, corner: String) -> Result<(), String> {
    state.overlay_settings.lock().unwrap().corner = corner.clone();
    crate::native_overlay::snap_corner(&corner);
    send_overlay_view(&app, &state);
    Ok(())
}

pub fn send_overlay_view(app: &AppHandle, state: &State<AppState>) {
    let settings = state.overlay_settings.lock().unwrap().clone();
    let align = if settings.corner.contains("right") { "right" } else { "left" };
    let idle_text = if settings.persistent_chat {
        "Haven In-Game HUD: Đang chờ tin nhắn..."
    } else {
        "Tham gia phòng thoại để thấy thành viên tại đây."
    };
    let view_payload = serde_json::json!({
        "persistentChat": settings.persistent_chat,
        "onlySpeaking": settings.only_speaking,
        "showMessages": settings.show_messages,
        "scale": settings.scale,
        "opacity": settings.opacity,
        "messageSeconds": if settings.persistent_chat { 60 } else { 12 },
        "editing": settings.editing,
        "align": align,
        "labels": {
            "move": "Kéo để di chuyển vị trí HUD",
            "done": "Xong",
            "idle": idle_text
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
