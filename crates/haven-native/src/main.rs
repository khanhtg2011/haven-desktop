slint::include_modules!();

mod audio;
mod dialog;
mod hotkey;
mod net;
mod overlay;
mod tray;

use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use slint::{Color, Model, VecModel};
use tokio::sync::mpsc;

use crate::audio::AudioEngine;
use crate::hotkey::{HotkeyCallbacks, HotkeyManager};
use crate::net::api::HavenApiClient;
use crate::net::client::HavenSocketClient;
use crate::net::models::{ChannelItem, ChatMessageItem, HavenEvent};
use crate::overlay::GameOverlay;
use crate::tray::{TrayCallbacks, TrayManager};

const BASE_URL: &str = "https://haven.khanhs.io.vn";
const WS_URL: &str = "wss://haven.khanhs.io.vn/socket.io/?EIO=4&transport=websocket";
const SESSION_FILE: &str = "haven_session.json";

#[derive(Serialize, Deserialize)]
struct SavedSession {
    username: String,
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("🚀 Khởi chạy Haven Native Client 100% Rust (Zero WebView2)...");

    let main_window = MainWindow::new()?;

    // State chia sẻ giữa các luồng
    let active_channel_code = Arc::new(Mutex::new(String::new()));
    let active_voice_code = Arc::new(Mutex::new(String::new()));
    let channels_cache = Arc::new(Mutex::new(Vec::<ChannelItem>::new()));
    let socket_client_holder = Arc::new(Mutex::new(Option::<Arc<HavenSocketClient>>::None));
    let current_token = Arc::new(Mutex::new(String::new()));
    let api_client = Arc::new(HavenApiClient::new(BASE_URL));

    // 0. Khởi tạo Native Audio Engine (WASAPI + Lock-Free Ringbuffer)
    let audio_engine = match AudioEngine::new() {
        Ok(engine) => {
            log::info!("Audio Engine WASAPI khởi tạo thành công");
            Some(engine)
        }
        Err(e) => {
            log::warn!("Không thể mở thiết bị âm thanh: {:?}", e);
            None
        }
    };

    let audio_ctrl = audio_engine.as_ref().map(|e| e.controller());

    // Callback mute mic
    {
        let audio_ctrl = audio_ctrl.clone();
        let window_weak = main_window.as_weak();
        let socket_holder = socket_client_holder.clone();
        let vcode_lock = active_voice_code.clone();

        main_window.on_toggle_mute(move || {
            if let Some(window) = window_weak.upgrade() {
                let new_state = !window.get_is_muted();
                window.set_is_muted(new_state);
                log::info!("Trạng thái Mute Micro: {}", new_state);
                if let Some(ctrl) = &audio_ctrl {
                    ctrl.set_muted(new_state);
                }

                let vcode = vcode_lock.lock().unwrap().clone();
                if !vcode.is_empty() {
                    let holder = socket_holder.lock().unwrap();
                    if let Some(client) = holder.as_ref() {
                        client.send_voice_mute_state(&vcode, new_state);
                    }
                }
            }
        });
    }

    // Callback deafen tai nghe
    {
        let audio_ctrl = audio_ctrl.clone();
        let window_weak = main_window.as_weak();
        main_window.on_toggle_deafen(move || {
            if let Some(window) = window_weak.upgrade() {
                let new_state = !window.get_is_deafened();
                window.set_is_deafened(new_state);
                log::info!("Trạng thái Deafen Audio: {}", new_state);
                if let Some(ctrl) = &audio_ctrl {
                    ctrl.set_deafened(new_state);
                }
            }
        });
    }

    // 0.2. Khởi tạo Win32 Game Click-Through Overlay
    let game_overlay = match GameOverlay::new() {
        Ok(overlay) => {
            log::info!("Game Click-Through Overlay khởi tạo thành công");
            Some(Arc::new(overlay))
        }
        Err(e) => {
            log::warn!("Không thể tạo Game Overlay: {:?}", e);
            None
        }
    };

