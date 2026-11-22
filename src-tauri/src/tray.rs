use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

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

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn GetCurrentProcess() -> isize;
    fn K32GetProcessMemoryInfo(process: isize, pps_mem_counters: *mut ProcessMemoryCounters, cb: u32) -> i32;
}

pub fn get_current_ram_mb() -> usize {
    #[cfg(target_os = "windows")]
    {
        let mut counters = ProcessMemoryCounters {
            cb: std::mem::size_of::<ProcessMemoryCounters>() as u32,
            page_fault_count: 0,
            peak_working_set_size: 0,
            working_set_size: 0,
            quota_peak_paged_pool_usage: 0,
            quota_paged_pool_usage: 0,
            quota_peak_non_paged_pool_usage: 0,
            quota_non_paged_pool_usage: 0,
            pagefile_usage: 0,
            peak_pagefile_usage: 0,
        };
        unsafe {
            if K32GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, counters.cb) != 0 {
                return counters.working_set_size / (1024 * 1024);
            }
        }
    }
    0
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let open_i = MenuItem::with_id(app, "open", "Mở Haven", true, None::<&str>)?;
    let reload_i = MenuItem::with_id(app, "reload", "Tải lại trang (F5)", true, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let mute_i = MenuItem::with_id(app, "toggle_mute", "Bật/Tắt Mic", true, None::<&str>)?;
    let deafen_i = MenuItem::with_id(app, "toggle_deafen", "Bật/Tắt Tai nghe", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit_i = MenuItem::with_id(app, "quit", "Thoát Haven", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_i, &reload_i, &sep1, &mute_i, &deafen_i, &sep2, &quit_i])?;

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
