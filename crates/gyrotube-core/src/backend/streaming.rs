use super::*;
use super::parsers::*;

impl Backend {
    fn streams_dir() -> Result<std::path::PathBuf, String> {
        let base = cache_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("gyrotube"))
            .join("streams");
        std::fs::create_dir_all(&base)
            .map_err(|e| format!("No se pudo crear caché de audio: {e}"))?;
        Ok(base)
    }

    pub(super) fn cached_stream(&self, video_id: &str) -> Option<StreamInfo> {
        validate_video_id(video_id).ok()?;
        let dir = Self::streams_dir().ok()?;
        audio_cache::find(video_id, &dir)
    }

    pub fn cached_track_stream(&self, track: &Track) -> Option<StreamInfo> {
        let video_id = &track.id;
        validate_video_id(video_id).ok()?;
        if !track.is_music && self.cached_music_proof(video_id).is_none() {
            return None;
        }
        self.cached_url(video_id).or_else(|| self.cached_stream(video_id))
    }

    pub fn cancel_stale_streams(&self, keep_ids: &[&str]) {
        if let Some(streamer) = crate::audio_streamer::AudioStreamer::try_global() {
            let aborted = streamer.abort_downloads_except(keep_ids);
            if !aborted.is_empty()
                && let Ok(mut cache) = self.stream_cache.write()
            {
                for vid in aborted {
                    cache.remove(&vid);
                }
            }
        }
    }

    pub(super) fn cached_url(&self, video_id: &str) -> Option<StreamInfo> {
        let cache = self.stream_cache.read().ok()?;
        let (created, stream) = cache.get(video_id)?;
        stream_url_is_fresh(*created, stream).then(|| stream.clone())
    }

    pub(super) fn audio_lock(
        &self,
        video_id: &str,
        work: AudioWork,
    ) -> Result<Arc<tokio::sync::Mutex<()>>, String> {
        let mut locks = self
            .audio_locks
            .lock()
            .map_err(|_| String::from("Error interno de sincronización"))?;
        locks.retain(|_, lock| Arc::strong_count(lock) > 1);
        Ok(locks.entry((video_id.to_owned(), work)).or_default().clone())
    }

    pub fn trim_audio_cache_on_startup() {
        runtime().spawn_blocking(|| {
            if let Ok(dir) = Self::streams_dir() {
                audio_cache::trim_existing(&dir);
            }
        });
    }

    fn cached_music_proof(&self, video_id: &str) -> Option<Track> {
        let proofs = self.music_proofs.read().ok()?;
        let (checked, track) = proofs.get(video_id)?;
        (checked.elapsed() < MUSIC_PROOF_TTL).then(|| track.clone())
    }

    pub async fn verify_music_track(&self, video_id: &str) -> Result<Track, String> {
        validate_video_id(video_id)?;
        if let Some(track) = self.cached_music_proof(video_id) {
            return Ok(track);
        }
        let lock = self.audio_lock(video_id, AudioWork::Verify)?;
        let _guard = lock.lock().await;
        if let Some(track) = self.cached_music_proof(video_id) {
            return Ok(track);
        }

        let started = Instant::now();
        let result = self.innertube.fetch_music_details(video_id).await;
        log_audio_timing(video_id, "catalogo", started);
        let details = result.map_err(|e| format!("No se pudo verificar la canción de YouTube Music: {e}"))?;
        let track = verified_music_track(video_id, &details.item)?;
        if let Ok(mut proofs) = self.music_proofs.write() {
            proofs.retain(|_, (checked, _)| checked.elapsed() < MUSIC_PROOF_TTL);
            proofs.insert(video_id.to_owned(), (Instant::now(), track.clone()));
        }
        Ok(track)
    }

    pub fn downloads_dir_ready() -> Result<std::path::PathBuf, String> {
        let dir = crate::models::downloads_dir()
            .ok_or_else(|| "Downloads directory unavailable".to_string())?;
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create downloads directory: {e}"))?;
        Ok(dir)
    }

    pub fn list_downloaded_track_ids() -> Vec<String> {
        let Some(dir) = crate::models::downloads_dir() else {
            return Vec::new();
        };
        let Ok(entries) = std::fs::read_dir(dir) else {
            return Vec::new();
        };
        let mut ids = Vec::new();
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else { continue };
            if !meta.is_file() || meta.len() == 0 {
                continue;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some((id, ext)) = name.split_once('.')
                && validate_video_id(id).is_ok()
                && (audio_cache::is_playable_extension(ext)
                    || ext
                        .strip_prefix("ytdlp.")
                        .is_some_and(audio_cache::is_playable_extension))
                && !ids.iter().any(|existing| existing == id)
            {
                ids.push(id.to_string());
            }
        }
        ids
    }

    pub fn remove_downloaded_tracks(track_ids: &[String]) -> Result<(), String> {
        let Some(dir) = crate::models::downloads_dir() else {
            return Ok(());
        };
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return Ok(());
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some((id, _)) = name.split_once('.')
                && track_ids.iter().any(|target| target == id)
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
        Ok(())
    }

    pub fn clear_all_downloads() -> Result<(), String> {
        let Some(dir) = crate::models::downloads_dir() else {
            return Ok(());
        };
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if entry.metadata().is_ok_and(|m| m.is_file()) {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
        Ok(())
    }

    pub fn downloads_bytes() -> u64 {
        let Some(dir) = crate::models::downloads_dir() else {
            return 0;
        };
        std::fs::read_dir(dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .map(|m| m.len())
            .sum()
    }

    async fn download_cover_offline(track: &Track, dl_dir: &std::path::Path) {
        let cover_path = dl_dir.join(format!("{}.cover", track.id));
        if std::fs::metadata(&cover_path).is_ok_and(|m| m.is_file() && m.len() > 0) {
            return;
        }
        let Some(url) = track
            .cover_large_url
            .as_deref()
            .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
            .or_else(|| {
                track
                    .cover_url
                    .as_deref()
                    .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
            })
        else {
            return;
        };
        let Ok(client) = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(6))
            .read_timeout(Duration::from_secs(10))
            .build()
        else {
            return;
        };
        if let Ok(resp) = client.get(url).send().await
            && resp.status().is_success()
            && let Ok(bytes) = resp.bytes().await
            && !bytes.is_empty()
            && bytes.len() <= 5 * 1024 * 1024
        {
            let tmp = dl_dir.join(format!("{}.cover.part", track.id));
            if tokio::fs::write(&tmp, &bytes).await.is_ok() {
                let _ = tokio::fs::rename(&tmp, &cover_path).await;
            }
        }
    }

    async fn download_lyrics_offline(&self, track: &Track, dl_dir: &std::path::Path) {
        let lyrics_path = dl_dir.join(format!("{}.lyrics.json", track.id));
        if std::fs::metadata(&lyrics_path).is_ok_and(|m| m.is_file() && m.len() > 0) {
            return;
        }
        if let Ok(data) = self.fetch_synced_lyrics(track).await
            && let Ok(json) = serde_json::to_vec(&data)
        {
            let tmp = dl_dir.join(format!("{}.lyrics.json.part", track.id));
            if tokio::fs::write(&tmp, &json).await.is_ok() {
                let _ = tokio::fs::rename(&tmp, &lyrics_path).await;
            }
        }
    }

    pub async fn download_track_offline(&self, track: &Track) -> Result<String, String> {
        validate_video_id(&track.id)?;
        let dl_dir = Self::downloads_dir_ready()?;
        let ((), ()) = tokio::join!(
            Self::download_cover_offline(track, &dl_dir),
            self.download_lyrics_offline(track, &dl_dir)
        );
        if audio_cache::find(&track.id, &dl_dir).is_some() {
            return Ok(track.id.clone());
        }

        // Fast-path: copy from warm audio cache if already downloaded during playback
        if let Ok(streams_dir) = Self::streams_dir()
            && let Some(cached) = audio_cache::find(&track.id, &streams_dir)
        {
            let src = std::path::PathBuf::from(&cached.url);
            if src.is_file() {
                let ext = src
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("webm");
                let tmp = dl_dir.join(format!("{}.part.{ext}", track.id));
                let dest = dl_dir.join(format!("{}.ytdlp.{ext}", track.id));
                tokio::fs::copy(&src, &tmp)
                    .await
                    .map_err(|e| format!("Failed copying cached track: {e}"))?;
                tokio::fs::rename(&tmp, &dest)
                    .await
                    .map_err(|e| format!("Failed finalizing downloaded track: {e}"))?;
                return Ok(track.id.clone());
            }
        }

        // Resolve via the standard stream pipeline (works on both desktop and Android)
        let info = self.resolve_track_stream(track).await?;
        if !info.url.starts_with("http://") && !info.url.starts_with("https://") {
            let src = std::path::PathBuf::from(&info.url);
            let ext = src.extension().and_then(|s| s.to_str()).unwrap_or("webm");
            let tmp = dl_dir.join(format!("{}.part.{ext}", track.id));
            let dest = dl_dir.join(format!("{}.ytdlp.{ext}", track.id));
            tokio::fs::copy(&src, &tmp)
                .await
                .map_err(|e| format!("Failed saving track: {e}"))?;
            tokio::fs::rename(&tmp, &dest)
                .await
                .map_err(|e| format!("Failed finalizing track: {e}"))?;
            return Ok(track.id.clone());
        }

        let ext = if info.url.contains(".m4a") || info.url.contains(".mp4") {
            "m4a"
        } else {
            "webm"
        };
        let tmp = dl_dir.join(format!("{}.part.{ext}", track.id));
        let dest = dl_dir.join(format!("{}.ytdlp.{ext}", track.id));

        let client = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| format!("Failed to build download client: {e}"))?;
        let mut resp = client
            .get(&info.url)
            .send()
            .await
            .map_err(|e| format!("Offline download request failed: {}", e.without_url()))?;
        if !resp.status().is_success() {
            return Err(format!("Offline download returned HTTP {}", resp.status()));
        }

        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(&tmp)
            .await
            .map_err(|e| format!("Failed to create download file: {e}"))?;
        let mut written = 0u64;
        while let Some(chunk) = resp
            .chunk()
            .await
            .map_err(|e| format!("Download interrupted: {}", e.without_url()))?
        {
            if !chunk.is_empty() {
                file.write_all(&chunk)
                    .await
                    .map_err(|e| format!("Failed writing download: {e}"))?;
                written += chunk.len() as u64;
            }
        }
        file.flush()
            .await
            .map_err(|e| format!("Failed flushing download: {e}"))?;
        drop(file);

        if written == 0 {
            let _ = tokio::fs::remove_file(&tmp).await;
            return Err("Downloaded audio was empty".to_string());
        }

        tokio::fs::rename(&tmp, &dest)
            .await
            .map_err(|e| format!("Failed finalizing download: {e}"))?;
        Ok(track.id.clone())
    }

    pub async fn resolve_track_stream(&self, track: &Track) -> Result<StreamInfo, String> {
        validate_video_id(&track.id)?;
        // Always prefer permanent offline downloads first (zero network required)
        if let Some(dl_dir) = crate::models::downloads_dir()
            && let Some(downloaded) = audio_cache::find(&track.id, &dl_dir)
        {
            return Ok(downloaded);
        }
        if !track.is_music && self.cached_music_proof(&track.id).is_none() {
            let has_cached =
                self.cached_stream(&track.id).is_some() || self.cached_url(&track.id).is_some();
            if has_cached {
                self.verify_music_track(&track.id).await?;
            } else {
                let (verify_res, stream_res) = tokio::join!(
                    self.verify_music_track(&track.id),
                    self.resolve_audio_stream(&track.id)
                );
                if let Err(err) = verify_res {
                    if let Ok(mut cache) = self.stream_cache.write() {
                        cache.remove(&track.id);
                    }
                    return Err(err);
                }
                return stream_res;
            }
        }
        self.resolve_audio_stream(&track.id).await
    }

    pub async fn resolve_video_stream(
        &self,
        video_id: &str,
        max_height: u32,
    ) -> Result<VideoStreamInfo, String> {
        validate_video_id(video_id)?;

        // Desktop: prefer yt-dlp (solves YouTube n-parameter / PO-token reliably without 403)
        #[cfg(not(target_os = "android"))]
        {
            #[cfg(target_os = "linux")]
            let fmt = format!(
                "bestvideo[vcodec^=vp9][height<={max_height}]/bestvideo[ext=webm][height<={max_height}]/bestvideo[vcodec^=avc1][height<={max_height}]/18"
            );
            #[cfg(not(target_os = "linux"))]
            let fmt = format!(
                "bestvideo[vcodec^=avc1][height<={max_height}]/bestvideo[ext=mp4][height<={max_height}]/bestvideo[ext=webm][height<={max_height}]/18"
            );
            let mut cmd = crate::audio_streamer::ytdlp_command();
            cmd.kill_on_drop(true)
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
                    &fmt,
                    "--",
                ])
                .arg(format!("https://www.youtube.com/watch?v={video_id}"));
            if let Ok(Ok(output)) = tokio::time::timeout(Duration::from_secs(12), cmd.output()).await
                && output.status.success()
                && let Ok(val) = serde_json::from_slice::<serde_json::Value>(&output.stdout)
                && let Some(raw_url) = val["url"].as_str()
                && let Ok(parsed) = reqwest::Url::parse(raw_url)
                && matches!(parsed.scheme(), "http" | "https")
            {
                let height = val["height"].as_u64().unwrap_or(720) as u32;
                let width = val["width"].as_u64().unwrap_or(1280) as u32;
                let ext = val["ext"].as_str().unwrap_or("webm");
                let mime_type = if ext == "webm" { "video/webm" } else { "video/mp4" };
                let ua = val["http_headers"]["User-Agent"]
                    .as_str()
                    .unwrap_or("Mozilla/5.0");
                let streamer = crate::audio_streamer::AudioStreamer::global().await?;
                let proxied = streamer.serve_remote_proxy(video_id, parsed.as_str(), ua, mime_type);
                return Ok(VideoStreamInfo {
                    url: proxied,
                    quality_label: format!("{height}p"),
                    width,
                    height,
                });
            }
        }

        let visitor_data = self.innertube.visitor_data(true).await;
        let candidate = self
            .innertube
            .extract_direct_video_stream(video_id, max_height, visitor_data.as_deref())
            .await?;
        let streamer = crate::audio_streamer::AudioStreamer::global().await?;
        let proxied = streamer.serve_remote_proxy(
            video_id,
            &candidate.url,
            &candidate.user_agent,
            candidate.mime_type,
        );
        Ok(VideoStreamInfo {
            url: proxied,
            quality_label: format!("{}p", candidate.height),
            width: candidate.width,
            height: candidate.height,
        })
    }

    pub async fn find_counterpart_track(
        &self,
        track: &Track,
        want_video: bool,
    ) -> Result<Option<Track>, String> {
        // Reuse InnerTubeClient's cached /next watch response (RDAMVM{id}) so official
        // Song <-> Video counterparts resolve with zero duplicate HTTP calls.
        if validate_video_id(&track.id).is_err() {
            return Ok(None);
        }
        let Ok(details) = self.innertube.fetch_music_details(&track.id).await else {
            return Ok(None);
        };
        Ok(select_official_counterpart(&details, track, want_video))
    }

    pub(super) async fn resolve_audio_stream(&self, video_id: &str) -> Result<StreamInfo, String> {
        validate_video_id(video_id)?;
        let cached = || self.cached_stream(video_id).or_else(|| self.cached_url(video_id));
        if let Some(stream) = cached() {
            return Ok(stream);
        }
        let lock = self.audio_lock(video_id, AudioWork::Resolve)?;
        let _guard = lock.lock().await;
        if let Some(stream) = cached() {
            return Ok(stream);
        }

        let streams_dir = Self::streams_dir()?;
        let streamer = crate::audio_streamer::AudioStreamer::global().await?;

        let visitor_data = self.innertube.visitor_data(false).await;
        let timeout = if cfg!(target_os = "android") {
            Duration::from_secs(6)
        } else {
            Duration::from_secs(4)
        };
        for source in self.innertube.ordered_profiles() {
            let started = Instant::now();
            let extracted = self
                .innertube
                .extract_direct_stream(
                    video_id,
                    source,
                    visitor_data.as_deref(),
                    timeout,
                    None,
                )
                .await;
            let (remote, ext, size_hint) = match extracted {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("[audio] id={video_id} fuente={source} extracción: {err}");
                    continue;
                }
            };
            match streamer
                .start_chunked_http_stream(video_id, remote, ext, size_hint, &streams_dir)
                .await
            {
                Ok(local_stream) => {
                    self.innertube.record_working_profile(source);
                    log_audio_timing(video_id, source, started);
                    if let Ok(mut cache) = self.stream_cache.write() {
                        cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
                        cache.insert(video_id.to_owned(), (Instant::now(), local_stream.clone()));
                    }
                    return Ok(local_stream);
                }
                Err(err) => eprintln!("[audio] id={video_id} fuente={source} descarga: {err}"),
            }
        }

        #[cfg(target_os = "android")]
        {
            Err("No se pudo resolver el flujo de audio (Android).".to_string())
        }

        #[cfg(not(target_os = "android"))]
        {

        // Fallback: extract stream URL via yt-dlp excluding tv_samsung (which triggers 6s ad / 403)
        // and skipping HLS/DASH manifest downloads.
        #[cfg(feature = "native-audio")]
        let ytdlp_format = match self.innertube.audio_quality() {
            crate::models::AudioQuality::High => "141/140/139/bestaudio[ext=m4a]",
            crate::models::AudioQuality::Normal => "140/139/bestaudio[ext=m4a]",
            crate::models::AudioQuality::Low => "139/140/bestaudio[ext=m4a]",
        };
        #[cfg(not(feature = "native-audio"))]
        let ytdlp_format = match self.innertube.audio_quality() {
            crate::models::AudioQuality::High => "251/140/bestaudio",
            crate::models::AudioQuality::Normal => "250/249/140/bestaudio",
            crate::models::AudioQuality::Low => "249/139/250/worstaudio/bestaudio",
        };
        let mut cmd = crate::audio_streamer::ytdlp_command();
        cmd.kill_on_drop(true)
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
                ytdlp_format,
                "--",
            ])
            .arg(format!("https://music.youtube.com/watch?v={video_id}"));
        let started = Instant::now();
        let result = tokio::time::timeout(Duration::from_secs(12), cmd.output()).await;
        log_audio_timing(video_id, "extraccion-yt-dlp", started);
        let mut expected_bytes_hint: Option<u64> = None;
        if let Ok(Ok(output)) = result
            && output.status.success()
        {
            expected_bytes_hint = filesize_from_ytdlp_json(&output.stdout);
            if let Ok(remote) = stream_from_ytdlp_json(&output.stdout) {
                let ext = if remote.url.contains("mime=audio%2Fmp4") {
                    "m4a"
                } else {
                    "webm"
                };
                if let Ok(local_stream) = streamer
                    .start_chunked_http_stream(
                        video_id,
                        remote,
                        ext,
                        expected_bytes_hint,
                        &streams_dir,
                    )
                    .await
                {
                    if let Ok(mut cache) = self.stream_cache.write() {
                        cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
                        cache.insert(video_id.to_string(), (Instant::now(), local_stream.clone()));
                    }
                    return Ok(local_stream);
                }
            }
        }

        let pipe_started = Instant::now();
        let resolved = streamer
            .start_ytdlp_pipe_stream(video_id, expected_bytes_hint, &streams_dir)
            .await?;
        log_audio_timing(video_id, "stream-tuberia-yt-dlp", pipe_started);

        if let Ok(mut cache) = self.stream_cache.write() {
            cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
            cache.insert(video_id.to_string(), (Instant::now(), resolved.clone()));
        }
        Ok(resolved)
        }
    }

    pub async fn download_with_ytdlp_fallback(&self, video_id: &str) -> Result<StreamInfo, String> {
        validate_video_id(video_id)?;
        let streams_dir = Self::streams_dir()?;
        if let Some(cached) = audio_cache::find(video_id, &streams_dir) {
            return Ok(cached);
        }

        let download_lock = self.audio_lock(video_id, AudioWork::Download)?;
        let _guard = download_lock.lock().await;
        if let Some(cached) = audio_cache::find(video_id, &streams_dir) {
            return Ok(cached);
        }
        self.run_ytdlp_download(video_id, &streams_dir).await
    }

    #[cfg(target_os = "android")]
    async fn run_ytdlp_download(
        &self,
        _video_id: &str,
        _streams_dir: &std::path::Path,
    ) -> Result<StreamInfo, String> {
        Err("La descarga de respaldo con yt-dlp no está disponible en Android.".to_string())
    }

    #[cfg(not(target_os = "android"))]
    async fn run_ytdlp_download(
        &self,
        video_id: &str,
        streams_dir: &std::path::Path,
    ) -> Result<StreamInfo, String> {
        if let Some(cached) = audio_cache::find(video_id, streams_dir) {
            return Ok(cached);
        }

        let staging = tempfile::tempdir_in(streams_dir)
            .map_err(|e| format!("Error al preparar descarga de audio: {e}"))?;

        let mut command = crate::audio_streamer::ytdlp_command();
        command
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-progress",
                "--socket-timeout",
                "5",
                "--retries",
                "1",
                "--concurrent-fragments",
                "4",
                "--extractor-args",
                "youtube:player_client=default,-tv_samsung;skip=hls,dash",
                "--js-runtimes",
                "node",
                "--remote-components",
                "ejs:github",
            ])
            .args([
                "-f",
                if cfg!(feature = "native-audio") {
                    "141/140/139/bestaudio[ext=m4a]"
                } else {
                    "251/140/bestaudio"
                },
                "-o",
            ])
            .arg(staging.path().join("audio.%(ext)s"))
            .arg("--")
            .arg(format!("https://music.youtube.com/watch?v={video_id}"));
        let started = Instant::now();
        let result = tokio::time::timeout(Duration::from_secs(90), command.output()).await;
        log_audio_timing(video_id, "descarga-de-respaldo", started);
        let output = result
            .map_err(|_| String::from("Tiempo de espera agotado en yt-dlp"))?
            .map_err(|e| format!("Error al ejecutar yt-dlp: {e}"))?;

        if !output.status.success() {
            eprintln!(
                "yt-dlp ({video_id}): {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(String::from("yt-dlp no pudo descargar el audio."));
        }

        let (downloaded, extension) = std::fs::read_dir(staging.path())
            .ok()
            .and_then(|entries| {
                entries.filter_map(Result::ok).find_map(|entry| {
                    let name = entry.file_name();
                    let extension = name.to_str()?.strip_prefix("audio.")?;
                    (audio_cache::audio_extension(extension)
                        && entry.metadata().is_ok_and(|m| m.is_file() && m.len() > 0))
                        .then(|| (entry.path(), extension.to_owned()))
                })
            })
            .ok_or_else(|| String::from("yt-dlp no generó archivo de audio."))?;
        let stream = audio_cache::finish(streams_dir, video_id, downloaded, &extension, staging);
        if let Ok(mut guard) = self.stream_cache.write() {
            guard.remove(video_id);
        }
        Ok(stream)
    }

    pub async fn prefetch_audio(&self, track: &Track) {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let _ = self.resolve_track_stream(track).await;
    }

}