    // Callback cài đặt / Bật tắt Game Overlay HUD
    {
        let game_overlay = game_overlay.clone();
        main_window.on_open_settings(move || {
            if let Some(overlay) = &game_overlay {
                if overlay.is_visible() {
                    overlay.hide();
                } else {
                    overlay.show();
                }
            }
        });
    }

    // 0.3. Khởi tạo Windows System Tray Icon
    let _tray_manager = {
        let window_weak = main_window.as_weak();
        let audio_ctrl = audio_ctrl.clone();
        let game_overlay = game_overlay.clone();

        let tray_callbacks = TrayCallbacks {
            on_show: Box::new({
                let window_weak = window_weak.clone();
                move || {
                    let _ = slint::invoke_from_event_loop({
                        let window_weak = window_weak.clone();
                        move || {
                            if let Some(window) = window_weak.upgrade() {
                                window.show().unwrap_or_default();
                            }
                        }
                    });
                }
            }),
            on_toggle_mute: Box::new({
                let window_weak = window_weak.clone();
                let audio_ctrl = audio_ctrl.clone();
                move || {
                    let audio_ctrl = audio_ctrl.clone();
                    let _ = slint::invoke_from_event_loop({
                        let window_weak = window_weak.clone();
                        move || {
                            if let Some(window) = window_weak.upgrade() {
                                let new_state = !window.get_is_muted();
                                window.set_is_muted(new_state);
                                if let Some(ctrl) = &audio_ctrl {
                                    ctrl.set_muted(new_state);
                                }
                            }
                        }
                    });
                }
            }),
            on_toggle_overlay: Box::new({
                let game_overlay = game_overlay.clone();
                move || {
                    if let Some(overlay) = &game_overlay {
                        if overlay.is_visible() {
                            overlay.hide();
                        } else {
                            overlay.show();
                        }
                    }
                }
            }),
            on_exit: Box::new(|| {
                log::info!("Thoát Haven Native Client...");
                std::process::exit(0);
            }),
        };

        match TrayManager::new(tray_callbacks) {
            Ok(tray) => {
                log::info!("System Tray Manager đã kích hoạt");
                Some(tray)
            }
            Err(e) => {
                log::warn!("Không thể tạo System Tray: {:?}", e);
                None
            }
        }
    };

