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
        let mutex_name: Vec<u16> = "Global\\HavenDesktopTauriSingleInstanceMutex\0".encode_utf16().collect();
        let _handle = unsafe { CreateMutexW(std::ptr::null(), 1, mutex_name.as_ptr()) };
        if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
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

        // 2. Auto-grant device permissions (Microphone, Camera, Audio) like Electron
        // 3. Aggressive RAM optimization: Cap V8 heap to 96MB, limit renderer processes to 1, enable low-end device mode
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            "--use-fake-ui-for-media-stream \
             --autoplay-policy=no-user-gesture-required \
             --enable-features=HardwareMediaKeyHandling \
             --disable-features=Translate,OptimizationHints,MediaRouter \
             --enable-low-end-device-mode \
             --renderer-process-limit=1 \
             --js-flags=\"--max-old-space-size=96 --lite-mode\" \
             --disk-cache-size=16777216",
        );
    }

    haven_desktop_lib::run();
}
