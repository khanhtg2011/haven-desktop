use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};
use crate::commands::AppState;

#[repr(C)]
struct ProcessMemoryCounters {
    cb: u32,
    page_fault_count: u32,
    peak_working_set_size: usize,
    working_set_size: usize,
    quota_peak_paged_pool_usage: usize,
    quota_paged_pool_usage: usize,
    quota_peak_non_paged_pool_usage: usize,
    quota_non_paged_pool_usage: usize,
    pagefile_usage: usize,
    peak_pagefile_usage: usize,
}

#[repr(C)]
struct PROCESSENTRY32W {
    dwSize: u32,
    cntUsage: u32,
    th32ProcessID: u32,
    th32DefaultHeapID: usize,
    th32ModuleID: u32,
    cntThreads: u32,
    th32ParentProcessID: u32,
    pcPriClassBase: i32,
    dwFlags: u32,
    szExeFile: [u16; 260],
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn GetCurrentProcess() -> isize;
    fn GetCurrentProcessId() -> u32;
    fn K32GetProcessMemoryInfo(process: isize, pps_mem_counters: *mut ProcessMemoryCounters, cb: u32) -> i32;
    fn K32EmptyWorkingSet(process: isize) -> i32;
    fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> isize;
    fn Process32FirstW(hSnapshot: isize, lppe: *mut PROCESSENTRY32W) -> i32;
    fn Process32NextW(hSnapshot: isize, lppe: *mut PROCESSENTRY32W) -> i32;
    fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> isize;
    fn CloseHandle(hObject: isize) -> i32;
}

pub fn trim_working_set() {
    #[cfg(target_os = "windows")]
    unsafe {
        // Trim main Haven process
        K32EmptyWorkingSet(GetCurrentProcess());

        // Trim all WebView2 subprocesses
        let my_pid = GetCurrentProcessId();
        let snapshot = CreateToolhelp32Snapshot(0x00000002, 0); // TH32CS_SNAPPROCESS
        if snapshot != -1 && snapshot != 0 {
            let mut entry = std::mem::zeroed::<PROCESSENTRY32W>();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    if entry.th32ParentProcessID == my_pid {
                        let child_handle = OpenProcess(0x0100 | 0x0400, 0, entry.th32ProcessID);
                        if child_handle != 0 {
                            K32EmptyWorkingSet(child_handle);
                            CloseHandle(child_handle);
                        }
                    }
                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);
        }
    }
}

pub fn get_current_ram_mb() -> usize {
    #[cfg(target_os = "windows")]
    unsafe {
        let mut total_bytes = 0usize;
        let mut counters: ProcessMemoryCounters = std::mem::zeroed();
        counters.cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
        if K32GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, counters.cb) != 0 {
            total_bytes += counters.working_set_size;
        }

        let my_pid = GetCurrentProcessId();
        let snapshot = CreateToolhelp32Snapshot(0x00000002, 0);
        if snapshot != -1 && snapshot != 0 {
            let mut entry = std::mem::zeroed::<PROCESSENTRY32W>();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    if entry.th32ParentProcessID == my_pid {
                        let child_handle = OpenProcess(0x0400, 0, entry.th32ProcessID);
                        if child_handle != 0 {
                            let mut child_counters: ProcessMemoryCounters = std::mem::zeroed();
                            child_counters.cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
                            if K32GetProcessMemoryInfo(child_handle, &mut child_counters, child_counters.cb) != 0 {
                                total_bytes += child_counters.working_set_size;
                            }
                            CloseHandle(child_handle);
                        }
                    }
                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);
        }
        return total_bytes / (1024 * 1024);
    }
    0
}

pub fn is_autostart_enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let output = std::process::Command::new("reg")
            .args(["query", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run", "/v", "HavenDesktop"])
            .creation_flags(0x08000000)
            .output();
        if let Ok(out) = output {
            return out.status.success();
        }
    }
    false
}