    // 0.4. Khởi tạo Global Hotkey Manager (PTT CapsLock, Ctrl+Shift+M, Ctrl+Shift+D, Ctrl+Shift+O)
    let _hotkey_manager = {
        let window_weak = main_window.as_weak();
        let audio_ctrl = audio_ctrl.clone();
        let game_overlay = game_overlay.clone();

        let hotkey_callbacks = HotkeyCallbacks {
            on_toggle_mute: Box::new({
                let window_weak = window_weak.clone();
                let audio_ctrl = audio_ctrl.clone();
                move || {
                    let audio_ctrl = audio_ctrl.clone();
                    let _ = slint::invoke_from_event_loop({
                        let window_weak = window_weak.clone();
                        move || {
                            if let Some(window) = window_weak.upgrade() {
                                let new_state = !window.get_is_muted();
                                window.set_is_muted(new_state);
                                if let Some(ctrl) = &audio_ctrl {
                                    ctrl.set_muted(new_state);
                                }
                                log::info!("Hotkey: Toggle Mute -> {}", new_state);
                            }
                        }
                    });
                }
            }),
            on_toggle_deafen: Box::new({
                let window_weak = window_weak.clone();
                let audio_ctrl = audio_ctrl.clone();
                move || {
                    let audio_ctrl = audio_ctrl.clone();
                    let _ = slint::invoke_from_event_loop({
                        let window_weak = window_weak.clone();
                        move || {
                            if let Some(window) = window_weak.upgrade() {
                                let new_state = !window.get_is_deafened();
                                window.set_is_deafened(new_state);
                                if let Some(ctrl) = &audio_ctrl {
                                    ctrl.set_deafened(new_state);
                                }
                                log::info!("Hotkey: Toggle Deafen -> {}", new_state);
                            }
                        }
                    });
                }
            }),
            on_toggle_overlay: Box::new({
                let game_overlay = game_overlay.clone();
                move || {
                    if let Some(overlay) = &game_overlay {
                        if overlay.is_visible() {
                            overlay.hide();
                        } else {
                            overlay.show();
                        }
                        log::info!("Hotkey: Toggle In-game Overlay HUD");
                    }
                }
            }),
            on_ptt_down: Box::new({
                let audio_ctrl = audio_ctrl.clone();
                let window_weak = window_weak.clone();
                let socket_holder = socket_client_holder.clone();
                let vcode_lock = active_voice_code.clone();
                move || {
                    if let Some(ctrl) = &audio_ctrl {
                        ctrl.set_muted(false);
                    }
                    let _ = slint::invoke_from_event_loop({
                        let window_weak = window_weak.clone();
                        move || {
                            if let Some(window) = window_weak.upgrade() {
                                window.set_is_muted(false);
                            }
                        }
                    });

                    let vcode = vcode_lock.lock().unwrap().clone();
                    if !vcode.is_empty() {
                        let holder = socket_holder.lock().unwrap();
                        if let Some(client) = holder.as_ref() {
                            client.send_voice_speaking(true);
                        }
                    }
                    log::info!("PTT Pressed (Transmitting Audio)");
                }
            }),
            on_ptt_up: Box::new({
                let audio_ctrl = audio_ctrl.clone();
                let window_weak = window_weak.clone();
                let socket_holder = socket_client_holder.clone();
                let vcode_lock = active_voice_code.clone();
                move || {
                    if let Some(ctrl) = &audio_ctrl {
                        ctrl.set_muted(true);
                    }
                    let _ = slint::invoke_from_event_loop({
                        let window_weak = window_weak.clone();
                        move || {
                            if let Some(window) = window_weak.upgrade() {
                                window.set_is_muted(true);
                            }
                        }
                    });

                    let vcode = vcode_lock.lock().unwrap().clone();
                    if !vcode.is_empty() {
                        let holder = socket_holder.lock().unwrap();
                        if let Some(client) = holder.as_ref() {
                            client.send_voice_speaking(false);
                        }
                    }
                    log::info!("PTT Released (Microphone Muted)");
                }
            }),
        };

        Some(HotkeyManager::start(hotkey_callbacks))
    };

