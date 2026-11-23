use crate::net::models::{ChannelItem, HavenEvent, MessageHistoryEvent, NewMessageEvent};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Clone)]
pub enum OutgoingCommand {
    #[allow(dead_code)]
    GetChannels,
    EnterChannel { code: String },
    SendMessage { code: String, content: String },
    JoinVoice { code: String },
    LeaveVoice { code: String },
    VoiceSpeaking { speaking: bool },
    VoiceMuteState { code: String, muted: bool },
}

pub struct HavenSocketClient {
    cmd_tx: mpsc::UnboundedSender<OutgoingCommand>,
}

impl HavenSocketClient {
    pub async fn connect(
        ws_url: &str,
        token: &str,
        event_tx: mpsc::UnboundedSender<HavenEvent>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Connecting to WebSocket: {}", ws_url);
        let (ws_stream, _) = connect_async(ws_url).await?;
        let (mut write, mut read) = ws_stream.split();

        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<OutgoingCommand>();

        // Step 1: Read Engine.IO Open packet ("0{...}")
        if let Some(Ok(Message::Text(msg))) = read.next().await {
            log::info!("Engine.IO Open: {}", msg);
        }

        // Step 2: Send Socket.IO Connect packet ("40{...}")
        let connect_pkt = format!(
            "40{}",
            json!({
                "token": token,
                "presenceDeltas": 1
            })
        );
        write.send(Message::Text(connect_pkt)).await?;

        // Step 3: Read Socket.IO Connect Ack ("40{...}")
        if let Some(Ok(Message::Text(ack))) = read.next().await {
            log::info!("Socket.IO Ack: {}", ack);
            let _ = event_tx.send(HavenEvent::Connected(ack));
        }

        // Request initial channels
        let init_pkt = format!("42{}", json!(["get-channels", {}]));
        write.send(Message::Text(init_pkt)).await?;

        // Background loop for full-duplex communication
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Outgoing commands from UI
                    Some(cmd) = cmd_rx.recv() => {
                        let packet_str = match cmd {
                            OutgoingCommand::GetChannels => {
                                format!("42{}", json!(["get-channels", {}]))
                            }
                            OutgoingCommand::EnterChannel { code } => {
                                let enter = format!("42{}", json!(["enter-channel", { "code": code }]));
                                let _ = write.send(Message::Text(enter)).await;
                                format!("42{}", json!(["get-messages", { "code": code }]))
                            }
                            OutgoingCommand::SendMessage { code, content } => {
                                format!("42{}", json!(["send-message", { "code": code, "content": content }]))
                            }
                            OutgoingCommand::JoinVoice { code } => {
                                format!("42{}", json!(["voice-join", { "code": code }]))
                            }
                            OutgoingCommand::LeaveVoice { code } => {
                                format!("42{}", json!(["voice-leave", { "code": code }]))
                            }
                            OutgoingCommand::VoiceSpeaking { speaking } => {
                                format!("42{}", json!(["voice-speaking", { "speaking": speaking }]))
                            }
                            OutgoingCommand::VoiceMuteState { code, muted } => {
                                format!("42{}", json!(["voice-mute-state", { "code": code, "muted": muted }]))
                            }
                        };
                        if let Err(e) = write.send(Message::Text(packet_str)).await {
                            log::error!("Failed to send outgoing packet: {:?}", e);
                            break;
                        }
                    }

                    // Incoming frames from server
                    msg_res = read.next() => {
                        match msg_res {
                            Some(Ok(Message::Text(text))) => {
                                // Engine.IO ping ("2") -> pong ("3")
                                if text == "2" {
                                    let _ = write.send(Message::Text("3".to_string())).await;
                                    continue;
                                }

                                // Socket.IO event ("42[...]")
                                if text.starts_with("42") {
                                    if let Ok(event_val) = serde_json::from_str::<serde_json::Value>(&text[2..]) {
                                        if let Some(arr) = event_val.as_array() {
                                            if let Some(event_name) = arr.get(0).and_then(|v| v.as_str()) {
                                                let payload = arr.get(1);

                                                match event_name {
                                                    "channels-list" => {
                                                        if let Some(p) = payload {
                                                            if let Ok(channels) = serde_json::from_value::<Vec<ChannelItem>>(p.clone()) {
                                                                let _ = event_tx.send(HavenEvent::ChannelsList(channels));
                                                            }
                                                        }
                                                    }
                                                    "message-history" => {
                                                        if let Some(p) = payload {
                                                            match serde_json::from_value::<MessageHistoryEvent>(p.clone()) {
                                                                Ok(hist) => {
                                                                    let _ = event_tx.send(HavenEvent::MessageHistory(hist));
                                                                }
                                                                Err(e) => {
                                                                    log::error!("Deserialization error on message-history: {:?}", e);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    "new-message" => {
                                                        if let Some(p) = payload {
                                                            match serde_json::from_value::<NewMessageEvent>(p.clone()) {
                                                                Ok(new_msg) => {
                                                                    let _ = event_tx.send(HavenEvent::NewMessage(new_msg));
                                                                }
                                                                Err(e) => {
                                                                    log::error!("Deserialization error on new-message: {:?}", e);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    "voice-existing-users" => {
                                                        if let Some(p) = payload {
                                                            let code = p.get("channelCode").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                                            let users = p.get("users").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                                                            let _ = event_tx.send(HavenEvent::VoiceExistingUsers {
                                                                channel_code: code,
                                                                users,
                                                            });
                                                        }
                                                    }
                                                    "voice-user-joined" => {
                                                        if let Some(p) = payload {
                                                            let code = p.get("channelCode").and_then(|v| v.as_str()).map(|s| s.to_string());
                                                            let user = p.get("user").cloned().unwrap_or(p.clone());
                                                            let _ = event_tx.send(HavenEvent::VoiceUserJoined {
                                                                channel_code: code,
                                                                user,
                                                            });
                                                        }
                                                    }
                                                    "voice-user-left" => {
                                                        if let Some(p) = payload {
                                                            let uid = p.get("userId").and_then(|v| v.as_i64());
                                                            let _ = event_tx.send(HavenEvent::VoiceUserLeft { user_id: uid });
                                                        }
                                                    }
                                                    "voice-speaking" => {
                                                        if let Some(p) = payload {
                                                            let uid = p.get("userId").and_then(|v| v.as_i64());
                                                            let spk = p.get("speaking").and_then(|v| v.as_bool()).unwrap_or(false);
                                                            let _ = event_tx.send(HavenEvent::VoiceSpeaking {
                                                                user_id: uid,
                                                                speaking: spk,
                                                            });
                                                        }
                                                    }
                                                    _ => {
                                                        log::debug!("Ignored event: {}", event_name);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Some(Ok(Message::Close(_))) | None => {
                                log::warn!("WebSocket connection closed.");
                                let _ = event_tx.send(HavenEvent::Disconnected("Server closed connection".into()));
                                break;
                            }
                            Some(Err(e)) => {
                                log::error!("WebSocket error: {:?}", e);
                                let _ = event_tx.send(HavenEvent::Disconnected(e.to_string()));
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(Self { cmd_tx })
    }

    pub fn send_message(&self, code: &str, content: &str) {
        let _ = self.cmd_tx.send(OutgoingCommand::SendMessage {
            code: code.to_string(),
            content: content.to_string(),
        });
    }

    pub fn enter_channel(&self, code: &str) {
        let _ = self.cmd_tx.send(OutgoingCommand::EnterChannel {
            code: code.to_string(),
        });
    }

    pub fn join_voice(&self, code: &str) {
        let _ = self.cmd_tx.send(OutgoingCommand::JoinVoice {
            code: code.to_string(),
        });
    }

    pub fn leave_voice(&self, code: &str) {
        let _ = self.cmd_tx.send(OutgoingCommand::LeaveVoice {
            code: code.to_string(),
        });
    }

    pub fn send_voice_speaking(&self, speaking: bool) {
        let _ = self.cmd_tx.send(OutgoingCommand::VoiceSpeaking { speaking });
    }

    pub fn send_voice_mute_state(&self, code: &str, muted: bool) {
        let _ = self.cmd_tx.send(OutgoingCommand::VoiceMuteState {
            code: code.to_string(),
            muted,
        });
    }
}
