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
    AddTrack(String, String),
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
    let mut buf = vec![0u8; 16384];
    let n = match tokio::time::timeout(std::time::Duration::from_secs(5), stream.read(&mut buf)).await {
        Ok(Ok(n)) if n > 0 => n,
        _ => return,
    };

    let req_bytes = &buf[..n];
    let header_end = req_bytes
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|p| p + 4)
        .unwrap_or(req_bytes.len());
    let header_str = String::from_utf8_lossy(&req_bytes[..header_end]);
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

    let mut body = req_bytes[header_end..].to_vec();
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
            }
            match serde_json::from_slice::<AddReq>(&body) {
                Ok(request) if !request.id.is_empty() => {
                    let _ = cmd_tx
                        .send(JamRequest {
                            source: JamSource::Lan(session_id),
                            command: JamCommand::AddTrack(request.id, guest_ip),
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

// Pure-Rust QR Code generator (Byte mode, ECC Level L, Versions 1..=5)
pub fn generate_qr_svg(data: &str) -> Option<String> {
    let bytes = data.as_bytes();
    // (version, size, data_codewords, ec_codewords, align_pos)
    const SPECS: &[(usize, usize, usize, usize, Option<usize>)] = &[
        (1, 21, 19, 7, None),
        (2, 25, 34, 10, Some(18)),
        (3, 29, 55, 15, Some(22)),
        (4, 33, 80, 20, Some(26)),
        (5, 37, 108, 26, Some(30)),
    ];

    let &(_ver, size, data_cw, ec_cw, align_pos) = SPECS
        .iter()
        .find(|&&(_, _, dcw, _, _)| bytes.len() + 2 <= dcw)?;

    let mut bits: Vec<u8> = Vec::with_capacity(data_cw * 8);
    push_bits(&mut bits, 0b0100, 4); // Byte mode
    push_bits(&mut bits, bytes.len() as u32, 8);
    for &b in bytes {
        push_bits(&mut bits, b as u32, 8);
    }
    let max_bits = data_cw * 8;
    let term = (max_bits - bits.len()).min(4);
    push_bits(&mut bits, 0, term);
    while !bits.len().is_multiple_of(8) {
        bits.push(0);
    }
    let mut codewords: Vec<u8> = bits
        .chunks_exact(8)
        .map(|c| c.iter().fold(0u8, |acc, &bit| (acc << 1) | bit))
        .collect();
    let pad = [0xECu8, 0x11u8];
    let mut pi = 0;
    while codewords.len() < data_cw {
        codewords.push(pad[pi % 2]);
        pi += 1;
    }

    let ec = rs_encode(&codewords, ec_cw);
    codewords.extend_from_slice(&ec);

    let mut grid = vec![vec![0i8; size]; size]; // 0 = empty, 1 = light, 2 = dark, <0 = reserved
    place_finder(&mut grid, 0, 0);
    place_finder(&mut grid, 0, size - 7);
    place_finder(&mut grid, size - 7, 0);
    if let Some(ap) = align_pos {
        place_alignment(&mut grid, ap, ap);
    }
    for i in 8..(size - 8) {
        let dark = if i % 2 == 0 { -2 } else { -1 };
        grid[6][i] = dark;
        grid[i][6] = dark;
    }
    grid[size - 8][8] = -2; // Dark module

    // Reserve format info
    for i in 0..9 {
        if i != 6 {
            if grid[8][i] == 0 {
                grid[8][i] = -1;
            }
            if grid[i][8] == 0 {
                grid[i][8] = -1;
            }
        }
    }
    for i in (size - 8)..size {
        if grid[8][i] == 0 {
            grid[8][i] = -1;
        }
        if grid[i][8] == 0 {
            grid[i][8] = -1;
        }
    }

    // Place data bits (zig-zag)
    let all_bits: Vec<u8> = codewords
        .iter()
        .flat_map(|&b| (0..8).rev().map(move |k| (b >> k) & 1))
        .collect();
    let mut bi = 0usize;
    let mut col = (size as isize) - 1;
    let mut up = true;
    while col > 0 {
        if col == 6 {
            col -= 1;
        }
        for step in 0..size {
            let r = if up { size - 1 - step } else { step };
            for dx in 0..2 {
                let c = (col - dx) as usize;
                if grid[r][c] == 0 {
                    let bit = all_bits.get(bi).copied().unwrap_or(0);
                    bi += 1;
                    grid[r][c] = if bit == 1 { 2 } else { 1 };
                }
            }
        }
        up = !up;
        col -= 2;
    }

    // Mask 0 ((r + c) % 2 == 0) + Format info for Level L, Mask 0: 0x77c4
    const FORMAT_L_M0: u16 = 0x77c4;
    let mut dark_grid = vec![vec![false; size]; size];
    for r in 0..size {
        for c in 0..size {
            dark_grid[r][c] = match grid[r][c] {
                -2 => true,
                -1 => false,
                2 => ((r + c) % 2) != 0,
                1 => ((r + c) % 2) == 0,
                _ => false,
            };
        }
    }
    for i in 0..15 {
        let bit = ((FORMAT_L_M0 >> i) & 1) == 1;
        let (r1, c1) = match i {
            0..=5 => (i, 8),
            6 => (7, 8),
            7 => (8, 8),
            8 => (8, 7),
            _ => (8, 14 - i),
        };
        dark_grid[r1][c1] = bit;
        let (r2, c2) = if i < 8 {
            (8, size - 1 - i)
        } else {
            (size - 15 + i, 8)
        };
        dark_grid[r2][c2] = bit;
    }

    let quiet = 4usize;
    let total = size + quiet * 2;
    let mut path = String::new();
    for (r, row) in dark_grid.iter().enumerate() {
        for (c, &is_dark) in row.iter().enumerate() {
            if is_dark {
                path.push_str(&format!("M{} {}h1v1h-1z", c + quiet, r + quiet));
            }
        }
    }
    let px = total * 16;
    Some(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{px}\" height=\"{px}\" viewBox=\"0 0 {total} {total}\" shape-rendering=\"crispEdges\"><rect width=\"{total}\" height=\"{total}\" fill=\"#ffffff\"/><path d=\"{path}\" fill=\"#111111\"/></svg>"
    ))
}

fn push_bits(out: &mut Vec<u8>, val: u32, count: usize) {
    for i in (0..count).rev() {
        out.push(((val >> i) & 1) as u8);
    }
}

fn place_finder(grid: &mut [Vec<i8>], r0: usize, c0: usize) {
    let n = grid.len() as isize;
    for dr in -1..=7 {
        for dc in -1..=7 {
            let r = r0 as isize + dr;
            let c = c0 as isize + dc;
            if r >= 0 && r < n && c >= 0 && c < n {
                let dark = (0..=6).contains(&dr)
                    && (0..=6).contains(&dc)
                    && (dr == 0 || dr == 6 || dc == 0 || dc == 6 || ((2..=4).contains(&dr) && (2..=4).contains(&dc)));
                grid[r as usize][c as usize] = if dark { -2 } else { -1 };
            }
        }
    }
}

fn place_alignment(grid: &mut [Vec<i8>], cr: usize, cc: usize) {
    for dr in -2isize..=2 {
        for dc in -2isize..=2 {
            let dark = dr.abs() == 2 || dc.abs() == 2 || (dr == 0 && dc == 0);
            grid[(cr as isize + dr) as usize][(cc as isize + dc) as usize] = if dark { -2 } else { -1 };
        }
    }
}

fn rs_encode(data: &[u8], ec_len: usize) -> Vec<u8> {
    let mut exp = [0u8; 512];
    let mut log = [0u8; 256];
    let mut x = 1u16;
    for i in 0..255 {
        exp[i] = x as u8;
        log[x as usize] = i as u8;
        x <<= 1;
        if x >= 256 {
            x ^= 0x11d;
        }
    }
    for i in 255..512 {
        exp[i] = exp[i - 255];
    }
    let gf_mul = |a: u8, b: u8| -> u8 {
        if a == 0 || b == 0 {
            0
        } else {
            exp[(log[a as usize] as usize) + (log[b as usize] as usize)]
        }
    };

    let mut gen_poly = vec![1u8];
    for i in 0..ec_len {
        let root = exp[i];
        let mut next = vec![0u8; gen_poly.len() + 1];
        for (j, &coef) in gen_poly.iter().enumerate() {
            next[j] ^= coef;
            next[j + 1] ^= gf_mul(coef, root);
        }
        gen_poly = next;
    }

    let mut msg = vec![0u8; data.len() + ec_len];
    msg[..data.len()].copy_from_slice(data);
    for i in 0..data.len() {
        let factor = msg[i];
        if factor != 0 {
            for (j, &g) in gen_poly.iter().enumerate() {
                msg[i + j] ^= gf_mul(g, factor);
            }
        }
    }
    msg[data.len()..].to_vec()
}

const JAM_HTML: &str = r##"<!doctype html>
<html lang="es">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>GyroTube SPIN</title>
<style>
:root{
  color-scheme:dark;
  --bg:#121417;
  --surface:#1a1d22;
  --surface-2:#22262d;
  --surface-hover:#292e37;
  --line:rgba(255,255,255,.11);
  --line-strong:rgba(255,255,255,.22);
  --fg:#f3f5f8;
  --muted:#a6afbe;
  --accent:#2190a4;
  --accent-hi:#2aa8be;
  --accent-soft:color-mix(in oklab,var(--accent) 20%,transparent);
  --accent-border:color-mix(in oklab,var(--accent) 55%,transparent);
  --radius:16px;
}
*,*::before,*::after{box-sizing:border-box}
body{
  margin:0;
  min-height:100dvh;
  font-family:system-ui,-apple-system,"Segoe UI",Roboto,Inter,sans-serif;
  background:
    radial-gradient(900px 420px at 15% -5%,color-mix(in oklab,var(--accent) 18%,transparent),transparent 70%),
    radial-gradient(700px 380px at 90% 0%,rgba(90,115,255,.10),transparent 65%),
    var(--bg);
  color:var(--fg);
  padding:16px 16px 72px;
  -webkit-font-smoothing:antialiased;
}
.sr-only{
  position:absolute;width:1px;height:1px;padding:0;margin:-1px;
  overflow:hidden;clip-path:inset(50%);white-space:nowrap;border:0;
}
.shell{max-width:1060px;margin:0 auto;display:flex;flex-direction:column;gap:18px}
header{
  display:flex;align-items:center;justify-content:space-between;gap:12px;
  padding:6px 4px;
}
.brand{display:flex;align-items:center;gap:10px;font-weight:800;font-size:1.2rem;letter-spacing:-.02em}
.brand-mark{
  width:32px;height:32px;border-radius:10px;
  background:linear-gradient(135deg,var(--accent-hi),#155e75);
  display:grid;place-items:center;
  box-shadow:0 6px 18px color-mix(in oklab,var(--accent) 35%,transparent);
  flex-shrink:0;
}
.pill{
  display:inline-flex;align-items:center;gap:6px;
  font-size:.75rem;font-weight:700;letter-spacing:.03em;
  padding:4px 10px;border-radius:999px;
  background:var(--accent-soft);
  border:1px solid var(--accent-border);
  color:#8de3f2;
}
.guests-pill{
  background:var(--surface);border:1px solid var(--line-strong);color:var(--fg);
}
.dot{width:7px;height:7px;border-radius:999px;background:#34d399;box-shadow:0 0 8px #34d399}
.grid{display:grid;grid-template-columns:1fr;gap:14px;align-items:start}
.mobile-tabs{
  display:grid;grid-template-columns:1fr 1fr;gap:6px;
  padding:4px;border-radius:999px;
  background:var(--surface);border:1px solid var(--line);
}
.tab-btn{
  border:0;border-radius:999px;padding:9px 12px;
  background:transparent;color:var(--muted);font-weight:700;font-size:.86rem;
}
.tab-btn[aria-selected="true"]{
  background:var(--surface-2);color:var(--fg);
  border:1px solid var(--line-strong);
  box-shadow:0 2px 8px rgba(0,0,0,.22);
}
@media (max-width:859px){
  body{padding:12px 12px 56px}
  .shell{gap:12px}
  .card{padding:13px}
  .np-main{gap:12px}
  .cover-wrap{width:56px;height:56px;border-radius:10px}
  .np-actions{
    display:flex;align-items:center;justify-content:space-between;gap:8px;
    margin-top:12px;flex-wrap:wrap;
  }
  .np-actions .ctrls,.np-actions .listen-row{margin-top:0}
  .np-actions .play-pill{min-width:44px;height:38px;padding:0 14px;font-size:.82rem}
  .np-actions .icon-btn{width:36px;height:36px}
  .np-actions #listen-btn{padding:8px 12px;font-size:.8rem}
  .queue-list{max-height:none;overflow:visible;padding-right:0}
  .grid[data-tab="search"] #queue-card{display:none}
  .grid[data-tab="queue"] #search-card{display:none}
}
@media (max-width:440px){
  #results .dur{display:none}
  #results button{padding:7px 11px;font-size:.8rem}
}
@media (min-width:860px){
  body{padding:24px 28px 64px}
  .mobile-tabs{display:none}
  .grid{grid-template-columns:360px minmax(0,1fr);gap:20px}
  .left-col{position:sticky;top:20px;display:flex;flex-direction:column;gap:16px}
}
.left-col,.right-col{display:flex;flex-direction:column;gap:16px;min-width:0}
.card{
  background:color-mix(in oklab,var(--surface) 92%,transparent);
  border:1px solid var(--line);
  border-radius:var(--radius);
  padding:16px;
  backdrop-filter:blur(14px);
  box-shadow:0 12px 32px rgba(0,0,0,.28);
}
.np-card{
  position:relative;overflow:hidden;
  background:linear-gradient(160deg,color-mix(in oklab,var(--accent) 16%,var(--surface)) 0%,var(--surface) 62%);
}
.section-head{
  display:flex;align-items:center;justify-content:space-between;gap:8px;
  margin-bottom:12px;
}
.h{
  font-size:.74rem;font-weight:700;color:var(--muted);
  text-transform:uppercase;letter-spacing:.08em;
}
.np-main{display:flex;align-items:center;gap:14px}
@media (min-width:860px){
  .np-main{flex-direction:column;text-align:center;gap:12px}
}
.cover-wrap{
  position:relative;width:68px;height:68px;border-radius:12px;overflow:hidden;
  background:var(--surface-2);border:1px solid var(--line);flex-shrink:0;
  box-shadow:0 10px 26px rgba(0,0,0,.42);
}
@media (min-width:860px){
  .cover-wrap{width:176px;height:176px;border-radius:16px}
}
.cover-wrap img{width:100%;height:100%;object-fit:cover;display:block}
.cover-wrap img:not([src]),.cover-wrap img[src=""]{opacity:0}
.cover-fallback{
  position:absolute;inset:0;display:grid;place-items:center;color:var(--muted);
}
.meta{min-width:0;flex:1;width:100%}
.title{
  font-weight:700;font-size:1.02rem;line-height:1.28;
  white-space:nowrap;overflow:hidden;text-overflow:ellipsis;
}
.sub{
  font-size:.85rem;color:var(--muted);line-height:1.3;
  white-space:nowrap;overflow:hidden;text-overflow:ellipsis;margin-top:3px;
}
.progress-wrap{margin-top:14px}
.progress-track{
  height:5px;border-radius:999px;background:rgba(255,255,255,.12);overflow:hidden;
}
.progress-fill{
  height:100%;width:0%;border-radius:999px;
  background:linear-gradient(90deg,var(--accent),var(--accent-hi));
  transition:width .35s linear;
}
.progress-times{
  display:flex;justify-content:space-between;margin-top:6px;
  font-size:.73rem;color:var(--muted);font-variant-numeric:tabular-nums;
}
.ctrls{
  display:flex;align-items:center;justify-content:center;gap:10px;margin-top:14px;
}
.listen-row{display:flex;justify-content:center;margin-top:12px}
button{
  display:inline-flex;align-items:center;justify-content:center;gap:7px;
  border:1px solid var(--line-strong);border-radius:999px;
  padding:9px 15px;font:inherit;font-size:.86rem;font-weight:600;
  cursor:pointer;background:var(--surface-2);color:var(--fg);
  transition:background-color .15s ease,border-color .15s ease,transform .1s ease;
}
button:hover{background:var(--surface-hover);border-color:rgba(255,255,255,.32)}
button:active{transform:scale(.97)}
button:focus-visible,input:focus-visible{
  outline:2px solid var(--accent-hi);outline-offset:2px;
}
button.primary{
  background:var(--accent);border-color:transparent;color:#fff;
  box-shadow:0 4px 14px color-mix(in oklab,var(--accent) 40%,transparent);
}
button.primary:hover{background:var(--accent-hi)}
button.icon-btn{width:40px;height:40px;padding:0}
button.play-pill{min-width:56px;height:44px;padding:0 18px}
button.added{
  background:rgba(16,185,129,.18);border-color:rgba(52,211,153,.5);color:#6ee7b7;
}
.search-form{display:flex;gap:8px;align-items:center}
.search-field{
  position:relative;flex:1;min-width:0;display:flex;align-items:center;
}
.search-icon{
  position:absolute;left:14px;color:var(--muted);pointer-events:none;display:flex;
}
input[type="search"]{
  width:100%;min-width:0;
  border:1px solid var(--line-strong);border-radius:999px;
  padding:11px 16px 11px 40px;
  background:var(--surface-2);color:var(--fg);font:inherit;font-size:.94rem;
}
input[type="search"]::placeholder{color:var(--muted)}
input[type="search"]:focus{border-color:var(--accent-hi)}
.list{display:flex;flex-direction:column;gap:4px;margin-top:12px}
.row{
  display:flex;align-items:center;gap:11px;padding:8px 10px;
  border-radius:12px;border:1px solid transparent;
  transition:background-color .14s ease,border-color .14s ease;
}
.row:hover{background:rgba(255,255,255,.045)}
.row.cur{
  background:var(--accent-soft);
  border-color:var(--accent-border);
}
.row.cur .row-title{color:#8de3f2}
.row.dragging{opacity:.42}
.row.drop-target{box-shadow:inset 0 2px 0 0 var(--accent-hi)}
.q-actions{display:flex;align-items:center;gap:4px;flex-shrink:0}
.q-move-btn{
  width:28px;height:28px;padding:0;border-radius:999px;
  background:rgba(255,255,255,.06);border:1px solid var(--line);
}
.q-move-btn:disabled{opacity:.28;cursor:default;pointer-events:none}
.drag-grip{color:var(--muted);opacity:.55;cursor:grab;display:flex;flex-shrink:0}
.row-art{
  width:44px;height:44px;border-radius:8px;object-fit:cover;
  background:var(--surface-2);flex-shrink:0;border:1px solid var(--line);
}
.row-title{
  font-weight:600;font-size:.92rem;
  white-space:nowrap;overflow:hidden;text-overflow:ellipsis;
}
.row-sub{
  font-size:.8rem;color:var(--muted);
  white-space:nowrap;overflow:hidden;text-overflow:ellipsis;margin-top:2px;
}
.dur{
  font-size:.76rem;color:var(--muted);font-variant-numeric:tabular-nums;
  margin-right:4px;flex-shrink:0;
}
.empty{
  text-align:center;padding:24px 12px;color:var(--muted);font-size:.88rem;
}
.queue-list{max-height:360px;overflow-y:auto;padding-right:2px}
.queue-list::-webkit-scrollbar{width:6px}
.queue-list::-webkit-scrollbar-thumb{background:rgba(255,255,255,.16);border-radius:999px}
.toast-region{position:fixed;inset:auto 0 18px 0;pointer-events:none;z-index:50}
#toast-pop{
  pointer-events:auto;
  position:fixed;inset:auto auto 18px 50%;transform:translateX(-50%);
  margin:0;max-width:min(92vw,420px);
  background:#232832;color:var(--fg);
  border:1px solid var(--line-strong);border-radius:14px;
  padding:10px 12px 10px 16px;
  box-shadow:0 14px 36px rgba(0,0,0,.5);
  display:none;align-items:center;gap:12px;font-size:.88rem;
  opacity:0;
  transition:opacity .2s ease,transform .2s ease;
  transition-behavior:allow-discrete;
}
#toast-pop:is(:popover-open,.\:popover-open){
  display:flex;opacity:1;
}
@starting-style{
  #toast-pop:is(:popover-open,.\:popover-open){
    opacity:0;transform:translateX(-50%) translateY(8px);
  }
}
.toast-close{
  width:28px;height:28px;padding:0;border-radius:999px;
  background:rgba(255,255,255,.08);border:1px solid transparent;flex-shrink:0;
}
@media (prefers-reduced-motion:reduce){
  *,*::before,*::after{transition:none!important;animation:none!important}
}
@media (forced-colors:active){
  .card,.pill,button,input[type="search"],#toast-pop{
    background:Canvas;color:CanvasText;border:1px solid ButtonBorder;box-shadow:none;
  }
  button.primary,.row.cur{border:2px solid Highlight}
  .progress-track{border:1px solid ButtonBorder}
  .progress-fill{background:Highlight}
}
</style>
</head>
<body>
<div class="shell">
  <header>
    <div class="brand">
      <span class="brand-mark" aria-hidden="true">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="2.5"/><path d="M12 3a9 9 0 0 1 9 9"/></svg>
      </span>
      <span>GyroTube</span>
      <span class="pill">SPIN</span>
    </div>
    <div class="pill guests-pill" id="guests"><span class="dot" aria-hidden="true"></span><span>0 invitados</span></div>
  </header>

  <main class="grid" id="main-grid" data-tab="search">
    <div class="left-col">
      <section class="card np-card" aria-label="Sonando ahora">
        <div class="section-head">
          <div class="h">Sonando ahora</div>
          <span class="h" id="np-state"></span>
        </div>
        <div class="np-main">
          <div class="cover-wrap">
            <div class="cover-fallback" aria-hidden="true">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
            </div>
            <img id="np-cover" alt=""/>
          </div>
          <div class="meta">
            <div class="title" id="np-title">Sin reproducción</div>
            <div class="sub" id="np-artist">Busca una canción para empezar</div>
          </div>
        </div>
        <div class="progress-wrap">
          <div class="progress-track"><div class="progress-fill" id="np-progress"></div></div>
          <div class="progress-times"><span id="np-cur">0:00</span><span id="np-dur">0:00</span></div>
        </div>
        <div class="np-actions">
          <div class="ctrls" id="ctrls" style="display:none">
            <button class="icon-btn" type="button" onclick="ctrl('prev')" aria-label="Anterior">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M6 6h2v12H6zm3.5 6 8.5 6V6z"/></svg>
            </button>
            <button class="primary play-pill" type="button" onclick="ctrl('play_pause')" id="pp-btn" aria-label="Reproducir o pausar"></button>
            <button class="icon-btn" type="button" onclick="ctrl('next')" aria-label="Siguiente">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M16 6h2v12h-2zM6 18l8.5-6L6 6z"/></svg>
            </button>
          </div>
          <div class="listen-row">
            <button id="listen-btn" type="button" onclick="toggleLocalAudio()"></button>
          </div>
        </div>
        <audio id="local-audio" preload="auto" style="display:none"></audio>
      </section>

      <div class="mobile-tabs" role="tablist" aria-label="Secciones de SPIN">
        <button class="tab-btn" type="button" role="tab" id="tab-search" aria-selected="true" aria-controls="search-card" onclick="setMobileTab('search')">Buscar</button>
        <button class="tab-btn" type="button" role="tab" id="tab-queue" aria-selected="false" aria-controls="queue-card" onclick="setMobileTab('queue')">Cola (0)</button>
      </div>

      <section class="card" id="queue-card" aria-label="Cola de reproducción">
        <div class="section-head">
          <div class="h" id="q-title">Cola</div>
        </div>
        <div class="list queue-list" id="queue">
          <div class="empty">La cola está vacía</div>
        </div>
      </section>
    </div>

    <div class="right-col">
      <section class="card" id="search-card" aria-label="Buscar canciones">
        <div class="section-head">
          <div class="h">Agregar a la cola</div>
        </div>
        <form class="search-form" onsubmit="doSearch(event)">
          <div class="search-field">
            <span class="search-icon" aria-hidden="true">
              <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"><circle cx="11" cy="11" r="7"/><path d="m20 20-3.5-3.5"/></svg>
            </span>
            <input id="q" type="search" autocomplete="off" placeholder="Buscar canción, artista o álbum…" aria-label="Buscar canción en YouTube Music"/>
          </div>
          <button class="primary" type="submit" id="search-btn">Buscar</button>
        </form>
        <div class="list" id="results">
          <div class="empty">Escribe el nombre de una canción o artista para agregarla a la cola.</div>
        </div>
      </section>
    </div>
  </main>
</div>

<div class="toast-region" role="region" aria-label="Notificaciones">
  <div id="toast-pop" popover="manual">
    <span id="toast-msg"></span>
    <button type="button" class="toast-close" popovertarget="toast-pop" popovertargetaction="hide" aria-label="Cerrar notificación">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round"><path d="M18 6 6 18M6 6l12 12"/></svg>
    </button>
  </div>
</div>
<p class="sr-only">Anuncios de estado</p>
<div id="live-region" class="sr-only" aria-live="polite" aria-atomic="true"></div>

<script>
const SVG_PLAY='<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>';
const SVG_PAUSE='<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M6 5h4v14H6zm8 0h4v14h-4z"/></svg>';
const SVG_VOL='<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.5 8.5a5 5 0 0 1 0 7"/><path d="M19 5a10 10 0 0 1 0 14"/></svg>';
const SVG_MUTE='<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><line x1="23" y1="9" x2="17" y2="15"/><line x1="17" y1="9" x2="23" y2="15"/></svg>';
const SVG_PLUS='<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" aria-hidden="true"><path d="M12 5v14M5 12h14"/></svg>';
const SVG_CHECK='<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 6 9 17l-5-5"/></svg>';
const SVG_UP='<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m18 15-6-6-6 6"/></svg>';
const SVG_DOWN='<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>';
const SVG_GRIP='<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><circle cx="9" cy="6" r="1.5"/><circle cx="15" cy="6" r="1.5"/><circle cx="9" cy="12" r="1.5"/><circle cx="15" cy="12" r="1.5"/><circle cx="9" cy="18" r="1.5"/><circle cx="15" cy="18" r="1.5"/></svg>';
const supportsPopover=Object.hasOwn(HTMLElement.prototype,'popover');

let toastTimer=null;
function hideToast(){
  const pop=document.getElementById('toast-pop');
  if(supportsPopover){try{pop.hidePopover();}catch(_){}}else{pop.classList.remove(':popover-open');}
}
function showToast(m){
  const pop=document.getElementById('toast-pop');
  document.getElementById('toast-msg').textContent=m;
  if(supportsPopover){try{pop.showPopover();}catch(_){}}else{pop.classList.add(':popover-open');}
  if(typeof document.ariaNotify==='function'){document.ariaNotify(m);}
  else{const lr=document.getElementById('live-region');lr.textContent='';setTimeout(()=>{lr.textContent=m;},20);}
  clearTimeout(toastTimer);
  toastTimer=setTimeout(()=>{
    if(!pop.contains(document.activeElement))hideToast();
  },2600);
}
if(!supportsPopover){
  document.querySelector('.toast-close').addEventListener('click',hideToast);
}
function esc(s){return (s||'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));}
function fmt(sec){
  const n=Math.max(0,Math.floor(sec||0));
  const m=Math.floor(n/60),s=n%60;
  return m+':'+(s<10?'0':'')+s;
}
let listenHere=false;let curTrackId=null;let dragFrom=null;
let latestState=null;let statePending=false;let renderedQueue=null;
const localAudio=document.getElementById('local-audio');
function setMobileTab(tab){
  document.getElementById('main-grid').dataset.tab=tab;
  document.getElementById('tab-search').setAttribute('aria-selected',String(tab==='search'));
  document.getElementById('tab-queue').setAttribute('aria-selected',String(tab==='queue'));
}
function renderListenBtn(){
  const btn=document.getElementById('listen-btn');
  btn.innerHTML=(listenHere?SVG_MUTE+'<span>Silenciar aquí</span>':SVG_VOL+'<span>Escuchar aquí</span>');
  btn.classList.toggle('primary',listenHere);
}
renderListenBtn();
function toggleLocalAudio(){
  listenHere=!listenHere;
  renderListenBtn();
  syncLocalAudio();
}
function stopLocalAudio(){
  localAudio.pause();
  if(curTrackId!==null){
    curTrackId=null;
    localAudio.removeAttribute('src');
    localAudio.load();
  }
}
function localAudioFailed(message){
  listenHere=false;
  stopLocalAudio();
  renderListenBtn();
  showToast(message);
}
function syncLocalAudio(){
  const track=latestState?.now_playing;
  if(!listenHere||!track){stopLocalAudio();return;}
  if(curTrackId!==track.id){
    curTrackId=track.id;
    localAudio.src='/api/audio?id='+encodeURIComponent(track.id);
  }
  const pos=latestState.position_secs||0;
  if(localAudio.readyState>=1&&!localAudio.seeking){
    const target=Number.isFinite(localAudio.duration)?Math.min(pos,localAudio.duration):pos;
    if(Math.abs(localAudio.currentTime-target)>(latestState.is_playing?1.5:0.35))localAudio.currentTime=target;
  }
  if(!latestState.is_playing){localAudio.pause();return;}
  if(localAudio.paused){
    const src=localAudio.src;
    localAudio.play().catch(error=>{
      if(!listenHere||localAudio.src!==src||!latestState?.is_playing||error.name==='AbortError')return;
      localAudioFailed(error.name==='NotAllowedError'
        ?'Toca «Escuchar aquí» para iniciar el audio.'
        :'No se pudo reproducir el audio. Toca «Escuchar aquí» para reintentar.');
    });
  }
}
localAudio.addEventListener('loadedmetadata',syncLocalAudio);
localAudio.addEventListener('error',()=>{
  if(listenHere&&curTrackId!==null)localAudioFailed('No se pudo cargar el audio. Toca «Escuchar aquí» para reintentar.');
});
async function loadState(){
  if(statePending)return;
  statePending=true;
  try{
    const r=await fetch('/api/state');if(!r.ok)return;
    const s=await r.json();
    latestState=s;
    const gTxt=s.guest_count+' '+(s.guest_count===1?'invitado':'invitados');
    document.getElementById('guests').innerHTML='<span class="dot" aria-hidden="true"></span><span>'+esc(gTxt)+'</span>';
    if(s.now_playing){
      document.getElementById('np-title').textContent=s.now_playing.title;
      document.getElementById('np-artist').textContent=s.now_playing.artist+(s.now_playing.album?' • '+s.now_playing.album:'');
      const cov=document.getElementById('np-cover');
      if(s.now_playing.cover_url&&cov.getAttribute('src')!==s.now_playing.cover_url){cov.src=s.now_playing.cover_url;}
      const dur=s.now_playing.duration_secs||0;
      const pos=Math.min(dur||s.position_secs||0,s.position_secs||0);
      document.getElementById('np-cur').textContent=fmt(pos);
      document.getElementById('np-dur').textContent=dur?fmt(dur):'--:--';
      document.getElementById('np-progress').style.width=(dur>0?Math.min(100,(pos/dur)*100):0)+'%';
      document.getElementById('np-state').textContent=s.is_playing?'Reproduciendo':'En pausa';
    }else{
      document.getElementById('np-title').textContent='Sin reproducción';
      document.getElementById('np-artist').textContent='Busca una canción para empezar';
      document.getElementById('np-cover').removeAttribute('src');
      document.getElementById('np-cur').textContent='0:00';
      document.getElementById('np-dur').textContent='0:00';
      document.getElementById('np-progress').style.width='0%';
      document.getElementById('np-state').textContent='';
    }
    const c=document.getElementById('ctrls');c.style.display=s.allow_controls?'flex':'none';
    const pp=document.getElementById('pp-btn');
    pp.innerHTML=s.is_playing?SVG_PAUSE+'<span>Pausar</span>':SVG_PLAY+'<span>Reproducir</span>';
    const qLabel='Cola ('+s.queue.length+')';
    document.getElementById('q-title').textContent=qLabel;
    document.getElementById('tab-queue').textContent=qLabel;
    const queueKey=JSON.stringify([s.allow_controls,s.queue]);
    if(dragFrom===null&&renderedQueue!==queueKey){
      const canReorder=!!s.allow_controls;
      document.getElementById('queue').innerHTML=s.queue.length
        ? s.queue.map((t,i)=>`<div class="row ${t.is_current?'cur':''}" ${canReorder?`draggable="true" ondragstart="qDragStart(event,${i})" ondragover="qDragOver(event)" ondragleave="qDragLeave(event)" ondrop="qDrop(event,${i})" ondragend="qDragEnd(event)"`:''}>${canReorder?`<span class="drag-grip" aria-hidden="true">${SVG_GRIP}</span>`:''}<img class="row-art" src="${esc(t.cover_url)}" alt="" loading="lazy"/><div class="meta"><div class="row-title">${esc(t.title)}</div><div class="row-sub">${esc(t.artist)}</div></div>${t.duration_secs?`<span class="dur">${fmt(t.duration_secs)}</span>`:''}${canReorder?`<div class="q-actions"><button type="button" class="q-move-btn" ${i===0?'disabled':''} onclick="moveQueue(${i},${i-1})" aria-label="Subir">${SVG_UP}</button><button type="button" class="q-move-btn" ${i===s.queue.length-1?'disabled':''} onclick="moveQueue(${i},${i+1})" aria-label="Bajar">${SVG_DOWN}</button></div>`:''}</div>`).join('')
        : '<div class="empty">La cola está vacía</div>';
      renderedQueue=queueKey;
    }
    syncLocalAudio();
  }catch(_){}finally{statePending=false;}
}
async function doSearch(e){
  e.preventDefault();
  const q=document.getElementById('q').value.trim();if(!q)return;
  const resEl=document.getElementById('results');
  resEl.innerHTML='<div class="empty">Buscando canciones…</div>';
  try{
    const r=await fetch('/api/search?q='+encodeURIComponent(q));
    const items=await r.json();
    window._tracks=Array.isArray(items)?items:[];
    resEl.innerHTML=window._tracks.length
      ? window._tracks.map((t,i)=>`<div class="row"><img class="row-art" src="${esc(t.cover_url)}" alt="" loading="lazy"/><div class="meta"><div class="row-title">${esc(t.title)}</div><div class="row-sub">${esc(t.artist)}${t.album?' • '+esc(t.album):''}</div></div>${t.duration_secs?`<span class="dur">${fmt(t.duration_secs)}</span>`:''}<button class="primary" type="button" id="add-${i}" onclick="addTrack(${i})">${SVG_PLUS}<span>Agregar</span></button></div>`).join('')
      : '<div class="empty">No se encontraron canciones</div>';
  }catch(_){
    resEl.innerHTML='<div class="empty">Error al buscar canciones</div>';
  }
}
async function addTrack(i){
  const t=window._tracks&&window._tracks[i];if(!t)return;
  const btn=document.getElementById('add-'+i);
  if(btn){btn.disabled=true;}
  const r=await fetch('/api/queue/add',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(t)});
  if(!r.ok){
    if(btn){btn.disabled=false;}
    showToast('No se pudo enviar la canción');
    return;
  }
  if(btn){
    btn.classList.remove('primary');btn.classList.add('added');
    btn.innerHTML=SVG_CHECK+'<span>Agregada</span>';
    setTimeout(()=>{
      btn.classList.remove('added');btn.classList.add('primary');
      btn.innerHTML=SVG_PLUS+'<span>Agregar</span>';btn.disabled=false;
    },1800);
  }
  showToast('Agregada a la cola: '+t.title);
  setTimeout(loadState,600);
}
async function ctrl(action){
  await fetch('/api/control',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({action})});
  setTimeout(loadState,250);
}
async function moveQueue(from,to){
  if(from===to||from<0||to<0)return;
  const r=await fetch('/api/queue/move',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({from,to})});
  if(r.ok){setTimeout(loadState,150);}
}
function qDragStart(e,i){dragFrom=i;e.currentTarget.classList.add('dragging');e.dataTransfer.effectAllowed='move';e.dataTransfer.setData('text/plain',String(i));}
function qDragOver(e){e.preventDefault();e.dataTransfer.dropEffect='move';e.currentTarget.classList.add('drop-target');}
function qDragLeave(e){e.currentTarget.classList.remove('drop-target');}
function qDrop(e,to){e.preventDefault();e.currentTarget.classList.remove('drop-target');const from=dragFrom;dragFrom=null;if(from!==null&&from!==to){moveQueue(from,to);}}
function qDragEnd(e){e.currentTarget.classList.remove('dragging');dragFrom=null;}
loadState();setInterval(loadState,750);
</script>
</body>
</html>"##;

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
        assert!(generate_qr_svg(&"x".repeat(107)).is_none());
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
}