    // 1. Fetch server public config (e.g. server title/name)
    {
        let api_client = api_client.clone();
        let window_weak = main_window.as_weak();
        tokio::spawn(async move {
            if let Ok(config) = api_client.get_public_config().await {
                if let Some(sname) = config.server_name {
                    log::info!("Server name từ VPS: {}", sname);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.set_server_name(sname.into());
                        }
                    });
                }
            }
        });
    }

    // Callback xử lý đăng nhập từ UI Slint
    {
        let api_client = api_client.clone();
        let window_weak = main_window.as_weak();
        let socket_client_holder = socket_client_holder.clone();
        let active_channel_code = active_channel_code.clone();
        let active_voice_code = active_voice_code.clone();
        let channels_cache = channels_cache.clone();
        let current_token = current_token.clone();

        main_window.on_do_login(move |username, password| {
            let u = username.to_string();
            let p = password.to_string();
            let api_client = api_client.clone();
            let window_weak = window_weak.clone();
            let socket_client_holder = socket_client_holder.clone();
            let active_channel_code = active_channel_code.clone();
            let active_voice_code = active_voice_code.clone();
            let channels_cache = channels_cache.clone();
            let current_token = current_token.clone();

            log::info!("Nhận yêu cầu đăng nhập từ tài khoản: {}", u);

            if let Some(window) = window_weak.upgrade() {
                window.set_login_loading(true);
                window.set_login_error("".into());
            }

            tokio::spawn(async move {
                match api_client.login(&u, &p).await {
                    Ok(res) => {
                        log::info!("Đăng nhập thành công! User: {}", res.user.username);
                        *current_token.lock().unwrap() = res.token.clone();

                        let session = SavedSession {
                            username: res.user.username.clone(),
                            token: res.token.clone(),
                        };
                        if let Ok(json_data) = serde_json::to_string(&session) {
                            let _ = fs::write(SESSION_FILE, json_data);
                        }

                        start_socket_session(
                            &res.token,
                            &res.user.username,
                            window_weak.clone(),
                            socket_client_holder,
                            active_channel_code,
                            active_voice_code,
                            channels_cache,
                        ).await;
                    }
                    Err(e) => {
                        let err_msg = format!("Đăng nhập thất bại: {}", e);
                        log::warn!("{}", err_msg);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(window) = window_weak.upgrade() {
                                window.set_login_loading(false);
                                window.set_login_error("Sai tài khoản, mật khẩu hoặc lỗi máy chủ!".into());
                            }
                        });
                    }
                }
            });
        });
    }

    // Callback khi gửi tin nhắn từ Slint UI
    {
        let socket_client_holder = socket_client_holder.clone();
        let active_channel_code = active_channel_code.clone();
        main_window.on_send_message(move |msg_text| {
            let text = msg_text.to_string();
            let code = active_channel_code.lock().unwrap().clone();
            log::info!("Gửi tin nhắn tới channel [{}]: {}", code, text);

            let holder = socket_client_holder.lock().unwrap();
            if let Some(client) = holder.as_ref() {
                if !code.is_empty() {
                    client.send_message(&code, &text);
                }
            }
        });
    }

    // Callback đính kèm file / ảnh
    {
        let api_client = api_client.clone();
        let current_token = current_token.clone();
        let socket_client_holder = socket_client_holder.clone();
        let active_channel_code = active_channel_code.clone();

        main_window.on_pick_attachment(move || {
            let api_client = api_client.clone();
            let current_token = current_token.clone();
            let socket_client_holder = socket_client_holder.clone();
            let active_channel_code = active_channel_code.clone();

            tokio::task::spawn_blocking(move || {
                if let Some(file_path) = dialog::pick_file() {
                    log::info!("Đã chọn file gửi: {:?}", file_path);
                    if let Ok(bytes) = std::fs::read(&file_path) {
                        let file_name = file_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("file")
                            .to_string();

                        let token = current_token.lock().unwrap().clone();
                        let code = active_channel_code.lock().unwrap().clone();

                        tokio::spawn(async move {
                            match api_client.upload_file(&token, &file_name, bytes).await {
                                Ok(res) => {
                                    log::info!("Upload thành công: {:?}", res);
                                    let msg_payload = if res.is_image.unwrap_or(false) {
                                        res.url
                                    } else {
                                        let size_str = format!("{} KB", res.file_size.unwrap_or(0) / 1024);
                                        format!("[file:{}]({}|{})", res.original_name.as_deref().unwrap_or(&file_name), res.url, size_str)
                                    };

                                    let holder = socket_client_holder.lock().unwrap();
                                    if let Some(client) = holder.as_ref() {
                                        client.send_message(&code, &msg_payload);
                                    }
                                }
                                Err(e) => {
                                    log::error!("Upload thất bại: {:?}", e);
                                }
                            }
                        });
                    }
                }
            });
        });
    }

    // Callback mở attachment trên trình duyệt
    main_window.on_open_attachment(|url| {
        let u = url.to_string();
        log::info!("Mở tệp đính kèm: {}", u);
        open_url(&u);
    });

    // Callback khi chuyển đổi channel (Text hoặc Voice)
    {
        let socket_client_holder = socket_client_holder.clone();
        let active_channel_code = active_channel_code.clone();
        let active_voice_code = active_voice_code.clone();
        let channels_cache = channels_cache.clone();
        let window_weak = main_window.as_weak();

        main_window.on_channel_selected(move |channel_id, is_voice| {
            let code = channel_id.to_string();
            log::info!("Đã chọn channel: {} (voice: {})", code, is_voice);

            if is_voice {
                *active_voice_code.lock().unwrap() = code.clone();
                let holder = socket_client_holder.lock().unwrap();
                if let Some(client) = holder.as_ref() {
                    client.join_voice(&code);
                }

                let ch_name = {
                    let cache = channels_cache.lock().unwrap();
                    cache.iter().find(|c| c.code == code).map(|c| c.name.clone()).unwrap_or(code.clone())
                };

                let window_weak = window_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.upgrade() {
                        window.set_is_voice_connected(true);
                        window.set_active_voice_channel(ch_name.into());
                    }
                });
            } else {
                *active_channel_code.lock().unwrap() = code.clone();

                let ch_name = {
                    let cache = channels_cache.lock().unwrap();
                    cache.iter().find(|c| c.code == code).map(|c| (c.name.clone(), c.topic.clone().unwrap_or_default()))
                };

                if let Some((name, topic)) = ch_name {
                    let window_weak = window_weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.set_active_channel_name(name.into());
                            window.set_channel_topic(topic.into());
                        }
                    });
                }

                let holder = socket_client_holder.lock().unwrap();
                if let Some(client) = holder.as_ref() {
                    client.enter_channel(&code);
                }
            }
        });
    }

    // Callback ngắt kết nối Voice
    {
        let socket_client_holder = socket_client_holder.clone();
        let active_voice_code = active_voice_code.clone();
        let window_weak = main_window.as_weak();

        main_window.on_disconnect_voice(move || {
            let code = active_voice_code.lock().unwrap().clone();
            log::info!("Ngắt kết nối phòng thoại: {}", code);

            let holder = socket_client_holder.lock().unwrap();
            if let Some(client) = holder.as_ref() {
                if !code.is_empty() {
                    client.leave_voice(&code);
                }
            }

            *active_voice_code.lock().unwrap() = String::new();
            let window_weak = window_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak.upgrade() {
                    window.set_is_voice_connected(false);
                    window.set_active_voice_channel("".into());
                }
            });
        });
    }

    // Callback khi chọn server/guild
    main_window.on_guild_selected(|guild_id| {
        log::info!("Đã chọn guild: {}", guild_id);
    });

    // 2. Tự động kiểm tra file session.json đã lưu trước đó
    if Path::new(SESSION_FILE).exists() {
        if let Ok(content) = fs::read_to_string(SESSION_FILE) {
            if let Ok(session) = serde_json::from_str::<SavedSession>(&content) {
                log::info!("Tìm thấy phiên đăng nhập đã lưu cho: {}", session.username);
                *current_token.lock().unwrap() = session.token.clone();
                let window_weak = main_window.as_weak();
                let socket_client_holder = socket_client_holder.clone();
                let active_channel_code = active_channel_code.clone();
                let active_voice_code = active_voice_code.clone();
                let channels_cache = channels_cache.clone();

                tokio::spawn(async move {
                    start_socket_session(
                        &session.token,
                        &session.username,
                        window_weak,
                        socket_client_holder,
                        active_channel_code,
                        active_voice_code,
                        channels_cache,
                    ).await;
                });
            }
        }
    }

    log::info!("Haven Native sẵn sàng. Hiển thị cửa sổ...");
    main_window.run()?;
    Ok(())
}

