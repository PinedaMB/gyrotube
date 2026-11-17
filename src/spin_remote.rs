use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::backend::runtime;
use crate::jam::JamCommand;
use crate::models::Track;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RemoteRole {
    #[default]
    Idle,
    Host(String),
    Guest(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSyncPayload {
    pub track: Option<Track>,
    pub position_secs: f64,
    pub is_playing: bool,
    pub sent_at_ms: u64,
    pub allow_controls: bool,
    pub source_name: String,
    #[serde(default)]
    pub host_name: String,
}

pub fn local_display_name() -> String {
    std::env::var("USER")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| String::from("Host"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum RemoteGuestEvent {
    AddTrack(Track),
    Control(String),
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn generate_room_code() -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut seed = now_ms() ^ 0x517c_c1b7_2722_0a95;
    let mut code = String::with_capacity(6);
    for _ in 0..6 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let idx = (seed as usize) % ALPHABET.len();
        code.push(ALPHABET[idx] as char);
    }
    code
}

pub fn normalize_room_code(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.trim_end_matches('/').to_string()
    } else {
        trimmed
            .trim_start_matches("SPIN-")
            .trim_start_matches("spin-")
            .to_ascii_uppercase()
    }
}

fn state_topic(room: &str) -> String {
    format!("https://ntfy.sh/gyrotube-spin-{room}")
}

fn cmd_topic(room: &str) -> String {
    format!("https://ntfy.sh/gyrotube-spin-{room}-cmd")
}

pub fn publish_host_state(room: String, payload: RemoteSyncPayload) {
    runtime().spawn(async move {
        let Ok(body) = serde_json::to_string(&payload) else {
            return;
        };
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(6))
            .build()
            .unwrap_or_default();
        let _ = client.post(state_topic(&room)).body(body).send().await;
    });
}

pub fn send_guest_event(target: String, event: RemoteGuestEvent) {
    runtime().spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(6))
            .build()
            .unwrap_or_default();
        if target.starts_with("http://") || target.starts_with("https://") {
            match event {
                RemoteGuestEvent::AddTrack(track) => {
                    let _ = client
                        .post(format!("{target}/api/queue/add"))
                        .json(&track)
                        .send()
                        .await;
                }
                RemoteGuestEvent::Control(action) => {
                    let _ = client
                        .post(format!("{target}/api/control"))
                        .json(&serde_json::json!({ "action": action }))
                        .send()
                        .await;
                }
            }
        } else if let Ok(body) = serde_json::to_string(&event) {
            let _ = client.post(cmd_topic(&target)).body(body).send().await;
        }
    });
}

#[derive(Deserialize)]
struct NtfyMsg {
    event: Option<String>,
    message: Option<String>,
}

pub fn start_host_cmd_listener(
    room: String,
    stop: Arc<AtomicBool>,
    cmd_tx: async_channel::Sender<JamCommand>,
) {
    runtime().spawn(async move {
        let client = reqwest::Client::new();
        while !stop.load(Ordering::Relaxed) {
            let url = format!("{}/json", cmd_topic(&room));
            let Ok(mut resp) = client.get(&url).send().await else {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            };
            let mut buf = String::new();
            while !stop.load(Ordering::Relaxed) {
                match resp.chunk().await {
                    Ok(Some(bytes)) => {
                        buf.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(pos) = buf.find('\n') {
                            let line = buf[..pos].trim().to_string();
                            buf.drain(..=pos);
                            if let Ok(msg) = serde_json::from_str::<NtfyMsg>(&line)
                                && msg.event.as_deref() == Some("message")
                                && let Some(raw) = msg.message
                                && let Ok(ev) = serde_json::from_str::<RemoteGuestEvent>(&raw)
                            {
                                let cmd = match ev {
                                    RemoteGuestEvent::AddTrack(t) => {
                                        Some(JamCommand::AddTrack(t, String::from("remoto")))
                                    }
                                    RemoteGuestEvent::Control(a) => match a.as_str() {
                                        "play_pause" => Some(JamCommand::TogglePlayPause),
                                        "next" => Some(JamCommand::NextTrack),
                                        "prev" => Some(JamCommand::PrevTrack),
                                        _ => None,
                                    },
                                };
                                if let Some(c) = cmd {
                                    let _ = cmd_tx.send(c).await;
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }
            if !stop.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });
}

pub fn start_guest_sync(
    target: String,
    stop: Arc<AtomicBool>,
    ui_tx: async_channel::Sender<RemoteSyncPayload>,
) {
    runtime().spawn(async move {
        let client = reqwest::Client::new();
        if target.starts_with("http://") || target.starts_with("https://") {
            #[derive(Deserialize)]
            struct LocalStateResp {
                allow_controls: bool,
                is_playing: bool,
                position_secs: f64,
                now_playing: Option<crate::jam::JamTrackInfo>,
            }
            while !stop.load(Ordering::Relaxed) {
                if let Ok(resp) = client
                    .get(format!("{target}/api/state"))
                    .timeout(Duration::from_secs(3))
                    .send()
                    .await
                    && let Ok(st) = resp.json::<LocalStateResp>().await
                {
                    let track = st.now_playing.map(|n| Track {
                        id: n.id,
                        title: n.title,
                        artist: n.artist,
                        album: n.album,
                        duration_secs: n.duration_secs,
                        cover_url: n.cover_url.clone(),
                        cover_large_url: n.cover_url,
                        is_music: true,
                    });
                    let _ = ui_tx
                        .send(RemoteSyncPayload {
                            track,
                            position_secs: st.position_secs,
                            is_playing: st.is_playing,
                            sent_at_ms: now_ms(),
                            allow_controls: st.allow_controls,
                            source_name: String::from("SPIN"),
                            host_name: String::new(),
                        })
                        .await;
                }
                tokio::time::sleep(Duration::from_millis(1500)).await;
            }
            return;
        }

        while !stop.load(Ordering::Relaxed) {
            let url = format!("{}/json?since=30s", state_topic(&target));
            let Ok(mut resp) = client.get(&url).send().await else {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            };
            let mut buf = String::new();
            while !stop.load(Ordering::Relaxed) {
                match resp.chunk().await {
                    Ok(Some(bytes)) => {
                        buf.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(pos) = buf.find('\n') {
                            let line = buf[..pos].trim().to_string();
                            buf.drain(..=pos);
                            if let Ok(msg) = serde_json::from_str::<NtfyMsg>(&line)
                                && msg.event.as_deref() == Some("message")
                                && let Some(raw) = msg.message
                                && let Ok(payload) = serde_json::from_str::<RemoteSyncPayload>(&raw)
                            {
                                let _ = ui_tx.send(payload).await;
                            }
                        }
                    }
                    _ => break,
                }
            }
            if !stop.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_6_char_room_codes() {
        let code = generate_room_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn normalizes_room_codes_and_urls() {
        assert_eq!(normalize_room_code("spin-a7k9m2"), "A7K9M2");
        assert_eq!(
            normalize_room_code("http://192.168.1.10:8765/"),
            "http://192.168.1.10:8765"
        );
    }
}
