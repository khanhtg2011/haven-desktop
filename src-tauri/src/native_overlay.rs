use std::ffi::c_void;
use std::sync::Mutex;

#[repr(C)]
struct GdiplusStartupInput {
    gdiplus_version: u32,
    debug_event_callback: usize,
    suppress_background_thread: i32,
    suppress_external_codecs: i32,
}

#[repr(C)]
struct RectF {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct SIZE {
    cx: i32,
    cy: i32,
}

#[repr(C)]
struct BLENDFUNCTION {
    blend_op: u8,
    blend_flags: u8,
    source_constant_alpha: u8,
    alpha_format: u8,
}

#[repr(C)]
struct BITMAPINFOHEADER {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

#[repr(C)]
struct BITMAPINFO {
    bmi_header: BITMAPINFOHEADER,
    bmi_colors: [u32; 1],
}

#[repr(C)]
struct WNDCLASSEXW {
    cb_size: u32,
    style: u32,
    lpfn_wnd_proc: Option<unsafe extern "system" fn(isize, u32, usize, isize) -> isize>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: isize,
    h_icon: isize,
    h_cursor: isize,
    hbr_background: isize,
    lpsz_menu_name: *const u16,
    lpsz_class_name: *const u16,
    h_icon_sm: isize,
}

#[repr(C)]
struct MSG {
    hwnd: isize,
    message: u32,
    w_param: usize,
    l_param: isize,
    time: u32,
    pt: POINT,
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(lp_module_name: *const u16) -> isize;
}

#[cfg(target_os = "windows")]
#[link(name = "gdiplus")]
extern "system" {
    fn GdiplusStartup(token: *mut usize, input: *const GdiplusStartupInput, output: *mut c_void) -> i32;
    fn GdiplusShutdown(token: usize);
    fn GdipCreateBitmapFromScan0(width: i32, height: i32, stride: i32, format: i32, scan0: *mut u8, bitmap: *mut *mut c_void) -> i32;
    fn GdipDisposeImage(image: *mut c_void) -> i32;
    fn GdipGetImageGraphicsContext(image: *mut c_void, graphics: *mut *mut c_void) -> i32;
    fn GdipDeleteGraphics(graphics: *mut c_void) -> i32;
    fn GdipGraphicsClear(graphics: *mut c_void, color: u32) -> i32;
    fn GdipSetSmoothingMode(graphics: *mut c_void, smoothing_mode: i32) -> i32;
    fn GdipSetTextRenderingHint(graphics: *mut c_void, mode: i32) -> i32;
    fn GdipCreateSolidFill(color: u32, brush: *mut *mut c_void) -> i32;
    fn GdipDeleteBrush(brush: *mut c_void) -> i32;
    fn GdipCreateFontFamilyFromName(name: *const u16, font_collection: *mut c_void, font_family: *mut *mut c_void) -> i32;
    fn GdipDeleteFontFamily(font_family: *mut c_void) -> i32;
    fn GdipCreateFont(font_family: *mut c_void, em_size: f32, style: i32, unit: i32, font: *mut *mut c_void) -> i32;
    fn GdipDeleteFont(font: *mut c_void) -> i32;
    fn GdipDrawString(graphics: *mut c_void, string: *const u16, length: i32, font: *mut c_void, layout_rect: *const RectF, string_format: *mut c_void, brush: *mut c_void) -> i32;
    fn GdipCreatePath(brush_mode: i32, path: *mut *mut c_void) -> i32;
    fn GdipDeletePath(path: *mut c_void) -> i32;
    fn GdipResetPath(path: *mut c_void) -> i32;
    fn GdipAddPathArc(path: *mut c_void, x: f32, y: f32, width: f32, height: f32, start_angle: f32, sweep_angle: f32) -> i32;
    fn GdipClosePathFigure(path: *mut c_void) -> i32;
    fn GdipFillPath(graphics: *mut c_void, brush: *mut c_void, path: *mut c_void) -> i32;
    fn GdipDrawPath(graphics: *mut c_void, pen: *mut c_void, path: *mut c_void) -> i32;
    fn GdipFillRectangle(graphics: *mut c_void, brush: *mut c_void, x: f32, y: f32, width: f32, height: f32) -> i32;
    fn GdipCreatePen1(color: u32, width: f32, unit: i32, pen: *mut *mut c_void) -> i32;
    fn GdipDeletePen(pen: *mut c_void) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "user32")]
extern "system" {
    fn RegisterClassExW(lp_wnd_class: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(
        dw_ex_style: u32,
        lp_class_name: *const u16,
        lp_window_name: *const u16,
        dw_style: u32,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        h_wnd_parent: isize,
        h_menu: isize,
        h_instance: isize,
        lp_param: *mut c_void,
    ) -> isize;
    fn DestroyWindow(h_wnd: isize) -> i32;
    fn ShowWindow(h_wnd: isize, n_cmd_show: i32) -> i32;
    fn SetWindowPos(h_wnd: isize, h_wnd_insert_after: isize, x: i32, y: i32, cx: i32, cy: i32, u_flags: u32) -> i32;
    fn GetMessageW(lp_msg: *mut MSG, h_wnd: isize, w_msg_filter_min: u32, w_msg_filter_max: u32) -> i32;
    fn TranslateMessage(lp_msg: *const MSG) -> i32;
    fn DispatchMessageW(lp_msg: *const MSG) -> isize;
    fn PostMessageW(h_wnd: isize, msg: u32, w_param: usize, l_param: isize) -> i32;
    fn DefWindowProcW(h_wnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
    fn GetSystemMetrics(n_index: i32) -> i32;
    fn UpdateLayeredWindow(
        h_wnd: isize,
        hdc_dst: isize,
        ppt_dst: *const POINT,
        psize: *const SIZE,
        hdc_src: isize,
        ppt_src: *const POINT,
        cr_key: u32,
        pblend: *const BLENDFUNCTION,
        dw_flags: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "gdi32")]
extern "system" {
    fn CreateCompatibleDC(hdc: isize) -> isize;
    fn DeleteDC(hdc: isize) -> i32;
    fn CreateDIBSection(
        hdc: isize,
        pbmi: *const BITMAPINFO,
        usage: u32,
        ppv_bits: *mut *mut u8,
        h_section: isize,
        offset: u32,
    ) -> isize;
    fn SelectObject(hdc: isize, h: isize) -> isize;
    fn DeleteObject(ho: isize) -> i32;
}

const WS_EX_LAYERED: u32 = 0x00080000;
const WS_EX_TRANSPARENT: u32 = 0x00000020;
const WS_EX_TOPMOST: u32 = 0x00000008;
const WS_EX_TOOLWINDOW: u32 = 0x00000080;
const WS_EX_NOACTIVATE: u32 = 0x08000000;
const WS_POPUP: u32 = 0x80000000;

const SWP_NOSIZE: u32 = 0x0001;
const SWP_NOACTIVATE: u32 = 0x0010;
const SWP_SHOWWINDOW: u32 = 0x0040;
const HWND_TOPMOST: isize = -1;

const ULW_ALPHA: u32 = 0x00000002;
const AC_SRC_OVER: u8 = 0x00;
const AC_SRC_ALPHA: u8 = 0x01;
const DIB_RGB_COLORS: u32 = 0;
const SM_CXSCREEN: i32 = 0;
const SM_CYSCREEN: i32 = 1;

const WM_USER: u32 = 0x0400;
const WM_HUD_UPDATE: u32 = WM_USER + 101;
const WM_HUD_CLEAR: u32 = WM_USER + 102;
const WM_HUD_CORNER: u32 = WM_USER + 103;
const WM_HUD_VISIBILITY: u32 = WM_USER + 104;

const OVERLAY_WIDTH: i32 = 360;
const OVERLAY_HEIGHT: i32 = 640;

#[derive(Debug, Clone)]
pub struct OverlayUser {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub talking: bool,
    pub muted: bool,
    pub deafened: bool,
}

#[derive(Debug, Clone)]
pub struct OverlayMessage {
    pub seq: u64,
    pub channel_name: String,
    pub name: String,
    pub color: Option<String>,
    pub content: String,
}

pub struct NativeOverlayState {
    pub enabled: bool,
    pub persistent_chat: bool,
    pub corner: String,
    pub in_voice: bool,
    pub channel_name: String,
    pub users: Vec<OverlayUser>,
    pub messages: Vec<OverlayMessage>,
    pub hwnd: isize,
}

impl Default for NativeOverlayState {
    fn default() -> Self {
        Self {
            enabled: true,
            persistent_chat: true,
            corner: "top-left".to_string(),
            in_voice: false,
            channel_name: String::new(),
            users: Vec::new(),
            messages: Vec::new(),
            hwnd: 0,
        }
    }
}

static OVERLAY_STATE: Mutex<NativeOverlayState> = Mutex::new(NativeOverlayState {
    enabled: true,
    persistent_chat: true,
    corner: String::new(),
    in_voice: false,
    channel_name: String::new(),
    users: Vec::new(),
    messages: Vec::new(),
    hwnd: 0,
});

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn parse_hex_color(hex: Option<&str>, default_color: u32) -> u32 {
    if let Some(h) = hex {
        let clean = h.trim().trim_start_matches('#');
        if clean.len() == 6 {
            if let Ok(val) = u32::from_str_radix(clean, 16) {
                return 0xFF000000 | val;
            }
        }
    }
    default_color
}

unsafe fn get_corner_pos(corner: &str) -> (i32, i32) {
    let screen_w = GetSystemMetrics(SM_CXSCREEN);
    let screen_h = GetSystemMetrics(SM_CYSCREEN);
    match corner {
        "top-right" => (screen_w - OVERLAY_WIDTH - 20, 20),
        "bottom-left" => (20, screen_h - OVERLAY_HEIGHT - 40),
        "bottom-right" => (screen_w - OVERLAY_WIDTH - 20, screen_h - OVERLAY_HEIGHT - 40),
        _ => (20, 20),
    }
}

unsafe fn add_round_rect(path: *mut c_void, x: f32, y: f32, w: f32, h: f32, r: f32) {
    GdipResetPath(path);
    let d = r * 2.0;
    GdipAddPathArc(path, x, y, d, d, 180.0, 90.0);
    GdipAddPathArc(path, x + w - d, y, d, d, 270.0, 90.0);
    GdipAddPathArc(path, x + w - d, y + h - d, d, d, 0.0, 90.0);
    GdipAddPathArc(path, x, y + h - d, d, d, 90.0, 90.0);
    GdipClosePathFigure(path);
}

unsafe fn draw_text_line(
    graphics: *mut c_void,
    font: *mut c_void,
    brush: *mut c_void,
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let wide: Vec<u16> = text.encode_utf16().collect();
    let rect = RectF { x, y, width: w, height: h };
    GdipDrawString(graphics, wide.as_ptr(), wide.len() as i32, font, &rect, std::ptr::null_mut(), brush);
}

struct RenderEngine {
    mem_dc: isize,
    dib: isize,
    bits: *mut u8,
    gp_bitmap: *mut c_void,
    graphics: *mut c_void,
    font_bold: *mut c_void,
    font_regular: *mut c_void,
    font_small: *mut c_void,
    path: *mut c_void,
}

impl RenderEngine {
    unsafe fn new() -> Option<Self> {
        let mem_dc = CreateCompatibleDC(0);
        let mut bmi = BITMAPINFO {
            bmi_header: BITMAPINFOHEADER {
                bi_size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                bi_width: OVERLAY_WIDTH,
                bi_height: -OVERLAY_HEIGHT, // Top-down
                bi_planes: 1,
                bi_bit_count: 32,
                bi_compression: 0,
                bi_size_image: (OVERLAY_WIDTH * OVERLAY_HEIGHT * 4) as u32,
                bi_x_pels_per_meter: 0,
                bi_y_pels_per_meter: 0,
                bi_clr_used: 0,
                bi_clr_important: 0,
            },
            bmi_colors: [0],
        };

        let mut bits: *mut u8 = std::ptr::null_mut();
        let dib = CreateDIBSection(mem_dc, &bmi, DIB_RGB_COLORS, &mut bits, 0, 0);
        if dib == 0 || bits.is_null() {
            return None;
        }
        SelectObject(mem_dc, dib);

        let mut gp_bitmap: *mut c_void = std::ptr::null_mut();
        GdipCreateBitmapFromScan0(
            OVERLAY_WIDTH,
            OVERLAY_HEIGHT,
            OVERLAY_WIDTH * 4,
            0x000E200B, // PixelFormat32bppPARGB
            bits,
            &mut gp_bitmap,
        );

        let mut graphics: *mut c_void = std::ptr::null_mut();
        GdipGetImageGraphicsContext(gp_bitmap, &mut graphics);
        GdipSetSmoothingMode(graphics, 2); // AntiAlias
        GdipSetTextRenderingHint(graphics, 5); // ClearTypeGridFit

        let font_family_name = to_wide("Segoe UI");
        let mut font_family: *mut c_void = std::ptr::null_mut();
        GdipCreateFontFamilyFromName(font_family_name.as_ptr(), std::ptr::null_mut(), &mut font_family);

        let mut font_bold: *mut c_void = std::ptr::null_mut();
        let mut font_regular: *mut c_void = std::ptr::null_mut();
        let mut font_small: *mut c_void = std::ptr::null_mut();

        GdipCreateFont(font_family, 10.0, 1 /* Bold */, 3 /* UnitPoint */, &mut font_bold);
        GdipCreateFont(font_family, 9.5, 0 /* Regular */, 3 /* UnitPoint */, &mut font_regular);
        GdipCreateFont(font_family, 8.0, 1 /* Bold */, 3 /* UnitPoint */, &mut font_small);
        GdipDeleteFontFamily(font_family);

        let mut path: *mut c_void = std::ptr::null_mut();
        GdipCreatePath(0, &mut path);

        Some(Self {
            mem_dc,
            dib,
            bits,
            gp_bitmap,
            graphics,
            font_bold,
            font_regular,
            font_small,
            path,
        })
    }

    unsafe fn render(&self, hwnd: isize) {
        GdipGraphicsClear(self.graphics, 0); // Clear to fully transparent (0x00000000)

        let state = OVERLAY_STATE.lock().unwrap();
        if !state.enabled {
            ShowWindow(hwnd, 0);
            return;
        }

        let show_overlay = state.persistent_chat || state.in_voice || !state.messages.is_empty();
        if !show_overlay {
            ShowWindow(hwnd, 0);
            return;
        }

        let mut cur_y = 10.0f32;

        // 1. Draw Voice Channel Header (if in voice)
        if state.in_voice && !state.channel_name.is_empty() {
            let mut bg_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xDC121424, &mut bg_brush); // 86% opacity dark pill
            add_round_rect(self.path, 10.0, cur_y, 340.0, 28.0, 6.0);
            GdipFillPath(self.graphics, bg_brush, self.path);
            GdipDeleteBrush(bg_brush);

            let mut text_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xFFFFFFFF, &mut text_brush);
            let title = format!("🔊 {}", state.channel_name);
            draw_text_line(self.graphics, self.font_bold, text_brush, &title, 20.0, cur_y + 6.0, 320.0, 20.0);
            GdipDeleteBrush(text_brush);

            cur_y += 34.0;
        }

        // 2. Draw Voice Participants (if in voice)
        if state.in_voice {
            for user in &state.users {
                if cur_y > 320.0 {
                    break;
                }
                let bg_color = if user.talking { 0xEA142D1C } else { 0xC810121D };
                let mut user_bg: *mut c_void = std::ptr::null_mut();
                GdipCreateSolidFill(bg_color, &mut user_bg);
                add_round_rect(self.path, 10.0, cur_y, 220.0, 30.0, 15.0);
                GdipFillPath(self.graphics, user_bg, self.path);
                GdipDeleteBrush(user_bg);

                if user.talking {
                    let mut green_pen: *mut c_void = std::ptr::null_mut();
                    GdipCreatePen1(0xFF43D17A, 1.5, 2 /* UnitPixel */, &mut green_pen);
                    GdipDrawPath(self.graphics, green_pen, self.path);
                    GdipDeletePen(green_pen);
                }

                // Avatar circle
                let av_color = parse_hex_color(user.color.as_deref(), 0xFF5B5F7A);
                let mut av_brush: *mut c_void = std::ptr::null_mut();
                GdipCreateSolidFill(av_color, &mut av_brush);
                add_round_rect(self.path, 14.0, cur_y + 4.0, 22.0, 22.0, 11.0);
                GdipFillPath(self.graphics, av_brush, self.path);
                GdipDeleteBrush(av_brush);

                // Initial
                let initial = user.name.chars().next().unwrap_or('?').to_string().to_uppercase();
                let mut init_brush: *mut c_void = std::ptr::null_mut();
                GdipCreateSolidFill(0xFFFFFFFF, &mut init_brush);
                draw_text_line(self.graphics, self.font_bold, init_brush, &initial, 20.0, cur_y + 7.0, 16.0, 16.0);
                GdipDeleteBrush(init_brush);

                // Username
                let mut name_brush: *mut c_void = std::ptr::null_mut();
                GdipCreateSolidFill(0xFFF1F5F9, &mut name_brush);
                draw_text_line(self.graphics, self.font_bold, name_brush, &user.name, 42.0, cur_y + 7.0, 150.0, 18.0);
                GdipDeleteBrush(name_brush);

                // Status icons
                if user.muted || user.deafened {
                    let icon_color = if user.deafened { 0xFFFFA94D } else { 0xFFFF6B6B };
                    let mut stat_brush: *mut c_void = std::ptr::null_mut();
                    GdipCreateSolidFill(icon_color, &mut stat_brush);
                    let label = if user.deafened { "[D]" } else { "[M]" };
                    draw_text_line(self.graphics, self.font_small, stat_brush, label, 195.0, cur_y + 8.0, 20.0, 14.0);
                    GdipDeleteBrush(stat_brush);
                }

                cur_y += 34.0;
            }
        }

        // 3. Draw Chat Feed (Messages)
        if state.messages.is_empty() && !state.in_voice && state.persistent_chat {
            let mut idle_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xB010121B, &mut idle_brush);
            add_round_rect(self.path, 10.0, cur_y, 280.0, 28.0, 6.0);
            GdipFillPath(self.graphics, idle_brush, self.path);
            GdipDeleteBrush(idle_brush);

            let mut text_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xFFA0A5B5, &mut text_brush);
            draw_text_line(self.graphics, self.font_regular, text_brush, "Haven Native HUD: Đang chờ tin nhắn...", 20.0, cur_y + 6.0, 260.0, 18.0);
            GdipDeleteBrush(text_brush);
            cur_y += 34.0;
        }

