use std::collections::HashMap;
use std::path::{Path, PathBuf};
#[cfg(not(target_os = "android"))]
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
#[cfg(not(target_os = "android"))]
use tokio::process::Command;
use tokio::sync::watch;

#[cfg(not(target_os = "android"))]
use crate::audio_cache;
use crate::models::StreamInfo;

const INITIAL_HTTP_BLOCK_BYTES: u64 = 256 * 1024;
const SUBSEQUENT_HTTP_BLOCK_BYTES: u64 = 512 * 1024;
const READY_THRESHOLD_BYTES: u64 = 32 * 1024;
const FALLBACK_SEEKABLE_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct ChunkProgress {
    pub written: u64,
    pub total_bytes: Option<u64>,
    pub done: bool,
    pub error: Option<String>,
}

struct StreamSession {
    file_path: PathBuf,
    mime_type: String,
    progress_rx: watch::Receiver<ChunkProgress>,
}

pub struct AudioStreamer {
    port: u16,
    sessions: Arc<RwLock<HashMap<String, Arc<StreamSession>>>>,
    in_flight: Arc<RwLock<HashMap<String, (String, tokio::task::AbortHandle)>>>,
    client: reqwest::Client,
    next_id: AtomicU64,
}

static GLOBAL_STREAMER: OnceLock<Arc<AudioStreamer>> = OnceLock::new();

impl AudioStreamer {
    pub fn try_global() -> Option<Arc<Self>> {
        GLOBAL_STREAMER.get().cloned()
    }

    pub async fn global() -> Result<Arc<Self>, String> {
        if let Some(s) = GLOBAL_STREAMER.get() {
            return Ok(s.clone());
        }
        let created = Self::start_new().await?;
        let _ = GLOBAL_STREAMER.set(created.clone());
        Ok(GLOBAL_STREAMER.get().cloned().unwrap_or(created))
    }

