use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use super::{FALLBACK_SEEKABLE_BYTES, ProxyTarget, StreamSession};

const PROXY_CHUNK_BYTES: u64 = 1024 * 1024;

fn content_range_total(resp: &reqwest::Response) -> Option<u64> {
    resp.headers()
        .get(reqwest::header::CONTENT_RANGE)?
        .to_str()
        .ok()?
        .rsplit_once('/')?
        .1
        .trim()
        .parse()
        .ok()
}

async fn fetch_proxy_chunk(
    client: &reqwest::Client,
    target: &ProxyTarget,
    start: u64,
    end: u64,
) -> Option<reqwest::Response> {
    for attempt in 0..3u64 {
        if let Ok(resp) = client
            .get(&target.url)
            .header("User-Agent", &target.user_agent)
            .header("Range", format!("bytes={start}-{end}"))
            .send()
            .await
        {
            let status = resp.status();
            if status.as_u16() == 206 {
                return Some(resp);
            }
            if status.is_client_error() {
                return None;
            }
        }
        tokio::time::sleep(Duration::from_millis(150 * (attempt + 1))).await;
    }
    None
}

/// Streams `[start, final_end]` to the client as sequential 1 MiB upstream ranges,
/// prefetching the next chunk while the current one is being written and
/// re-requesting the remainder if an upstream body ends early.
async fn stream_proxy_chunks(
    write_half: &mut tokio::net::tcp::OwnedWriteHalf,
    client: &reqwest::Client,
    target: &ProxyTarget,
    first: reqwest::Response,
    start: u64,
    final_end: u64,
) {
    let mut current = Some(first);
    let mut pos = start;
    while pos <= final_end {
        let cur_end = (pos + PROXY_CHUNK_BYTES - 1).min(final_end);
        let next_start = cur_end + 1;
        let next_handle = (next_start <= final_end).then(|| {
            let c = client.clone();
            let t = target.clone();
            let ne = (next_start + PROXY_CHUNK_BYTES - 1).min(final_end);
            tokio::spawn(async move { fetch_proxy_chunk(&c, &t, next_start, ne).await })
        });

        let mut written: u64 = 0;
        let expected = cur_end - pos + 1;
        let mut retries = 0;
        let mut client_gone = false;
        while written < expected {
            let mut resp = match current.take() {
                Some(r) => r,
                None => match fetch_proxy_chunk(client, target, pos + written, cur_end).await {
                    Some(r) => r,
                    None => break,
                },
            };
            while let Ok(Some(chunk)) = resp.chunk().await {
                let remaining = (expected - written) as usize;
                let slice = &chunk[..chunk.len().min(remaining)];
                if write_half.write_all(slice).await.is_err() {
                    client_gone = true;
                    break;
                }
                written += slice.len() as u64;
                if written >= expected {
                    break;
                }
            }
            if client_gone || written >= expected {
                break;
            }
            retries += 1;
            if retries > 3 {
                break;
            }
        }

        if client_gone || written < expected {
            if let Some(h) = next_handle {
                h.abort();
            }
            return;
        }
        pos = next_start;
        current = match next_handle {
            Some(h) => h.await.ok().flatten(),
            None => None,
        };
        if current.is_none() && pos <= final_end {
            current = fetch_proxy_chunk(client, target, pos, (pos + PROXY_CHUNK_BYTES - 1).min(final_end)).await;
            if current.is_none() {
                return;
            }
        }
    }
}

pub(super) fn stream_session_key(path: &str) -> Option<&str> {
    let raw_key = path.split('?').next()?.strip_prefix("/stream/")?;
    Some(raw_key.rsplit_once('.').map_or(raw_key, |(key, _)| key))
}

pub(super) fn allowed_cors_origin(origin: Option<&str>) -> Option<&str> {
    let origin = origin?.trim();
    if origin == "tauri://localhost"
        || origin == "http://tauri.localhost"
        || origin == "https://tauri.localhost"
        || origin.starts_with("http://localhost:")
        || origin.starts_with("http://127.0.0.1:")
    {
        Some(origin)
    } else {
        None
    }
}

pub(super) fn build_cors_headers(origin: Option<&str>) -> String {
    let mut out = String::new();
    if let Some(allowed) = allowed_cors_origin(origin) {
        out.push_str(&format!("Access-Control-Allow-Origin: {allowed}\r\nVary: Origin\r\n"));
    }
    out.push_str("Access-Control-Allow-Methods: GET, HEAD, OPTIONS\r\nAccess-Control-Allow-Headers: Range\r\nAccess-Control-Expose-Headers: Content-Range, Content-Length, Accept-Ranges\r\n");
    out
}

