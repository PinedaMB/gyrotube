use serde::Deserialize;
use tauri::State;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(not(desktop), allow(dead_code))]
pub struct DiscordRpcPayload {
    pub enabled: bool,
    pub client_id: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub art_url: Option<String>,
    pub duration_secs: f64,
    pub position_secs: f64,
    pub is_playing: bool,
}

#[cfg(desktop)]
mod desktop {
    use std::io::{Read, Write};
    use std::sync::mpsc::{self, Sender};
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde_json::json;

    use super::DiscordRpcPayload;

    #[cfg(unix)]
    type IpcStream = std::os::unix::net::UnixStream;
    #[cfg(windows)]
    type IpcStream = std::fs::File;

    pub struct DiscordRpcHandle {
        tx: Sender<DiscordRpcPayload>,
    }

    impl DiscordRpcHandle {
        pub fn start() -> Self {
            let (tx, rx) = mpsc::channel::<DiscordRpcPayload>();
            std::thread::Builder::new()
                .name("gyro-discord-rpc".into())
                .spawn(move || {
                    let mut conn: Option<(String, IpcStream)> = None;
                    let mut nonce: u64 = 1;

                    while let Ok(mut payload) = rx.recv() {
                        // Drain queued bursts and only process the latest snapshot
                        while let Ok(newer) = rx.try_recv() {
                            payload = newer;
                        }

                        let client_id = payload.client_id.trim().to_string();
                        let valid_id = !client_id.is_empty()
                            && client_id.len() >= 16
                            && client_id.chars().all(|c| c.is_ascii_digit());

                        if !payload.enabled || !valid_id {
                            if let Some((_, mut stream)) = conn.take() {
                                let _ = send_clear_activity(&mut stream, &mut nonce);
                            }
                            continue;
                        }

                        // Reconnect if client_id changed or disconnected
                        let needs_connect = match &conn {
                            Some((id, _)) => id != &client_id,
                            None => true,
                        };
                        if needs_connect {
                            if let Some((_, mut old)) = conn.take() {
                                let _ = send_clear_activity(&mut old, &mut nonce);
                            }
                            if let Some(stream) = connect_and_handshake(&client_id) {
                                conn = Some((client_id.clone(), stream));
                            } else {
                                continue;
                            }
                        }

                        if let Some((_, ref mut stream)) = conn {
                            let res = if payload.is_playing && payload.title.is_some() {
                                send_set_activity(stream, &payload, &mut nonce)
                            } else {
                                send_clear_activity(stream, &mut nonce)
                            };
                            if res.is_err() {
                                conn = None;
                            }
                        }
                    }
                })
                .ok();
            Self { tx }
        }

        pub fn sync(&self, payload: DiscordRpcPayload) {
            let _ = self.tx.send(payload);
        }
    }

    #[cfg(unix)]
    fn open_ipc_socket() -> Option<IpcStream> {
        use std::os::unix::net::UnixStream;
        use std::path::PathBuf;
        use std::time::Duration;

        let mut dirs: Vec<PathBuf> = Vec::new();
        for key in ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"] {
            if let Ok(val) = std::env::var(key)
                && !val.is_empty()
            {
                let base = PathBuf::from(val);
                dirs.push(base.clone());
                dirs.push(base.join("app/com.discordapp.Discord"));
                dirs.push(base.join("snap.discord"));
            }
        }
        dirs.push(PathBuf::from("/tmp"));