    pub async fn start_new() -> Result<Arc<Self>, String> {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| format!("Failed to bind local audio streamer: {e}"))?;
        let port = listener
            .local_addr()
            .map_err(|e| format!("Failed to read local audio streamer addr: {e}"))?
            .port();
        let sessions: Arc<RwLock<HashMap<String, Arc<StreamSession>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let client = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(4))
            .read_timeout(Duration::from_secs(12))
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to build chunked HTTP client: {e}"))?;

        let sessions_for_loop = sessions.clone();
        tokio::spawn(async move {
            loop {
                let Ok((socket, _)) = listener.accept().await else {
                    break;
                };
                let sessions_ref = sessions_for_loop.clone();
                tokio::spawn(async move {
                    let _ = handle_client(socket, sessions_ref).await;
                });
            }
        });

        Ok(Arc::new(Self {
            port,
            sessions,
            in_flight: Arc::new(RwLock::new(HashMap::new())),
            client,
            next_id: AtomicU64::new(1),
        }))
    }

    /// Aborts any in-flight background download whose `video_id` is not in `keep_ids`,
    /// returning the video IDs that were aborted so `Backend` can evict them from `stream_cache`.
    pub fn abort_downloads_except(&self, keep_ids: &[&str]) -> Vec<String> {
        let mut aborted_ids = Vec::new();
        let mut aborted_keys = Vec::new();
        if let Ok(mut map) = self.in_flight.write() {
            map.retain(|session_key, (vid, handle)| {
                if keep_ids.contains(&vid.as_str()) {
                    true
                } else {
                    handle.abort();
                    aborted_ids.push(vid.clone());
                    aborted_keys.push(session_key.clone());
                    false
                }
            });
        }
        if !aborted_keys.is_empty()
            && let Ok(mut sessions) = self.sessions.write()
        {
            for key in aborted_keys {
                sessions.remove(&key);
            }
        }
        aborted_ids
    }

    fn allocate_session_key(&self, video_id: &str) -> String {
        let seq = self.next_id.fetch_add(1, Ordering::Relaxed);
        format!("{video_id}-{seq}")
    }

    fn local_url(&self, session_key: &str, ext: &str) -> String {
        format!("http://127.0.0.1:{}/stream/{session_key}.{ext}", self.port)
    }

    pub(crate) fn can_reuse_url(&self, url: &reqwest::Url) -> bool {
        if url.port() != Some(self.port) {
            return false;
        }
        let Some(key) = stream_session_key(url.path()) else {
            return false;
        };
        self.sessions.read().is_ok_and(|sessions| {
            sessions.get(key).is_some_and(|session| {
                let progress = session.progress_rx.borrow();
                progress.error.is_none()
                    && (progress.done || session.progress_rx.has_changed().is_ok())
            })
        })
    }

    /// Registers an already-downloaded local audio file on disk so WebViews
    /// (which block `file://` URLs) can stream it over `http://127.0.0.1` with
    /// full HTTP Range and CORS support.
    pub fn serve_local_file(&self, video_id: &str, file_path: PathBuf) -> Result<String, String> {
        let meta = std::fs::metadata(&file_path)
            .map_err(|e| format!("Failed to read cached audio file metadata: {e}"))?;
        let total = meta.len();
        let ext = file_path
            .extension()
            .and_then(|s| s.to_str())
            .map(normalize_ext)
            .unwrap_or_else(|| "webm".to_string());

        let (_tx, progress_rx) = watch::channel(ChunkProgress {
            written: total,
            total_bytes: Some(total),
            done: true,
            error: None,
        });
        let session_key = self.allocate_session_key(video_id);
        let session = Arc::new(StreamSession {
            file_path,
            mime_type: mime_for_ext(&ext).to_string(),
            progress_rx,
        });
        if let Ok(mut map) = self.sessions.write() {
            map.insert(session_key.clone(), session);
        }

        let sessions_map = self.sessions.clone();
        let cleanup_key = session_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1800)).await;
            if let Ok(mut map) = sessions_map.write() {
                map.remove(&cleanup_key);
            }
        });

        Ok(self.local_url(&session_key, &ext))
    }

    /// Converts a `file://...` URL returned by the disk cache into a loopback HTTP stream URL.
    pub fn serve_file_url(&self, video_id: &str, file_url: &str) -> Result<String, String> {
        let parsed = reqwest::Url::parse(file_url)
            .map_err(|e| format!("Invalid cached file URL: {e}"))?;
        let path = parsed
            .to_file_path()
            .map_err(|()| format!("Cached URL is not a local file path: {file_url}"))?;
        self.serve_local_file(video_id, path)
    }

    pub async fn start_chunked_http_stream(
        &self,
        video_id: &str,
        remote: StreamInfo,
        extension: &str,
        expected_total_bytes: Option<u64>,
        streams_dir: &Path,
    ) -> Result<StreamInfo, String> {
        let staging = tempfile::tempdir_in(streams_dir)
            .map_err(|e| format!("Failed to create stream staging dir: {e}"))?;
        let ext = normalize_ext(extension);
        let staging_file_path = staging.path().join(format!("{video_id}.part.{ext}"));
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(&staging_file_path)
            .await
            .map_err(|e| format!("Failed to open staging stream file: {e}"))?;

        // Fetch Block 0 first to validate the URL and prime the initial playback buffer.
        let first_end = INITIAL_HTTP_BLOCK_BYTES - 1;
        let mut req = self
            .client
            .get(&remote.url)
            .header(reqwest::header::RANGE, format!("bytes=0-{first_end}"));
        if let Some(ua) = &remote.user_agent {
            req = req.header(reqwest::header::USER_AGENT, ua);
        }
        for (k, v) in &remote.headers {
            if !k.eq_ignore_ascii_case("range") {
                req = req.header(k.as_str(), v.as_str());
            }
        }

        let mut resp = req
            .send()
            .await
            .map_err(|e| format!("Initial audio block request failed: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            return Err(format!("Initial audio block returned HTTP {status}"));
        }

        let is_partial = status == reqwest::StatusCode::PARTIAL_CONTENT;
        let total_bytes = parse_total_bytes_from_headers(resp.headers(), is_partial)
            .or_else(|| parse_clen_from_url(&remote.url))
            .or(expected_total_bytes);

        if cfg!(target_os = "android") {
            let total = total_bytes.filter(|&total| total > 0)
                .ok_or_else(|| "Audio source did not report its total size".to_string())?;
            if parse_clen_from_url(&remote.url).or(expected_total_bytes).is_some_and(|size| size != total) {
                return Err("Audio source reported a different file size".into());
            }
            if is_partial && response_byte_range(resp.headers()) != Some((0, first_end.min(total - 1))) {
                return Err("Audio source did not deliver the requested initial range".into());
            }
            if is_partial && total > INITIAL_HTTP_BLOCK_BYTES {
                verify_audio_tail(&self.client, &remote, total).await?;
            }
        }

        let itag_hint = parse_itag_from_url(&remote.url);

        let (progress_tx, mut progress_rx) = watch::channel(ChunkProgress {
            written: 0,
            total_bytes,
            done: false,
            error: None,
        });

        let session_key = self.allocate_session_key(video_id);
        let session = Arc::new(StreamSession {
            file_path: staging_file_path.clone(),
            mime_type: mime_for_ext(&ext).to_string(),
            progress_rx: progress_rx.clone(),
        });
        if let Ok(mut map) = self.sessions.write() {
            map.insert(session_key.clone(), session);
        }

        let client = self.client.clone();
        let video_id_owned = video_id.to_string();
        let ext_owned = ext.clone();
        let streams_dir_owned = streams_dir.to_path_buf();
        let remote_clone = remote.clone();
        let sessions_map = self.sessions.clone();
        let in_flight_map = self.in_flight.clone();
        let session_key_for_cleanup = session_key.clone();
        let session_key_for_inflight = session_key.clone();
        let video_id_for_inflight = video_id.to_string();

        let join = tokio::spawn(async move {
            let mut written: u64 = 0;
            let mut fatal_err: Option<String> = None;

            // Stream Block 0 body in network chunks
            while let Some(chunk_res) = resp.chunk().await.transpose() {
                match chunk_res {
                    Ok(chunk) => {
                        if chunk.is_empty() {
                            break;
                        }
                        if let Err(e) = file.write_all(&chunk).await {
                            fatal_err = Some(format!("Failed writing audio block 0: {e}"));
                            break;
                        }
                        written += chunk.len() as u64;
                        let _ = progress_tx.send(ChunkProgress {
                            written,
                            total_bytes,
                            done: false,
                            error: None,
                        });
                    }
                    Err(e) => {
                        fatal_err = Some(format!("Network error reading block 0: {e}"));
                        break;
                    }
                }
            }

            // Fetch subsequent 512 KB blocks sequentially
            // so YouTube's CDN never rejects concurrent connections and if a block drops mid-stream,
            // we seamlessly resume from `written` via yt-dlp or fresh retries without truncating!
            if fatal_err.is_none() && is_partial {
                let mut known_total = total_bytes;
                let mut active_url = remote_clone.url.clone();
                let mut active_remote = remote_clone.clone();
                let mut ios_refreshed = !active_url.contains("c=IOS");
                loop {
                    if let Some(total) = known_total {
                        if written >= total {
                            break;
                        }
                    }
                    let start = written;
                    let end = match known_total {
                        Some(total) => (start + SUBSEQUENT_HTTP_BLOCK_BYTES - 1).min(total - 1),
                        None => start + SUBSEQUENT_HTTP_BLOCK_BYTES - 1,
                    };

                    // If we fast-started with a c=IOS URL (which can stop serving ranges),
                    // fetch Block 1 (256KB..768KB) concurrently with refreshing the full visionos URL
                    // so we switch URLs before hitting 1 MB with zero pause or 403 retry!
                    if !ios_refreshed {
                        ios_refreshed = true;
                        let (block_res, fresh_opt) = tokio::join!(
                            fetch_range_block_url(&client, &active_url, &active_remote, start, end),
                            refresh_ytdlp_audio_stream(
                                &video_id_owned,
                                itag_hint.as_deref(),
                                &ext_owned
                            )
                        );
                        if let Some(fresh_remote) = fresh_opt {
                            active_url = fresh_remote.url.clone();
                            active_remote = fresh_remote;
                        }
                        if let Ok((bytes, discovered_total)) = block_res
                            && !bytes.is_empty()
                        {
                            if known_total.is_none() {
                                known_total = discovered_total;
                            }
                            if let Err(e) = file.write_all(&bytes).await {
                                fatal_err = Some(format!("Failed writing audio block: {e}"));
                                break;
                            }
                            written += bytes.len() as u64;
                            let _ = progress_tx.send(ChunkProgress {
                                written,
                                total_bytes: known_total,
                                done: false,
                                error: None,
                            });
                            continue;
                        }
                    }

                    match fetch_range_block_url(&client, &active_url, &active_remote, start, end).await {
                        Ok((bytes, discovered_total)) => {
                            if known_total.is_none() {
                                known_total = discovered_total;
                            }
                            if bytes.is_empty() {
                                break;
                            }
                            if let Err(e) = file.write_all(&bytes).await {
                                fatal_err = Some(format!("Failed writing audio block: {e}"));
                                break;
                            }
                            written += bytes.len() as u64;
                            let _ = progress_tx.send(ChunkProgress {
                                written,
                                total_bytes: known_total,
                                done: false,
                                error: None,
                            });
                        }
                        Err(e) => {
                            // If a range request dropped or returned 403 mid-song, refresh the URL via yt-dlp
                            // using the EXACT same itag/format and continue from `written`!
                            if let Some(fresh_remote) =
                                refresh_ytdlp_audio_stream(&video_id_owned, itag_hint.as_deref(), &ext_owned)
                                    .await
                                && let Ok((bytes, _)) = fetch_range_block_url(
                                    &client,
                                    &fresh_remote.url,
                                    &fresh_remote,
                                    start,
                                    end,
                                )
                                .await
                                && !bytes.is_empty()
                            {
                                active_url = fresh_remote.url.clone();
                                active_remote = fresh_remote;
                                if file.write_all(&bytes).await.is_ok() {
                                    written += bytes.len() as u64;
                                    let _ = progress_tx.send(ChunkProgress {
                                        written,
                                        total_bytes: known_total,
                                        done: false,
                                        error: None,
                                    });
                                    continue;
                                }
                            }
                            fatal_err = Some(e);
                            break;
                        }
                    }
                }
            }

            let _ = file.flush().await;
            let completed_cleanly = fatal_err.is_none()
                && written > 0
                && total_bytes.map(|t| written >= t).unwrap_or(true);

            let _ = progress_tx.send(ChunkProgress {
                written,
                total_bytes: total_bytes.or(Some(written)),
                done: true,
                error: if completed_cleanly {
                    None
                } else {
                    Some(
                        fatal_err
                            .unwrap_or_else(|| format!("Incomplete stream ({written} bytes)")),
                    )
                },
            });

            if let Ok(mut inflight) = in_flight_map.write() {
                inflight.remove(&session_key_for_cleanup);
            }

            if completed_cleanly {
                // Copy into final cache file while keeping the staging file readable for active localhost clients
                let final_cache_path =
                    streams_dir_owned.join(format!("{video_id_owned}.ytdlp.{ext_owned}"));
                let _ = tokio::fs::copy(&staging_file_path, &final_cache_path).await;
            }

            // Keep staging alive long enough for active playback to finish reading from localhost
            tokio::time::sleep(Duration::from_secs(600)).await;
            if let Ok(mut map) = sessions_map.write() {
                map.remove(&session_key_for_cleanup);
            }
            drop(staging);
        });
        if let Ok(mut inflight) = self.in_flight.write() {
            inflight.insert(
                session_key_for_inflight,
                (video_id_for_inflight, join.abort_handle()),
            );
        }

        wait_until_initial_chunk_ready(&mut progress_rx).await?;

        Ok(StreamInfo {
            url: self.local_url(&session_key, &ext),
            quality_label: remote.quality_label,
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        })
    }

    #[cfg(target_os = "android")]
    pub async fn start_ytdlp_pipe_stream(
        &self,
        _video_id: &str,
        _expected_total_bytes: Option<u64>,
        _streams_dir: &Path,
    ) -> Result<StreamInfo, String> {
        Err("yt-dlp progressive pipe is not available on Android".to_string())
    }

    #[cfg(not(target_os = "android"))]
    pub async fn start_ytdlp_pipe_stream(
        &self,
        video_id: &str,
        expected_total_bytes: Option<u64>,
        streams_dir: &Path,
    ) -> Result<StreamInfo, String> {
        if let Some(cached) = audio_cache::find(video_id, streams_dir) {
            return Ok(cached);
        }

        let staging = tempfile::tempdir_in(streams_dir)
            .map_err(|e| format!("Failed to create yt-dlp pipe staging dir: {e}"))?;
        let ext = "webm".to_string();
        let staging_file_path = staging.path().join(format!("{video_id}.pipe.{ext}"));
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(&staging_file_path)
            .await
            .map_err(|e| format!("Failed to open yt-dlp pipe staging file: {e}"))?;

        let watch_url = format!("https://music.youtube.com/watch?v={video_id}");
        let mut child = Command::new("yt-dlp")
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-warnings",
                "--quiet",
                "--no-part",
                "--socket-timeout",
                "5",
                "--retries",
                "1",
                "--extractor-args",
                "youtube:player_client=default,-tv_samsung;skip=hls,dash",
                "--js-runtimes",
                "node",
                "--remote-components",
                "ejs:github",
                "--http-chunk-size",
                "512K",
                "-f",
                "251/250/249/140/bestaudio[acodec=opus]/bestaudio/best",
                "-o",
                "-",
                "--",
                &watch_url,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn yt-dlp progressive stream: {e}"))?;

        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Failed to capture yt-dlp stdout pipe".to_string())?;

        let initial_total = expected_total_bytes.filter(|&sz| sz > 0);
        let (progress_tx, mut progress_rx) = watch::channel(ChunkProgress {
            written: 0,
            total_bytes: initial_total,
            done: false,
            error: None,
        });

        let session_key = self.allocate_session_key(video_id);
        let session = Arc::new(StreamSession {
            file_path: staging_file_path.clone(),
            mime_type: mime_for_ext(&ext).to_string(),
            progress_rx: progress_rx.clone(),
        });
        if let Ok(mut map) = self.sessions.write() {
            map.insert(session_key.clone(), session);
        }

        let video_id_owned = video_id.to_string();
        let ext_owned = ext.clone();
        let streams_dir_owned = streams_dir.to_path_buf();
        let sessions_map = self.sessions.clone();
        let in_flight_map = self.in_flight.clone();
        let session_key_for_cleanup = session_key.clone();
        let session_key_for_inflight = session_key.clone();
        let video_id_for_inflight = video_id.to_string();

        let join = tokio::spawn(async move {
            let mut buf = vec![0u8; 32 * 1024];
            let mut written: u64 = 0;
            let mut fatal_err: Option<String> = None;

            loop {
                match stdout.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Err(e) = file.write_all(&buf[..n]).await {
                            fatal_err = Some(format!("Failed writing yt-dlp pipe chunk: {e}"));
                            break;
                        }
                        written += n as u64;
                        let _ = progress_tx.send(ChunkProgress {
                            written,
                            total_bytes: initial_total.map(|t| t.max(written)),
                            done: false,
                            error: None,
                        });
                    }
                    Err(e) => {
                        fatal_err = Some(format!("Failed reading yt-dlp stdout chunk: {e}"));
                        break;
                    }
                }
            }

            let _ = file.flush().await;
            let status_ok = child
                .wait()
                .await
                .map(|s| s.success())
                .unwrap_or(false);

            if fatal_err.is_none() {
                if !status_ok {
                    fatal_err = Some("yt-dlp pipe exited unsuccessfully".to_string());
                } else if written == 0 {
                    fatal_err = Some("yt-dlp pipe produced 0 bytes".to_string());
                }
            }

            let _ = progress_tx.send(ChunkProgress {
                written,
                total_bytes: Some(written),
                done: true,
                error: fatal_err.clone(),
            });

            if let Ok(mut inflight) = in_flight_map.write() {
                inflight.remove(&session_key_for_cleanup);
            }

            if fatal_err.is_none() && status_ok && written > 0 {
                let final_cache_path =
                    streams_dir_owned.join(format!("{video_id_owned}.ytdlp.{ext_owned}"));
                let _ = tokio::fs::copy(&staging_file_path, &final_cache_path).await;
            }

            tokio::time::sleep(Duration::from_secs(600)).await;
            if let Ok(mut map) = sessions_map.write() {
                map.remove(&session_key_for_cleanup);
            }
            drop(staging);
        });
        if let Ok(mut inflight) = self.in_flight.write() {
            inflight.insert(
                session_key_for_inflight,
                (video_id_for_inflight, join.abort_handle()),
            );
        }

        wait_until_initial_chunk_ready(&mut progress_rx).await?;

        Ok(StreamInfo {
            url: self.local_url(&session_key, &ext),
            quality_label: "160kbps (WEBM)".to_string(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        })
    }
}

