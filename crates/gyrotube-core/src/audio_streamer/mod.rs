mod http;
mod ytdlp;
#[cfg(test)]
mod tests;

#[cfg(not(target_os = "android"))]
pub(crate) use ytdlp::ytdlp_command;
use http::{
    handle_client, mime_for_ext, normalize_ext, parse_total_bytes_from_headers, stream_session_key,
};
use ytdlp::{
    fetch_range_block_url, refresh_ytdlp_audio_stream, response_byte_range, verify_audio_tail,
};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
#[cfg(not(target_os = "android"))]
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;

#[cfg(not(target_os = "android"))]
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::watch;

#[cfg(not(target_os = "android"))]
use crate::audio_cache;
use crate::models::StreamInfo;

const INITIAL_HTTP_BLOCK_BYTES: u64 = 256 * 1024;
const SUBSEQUENT_HTTP_BLOCK_BYTES: u64 = 512 * 1024;
const READY_THRESHOLD_BYTES: u64 = 32 * 1024;
pub(super) const FALLBACK_SEEKABLE_BYTES: u64 = 32 * 1024 * 1024;


#[derive(Clone, Debug)]
pub struct ChunkProgress {
    pub written: u64,
    pub total_bytes: Option<u64>,
    pub done: bool,
    pub error: Option<String>,
}

pub(super) struct StreamSession {
    pub(super) file_path: PathBuf,
    pub(super) mime_type: String,
    pub(super) progress_rx: watch::Receiver<ChunkProgress>,
}

#[derive(Clone)]
pub(super) struct ProxyTarget {
    pub(super) url: String,
    pub(super) user_agent: String,
    pub(super) mime_type: String,
}

pub struct AudioStreamer {
    port: u16,
    sessions: Arc<RwLock<HashMap<String, Arc<StreamSession>>>>,
    proxies: Arc<RwLock<HashMap<String, ProxyTarget>>>,
    in_flight: Arc<RwLock<HashMap<String, (String, tokio::task::AbortHandle)>>>,
    client: reqwest::Client,
    next_id: AtomicU64,
}

static GLOBAL_STREAMER: OnceLock<Arc<AudioStreamer>> = OnceLock::new();

impl AudioStreamer {
    pub fn serve_remote_proxy(
        &self,
        video_id: &str,
        url: &str,
        user_agent: &str,
        mime_type: &str,
    ) -> String {
        let seq = self.next_id.fetch_add(1, Ordering::Relaxed);
        let ext = if mime_type.contains("webm") { "webm" } else { "mp4" };
        let key = format!("{video_id}-{seq}.{ext}");
        if let Ok(mut map) = self.proxies.write() {
            if map.len() > 24 {
                map.clear();
            }
            map.insert(
                key.clone(),
                ProxyTarget {
                    url: url.to_string(),
                    user_agent: user_agent.to_string(),
                    mime_type: mime_type.to_string(),
                },
            );
        }
        format!("http://127.0.0.1:{}/proxy/{key}", self.port)
    }

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
        let proxies: Arc<RwLock<HashMap<String, ProxyTarget>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let client = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(4))
            .read_timeout(Duration::from_secs(12))
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to build chunked HTTP client: {e}"))?;

        let sessions_for_loop = sessions.clone();
        let proxies_for_loop = proxies.clone();
        let client_for_loop = client.clone();
        tokio::spawn(async move {
            loop {
                let Ok((socket, _)) = listener.accept().await else {
                    break;
                };
                let sessions_ref = sessions_for_loop.clone();
                let proxies_ref = proxies_for_loop.clone();
                let client_ref = client_for_loop.clone();
                tokio::spawn(async move {
                    let _ = handle_client(socket, sessions_ref, proxies_ref, client_ref).await;
                });
            }
        });

        Ok(Arc::new(Self {
            port,
            sessions,
            proxies,
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

    pub fn cover_base_url(&self) -> String {
        format!("http://127.0.0.1:{}/cover", self.port)
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
            .map_err(|e| format!("Initial audio block request failed: {}", e.without_url()))?;
        let status = resp.status();
        if !status.is_success() {
            return Err(format!("Initial audio block returned HTTP {status}"));
        }

        let is_partial = status == reqwest::StatusCode::PARTIAL_CONTENT;
        let total_bytes = parse_total_bytes_from_headers(resp.headers(), is_partial)
            .or_else(|| parse_clen_from_url(&remote.url))
            .or(expected_total_bytes);

        if is_partial {
            let total = total_bytes
                .filter(|&total| total > 0)
                .ok_or_else(|| "Audio source did not report its total size".to_string())?;
            if parse_clen_from_url(&remote.url)
                .or(expected_total_bytes)
                .is_some_and(|size| size != total)
            {
                return Err("Audio source reported a different file size".into());
            }
            if response_byte_range(resp.headers()) != Some((0, first_end.min(total - 1))) {
                return Err("Audio source did not deliver the requested initial range".into());
            }
            if total > INITIAL_HTTP_BLOCK_BYTES {
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
                        fatal_err = Some(format!("Network error reading block 0: {}", e.without_url()));
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
                    if let Some(total) = known_total
                        && written >= total
                    {
                        break;
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
                            eprintln!(
                                "[audio] id={video_id_owned} bloque fallido en {written} bytes: {e}"
                            );
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
        let mut child = ytdlp_command()
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

fn parse_clen_from_url(raw_url: &str) -> Option<u64> {
    let parsed = reqwest::Url::parse(raw_url).ok()?;
    parsed
        .query_pairs()
        .find(|(k, _)| k == "clen")
        .and_then(|(_, v)| v.parse::<u64>().ok())
        .filter(|&sz| sz > 0)
}