        let slice_start = if state.messages.len() > 6 { state.messages.len() - 6 } else { 0 };
        for msg in &state.messages[slice_start..] {
            if cur_y > 580.0 {
                break;
            }

            // Message Bubble Card
            let mut card_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xEA0F111A, &mut card_brush); // 92% deep dark glass
            add_round_rect(self.path, 10.0, cur_y, 340.0, 50.0, 8.0);
            GdipFillPath(self.graphics, card_brush, self.path);
            GdipDeleteBrush(card_brush);

            // Left accent purple bar
            let mut bar_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xFF6366F1, &mut bar_brush); // Haven Purple
            GdipFillRectangle(self.graphics, bar_brush, 10.0, cur_y + 5.0, 3.0, 40.0);
            GdipDeleteBrush(bar_brush);

            // Channel badge & author
            let mut tag_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xFFC9C2FF, &mut tag_brush);
            let channel_tag = format!("#{}", msg.channel_name);
            draw_text_line(self.graphics, self.font_small, tag_brush, &channel_tag, 20.0, cur_y + 6.0, 100.0, 14.0);
            GdipDeleteBrush(tag_brush);

            let author_color = parse_hex_color(msg.color.as_deref(), 0xFFA78BFA);
            let mut author_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(author_color, &mut author_brush);
            draw_text_line(self.graphics, self.font_bold, author_brush, &msg.name, 120.0, cur_y + 5.0, 210.0, 16.0);
            GdipDeleteBrush(author_brush);

            // Message content
            let mut content_brush: *mut c_void = std::ptr::null_mut();
            GdipCreateSolidFill(0xFFF1F5F9, &mut content_brush);
            draw_text_line(self.graphics, self.font_regular, content_brush, &msg.content, 20.0, cur_y + 24.0, 315.0, 22.0);
            GdipDeleteBrush(content_brush);

            cur_y += 56.0;
        }

        let corner_str = state.corner.clone();
        drop(state);

        // Blit to screen with per-pixel alpha transparency
        let pt_src = POINT { x: 0, y: 0 };
        let sz = SIZE { cx: OVERLAY_WIDTH, cy: OVERLAY_HEIGHT };
        let (pos_x, pos_y) = get_corner_pos(&corner_str);
        let pt_dst = POINT { x: pos_x, y: pos_y };
        let blend = BLENDFUNCTION {
            blend_op: AC_SRC_OVER,
            blend_flags: 0,
            source_constant_alpha: 255,
            alpha_format: AC_SRC_ALPHA,
        };

        UpdateLayeredWindow(
            hwnd,
            0,
            &pt_dst,
            &sz,
            self.mem_dc,
            &pt_src,
            0,
            &blend,
            ULW_ALPHA,
        );

        ShowWindow(hwnd, 8); // SW_SHOWNA: Show without activating
    }
}