pub(super) async fn handle_client(
    socket: TcpStream,
    sessions: Arc<RwLock<HashMap<String, Arc<StreamSession>>>>,
    proxies: Arc<RwLock<HashMap<String, ProxyTarget>>>,
    client: reqwest::Client,
) -> Result<(), std::io::Error> {
    const MAX_HEADERS: usize = 64;
    const MAX_HEADER_LINE_BYTES: usize = 8 * 1024;

    let (read_half, mut write_half) = socket.into_split();
    let mut reader = BufReader::new(read_half);

    let mut request_line = String::new();
    let req_read = tokio::time::timeout(
        Duration::from_secs(5),
        reader.read_line(&mut request_line),
    )
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "Header read timeout"))??;
    if req_read == 0 || request_line.len() > MAX_HEADER_LINE_BYTES {
        return Ok(());
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }
    let method = parts[0];
    let path = parts[1];

    let mut range_header: Option<String> = None;
    let mut origin_header: Option<String> = None;
    for _ in 0..MAX_HEADERS {
        let mut line = String::new();
        let n = tokio::time::timeout(Duration::from_secs(5), reader.read_line(&mut line))
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "Header read timeout"))??;
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if line.len() > MAX_HEADER_LINE_BYTES {
            return Ok(());
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            if key.eq_ignore_ascii_case("range") {
                range_header = Some(v.trim().to_string());
            } else if key.eq_ignore_ascii_case("origin") {
                origin_header = Some(v.trim().to_string());
            }
        }
    }

    let cors_headers = build_cors_headers(origin_header.as_deref());
    #[allow(non_snake_case)]
    let CORS_HEADERS = cors_headers.as_str();

    if method.eq_ignore_ascii_case("OPTIONS") {
        let hdr = format!(
            "HTTP/1.1 204 No Content\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
        );
        write_half.write_all(hdr.as_bytes()).await?;
        return Ok(());
    }

    if let Some(raw_cover_id) = path.strip_prefix("/cover/") {
        let video_id = raw_cover_id.split('?').next().unwrap_or(raw_cover_id);
        if crate::backend::validate_video_id(video_id).is_ok()
            && let Some(dl_dir) = crate::models::downloads_dir()
        {
            let cover_path = dl_dir.join(format!("{video_id}.cover"));
            if let Ok(bytes) = tokio::fs::read(&cover_path).await
                && !bytes.is_empty()
            {
                let mime = if bytes.starts_with(b"\x89PNG") {
                    "image/png"
                } else if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
                    "image/webp"
                } else {
                    "image/jpeg"
                };
                let hdr = format!(
                    "HTTP/1.1 200 OK\r\n{CORS_HEADERS}Content-Type: {mime}\r\nCache-Control: public, max-age=31536000\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    bytes.len()
                );
                write_half.write_all(hdr.as_bytes()).await?;
                if !method.eq_ignore_ascii_case("HEAD") {
                    write_half.write_all(&bytes).await?;
                }
                return Ok(());
            }
        }
        let not_found = format!(
            "HTTP/1.1 404 Not Found\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
        );
        write_half.write_all(not_found.as_bytes()).await?;
        return Ok(());
    }

    if let Some(proxy_key) = path.strip_prefix("/proxy/") {
        let proxy_key = proxy_key.split('?').next().unwrap_or(proxy_key);
        let target = proxies.read().ok().and_then(|m| m.get(proxy_key).cloned());
        let Some(target) = target else {
            let hdr = format!(
                "HTTP/1.1 404 Not Found\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
            );
            write_half.write_all(hdr.as_bytes()).await?;
            return Ok(());
        };

        // googlevideo throttles long open-ended ranges to ~realtime (measured 155–386 KB/s)
        // while short 1 MiB ranges download at full link speed (~2 MB/s). Split every GET
        // into pipelined 1 MiB upstream requests so <video> never starves its buffer.
        if !method.eq_ignore_ascii_case("HEAD") {
            let requested = range_header.as_deref().and_then(parse_range_header);
            let start = requested.map_or(0, |(s, _)| s);
            let client_end = requested.and_then(|(_, e)| e);
            let first_end = client_end.map_or(start + PROXY_CHUNK_BYTES - 1, |e| {
                e.min(start + PROXY_CHUNK_BYTES - 1)
            });
            if client_end.is_none_or(|e| e >= start)
                && let Some(first) = fetch_proxy_chunk(&client, &target, start, first_end).await
                && let Some(total) = content_range_total(&first)
                && total > start
            {
                let final_end = client_end.unwrap_or(total - 1).min(total - 1);
                let len = final_end - start + 1;
                let content_type = first
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or(&target.mime_type)
                    .to_string();
                let hdr = if requested.is_some() {
                    format!(
                        "HTTP/1.1 206 Partial Content\r\n{CORS_HEADERS}Content-Type: {content_type}\r\nAccept-Ranges: bytes\r\nContent-Range: bytes {start}-{final_end}/{total}\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n"
                    )
                } else {
                    format!(
                        "HTTP/1.1 200 OK\r\n{CORS_HEADERS}Content-Type: {content_type}\r\nAccept-Ranges: bytes\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n"
                    )
                };
                write_half.write_all(hdr.as_bytes()).await?;
                stream_proxy_chunks(&mut write_half, &client, &target, first, start, final_end).await;
                let _ = write_half.shutdown().await;
                return Ok(());
            }
        }

        let mut req = if method.eq_ignore_ascii_case("HEAD") {
            client.head(&target.url)
        } else {
            client.get(&target.url)
        }
        .header("User-Agent", &target.user_agent);
        if let Some(ref range) = range_header {
            req = req.header("Range", range);
        }
        let Ok(mut upstream) = req.send().await else {
            eprintln!("[gyrotube-proxy] upstream send failed for {proxy_key}");
            let hdr = format!(
                "HTTP/1.1 502 Bad Gateway\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
            );
            write_half.write_all(hdr.as_bytes()).await?;
            return Ok(());
        };
        let status = upstream.status();
        if !status.is_success() && status.as_u16() != 206 {
            eprintln!("[gyrotube-proxy] upstream status {} for {}", status, proxy_key);
        }
        let status_line = format!(
            "HTTP/1.1 {} {}",
            status.as_u16(),
            status.canonical_reason().unwrap_or("OK")
        );
        let content_type = upstream
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or(&target.mime_type)
            .to_string();
        let mut hdr = format!(
            "{status_line}\r\n{CORS_HEADERS}Content-Type: {content_type}\r\nAccept-Ranges: bytes\r\n"
        );
        if let Some(cr) = upstream
            .headers()
            .get(reqwest::header::CONTENT_RANGE)
            .and_then(|v| v.to_str().ok())
        {
            hdr.push_str(&format!("Content-Range: {cr}\r\n"));
        }
        if let Some(cl) = upstream
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
        {
            hdr.push_str(&format!("Content-Length: {cl}\r\n"));
        }
        hdr.push_str("Connection: close\r\n\r\n");
        write_half.write_all(hdr.as_bytes()).await?;
        if !method.eq_ignore_ascii_case("HEAD") {
            while let Ok(Some(chunk)) = upstream.chunk().await {
                if write_half.write_all(&chunk).await.is_err() {
                    break;
                }
            }
        }
        let _ = write_half.shutdown().await;
        return Ok(());
    }

    let session = stream_session_key(path)
        .and_then(|key| sessions.read().ok()?.get(key).cloned());

    let Some(session) = session else {
        let hdr = format!(
            "HTTP/1.1 404 Not Found\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
        );
        write_half.write_all(hdr.as_bytes()).await?;
        return Ok(());
    };

    let mut progress_rx = session.progress_rx.clone();
    let (known_total, is_done, failed) = {
        let p = progress_rx.borrow();
        (p.total_bytes, p.done, p.error.is_some())
    };
    if failed {
        write_half.write_all(format!(
            "HTTP/1.1 502 Bad Gateway\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
        ).as_bytes()).await?;
        return Ok(());
    }

    let requested_range = range_header.as_deref().and_then(parse_range_header);

    let (status_line, start_pos, end_pos_inclusive, headers_block) = match (requested_range, known_total) {
        (Some((start, end_opt)), Some(total)) if total > 0 => {
            if start >= total {
                let hdr = format!(
                    "HTTP/1.1 416 Range Not Satisfiable\r\n{CORS_HEADERS}Content-Range: bytes */{total}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                );
                write_half.write_all(hdr.as_bytes()).await?;
                return Ok(());
            }
            let end = end_opt.map(|e| e.min(total - 1)).unwrap_or(total - 1);
            let len = end - start + 1;
            let hdr = format!(
                "HTTP/1.1 206 Partial Content\r\n{CORS_HEADERS}Content-Type: {}\r\nAccept-Ranges: bytes\r\nContent-Range: bytes {start}-{end}/{total}\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n",
                session.mime_type
            );
            ("206", start, Some(end), hdr)
        }
        (Some((start, end_opt)), None) => {
            // Total unknown (e.g. yt-dlp pipe without prior filesize): advertise 206 Partial Content
            // with a generous upper bound so GStreamer souphttpsrc enables push-mode byte-range seeking
            // instead of treating the response as a restart from byte 0.
            let synth_total = FALLBACK_SEEKABLE_BYTES.max(start + 1024 * 1024);
            let end = end_opt.map(|e| e.min(synth_total - 1)).unwrap_or(synth_total - 1);
            let len = end - start + 1;
            let hdr = format!(
                "HTTP/1.1 206 Partial Content\r\n{CORS_HEADERS}Content-Type: {}\r\nAccept-Ranges: bytes\r\nContent-Range: bytes {start}-{end}/{synth_total}\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n",
                session.mime_type
            );
            ("206", start, end_opt, hdr)
        }
        (None, Some(total)) if total > 0 => {
            let hdr = format!(
                "HTTP/1.1 200 OK\r\n{CORS_HEADERS}Content-Type: {}\r\nAccept-Ranges: bytes\r\nContent-Length: {total}\r\nConnection: close\r\n\r\n",
                session.mime_type
            );
            ("200", 0, Some(total - 1), hdr)
        }
        _ => {
            let hdr = if is_done {
                let total = known_total.unwrap_or(0);
                format!(
                    "HTTP/1.1 200 OK\r\n{CORS_HEADERS}Content-Type: {}\r\nAccept-Ranges: bytes\r\nContent-Length: {total}\r\nConnection: close\r\n\r\n",
                    session.mime_type
                )
            } else {
                format!(
                    "HTTP/1.1 200 OK\r\n{CORS_HEADERS}Content-Type: {}\r\nAccept-Ranges: bytes\r\nConnection: close\r\n\r\n",
                    session.mime_type
                )
            };
            ("200", 0, None, hdr)
        }
    };

    let _ = status_line;
    write_half.write_all(headers_block.as_bytes()).await?;
    if method.eq_ignore_ascii_case("HEAD") {
        return Ok(());
    }

    let mut file = tokio::fs::File::open(&session.file_path).await?;
    let mut pos = start_pos;
    let mut buf = vec![0u8; 32 * 1024];

    loop {
        if let Some(end_inc) = end_pos_inclusive
            && pos > end_inc
        {
            break;
        }

        let (written, done) = {
            let p = progress_rx.borrow();
            (p.written, p.done)
        };

        if pos < written {
            file.seek(std::io::SeekFrom::Start(pos)).await?;
            let max_to_read = match end_pos_inclusive {
                Some(end_inc) => ((end_inc + 1 - pos).min(written - pos)) as usize,
                None => (written - pos) as usize,
            };
            let to_read = max_to_read.min(buf.len());
            let n = file.read(&mut buf[..to_read]).await?;
            if n > 0 {
                write_half.write_all(&buf[..n]).await?;
                pos += n as u64;
                continue;
            }
        }

        if done {
            break;
        }

        if progress_rx.changed().await.is_err() {
            break;
        }
    }

    let _ = write_half.flush().await;
    Ok(())
}

