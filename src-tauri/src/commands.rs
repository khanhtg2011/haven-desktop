use tauri::{AppHandle, Manager, State};
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

pub struct AppState {
    pub shortcuts: Mutex<ShortcutsConfig>,
    pub server_history: Mutex<Vec<String>>,
    pub voice_state: Mutex<serde_json::Value>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            shortcuts: Mutex::new(ShortcutsConfig::default()),
            server_history: Mutex::new(vec!["https://haven.khanhs.io.vn".to_string()]),
            voice_state: Mutex::new(serde_json::json!({})),
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
