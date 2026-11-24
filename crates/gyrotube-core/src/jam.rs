use std::collections::HashSet;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

use crate::backend::{Backend, runtime};
use crate::models::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JamTrackInfo {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration_secs: Option<u32>,
    pub cover_url: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct JamSnapshot {
    pub active: bool,
    pub url: String,
    pub host_name: String,
    pub allow_controls: bool,
    pub is_playing: bool,
    pub position_secs: f64,
    pub now_playing: Option<JamTrackInfo>,
    pub queue: Vec<JamTrackInfo>,
    pub guest_count: usize,
    pub added_count: usize,
    pub guest_list: Vec<String>,
    #[serde(skip)]
    pub guests: HashSet<String>,
}

#[derive(Debug, Clone)]
pub enum JamCommand {
    AddTrack {
        video_id: String,
        guest_name: String,
        track: Option<Track>,
    },
    TogglePlayPause,
    NextTrack,
    PrevTrack,
    MoveTrack(usize, usize),
}

#[derive(Debug, Clone)]
pub enum JamSource {
    Lan(u64),
    Remote(String),
}

impl JamSource {
    pub fn is_current(&self, lan_session: Option<u64>, host_room: Option<&str>) -> bool {
        match self {
            Self::Lan(session) => lan_session == Some(*session),
            Self::Remote(room) => host_room == Some(room.as_str()),
        }
    }
}

pub struct JamRequest {
    pub source: JamSource,
    pub command: JamCommand,
}

pub struct JamServerHandle {
    pub url: String,
    pub session_id: u64,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl JamServerHandle {
    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for JamServerHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn detect_lan_ipv4() -> String {
    if let Ok(sock) = UdpSocket::bind("0.0.0.0:0")
        && sock.connect("8.8.8.8:80").is_ok()
        && let Ok(addr) = sock.local_addr()
        && let std::net::IpAddr::V4(ip) = addr.ip()
        && !ip.is_loopback()
    {
        return ip.to_string();
    }
    String::from("127.0.0.1")
}

pub fn start_jam_server(
    backend: Backend,
    snapshot: Arc<RwLock<JamSnapshot>>,
    cmd_tx: async_channel::Sender<JamRequest>,
) -> Result<JamServerHandle, String> {
    static NEXT_SESSION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let session_id = NEXT_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let (ready_tx, ready_rx) =
        std::sync::mpsc::sync_channel::<Result<(u16, oneshot::Sender<()>), String>>(1);

    runtime().spawn(async move {
        let listener = match TcpListener::bind("0.0.0.0:8765").await {
            Ok(l) => l,
            Err(_) => match TcpListener::bind("0.0.0.0:0").await {
                Ok(l) => l,
                Err(e) => {
                    let _ = ready_tx.send(Err(format!("No se pudo iniciar el servidor JAM: {e}")));
                    return;
                }
            },
        };
        let port = match listener.local_addr() {
            Ok(a) => a.port(),
            Err(e) => {
                let _ = ready_tx.send(Err(format!("Error obteniendo puerto local: {e}")));
                return;
            }
        };

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
        let _ = ready_tx.send(Ok((port, shutdown_tx)));

        loop {
            tokio::select! {
                _ = &mut shutdown_rx => break,
                accepted = listener.accept() => {
                    if let Ok((stream, peer)) = accepted {
                        let backend = backend.clone();
                        let snapshot = Arc::clone(&snapshot);
                        let cmd_tx = cmd_tx.clone();
                        tokio::spawn(async move {
                            handle_connection(stream, peer, backend, snapshot, cmd_tx, session_id).await;
                        });
                    }
                }
            }
        }
    });

    let (port, shutdown_tx) = ready_rx
        .recv()
        .map_err(|_| String::from("No se pudo iniciar el servidor JAM."))??;

    let ip = detect_lan_ipv4();
    let url = format!("http://{ip}:{port}");
    Ok(JamServerHandle {
        url,
        session_id,
        shutdown_tx: Some(shutdown_tx),
    })
}

async fn handle_connection(
    mut stream: TcpStream,
    peer: SocketAddr,
    backend: Backend,
    snapshot: Arc<RwLock<JamSnapshot>>,
    cmd_tx: async_channel::Sender<JamRequest>,
    session_id: u64,
) {
    let mut req_buf = Vec::with_capacity(2048);
    let mut header_end = None;
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    while req_buf.len() < 16384 {
        let mut chunk = [0u8; 4096];
        match tokio::time::timeout_at(deadline, stream.read(&mut chunk)).await {
            Ok(Ok(n)) if n > 0 => {
                req_buf.extend_from_slice(&chunk[..n]);
                if let Some(pos) = req_buf.windows(4).position(|w| w == b"\r\n\r\n") {
                    header_end = Some(pos + 4);
                    break;
                }
            }
            _ => break,
        }
    }
    if req_buf.is_empty() {
        return;
    }

    let header_end = header_end.unwrap_or(req_buf.len());
    let header_str = String::from_utf8_lossy(&req_buf[..header_end]);
    let mut lines = header_str.lines();
    let first_line = lines.next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");

    let mut content_length = 0usize;
    let mut range_header: Option<String> = None;
    for l in lines {
        if let Some((k, v)) = l.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                content_length = v.trim().parse::<usize>().unwrap_or(0).min(65536);
            } else if k.trim().eq_ignore_ascii_case("range") {
                range_header = Some(v.trim().to_string());
            }
        }
    }

    let mut body = req_buf[header_end..].to_vec();
    while body.len() < content_length {
        let mut chunk = vec![0u8; content_length - body.len()];
        match tokio::time::timeout(std::time::Duration::from_secs(3), stream.read(&mut chunk)).await {
            Ok(Ok(r)) if r > 0 => body.extend_from_slice(&chunk[..r]),
            _ => break,
        }
    }
    body.truncate(content_length);

    let guest_ip = peer.ip().to_string();
    if let Ok(mut snap) = snapshot.write() {
        snap.guests.insert(guest_ip.clone());
        snap.guest_count = snap.guests.len();
        snap.guest_list = snap.guests.iter().cloned().collect();
        snap.guest_list.sort();
    }

    let (path, query) = target.split_once('?').unwrap_or((target, ""));

    match (method, path) {
        ("GET", "/") => {
            send_response(
                &mut stream,
                "200 OK",
                "text/html; charset=utf-8",
                JAM_HTML.as_bytes(),
            )
            .await;
        }
        ("GET", "/api/state") => {
            let json = snapshot
                .read()
                .ok()
                .and_then(|s| serde_json::to_vec(&*s).ok())
                .unwrap_or_else(|| b"{}".to_vec());
            send_response(&mut stream, "200 OK", "application/json", &json).await;
        }
        ("GET", "/api/audio") => {
            let id = query_param(query, "id").unwrap_or_default();
            if id.is_empty() {
                send_response(&mut stream, "400 Bad Request", "text/plain", b"Missing id").await;
                return;
            }
            let result = match backend.verify_music_track(id.trim()).await {
                Ok(track) => backend.resolve_track_stream(&track).await,
                Err(err) => Err(err),
            };
            match result {
                Ok(info) => {
                    if info.url.starts_with("http://127.0.0.1:") {
                        proxy_loopback_audio(&mut stream, &info.url, range_header.as_deref()).await;
                    } else if info.url.starts_with("http://") || info.url.starts_with("https://") {
                        let resp = format!(
                            "HTTP/1.1 302 Found\r\nLocation: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
                            info.url
                        );
                        let _ = stream.write_all(resp.as_bytes()).await;
                    } else {
                        serve_local_audio(&mut stream, &info.url, range_header.as_deref()).await;
                    }
                }
                Err(_) => {
                    send_response(
                        &mut stream,
                        "404 Not Found",
                        "text/plain",
                        b"Audio unavailable",
                    )
                    .await;
                }
            }
        }
        ("GET", "/api/search") => {
            let q = query_param(query, "q").unwrap_or_default();
            if q.trim().is_empty() {
                send_response(&mut stream, "200 OK", "application/json", b"[]").await;
                return;
            }
            match backend.search_all(q.trim()).await {
                Ok(res) => {
                    let tracks: Vec<&Track> = res.tracks.iter().take(15).collect();
                    let json = serde_json::to_vec(&tracks).unwrap_or_else(|_| b"[]".to_vec());
                    send_response(&mut stream, "200 OK", "application/json", &json).await;
                }
                Err(err) => {
                    let json = serde_json::json!({ "error": err }).to_string();
                    send_response(
                        &mut stream,
                        "500 Internal Server Error",
                        "application/json",
                        json.as_bytes(),
                    )
                    .await;
                }
            }
        }
        ("POST", "/api/queue/add") => {
            #[derive(Deserialize)]
            struct AddReq {
                id: String,
                #[serde(default)]
                title: Option<String>,
                #[serde(default)]
                artist: Option<String>,
                #[serde(default)]
                artist_id: Option<String>,
                #[serde(default)]
                album: Option<String>,
                #[serde(default)]
                album_id: Option<String>,
                #[serde(default)]
                duration_secs: Option<u32>,
                #[serde(default)]
                view_count: Option<u64>,
                #[serde(default)]
                cover_url: Option<String>,
                #[serde(default)]
                cover_large_url: Option<String>,
                #[serde(default)]
                is_video: bool,
            }
            match serde_json::from_slice::<AddReq>(&body) {
                Ok(request) if !request.id.trim().is_empty() => {
                    let video_id = request.id.trim().to_string();
                    let track = request
                        .title
                        .as_deref()
                        .map(str::trim)
                        .filter(|t| !t.is_empty())
                        .map(|title| Track {
                            is_music: false,
                            is_video: request.is_video,
                            id: video_id.clone(),
                            title: title.to_string(),
                            artist: request.artist.unwrap_or_default(),
                            artist_id: request.artist_id,
                            album: request.album,
                            album_id: request.album_id,
                            duration_secs: request.duration_secs,
                            view_count: request.view_count,
                            cover_url: request.cover_url,
                            cover_large_url: request.cover_large_url,
                        });
                    if let Ok(mut snap) = snapshot.write() {
                        snap.added_count = snap.added_count.saturating_add(1);
                    }
                    let _ = cmd_tx
                        .send(JamRequest {
                            source: JamSource::Lan(session_id),
                            command: JamCommand::AddTrack {
                                video_id,
                                guest_name: guest_ip,
                                track,
                            },
                        })
                        .await;
                    send_response(
                        &mut stream,
                        "202 Accepted",
                        "application/json",
                        br#"{"accepted":true}"#,
                    )
                    .await;
                }
                _ => {
                    send_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json",
                        br#"{"error":"invalid track"}"#,
                    )
                    .await;
                }
            }
        }

        ("POST", "/api/queue/move") => {
            let allowed = snapshot.read().map(|s| s.allow_controls).unwrap_or(false);
            if !allowed {
                send_response(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    br#"{"error":"controls disabled"}"#,
                )
                .await;
                return;
            }
            #[derive(Deserialize)]
            struct MoveReq {
                from: usize,
                to: usize,
            }
            if let Ok(req) = serde_json::from_slice::<MoveReq>(&body) {
                let _ = cmd_tx
                    .send(JamRequest {
                        source: JamSource::Lan(session_id),
                        command: JamCommand::MoveTrack(req.from, req.to),
                    })
                    .await;
                send_response(&mut stream, "200 OK", "application/json", br#"{"ok":true}"#).await;
                return;
            }
            send_response(
                &mut stream,
                "400 Bad Request",
                "application/json",
                br#"{"error":"invalid move"}"#,
            )
            .await;
        }
        ("POST", "/api/control") => {
            let allowed = snapshot.read().map(|s| s.allow_controls).unwrap_or(false);
            if !allowed {
                send_response(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    br#"{"error":"controls disabled"}"#,
                )
                .await;
                return;
            }
            #[derive(Deserialize)]
            struct CtrlReq {
                action: String,
            }
            if let Ok(req) = serde_json::from_slice::<CtrlReq>(&body) {
                let cmd = match req.action.as_str() {
                    "play_pause" => Some(JamCommand::TogglePlayPause),
                    "next" => Some(JamCommand::NextTrack),
                    "prev" => Some(JamCommand::PrevTrack),
                    _ => None,
                };
                if let Some(c) = cmd {
                    let _ = cmd_tx
                        .send(JamRequest {
                            source: JamSource::Lan(session_id),
                            command: c,
                        })
                        .await;
                    send_response(&mut stream, "200 OK", "application/json", br#"{"ok":true}"#)
                        .await;
                    return;
                }
            }
            send_response(
                &mut stream,
                "400 Bad Request",
                "application/json",
                br#"{"error":"invalid action"}"#,
            )
            .await;
        }
        _ => {
            send_response(&mut stream, "404 Not Found", "text/plain", b"Not Found").await;
        }
    }
}

async fn proxy_loopback_audio(stream: &mut TcpStream, url: &str, range_header: Option<&str>) {
    let Ok(parsed) = reqwest::Url::parse(url) else {
        send_response(stream, "502 Bad Gateway", "text/plain", b"Bad stream URL").await;
        return;
    };
    let Some(port) = parsed.port() else {
        send_response(stream, "502 Bad Gateway", "text/plain", b"Missing port").await;
        return;
    };
    let Ok(mut upstream) = TcpStream::connect(("127.0.0.1", port)).await else {
        send_response(stream, "502 Bad Gateway", "text/plain", b"Stream unavailable").await;
        return;
    };
    let path_and_query = match parsed.query() {
        Some(q) => format!("{}?{q}", parsed.path()),
        None => parsed.path().to_string(),
    };
    let mut req = format!(
        "GET {path_and_query} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n"
    );
    if let Some(range) = range_header {
        req.push_str(&format!("Range: {range}\r\n"));
    }
    req.push_str("\r\n");
    if upstream.write_all(req.as_bytes()).await.is_err() {
        return;
    }
    let _ = tokio::io::copy(&mut upstream, stream).await;
}

async fn serve_local_audio(stream: &mut TcpStream, path: &str, range_header: Option<&str>) {
    use tokio::io::AsyncSeekExt;
    let Ok(mut file) = tokio::fs::File::open(path).await else {
        send_response(stream, "404 Not Found", "text/plain", b"Not Found").await;
        return;
    };
    let Ok(meta) = file.metadata().await else {
        send_response(stream, "500 Internal Server Error", "text/plain", b"Error").await;
        return;
    };
    let total = meta.len();
    let mime = if path.ends_with(".webm") {
        "audio/webm"
    } else {
        "audio/mp4"
    };

    let (status, start, end) = if let Some(r) = range_header.and_then(|h| h.strip_prefix("bytes=")) {
        let (s_str, e_str) = r.split_once('-').unwrap_or((r, ""));
        let start = s_str.parse::<u64>().unwrap_or(0).min(total.saturating_sub(1));
        let end = e_str
            .parse::<u64>()
            .ok()
            .map(|e| e.min(total.saturating_sub(1)))
            .unwrap_or_else(|| total.saturating_sub(1));
        ("206 Partial Content", start, end.max(start))
    } else {
        ("200 OK", 0, total.saturating_sub(1))
    };

    let len = if total == 0 { 0 } else { end - start + 1 };
    let mut header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {mime}\r\nContent-Length: {len}\r\nAccept-Ranges: bytes\r\nCache-Control: no-store\r\nConnection: close\r\n"
    );
    if status == "206 Partial Content" {
        header.push_str(&format!("Content-Range: bytes {start}-{end}/{total}\r\n"));
    }
    header.push_str("\r\n");
    if stream.write_all(header.as_bytes()).await.is_err() {
        return;
    }
    if start > 0 && file.seek(std::io::SeekFrom::Start(start)).await.is_err() {
        return;
    }
    let mut limited = file.take(len);
    let _ = tokio::io::copy(&mut limited, stream).await;
}

async fn send_response(stream: &mut TcpStream, status: &str, content_type: &str, body: &[u8]) {
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(header.as_bytes()).await;
    let _ = stream.write_all(body).await;
    let _ = stream.flush().await;
    let _ = stream.shutdown().await;
}

fn query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=')?;
        if k == key {
            return Some(percent_decode(v));
        }
    }
    None
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                if let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                    out.push((h << 4) | l);
                    i += 3;
                } else {
                    out.push(b'%');
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

pub fn generate_qr_svg(data: &str) -> Option<String> {
    let code = qrcode::QrCode::new(data).ok()?;
    let width = code.width();
    let quiet = 4usize;
    let total = width + quiet * 2;
    let colors = code.to_colors();
    let mut path = String::new();
    for y in 0..width {
        for x in 0..width {
            if matches!(colors[y * width + x], qrcode::Color::Dark) {
                let px = x + quiet;
                let py = y + quiet;
                path.push_str(&format!("M{px} {py}h1v1h-1z"));
            }
        }
    }
    Some(format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {total} {total}" shape-rendering="crispEdges"><rect width="100%" height="100%" fill="#ffffff"/><path d="{path}" fill="#000000"/></svg>"##
    ))
}

