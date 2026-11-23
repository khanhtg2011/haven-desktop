// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
extern "system" {
    fn CreateMutexW(lpMutexAttributes: *const std::ffi::c_void, bInitialOwner: i32, lpName: *const u16) -> isize;
    fn GetLastError() -> u32;
    fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> isize;
    fn ShowWindow(hWnd: isize, nCmdShow: i32) -> i32;
    fn SetForegroundWindow(hWnd: isize) -> i32;
}

#[cfg(target_os = "windows")]
const ERROR_ALREADY_EXISTS: u32 = 183;
#[cfg(target_os = "windows")]
const SW_RESTORE: i32 = 9;

fn main() {
    #[cfg(target_os = "windows")]
    {
        // 1. Single-Instance Lock: Prevent duplicate Haven processes
        let mutex_name: Vec<u16> = "HavenDesktopTauriSingleInstanceMutex\0".encode_utf16().collect();
        let _handle = unsafe { CreateMutexW(std::ptr::null(), 1, mutex_name.as_ptr()) };
        if _handle != 0 && unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            let title: Vec<u16> = "Haven\0".encode_utf16().collect();
            let hwnd = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };
            if hwnd != 0 {
                unsafe {
                    ShowWindow(hwnd, SW_RESTORE);
                    SetForegroundWindow(hwnd);
                }
            }
            return;
        }

        // 2. Maximum Low-Memory & High-Performance Gaming Flags for WebView2:
        // - renderer-process-limit=1: Cap to a single renderer process (saves ~100MB+)
        // - disable BackForwardCache: Do not keep previous pages in memory (saves ~80MB+)
        // - NetworkServiceInProcess: Merge network service into main browser process
        // - js-flags: Bound V8 heap to 128MB with aggressive garbage collection
        // - CalculateNativeWinOcclusion & IntensiveWakeupThrottling: Stop background rendering
        let current_args = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
        let max_opt_args = [
            "--renderer-process-limit=1",
            "--disable-gpu-shader-disk-cache",
            "--disable-background-networking",
            "--disable-sync",
            "--disable-component-update",
            "--disable-domain-reliability",
            "--disable-breakpad",
            "--disable-features=RendererCodeIntegrity,BackForwardCache,MediaEngagementBypassAutoplayPolicies,PreloadMediaEngagementData",
            "--enable-features=CalculateNativeWinOcclusion,IntensiveWakeupThrottling,NetworkServiceInProcess",
            "--disable-background-timer-throttling=false",
            "--disable-renderer-backgrounding=false",
            "--js-flags=--max-old-space-size=128",
        ].join(" ");
        let final_args = if current_args.is_empty() {
            max_opt_args
        } else {
            format!("{} {}", current_args, max_opt_args)
        };
        std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", final_args);
    }

    haven_desktop_lib::run();
}