        for dir in dirs {
            for idx in 0..10 {
                let path = dir.join(format!("discord-ipc-{idx}"));
                if let Ok(stream) = UnixStream::connect(&path) {
                    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
                    let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));
                    return Some(stream);
                }
            }
        }
        None
    }

    #[cfg(windows)]
    fn open_ipc_socket() -> Option<IpcStream> {
        use std::fs::OpenOptions;

        for idx in 0..10 {
            let path = format!(r"\\.\pipe\discord-ipc-{idx}");
            if let Ok(file) = OpenOptions::new().read(true).write(true).open(&path) {
                return Some(file);
            }
        }
        None
    }

    fn write_frame(stream: &mut IpcStream, opcode: u32, body: &serde_json::Value) -> std::io::Result<()> {
        let bytes = serde_json::to_vec(body)?;
        let len = bytes.len() as u32;
        stream.write_all(&opcode.to_le_bytes())?;
        stream.write_all(&len.to_le_bytes())?;
        stream.write_all(&bytes)?;
        stream.flush()
    }

    fn read_frame(stream: &mut IpcStream) -> std::io::Result<()> {
        let mut header = [0u8; 8];
        stream.read_exact(&mut header)?;
        let len = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;
        if len > 256 * 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Discord IPC frame too large",
            ));
        }
        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf)?;
        Ok(())
    }

    fn connect_and_handshake(client_id: &str) -> Option<IpcStream> {
        let mut stream = open_ipc_socket()?;
        let handshake = json!({
            "v": 1,
            "client_id": client_id,
        });
        write_frame(&mut stream, 0, &handshake).ok()?;
        read_frame(&mut stream).ok()?;
        Some(stream)
    }

    fn send_clear_activity(stream: &mut IpcStream, nonce: &mut u64) -> std::io::Result<()> {
        *nonce += 1;
        let frame = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": null
            },
            "nonce": nonce.to_string()
        });
        write_frame(stream, 1, &frame)?;
        let _ = read_frame(stream);
        Ok(())
    }

    fn send_set_activity(
        stream: &mut IpcStream,
        payload: &DiscordRpcPayload,
        nonce: &mut u64,
    ) -> std::io::Result<()> {
        *nonce += 1;
        let title = payload.title.as_deref().unwrap_or("Gyro");
        let artist = payload.artist.as_deref().unwrap_or("");
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        let mut activity = serde_json::Map::new();
        // ActivityType 2 = Listening
        activity.insert("type".into(), json!(2));
        activity.insert("details".into(), json!(truncate_discord(title, 128)));
        if !artist.trim().is_empty() {
            activity.insert("state".into(), json!(truncate_discord(artist, 128)));
        }

        if payload.duration_secs > 1.0 {
            let start_ms = now_ms - (payload.position_secs.max(0.0) * 1000.0) as i64;
            let end_ms = start_ms + (payload.duration_secs * 1000.0) as i64;
            activity.insert(
                "timestamps".into(),
                json!({ "start": start_ms, "end": end_ms }),
            );
        }

        let mut assets = serde_json::Map::new();
        if let Some(ref art) = payload.art_url
            && (art.starts_with("https://") || art.starts_with("http://"))
        {
            assets.insert("large_image".into(), json!(art));
        }
        let large_text = payload
            .album
            .as_deref()
            .filter(|a| !a.trim().is_empty())
            .unwrap_or("Gyro");
        assets.insert("large_text".into(), json!(truncate_discord(large_text, 128)));
        activity.insert("assets".into(), serde_json::Value::Object(assets));

        let frame = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": serde_json::Value::Object(activity)
            },
            "nonce": nonce.to_string()
        });
        write_frame(stream, 1, &frame)?;
        let _ = read_frame(stream);
        Ok(())
    }

    fn truncate_discord(s: &str, max_chars: usize) -> String {
        let trimmed = s.trim();
        if trimmed.chars().count() <= max_chars {
            trimmed.to_string()
        } else {
            let mut out: String = trimmed.chars().take(max_chars.saturating_sub(1)).collect();
            out.push('…');
            out
        }
    }
}

#[cfg(desktop)]
pub use desktop::DiscordRpcHandle;

#[cfg(not(desktop))]
pub struct DiscordRpcHandle;

#[cfg(not(desktop))]
impl DiscordRpcHandle {
    pub fn start() -> Self {
        Self
    }

    pub fn sync(&self, _payload: DiscordRpcPayload) {}
}

#[tauri::command]
pub fn discord_rpc_sync(state: State<'_, DiscordRpcHandle>, payload: DiscordRpcPayload) {
    state.sync(payload);
}