const JAM_HTML: &str = include_str!("../assets/spin.html");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_from_closed_or_replaced_sessions_are_rejected() {
        assert!(JamSource::Lan(1).is_current(Some(1), None));
        assert!(!JamSource::Lan(1).is_current(Some(2), None));
        assert!(!JamSource::Lan(1).is_current(None, None));
        assert!(JamSource::Remote("room-a".into()).is_current(None, Some("room-a")));
        assert!(!JamSource::Remote("room-a".into()).is_current(None, Some("room-b")));
        assert!(!JamSource::Remote("room-a".into()).is_current(None, None));
    }

    #[test]
    fn generated_svg_decodes_to_original_bytes_for_all_supported_versions() {
        let cases = [
            String::from("HELLO"),
            String::from("http://192.168.1.42:8765"),
            "x".repeat(40),
            "x".repeat(65),
            "é".repeat(50),
        ];
        for expected in cases {
            let svg = generate_qr_svg(&expected).unwrap();
            let modules: usize = svg
                .split("viewBox=\"0 0 ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let scale = 8;
            let width = modules * scale;
            let mut image = vec![255u8; width * width];
            let path = svg
                .split("<path d=\"")
                .nth(1)
                .unwrap()
                .split('"')
                .next()
                .unwrap();
            for cell in path.split('M').skip(1) {
                let (x, y) = cell.split('h').next().unwrap().split_once(' ').unwrap();
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                for row in y * scale..(y + 1) * scale {
                    image[row * width + x * scale..row * width + (x + 1) * scale].fill(0);
                }
            }
            let mut decoder = quircs::Quirc::default();
            let decoded: Vec<_> = decoder
                .identify(width, width, &image)
                .filter_map(Result::ok)
                .filter_map(|code| code.decode().ok())
                .collect();
            assert!(
                decoded
                    .iter()
                    .any(|data| data.payload == expected.as_bytes()),
                "QR failed to decode {} bytes",
                expected.len()
            );
        }
        assert!(generate_qr_svg(&"x".repeat(107)).is_some());
    }

    #[test]
    fn generates_valid_qr_svg_for_lan_url() {
        let svg = generate_qr_svg("http://192.168.1.42:8765").expect("qr svg");
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("viewBox=\"0 0 33 33\"")); // V2 (25) + 8 quiet = 33
        assert!(svg.contains("<path d=\""));
    }

    #[tokio::test]
    async fn proxies_loopback_audio_with_range_header() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let upstream_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let upstream_port = upstream_listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            let (mut sock, _) = upstream_listener.accept().await.unwrap();
            let mut buf = [0u8; 512];
            let n = sock.read(&mut buf).await.unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);
            assert!(req.contains("GET /stream/test-1.webm HTTP/1.1"));
            assert!(req.contains("Range: bytes=0-3"));
            let _ = sock
                .write_all(
                    b"HTTP/1.1 206 Partial Content\r\nContent-Type: audio/webm\r\nContent-Length: 4\r\n\r\nABCD",
                )
                .await;
        });

        let client_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client_addr = client_listener.local_addr().unwrap();
        let url = format!("http://127.0.0.1:{upstream_port}/stream/test-1.webm");
        let proxy_task = tokio::spawn(async move {
            let (mut srv_stream, _) = client_listener.accept().await.unwrap();
            proxy_loopback_audio(&mut srv_stream, &url, Some("bytes=0-3")).await;
        });

        let mut client = TcpStream::connect(client_addr).await.unwrap();
        let mut resp = Vec::new();
        client.read_to_end(&mut resp).await.unwrap();
        proxy_task.await.unwrap();
        let resp_str = String::from_utf8_lossy(&resp);
        assert!(resp_str.starts_with("HTTP/1.1 206 Partial Content"));
        assert!(resp_str.ends_with("\r\n\r\nABCD"));
    }

    #[tokio::test]
    async fn accepts_multiple_queue_add_requests_with_full_track_metadata() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let snapshot = Arc::new(RwLock::new(JamSnapshot::default()));
        let (cmd_tx, cmd_rx) = async_channel::unbounded();
        let backend = Backend::new();

        let srv_snap = Arc::clone(&snapshot);
        let server = tokio::spawn(async move {
            for _ in 0..2 {
                let (stream, peer) = listener.accept().await.unwrap();
                handle_connection(
                    stream,
                    peer,
                    backend.clone(),
                    Arc::clone(&srv_snap),
                    cmd_tx.clone(),
                    42,
                )
                .await;
            }
        });

        for (id, title) in [("vid-1", "First Song"), ("vid-2", "Second Song")] {
            let body = serde_json::json!({
                "id": id,
                "title": title,
                "artist": "Test Artist",
                "verified_catalog_audio": true,
            })
            .to_string();
            let header = format!(
                "POST /api/queue/add HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let mut client = TcpStream::connect(addr).await.unwrap();
            // Send headers and body in separate writes to test chunked HTTP parsing
            client.write_all(header.as_bytes()).await.unwrap();
            client.write_all(body.as_bytes()).await.unwrap();
            let mut resp = Vec::new();
            client.read_to_end(&mut resp).await.unwrap();
            let resp_str = String::from_utf8_lossy(&resp);
            assert!(resp_str.starts_with("HTTP/1.1 202 Accepted"), "{resp_str}");
        }

        server.await.unwrap();
        assert_eq!(snapshot.read().unwrap().added_count, 2);

        for (expected_id, expected_title) in [("vid-1", "First Song"), ("vid-2", "Second Song")] {
            let req = cmd_rx.recv().await.unwrap();
            assert!(req.source.is_current(Some(42), None));
            assert!(matches!(
                req.command,
                JamCommand::AddTrack {
                    ref video_id,
                    track: Some(ref track),
                    ..
                } if video_id == expected_id && track.title == expected_title && !track.is_music
            ));
        }
    }
}