pub(super) fn parse_total_bytes_from_headers(
    headers: &reqwest::header::HeaderMap,
    is_partial: bool,
) -> Option<u64> {
    if let Some(cr) = headers
        .get(reqwest::header::CONTENT_RANGE)
        .and_then(|v| v.to_str().ok())
        && let Some((_, total_str)) = cr.rsplit_once('/')
        && let Ok(total) = total_str.trim().parse::<u64>()
    {
        return Some(total);
    }
    if !is_partial
        && let Some(cl) = headers
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
        && let Ok(total) = cl.trim().parse::<u64>()
    {
        return Some(total);
    }
    None
}

pub(super) fn parse_range_header(header_val: &str) -> Option<(u64, Option<u64>)> {
    let range_spec = header_val.strip_prefix("bytes=")?.trim();
    let (start_str, end_str) = range_spec.split_once('-')?;
    let start = start_str.trim().parse::<u64>().ok()?;
    let end = if end_str.trim().is_empty() {
        None
    } else {
        Some(end_str.trim().parse::<u64>().ok()?)
    };
    Some((start, end))
}

pub(super) fn normalize_ext(ext: &str) -> String {
    let trimmed = ext.trim().trim_start_matches('.').to_ascii_lowercase();
    match trimmed.as_str() {
        "webm" | "opus" => "webm".to_string(),
        "m4a" | "mp4" | "aac" => "m4a".to_string(),
        "mp3" => "mp3".to_string(),
        "ogg" => "ogg".to_string(),
        _ => "webm".to_string(),
    }
}

pub(super) fn mime_for_ext(ext: &str) -> &'static str {
    match ext {
        "m4a" | "mp4" => "audio/mp4",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        _ => "audio/webm",
    }
}