async fn fetch_range_block_url(
    client: &reqwest::Client,
    url: &str,
    remote: &StreamInfo,
    start: u64,
    end: u64,
) -> Result<(Vec<u8>, Option<u64>), String> {
    let mut last_err = String::new();
    for attempt in 0..4 {
        let mut r = client
            .get(url)
            .header(reqwest::header::RANGE, format!("bytes={start}-{end}"));
        if let Some(ua) = &remote.user_agent {
            r = r.header(reqwest::header::USER_AGENT, ua);
        }
        for (k, v) in &remote.headers {
            if !k.eq_ignore_ascii_case("range") {
                r = r.header(k.as_str(), v.as_str());
            }
        }
        match r.send().await {
            Ok(resp) if resp.status() == reqwest::StatusCode::PARTIAL_CONTENT => {
                let Some((first, last)) = response_byte_range(resp.headers()) else {
                    return Err(format!("Missing Content-Range for audio block {start}-{end}"));
                };
                if first != start || last < first || last > end {
                    return Err(format!("Unexpected Content-Range for audio block {start}-{end}"));
                }
                let total = parse_total_bytes_from_headers(
                    resp.headers(),
                    true,
                );
                match resp.bytes().await {
                    Ok(b) if b.len() as u64 == last - first + 1 => return Ok((b.to_vec(), total)),
                    Ok(_) => last_err = format!("Incomplete audio block {start}-{end}"),
                    Err(e) => last_err = format!("Error reading block {start}-{end}: {e}"),
                }
            }
            Ok(resp) if resp.status() == reqwest::StatusCode::RANGE_NOT_SATISFIABLE => {
                return Ok((Vec::new(), None));
            }
            Ok(resp) => {
                last_err = format!("Block {start}-{end} HTTP {}", resp.status());
            }
            Err(e) => {
                last_err = format!("Block {start}-{end} failed: {e}");
            }
        }
        if attempt < 3 {
            tokio::time::sleep(Duration::from_millis(120 * (attempt as u64 + 1))).await;
        }
    }
    Err(last_err)
}