impl Drop for RenderEngine {
    fn drop(&mut self) {
        unsafe {
            GdipDeletePath(self.path);
            GdipDeleteFont(self.font_small);
            GdipDeleteFont(self.font_regular);
            GdipDeleteFont(self.font_bold);
            GdipDeleteGraphics(self.graphics);
            GdipDisposeImage(self.gp_bitmap);
            DeleteObject(self.dib);
            DeleteDC(self.mem_dc);
        }
    }
}

unsafe extern "system" fn overlay_wndproc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    static mut ENGINE: Option<RenderEngine> = None;

    match msg {
        0x0001 /* WM_CREATE */ => {
            ENGINE = RenderEngine::new();
            0
        }
        WM_HUD_UPDATE => {
            if let Some(ref engine) = ENGINE {
                engine.render(hwnd);
            }
            0
        }
        WM_HUD_CLEAR => {
            {
                let mut state = OVERLAY_STATE.lock().unwrap();
                state.messages.clear();
            }
            if let Some(ref engine) = ENGINE {
                engine.render(hwnd);
            }
            0
        }
        WM_HUD_CORNER => {
            let corner = {
                let state = OVERLAY_STATE.lock().unwrap();
                state.corner.clone()
            };
            let (x, y) = get_corner_pos(&corner);
            SetWindowPos(hwnd, HWND_TOPMOST, x, y, OVERLAY_WIDTH, OVERLAY_HEIGHT, SWP_NOACTIVATE | SWP_SHOWWINDOW);
            if let Some(ref engine) = ENGINE {
                engine.render(hwnd);
            }
            0
        }
        WM_HUD_VISIBILITY => {
            let enabled = {
                let state = OVERLAY_STATE.lock().unwrap();
                state.enabled
            };
            if !enabled {
                ShowWindow(hwnd, 0);
            } else if let Some(ref engine) = ENGINE {
                engine.render(hwnd);
            }
            0
        }
        0x0002 /* WM_DESTROY */ => {
            ENGINE = None;
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub fn init() {
    #[cfg(target_os = "windows")]
    std::thread::spawn(|| unsafe {
        let mut token: usize = 0;
        let startup_input = GdiplusStartupInput {
            gdiplus_version: 1,
            debug_event_callback: 0,
            suppress_background_thread: 0,
            suppress_external_codecs: 0,
        };
        if GdiplusStartup(&mut token, &startup_input, std::ptr::null_mut()) != 0 {
            log::error!("Failed to initialize GDI+ for Native Overlay");
            return;
        }

        let h_instance = GetModuleHandleW(std::ptr::null());
        let class_name = to_wide("HavenNativeOverlayWindow");
        let wnd_class = WNDCLASSEXW {
            cb_size: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
            lpfn_wnd_proc: Some(overlay_wndproc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance,
            h_icon: 0,
            h_cursor: 0,
            hbr_background: 0,
            lpsz_menu_name: std::ptr::null(),
            lpsz_class_name: class_name.as_ptr(),
            h_icon_sm: 0,
        };

        RegisterClassExW(&wnd_class);

        let (x, y) = get_corner_pos("top-left");
        let ex_style = WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;

        let title = to_wide("Haven Native HUD");
        let hwnd = CreateWindowExW(
            ex_style,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_POPUP,
            x,
            y,
            OVERLAY_WIDTH,
            OVERLAY_HEIGHT,
            0,
            0,
            h_instance,
            std::ptr::null_mut(),
        );

        if hwnd != 0 {
            {
                let mut state = OVERLAY_STATE.lock().unwrap();
                state.hwnd = hwnd;
                state.corner = "top-left".to_string();
            }
            log::info!("Haven Native Win32 Overlay Window initialized. HWND: 0x{:X}", hwnd);
            PostMessageW(hwnd, WM_HUD_UPDATE, 0, 0);

            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        GdiplusShutdown(token);
    });
}

pub fn update_report(report: serde_json::Value) {
    let mut state = OVERLAY_STATE.lock().unwrap();
    state.in_voice = report.get("inVoice").and_then(|v| v.as_bool()).unwrap_or(false);
    state.channel_name = report.get("channelName").and_then(|v| v.as_str()).unwrap_or("").to_string();

    if let Some(users_arr) = report.get("users").and_then(|u| u.as_array()) {
        state.users = users_arr.iter().filter_map(|u| {
            Some(OverlayUser {
                id: u.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                name: u.get("name").and_then(|v| v.as_str()).unwrap_or("User").to_string(),
                color: u.get("color").and_then(|v| v.as_str()).map(|s| s.to_string()),
                talking: u.get("talking").and_then(|v| v.as_bool()).unwrap_or(false),
                muted: u.get("muted").and_then(|v| v.as_bool()).unwrap_or(false),
                deafened: u.get("deafened").and_then(|v| v.as_bool()).unwrap_or(false),
            })
        }).collect();
    }

    if let Some(msgs_arr) = report.get("messages").and_then(|m| m.as_array()) {
        state.messages = msgs_arr.iter().filter_map(|m| {
            Some(OverlayMessage {
                seq: m.get("seq").and_then(|v| v.as_u64()).unwrap_or(0),
                channel_name: m.get("channelName").and_then(|v| v.as_str()).unwrap_or("chat").to_string(),
                name: m.get("name").and_then(|v| v.as_str()).unwrap_or("User").to_string(),
                color: m.get("color").and_then(|v| v.as_str()).map(|s| s.to_string()),
                content: m.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            })
        }).collect();
    }

    let hwnd = state.hwnd;
    drop(state);

    if hwnd != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            PostMessageW(hwnd, WM_HUD_UPDATE, 0, 0);
        }
    }
}

pub fn set_enabled(enabled: bool) {
    let mut state = OVERLAY_STATE.lock().unwrap();
    state.enabled = enabled;
    let hwnd = state.hwnd;
    drop(state);

    if hwnd != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            PostMessageW(hwnd, WM_HUD_VISIBILITY, 0, 0);
        }
    }
}

pub fn toggle_persistent() -> bool {
    let mut state = OVERLAY_STATE.lock().unwrap();
    state.persistent_chat = !state.persistent_chat;
    let new_val = state.persistent_chat;
    let hwnd = state.hwnd;
    drop(state);

    if hwnd != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            PostMessageW(hwnd, WM_HUD_UPDATE, 0, 0);
        }
    }
    new_val
}

pub fn clear_messages() {
    let hwnd = {
        let state = OVERLAY_STATE.lock().unwrap();
        state.hwnd
    };
    if hwnd != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            PostMessageW(hwnd, WM_HUD_CLEAR, 0, 0);
        }
    }
}

pub fn snap_corner(corner: &str) {
    let hwnd = {
        let mut state = OVERLAY_STATE.lock().unwrap();
        state.corner = corner.to_string();
        state.hwnd
    };
    if hwnd != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            PostMessageW(hwnd, WM_HUD_CORNER, 0, 0);
        }
    }
}

pub fn is_enabled() -> bool {
    OVERLAY_STATE.lock().unwrap().enabled
}

pub fn is_persistent() -> bool {
    OVERLAY_STATE.lock().unwrap().persistent_chat
}