pub fn toggle_autostart() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let currently_enabled = is_autostart_enabled();
        if currently_enabled {
            let _ = std::process::Command::new("reg")
                .args(["delete", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run", "/v", "HavenDesktop", "/f"])
                .creation_flags(0x08000000)
                .output();
            return false;
        } else if let Ok(exe_path) = std::env::current_exe() {
            let exe_str = exe_path.to_string_lossy().to_string();
            let _ = std::process::Command::new("reg")
                .args([
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "HavenDesktop",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{}\"", exe_str),
                    "/f",
                ])
                .creation_flags(0x08000000)
                .output();
            return true;
        }
    }
    false
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let open_i = MenuItem::with_id(app, "open", "Mở Haven", true, None::<&str>)?;
    let reload_i = MenuItem::with_id(app, "reload", "Tải lại trang (F5)", true, None::<&str>)?;
    let trim_i = MenuItem::with_id(app, "trim_ram", "Giải phóng RAM ngay", true, None::<&str>)?;

    let autostart_label = if is_autostart_enabled() {
        "Khởi động cùng Windows: [BẬT]"
    } else {
        "Khởi động cùng Windows: [TẮT]"
    };
    let autostart_i = MenuItem::with_id(app, "toggle_autostart", autostart_label, true, None::<&str>)?;

    // Voice Overlay Submenu
    let overlay_toggle_i = MenuItem::with_id(app, "toggle_overlay", "Bật/Tắt Voice Overlay (Ctrl+Shift+O)", true, None::<&str>)?;
    let overlay_move_i = MenuItem::with_id(app, "move_overlay", "Di chuyển vị trí Overlay", true, None::<&str>)?;
    let snap_tl_i = MenuItem::with_id(app, "snap_tl", "Góc trên - Bên trái", true, None::<&str>)?;
    let snap_tr_i = MenuItem::with_id(app, "snap_tr", "Góc trên - Bên phải", true, None::<&str>)?;
    let snap_bl_i = MenuItem::with_id(app, "snap_bl", "Góc dưới - Bên trái", true, None::<&str>)?;
    let snap_br_i = MenuItem::with_id(app, "snap_br", "Góc dưới - Bên phải", true, None::<&str>)?;
    let snap_sub = Submenu::with_items(app, "Ghim vào góc màn hình", true, &[&snap_tl_i, &snap_tr_i, &snap_bl_i, &snap_br_i])?;
    let overlay_sub = Submenu::with_items(app, "Voice Overlay (In-Game HUD)", true, &[&overlay_toggle_i, &overlay_move_i, &snap_sub])?;

    let sep1 = PredefinedMenuItem::separator(app)?;
    let mute_i = MenuItem::with_id(app, "toggle_mute", "Bật/Tắt Mic", true, None::<&str>)?;
    let deafen_i = MenuItem::with_id(app, "toggle_deafen", "Bật/Tắt Tai nghe", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit_i = MenuItem::with_id(app, "quit", "Thoát Haven", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &open_i,
            &reload_i,
            &trim_i,
            &autostart_i,
            &sep1,
            &overlay_sub,
            &mute_i,
            &deafen_i,
            &sep2,
            &quit_i,
        ],
    )?;

    let ram_mb = get_current_ram_mb();
    let initial_tooltip = if ram_mb > 0 {
        format!("Haven Desktop (Rust) | RAM: {} MB", ram_mb)
    } else {
        "Haven Desktop (Rust Ultra-light)".to_string()
    };

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip(&initial_tooltip)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "reload" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.location.reload();");
                }
            }
            "trim_ram" => {
                trim_working_set();
                log::info!("Deep RAM working set trim triggered");
            }
            "toggle_autostart" => {
                let enabled = toggle_autostart();
                log::info!("Auto-start toggled: {}", enabled);
            }
            "toggle_overlay" => {
                let state: State<AppState> = app.state();
                let current = state.overlay_settings.lock().unwrap().enabled;
                let _ = crate::commands::set_voice_overlay_enabled(app.clone(), state, !current);
            }
            "move_overlay" => {
                let state: State<AppState> = app.state();
                let editing = state.overlay_settings.lock().unwrap().editing;
                let _ = crate::commands::voice_overlay_set_editing(app.clone(), state, !editing);
            }
            "snap_tl" => {
                let state: State<AppState> = app.state();
                let _ = crate::commands::voice_overlay_snap(app.clone(), state, "top-left".into());
            }
            "snap_tr" => {
                let state: State<AppState> = app.state();
                let _ = crate::commands::voice_overlay_snap(app.clone(), state, "top-right".into());
            }
            "snap_bl" => {
                let state: State<AppState> = app.state();
                let _ = crate::commands::voice_overlay_snap(app.clone(), state, "bottom-left".into());
            }
            "snap_br" => {
                let state: State<AppState> = app.state();
                let _ = crate::commands::voice_overlay_snap(app.clone(), state, "bottom-right".into());
            }
            "toggle_mute" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("document.getElementById('mic-toggle-btn')?.click();");
                }
            }
            "toggle_deafen" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("document.getElementById('deafen-toggle-btn')?.click();");
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }

                // Update RAM tooltip on tray click
                let ram = get_current_ram_mb();
                if ram > 0 {
                    let _ = tray.set_tooltip(Some(format!("Haven Desktop (Rust) | RAM: {} MB", ram)));
                }
            }
        })
        .build(app)?;

    Ok(())
}