fn response_byte_range(headers: &reqwest::header::HeaderMap) -> Option<(u64, u64)> {
    let value = headers.get(reqwest::header::CONTENT_RANGE)?.to_str().ok()?;
    let (range, _) = value.strip_prefix("bytes ")?.split_once('/')?;
    let (first, last) = range.split_once('-')?;
    Some((first.parse().ok()?, last.parse().ok()?))
}

async fn verify_audio_tail(
    client: &reqwest::Client,
    remote: &StreamInfo,
    total: u64,
) -> Result<(), String> {
    let last = total - 1;
    let (bytes, reported_total) = tokio::time::timeout(
        Duration::from_secs(4),
        fetch_range_block_url(client, &remote.url, remote, last, last),
    )
    .await
    .map_err(|_| "Timed out checking the end of the audio".to_string())??;
    if bytes.len() != 1 || reported_total != Some(total) {
        return Err("Audio source did not deliver the requested last byte".into());
    }
    Ok(())
}

#[cfg(target_os = "android")]
async fn refresh_ytdlp_audio_stream(
    _video_id: &str,
    _itag: Option<&str>,
    _ext: &str,
) -> Option<StreamInfo> {
    None
}

#[cfg(not(target_os = "android"))]
async fn refresh_ytdlp_audio_stream(
    video_id: &str,
    itag: Option<&str>,
    ext: &str,
) -> Option<StreamInfo> {
    let format_selector = match itag {
        Some(tag) if !tag.is_empty() => format!("{tag}/bestaudio[ext={ext}]/251/140/bestaudio"),
        _ => format!("bestaudio[ext={ext}]/251/140/bestaudio"),
    };
    let watch_url = format!("https://music.youtube.com/watch?v={video_id}");
    let out = tokio::time::timeout(
        Duration::from_secs(10),
        Command::new("yt-dlp")
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-warnings",
                "--socket-timeout",
                "5",
                "--retries",
                "1",
                "--dump-single-json",
                "--skip-download",
                "--extractor-args",
                "youtube:player_client=default,-tv_samsung;skip=hls,dash",
                "--js-runtimes",
                "node",
                "--remote-components",
                "ejs:github",
                "-f",
                &format_selector,
                "--",
                &watch_url,
            ])
            .output(),
    )
    .await
    .ok()?
    .ok()?;
    if !out.status.success() {
        return None;
    }
    #[derive(serde::Deserialize)]
    struct FreshAudio {
        url: String,
        #[serde(default)]
        http_headers: HashMap<String, String>,
    }
    let parsed: FreshAudio = serde_json::from_slice(&out.stdout).ok()?;
    if !parsed.url.starts_with("http") {
        return None;
    }
    let ua = parsed
        .http_headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("user-agent"))
        .map(|(_, v)| v.clone());
    Some(StreamInfo {
        url: parsed.url,
        quality_label: ext.to_uppercase(),
        user_agent: ua,
        headers: parsed.http_headers,
        local_audio: None,
    })
}

