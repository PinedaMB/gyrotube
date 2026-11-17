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
                    if info.url.starts_with("http://") || info.url.starts_with("https://") {
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

const JAM_HTML: &str = r#"<!doctype html>
<html lang="es">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>GyroTube SPIN</title>
<style>
:root{color-scheme:dark;--bg:#121316;--card:#1b1d22;--line:rgba(255,255,255,.08);--fg:#f2f4f8;--muted:#9aa3b2;--accent:#1f9cf0}
*{box-sizing:border-box;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif}
body{margin:0;background:var(--bg);color:var(--fg);padding:16px 14px 48px}
.wrap{max-width:600px;margin:0 auto;display:flex;flex-direction:column;gap:14px}
header{display:flex;align-items:center;justify-content:space-between}
.brand{display:flex;align-items:center;gap:8px;font-weight:800;font-size:20px}
.badge{font-size:12px;padding:4px 10px;border-radius:999px;background:rgba(31,156,240,.18);color:#6bc2ff;font-weight:700}
.card{background:var(--card);border:1px solid var(--line);border-radius:16px;padding:14px}
.np{display:flex;align-items:center;gap:12px}
.cover{width:54px;height:54px;border-radius:10px;object-fit:cover;background:#262930;flex-shrink:0}
.meta{min-width:0;flex:1}
.title{font-weight:700;font-size:15px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.sub{font-size:13px;color:var(--muted);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;margin-top:2px}
.ctrls{display:flex;gap:8px;margin-top:10px}
button{border:0;border-radius:999px;padding:8px 14px;font-weight:600;cursor:pointer;background:rgba(255,255,255,.1);color:var(--fg)}
button.primary{background:var(--accent);color:#fff}
.search{display:flex;gap:8px}
input{flex:1;min-width:0;border:1px solid var(--line);border-radius:999px;padding:10px 14px;background:var(--card);color:var(--fg);font-size:15px;outline:none}
input:focus{border-color:var(--accent)}
.list{display:flex;flex-direction:column;gap:8px;margin-top:10px}
.row{display:flex;align-items:center;gap:10px;padding:6px;border-radius:12px}
.row.cur{background:rgba(31,156,240,.14)}
.row img{width:42px;height:42px;border-radius:8px;object-fit:cover;background:#262930;flex-shrink:0}
.h{font-size:13px;font-weight:700;color:var(--muted);text-transform:uppercase;letter-spacing:.05em}
.toast{position:fixed;bottom:16px;left:50%;transform:translateX(-50%);background:#232730;border:1px solid var(--line);padding:10px 16px;border-radius:999px;font-size:13px;display:none}
</style>
</head>
<body>
<div class="wrap">
  <header>
    <div class="brand">GyroTube <span class="badge">SPIN</span></div>
    <div class="badge" id="guests">0 invitados</div>
  </header>
  <div class="card">
    <div class="h">Sonando ahora</div>
    <div class="np" style="margin-top:8px">
      <img class="cover" id="np-cover" alt=""/>
      <div class="meta">
        <div class="title" id="np-title">Sin reproducción</div>
        <div class="sub" id="np-artist">Busca una canción abajo</div>
      </div>
    </div>
    <div class="ctrls">
      <button id="listen-btn" onclick="toggleLocalAudio()">🔊 Escuchar aquí</button>
    </div>
    <div class="ctrls" id="ctrls" style="display:none">
      <button onclick="ctrl('prev')">⏮</button>
      <button class="primary" onclick="ctrl('play_pause')" id="pp-btn">⏯</button>
      <button onclick="ctrl('next')">⏭</button>
    </div>
    <audio id="local-audio" preload="auto" style="display:none"></audio>
  </div>
  <div class="card">
    <form class="search" onsubmit="doSearch(event)">
      <input id="q" placeholder="Buscar canción en YouTube Music…"/>
      <button class="primary" type="submit">Buscar</button>
    </form>
    <div class="list" id="results"></div>
  </div>
  <div class="card">
    <div class="h" id="q-title">Cola</div>
    <div class="list" id="queue"></div>
  </div>
</div>
<div class="toast" id="toast"></div>
<script>
function showToast(m){const t=document.getElementById('toast');t.textContent=m;t.style.display='block';setTimeout(()=>t.style.display='none',2200);}
function esc(s){return (s||'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));}
let listenHere=false;let curTrackId=null;
function toggleLocalAudio(){
  listenHere=!listenHere;
  const btn=document.getElementById('listen-btn');
  const aud=document.getElementById('local-audio');
  btn.textContent=listenHere?'🔇 Silenciar aquí':'🔊 Escuchar aquí';
  btn.classList.toggle('primary',listenHere);
  if(!listenHere){aud.pause();curTrackId=null;}else{loadState();}
}
async function loadState(){
  try{
    const r=await fetch('/api/state');const s=await r.json();
    document.getElementById('guests').textContent=s.guest_count+' '+(s.guest_count===1?'invitado':'invitados');
    if(s.now_playing){
      document.getElementById('np-title').textContent=s.now_playing.title;
      document.getElementById('np-artist').textContent=s.now_playing.artist;
      document.getElementById('np-cover').src=s.now_playing.cover_url||'';
      if(listenHere){
        const aud=document.getElementById('local-audio');
        if(curTrackId!==s.now_playing.id){
          curTrackId=s.now_playing.id;
          aud.src='/api/audio?id='+encodeURIComponent(curTrackId);
          aud.currentTime=s.position_secs||0;
          if(s.is_playing)aud.play().catch(()=>{});
        }else{
          if(s.is_playing&&aud.paused)aud.play().catch(()=>{});
          else if(!s.is_playing&&!aud.paused)aud.pause();
          if(Math.abs((aud.currentTime||0)-(s.position_secs||0))>2.5)aud.currentTime=s.position_secs||0;
        }
      }
    }
    const c=document.getElementById('ctrls');c.style.display=s.allow_controls?'flex':'none';
    document.getElementById('pp-btn').textContent=s.is_playing?'⏸ Pausar':'▶ Reproducir';
    document.getElementById('q-title').textContent='Cola ('+s.queue.length+')';
    document.getElementById('queue').innerHTML=s.queue.map(t=>`<div class="row ${t.is_current?'cur':''}"><img src="${esc(t.cover_url)}"><div class="meta"><div class="title">${esc(t.title)}</div><div class="sub">${esc(t.artist)}</div></div></div>`).join('');
  }catch(_){}
}
async function doSearch(e){
  e.preventDefault();const q=document.getElementById('q').value.trim();if(!q)return;
  const resEl=document.getElementById('results');resEl.innerHTML='<div class="sub">Buscando…</div>';
  const r=await fetch('/api/search?q='+encodeURIComponent(q));const items=await r.json();
  window._tracks=items;
  resEl.innerHTML=items.map((t,i)=>`<div class="row"><img src="${esc(t.cover_url)}"><div class="meta"><div class="title">${esc(t.title)}</div><div class="sub">${esc(t.artist)}</div></div><button class="primary" onclick="addTrack(${i})">+ Agregar</button></div>`).join('')||'<div class="sub">Sin resultados</div>';
}
async function addTrack(i){
  const t=window._tracks[i];if(!t)return;
  const r=await fetch('/api/queue/add',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(t)});
  if(!r.ok){showToast('No se pudo enviar la canción');return;}
  showToast('Enviada para verificar: '+t.title);loadState();
}
async function ctrl(action){
  await fetch('/api/control',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({action})});
  setTimeout(loadState,250);
}
loadState();setInterval(loadState,2500);
</script>
</body>
</html>"#;

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
}