async fn start_socket_session(
    token: &str,
    username: &str,
    window_weak: slint::Weak<MainWindow>,
    socket_client_holder: Arc<Mutex<Option<Arc<HavenSocketClient>>>>,
    active_channel_code: Arc<Mutex<String>>,
    active_voice_code: Arc<Mutex<String>>,
    channels_cache: Arc<Mutex<Vec<ChannelItem>>>,
) {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<HavenEvent>();

    match HavenSocketClient::connect(WS_URL, token, event_tx).await {
        Ok(client) => {
            let client_arc = Arc::new(client);
            *socket_client_holder.lock().unwrap() = Some(client_arc.clone());

            let u_name = username.to_string();
            let w_init = window_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = w_init.upgrade() {
                    window.set_is_logged_in(true);
                    window.set_login_loading(false);
                    window.set_current_username(u_name.into());
                }
            });

            // Lắng nghe sự kiện WebSocket realtime
            while let Some(event) = event_rx.recv().await {
                match event {
                    HavenEvent::Connected(sid) => {
                        log::info!("WebSocket đã kết nối. SID: {}", sid);
                    }
                    HavenEvent::ChannelsList(channels) => {
                        log::info!("Nhận danh sách {} kênh từ máy chủ", channels.len());
                        *channels_cache.lock().unwrap() = channels.clone();

                        let mut text_items = Vec::new();
                        let mut voice_items = Vec::new();
                        let mut first_text_code = String::new();
                        let mut first_text_name = String::new();
                        let mut first_text_topic = String::new();

                        for (idx, ch) in channels.iter().enumerate() {
                            let is_selected = idx == 0;
                            if is_selected && ch.text_enabled == 1 {
                                first_text_code = ch.code.clone();
                                first_text_name = ch.name.clone();
                                first_text_topic = ch.topic.clone().unwrap_or_default();
                            }

                            let item = ChannelItemData {
                                id: ch.code.clone().into(),
                                name: ch.name.clone().into(),
                                is_voice: ch.voice_enabled == 1 && ch.text_enabled == 0,
                                is_selected,
                                unread: false,
                            };

                            if ch.voice_enabled == 1 && ch.text_enabled == 0 {
                                voice_items.push(item);
                            } else {
                                text_items.push(item);
                            }
                        }

                        if !first_text_code.is_empty() {
                            *active_channel_code.lock().unwrap() = first_text_code.clone();
                            client_arc.enter_channel(&first_text_code);
                        }

                        let window_weak = window_weak.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(window) = window_weak.upgrade() {
                                window.set_text_channels(Rc::new(VecModel::from(text_items)).into());
                                window.set_voice_channels(Rc::new(VecModel::from(voice_items)).into());
                                if !first_text_name.is_empty() {
                                    window.set_active_channel_name(first_text_name.into());
                                    window.set_channel_topic(first_text_topic.into());
                                }
                            }
                        });
                    }
                    HavenEvent::MessageHistory(history) => {
                        let cur_code = active_channel_code.lock().unwrap().clone();
                        if history.channel_code == cur_code {
                            log::info!("Nạp {} tin nhắn lịch sử cho channel {}", history.messages.len(), cur_code);
                            let msgs: Vec<MessageData> = history.messages.into_iter().map(convert_msg).collect();
                            let window_weak = window_weak.clone();
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(window) = window_weak.upgrade() {
                                    window.set_messages(Rc::new(VecModel::from(msgs)).into());
                                }
                            });
                        }
                    }
                    HavenEvent::NewMessage(new_msg) => {
                        let cur_code = active_channel_code.lock().unwrap().clone();
                        if new_msg.channel_code == cur_code {
                            let u = new_msg.message.username.as_deref().unwrap_or("Unknown");
                            let c = new_msg.message.content.as_deref().unwrap_or("");
                            log::info!("Tin nhắn mới từ [{}]: {}", u, c);
                            let msg_data = convert_msg(new_msg.message);
                            let window_weak = window_weak.clone();
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(window) = window_weak.upgrade() {
                                    let mut current_msgs: Vec<MessageData> = window.get_messages().iter().collect();
                                    current_msgs.push(msg_data);
                                    window.set_messages(Rc::new(VecModel::from(current_msgs)).into());
                                }
                            });
                        }
                    }
                    HavenEvent::VoiceExistingUsers { channel_code, users } => {
                        log::info!("Phòng thoại [{}] có {} thành viên đang tham gia", channel_code, users.len());
                        *active_voice_code.lock().unwrap() = channel_code.clone();
                        let window_weak = window_weak.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(window) = window_weak.upgrade() {
                                window.set_is_voice_connected(true);
                                window.set_active_voice_channel(channel_code.into());
                            }
                        });
                    }
                    HavenEvent::VoiceUserJoined { channel_code, user } => {
                        log::info!("Người dùng mới vào voice [{:?}]: {:?}", channel_code, user);
                    }
                    HavenEvent::VoiceUserLeft { user_id } => {
                        log::info!("Người dùng rời voice: {:?}", user_id);
                    }
                    HavenEvent::VoiceSpeaking { user_id, speaking } => {
                        log::debug!("Trạng thái nói: user {:?} -> {}", user_id, speaking);
                    }
                    HavenEvent::Disconnected(reason) => {
                        log::warn!("Mất kết nối WebSocket: {}", reason);
                    }
                }
            }
        }
        Err(e) => {
            log::error!("Không thể kết nối WebSocket: {:?}", e);
            let w_err = window_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = w_err.upgrade() {
                    window.set_is_logged_in(false);
                    window.set_login_error("Mất kết nối đến máy chủ WebSocket!".into());
                }
            });
        }
    }
}