fn parse_itag_from_url(raw_url: &str) -> Option<String> {
    let parsed = reqwest::Url::parse(raw_url).ok()?;
    parsed
        .query_pairs()
        .find(|(k, _)| k == "itag")
        .map(|(_, v)| v.into_owned())
        .filter(|s| !s.is_empty())
}

async fn wait_until_initial_chunk_ready(
    progress_rx: &mut watch::Receiver<ChunkProgress>,
) -> Result<(), String> {
    loop {
        {
            let state = progress_rx.borrow();
            if let Some(err) = &state.error {
                return Err(err.clone());
            }
            if state.written >= READY_THRESHOLD_BYTES || (state.done && state.written > 0) {
                return Ok(());
            }
            if state.done && state.written == 0 {
                return Err("Audio stream ended with 0 bytes".to_string());
            }
        }
        progress_rx.changed().await.map_err(|_| "Audio stream channel closed early".to_string())?;
    }
}

fn stream_session_key(path: &str) -> Option<&str> {
    let raw_key = path.split('?').next()?.strip_prefix("/stream/")?;
    Some(raw_key.rsplit_once('.').map_or(raw_key, |(key, _)| key))
}

async fn handle_client(
    socket: TcpStream,
    sessions: Arc<RwLock<HashMap<String, Arc<StreamSession>>>>,
) -> Result<(), std::io::Error> {
    let (read_half, mut write_half) = socket.into_split();
    let mut reader = BufReader::new(read_half);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).await? == 0 {
        return Ok(());
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }
    let method = parts[0];
    let path = parts[1];

    let mut range_header: Option<String> = None;
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).await?;
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("range") {
                range_header = Some(v.trim().to_string());
            }
        }
    }

    const CORS_HEADERS: &str = "Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, HEAD, OPTIONS\r\nAccess-Control-Allow-Headers: Range\r\nAccess-Control-Expose-Headers: Content-Range, Content-Length, Accept-Ranges\r\n";

    if method.eq_ignore_ascii_case("OPTIONS") {
        let hdr = format!(
            "HTTP/1.1 204 No Content\r\n{CORS_HEADERS}Content-Length: 0\r\nConnection: close\r\n\r\n"
        );
        write_half.write_all(hdr.as_bytes()).await?;
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
        if let Some(end_inc) = end_pos_inclusive {
            if pos > end_inc {
                break;
            }
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

fn parse_total_bytes_from_headers(
    headers: &reqwest::header::HeaderMap,
    is_partial: bool,
) -> Option<u64> {
    if let Some(cr) = headers
        .get(reqwest::header::CONTENT_RANGE)
        .and_then(|v| v.to_str().ok())
    {
        if let Some((_, total_str)) = cr.rsplit_once('/') {
            if let Ok(total) = total_str.trim().parse::<u64>() {
                return Some(total);
            }
        }
    }
    if !is_partial {
        if let Some(cl) = headers
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
        {
            if let Ok(total) = cl.trim().parse::<u64>() {
                return Some(total);
            }
        }
    }
    None
}

fn parse_clen_from_url(raw_url: &str) -> Option<u64> {
    let parsed = reqwest::Url::parse(raw_url).ok()?;
    parsed
        .query_pairs()
        .find(|(k, _)| k == "clen")
        .and_then(|(_, v)| v.parse::<u64>().ok())
        .filter(|&sz| sz > 0)
}

fn parse_range_header(header_val: &str) -> Option<(u64, Option<u64>)> {
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

fn normalize_ext(ext: &str) -> String {
    let trimmed = ext.trim().trim_start_matches('.').to_ascii_lowercase();
    match trimmed.as_str() {
        "webm" | "opus" => "webm".to_string(),
        "m4a" | "mp4" | "aac" => "m4a".to_string(),
        "mp3" => "mp3".to_string(),
        "ogg" => "ogg".to_string(),
        _ => "webm".to_string(),
    }
}

fn mime_for_ext(ext: &str) -> &'static str {
    match ext {
        "m4a" | "mp4" => "audio/mp4",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        _ => "audio/webm",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn spawn_slow_range_origin(payload: Arc<Vec<u8>>, cutoff: Option<u64>) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            loop {
                let Ok((socket, _)) = listener.accept().await else {
                    break;
                };
                let data = payload.clone();
                tokio::spawn(async move {
                    let (read_half, mut write_half) = socket.into_split();
                    let mut reader = BufReader::new(read_half);
                    let mut range_hdr: Option<String> = None;
                    loop {
                        let mut line = String::new();
                        let Ok(n) = reader.read_line(&mut line).await else {
                            return;
                        };
                        if n == 0 || line == "\r\n" || line == "\n" {
                            break;
                        }
                        if let Some((k, v)) = line.split_once(':') {
                            if k.trim().eq_ignore_ascii_case("range") {
                                range_hdr = Some(v.trim().to_string());
                            }
                        }
                    }

                    let total = data.len() as u64;
                    let (start, end) = range_hdr
                        .as_deref()
                        .and_then(parse_range_header)
                        .map(|(s, e)| (s, e.unwrap_or(total - 1).min(total - 1)))
                        .unwrap_or((0, total - 1));

                    if cutoff.is_some_and(|limit| end >= limit) {
                        let _ = write_half.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n").await;
                        return;
                    }

                    if start >= total {
                        let _ = write_half
                            .write_all(b"HTTP/1.1 416 Range Not Satisfiable\r\nContent-Length: 0\r\n\r\n")
                            .await;
                        return;
                    }

                    if start > 0 {
                        // Delay subsequent blocks so the test can verify Block 0 plays before full download completes
                        tokio::time::sleep(Duration::from_millis(250)).await;
                    }

                    let slice = &data[start as usize..=end as usize];
                    let hdr = format!(
                        "HTTP/1.1 206 Partial Content\r\nContent-Type: audio/webm\r\nContent-Range: bytes {start}-{end}/{total}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        slice.len()
                    );
                    let _ = write_half.write_all(hdr.as_bytes()).await;
                    let _ = write_half.write_all(slice).await;
                });
            }
        });
        port
    }

    #[tokio::test]
    async fn streams_audio_in_blocks_before_full_download_completes() {
        let total_size = 900 * 1024; // 900 KB (> 256 KB initial block + 512 KB second block)
        let payload: Arc<Vec<u8>> =
            Arc::new((0..total_size).map(|i| (i % 251) as u8).collect());
        let origin_port = spawn_slow_range_origin(payload.clone(), None).await;

        let temp = tempfile::tempdir().unwrap();
        let streamer = AudioStreamer::start_new().await.unwrap();
        let remote = StreamInfo {
            url: format!("http://127.0.0.1:{origin_port}/audio.webm"),
            quality_label: "160kbps (WEBM)".to_string(),
            user_agent: Some("GyroTubeTest/1.0".to_string()),
            headers: HashMap::new(),
            local_audio: None,
        };

        let started = std::time::Instant::now();
        let local_stream = streamer
            .start_chunked_http_stream("dQw4w9WgXcQ", remote, "webm", None, temp.path())
            .await
            .unwrap();

        // start_chunked_http_stream must return as soon as Block 0 is ready (< 150ms),
        // without waiting for the delayed subsequent blocks (which take > 500ms).
        assert!(
            started.elapsed() < Duration::from_millis(200),
            "Expected stream to become ready before subsequent blocks finished, took {:?}",
            started.elapsed()
        );

        let client = reqwest::Client::new();
        let full_bytes = client
            .get(&local_stream.url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        assert_eq!(full_bytes.len(), total_size);
        assert_eq!(&full_bytes[..], &payload[..]);
    }

    #[tokio::test]
    async fn supports_byte_range_requests_on_progressive_stream() {
        let total_size = 512 * 1024;
        let payload: Arc<Vec<u8>> =
            Arc::new((0..total_size).map(|i| (i % 239) as u8).collect());
        let origin_port = spawn_slow_range_origin(payload.clone(), None).await;

        let temp = tempfile::tempdir().unwrap();
        let streamer = AudioStreamer::start_new().await.unwrap();
        let remote = StreamInfo {
            url: format!("http://127.0.0.1:{origin_port}/audio.webm"),
            quality_label: "160kbps (WEBM)".to_string(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };

        let local_stream = streamer
            .start_chunked_http_stream("3JZ_D3ELwOQ", remote, "webm", None, temp.path())
            .await
            .unwrap();

        let client = reqwest::Client::new();
        let resp = client
            .get(&local_stream.url)
            .header(reqwest::header::RANGE, "bytes=1000-4999")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), reqwest::StatusCode::PARTIAL_CONTENT);
        let body = resp.bytes().await.unwrap();
        assert_eq!(body.len(), 4000);
        assert_eq!(&body[..], &payload[1000..5000]);
    }

    #[tokio::test]
    async fn rejects_sources_that_serve_the_start_but_block_the_rest() {
        let total = 900 * 1024;
        let payload = Arc::new(vec![0; total as usize]);
        let limited = spawn_slow_range_origin(payload.clone(), Some(768 * 1024)).await;
        let complete = spawn_slow_range_origin(payload, None).await;
        let mut remote = StreamInfo {
            url: format!("http://127.0.0.1:{limited}/audio.webm"),
            quality_label: "WEBM".into(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };
        let client = reqwest::Client::new();
        assert!(fetch_range_block_url(&client, &remote.url, &remote, 0, INITIAL_HTTP_BLOCK_BYTES - 1).await.is_ok());
        assert!(verify_audio_tail(&client, &remote, total).await.unwrap_err().contains("403"));
        remote.url = format!("http://127.0.0.1:{complete}/audio.webm");
        verify_audio_tail(&client, &remote, total).await.unwrap();
    }

    #[test]
    fn extracts_clen_query_param_from_googlevideo_url() {
        let url = "https://rr2---sn-abc.googlevideo.com/videoplayback?expire=1700000000&clen=3845120&mime=audio%2Fwebm";
        assert_eq!(parse_clen_from_url(url), Some(3_845_120));
    }

    #[tokio::test]
    async fn a_stream_that_fails_after_buffering_cannot_be_reused() {
        let streamer = AudioStreamer::start_new().await.unwrap();
        let key = "test-1";
        let url = reqwest::Url::parse(&streamer.local_url(key, "webm")).unwrap();
        let progress = ChunkProgress {
            written: READY_THRESHOLD_BYTES,
            total_bytes: Some(2 * READY_THRESHOLD_BYTES),
            done: false,
            error: None,
        };
        let (tx, mut rx) = watch::channel(progress.clone());
        streamer.sessions.write().unwrap().insert(key.into(), Arc::new(StreamSession {
            file_path: PathBuf::new(),
            mime_type: "audio/webm".into(),
            progress_rx: rx.clone(),
        }));
        assert!(streamer.can_reuse_url(&url));
        tx.send_replace(ChunkProgress {
            done: true,
            error: Some("network interrupted".into()),
            ..progress
        });
        assert!(!streamer.can_reuse_url(&url));
        assert_eq!(wait_until_initial_chunk_ready(&mut rx).await.unwrap_err(), "network interrupted");
        let response = reqwest::Client::new().get(url).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::BAD_GATEWAY);
    }
}