fn parse_attachment_content(content: &str) -> (String, bool, bool, String, String) {
    let trimmed = content.trim();

    // Check if [file:originalName](url|size)
    if trimmed.starts_with("[file:") && trimmed.contains("](") && trimmed.ends_with(')') {
        if let Some(close_bracket) = trimmed.find("](") {
            let name = &trimmed[6..close_bracket];
            let inner = &trimmed[close_bracket + 2..trimmed.len() - 1];
            let (url, _) = if let Some(sep) = inner.find('|') {
                (&inner[..sep], &inner[sep + 1..])
            } else {
                (inner, "")
            };
            return (String::new(), true, false, url.to_string(), name.to_string());
        }
    }

    // Check if raw upload URL or image URL
    let is_upload = trimmed.starts_with("/uploads/") || (trimmed.starts_with("http") && trimmed.contains("/uploads/"));
    let lower = trimmed.to_lowercase();
    let is_img = lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg") || lower.ends_with(".gif") || lower.ends_with(".webp");

    if is_upload || is_img {
        let name = Path::new(trimmed)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image")
            .to_string();
        return (String::new(), true, is_img, trimmed.to_string(), name);
    }

    (content.to_string(), false, false, String::new(), String::new())
}

fn convert_msg(m: ChatMessageItem) -> MessageData {
    let username = m.username.unwrap_or_else(|| "Unknown".to_string());
    let raw_content = m.content.unwrap_or_default();
    let created_at = m.created_at.unwrap_or_default();

    let (content, is_attachment, is_image, attachment_url, attachment_name) = parse_attachment_content(&raw_content);

    let initials = if username.len() >= 2 {
        username[..2].to_uppercase()
    } else {
        username.to_uppercase()
    };

    let is_bot = username.to_lowercase().contains("bot");
    let color = if is_bot {
        Color::from_rgb_u8(88, 101, 242) // Blurple
    } else {
        Color::from_rgb_u8(35, 165, 90) // Discord Green
    };

    let time_str = if created_at.len() >= 16 {
        created_at[11..16].to_string()
    } else {
        created_at
    };

    MessageData {
        id: format!("{}", m.id).into(),
        author: username.into(),
        avatar_text: initials.into(),
        avatar_color: color,
        timestamp: time_str.into(),
        content: content.into(),
        is_bot,
        is_attachment,
        attachment_url: attachment_url.into(),
        attachment_name: attachment_name.into(),
        is_image,
    }
}

fn open_url(url: &str) {
    let full_url = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("{}{}", BASE_URL, url)
    };

    #[cfg(windows)]
    unsafe {
        let wide: Vec<u16> = full_url.encode_utf16().chain(std::iter::once(0)).collect();
        let op: Vec<u16> = "open\0".encode_utf16().collect();
        windows_sys::Win32::UI::Shell::ShellExecuteW(
            0,
            op.as_ptr(),
            wide.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
        );
    }

    #[cfg(not(windows))]
    {
        let _ = open::that(full_url);
    }
}
