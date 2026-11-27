use gyrotube_core::audio_streamer::AudioStreamer;
use gyrotube_core::models::{LastFmCreds, SponsorSegment, Track, VideoStreamInfo, cache_dir};
use serde::Serialize;
use tauri::State;

use super::{AppCoreState, sanitize_export_filename};

const MAX_LYRICS_PNG_BYTES: usize = 20 * 1024 * 1024;

#[derive(Serialize)]
pub struct StreamPayload {
    pub url: String,
    pub quality_label: String,
}

#[tauri::command]
pub async fn resolve_audio_stream(
    state: State<'_, AppCoreState>,
    track: Track,
) -> Result<StreamPayload, String> {
    let info = state.backend.resolve_track_stream(&track).await?;
    let stream_url = if info.url.starts_with("http://") || info.url.starts_with("https://") {
        info.url
    } else if info.url.starts_with("file://") {
        let streamer = AudioStreamer::global().await?;
        streamer.serve_file_url(&track.id, &info.url)?
    } else {
        let streamer = AudioStreamer::global().await?;
        streamer.serve_local_file(&track.id, std::path::PathBuf::from(&info.url))?
    };
    eprintln!(
        "[gyro] audio stream resolved: id={} quality={} url={}",
        track.id, info.quality_label, stream_url
    );
    if let Ok(mut lease) = state.active_audio_lease.lock() {
        *lease = info.local_audio;
    }
    Ok(StreamPayload {
        url: stream_url,
        quality_label: info.quality_label,
    })
}

#[tauri::command]
pub async fn resolve_video_stream(
    state: State<'_, AppCoreState>,
    track_id: String,
    max_height: Option<u32>,
) -> Result<VideoStreamInfo, String> {
    let info = state
        .backend
        .resolve_video_stream(&track_id, max_height.unwrap_or(720))
        .await?;
    eprintln!(
        "[gyro] video stream resolved: id={} quality={} url={}",
        track_id, info.quality_label, info.url
    );
    Ok(info)
}

#[tauri::command]
pub async fn find_counterpart_track(
    state: State<'_, AppCoreState>,
    track: Track,
    want_video: bool,
) -> Result<Option<Track>, String> {
    state
        .backend
        .find_counterpart_track(&track, want_video)
        .await
}

#[tauri::command]
pub async fn get_sponsorblock_segments(
    state: State<'_, AppCoreState>,
    track_id: String,
) -> Result<Vec<SponsorSegment>, String> {
    state.backend.fetch_sponsorblock_segments(&track_id).await
}

#[tauri::command]
pub async fn validate_listenbrainz_token(
    state: State<'_, AppCoreState>,
    token: String,
) -> Result<String, String> {
    state.backend.validate_listenbrainz_token(&token).await
}

#[tauri::command]
pub async fn scrobble_listenbrainz(
    state: State<'_, AppCoreState>,
    token: String,
    track: Track,
    now_playing: bool,
    timestamp: Option<u64>,
) -> Result<(), String> {
    state
        .backend
        .scrobble_listenbrainz(&token, &track, now_playing, timestamp)
        .await
}

#[tauri::command]
pub async fn scrobble_lastfm(
    state: State<'_, AppCoreState>,
    creds: LastFmCreds,
    track: Track,
    now_playing: bool,
    timestamp: Option<u64>,
) -> Result<(), String> {
    state
        .backend
        .scrobble_lastfm(&creds, &track, now_playing, timestamp)
        .await
}

#[tauri::command]
pub async fn prefetch_audio(state: State<'_, AppCoreState>, track: Track) -> Result<(), String> {
    state.backend.resolve_track_stream(&track).await?;
    Ok(())
}

#[tauri::command]
pub async fn cancel_stale_streams(
    state: State<'_, AppCoreState>,
    keep_ids: Vec<String>,
) -> Result<(), String> {
    let refs: Vec<&str> = keep_ids.iter().map(String::as_str).collect();
    state.backend.cancel_stale_streams(&refs);
    Ok(())
}

#[tauri::command]
pub async fn get_cache_bytes() -> u64 {
    tokio::task::spawn_blocking(|| {
        let Some(dir) = cache_dir().map(|d| d.join("streams")) else {
            return 0;
        };
        std::fs::read_dir(dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    })
    .await
    .unwrap_or(0)
}

#[tauri::command]
pub async fn clear_audio_cache() -> Result<(), String> {
    tokio::task::spawn_blocking(|| {
        if let Some(dir) = cache_dir().map(|d| d.join("streams"))
            && let Ok(entries) = std::fs::read_dir(dir)
        {
            for entry in entries.flatten() {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_downloaded_tracks() -> Vec<String> {
    tokio::task::spawn_blocking(gyrotube_core::backend::Backend::list_downloaded_track_ids)
        .await
        .unwrap_or_default()
}

#[tauri::command]
pub async fn download_track_offline(
    state: State<'_, AppCoreState>,
    track: Track,
) -> Result<String, String> {
    state.backend.download_track_offline(&track).await
}

#[tauri::command]
pub async fn remove_downloaded_tracks(track_ids: Vec<String>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        gyrotube_core::backend::Backend::remove_downloaded_tracks(&track_ids)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_downloads_bytes() -> u64 {
    tokio::task::spawn_blocking(gyrotube_core::backend::Backend::downloads_bytes)
        .await
        .unwrap_or(0)
}

#[tauri::command]
pub async fn clear_all_downloads() -> Result<(), String> {
    tokio::task::spawn_blocking(gyrotube_core::backend::Backend::clear_all_downloads)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_offline_cover_base_url() -> Result<String, String> {
    let streamer = AudioStreamer::global().await?;
    Ok(streamer.cover_base_url())
}

#[tauri::command]
pub async fn save_lyrics_card_png(
    app: tauri::AppHandle,
    filename: String,
    png_bytes: Vec<u8>,
) -> Result<String, String> {
    use tauri::Manager;

    if png_bytes.is_empty() {
        return Err("Empty PNG payload".to_string());
    }
    if png_bytes.len() > MAX_LYRICS_PNG_BYTES {
        return Err("PNG payload exceeds 20 MiB limit".to_string());
    }
    let final_name = sanitize_export_filename(&filename, "gyro-lyrics.png", ".png");

    let base_dir = app
        .path()
        .picture_dir()
        .or_else(|_| app.path().download_dir())
        .or_else(|_| app.path().document_dir())
        .map_err(|e| format!("No se pudo localizar la carpeta de imágenes: {e}"))?;
    let target_dir = base_dir.join("Gyro");
    tokio::fs::create_dir_all(&target_dir)
        .await
        .map_err(|e| format!("No se pudo crear la carpeta Gyro: {e}"))?;
    let target_path = target_dir.join(&final_name);
    tokio::fs::write(&target_path, png_bytes)
        .await
        .map_err(|e| format!("No se pudo guardar la imagen: {e}"))?;
    Ok(target_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn native_audio_load(
    state: State<'_, AppCoreState>,
    url: String,
    start_secs: f64,
    use_crossfade: bool,
    start_paused: bool,
) -> Result<(), String> {
    #[cfg(desktop)]
    {
        let streamer = AudioStreamer::global().await?;
        let source = streamer.open_native_source(&url)?;
        state
            .native_audio
            .load(source, start_secs, use_crossfade, start_paused)
    }
    #[cfg(not(desktop))]
    {
        let _ = (state, url, start_secs, use_crossfade, start_paused);
        Err("Native audio engine is desktop-only".into())
    }
}

#[tauri::command]
pub fn native_audio_play(state: State<'_, AppCoreState>) {
    #[cfg(desktop)]
    state.native_audio.play();
    #[cfg(not(desktop))]
    let _ = state;
}

#[tauri::command]
pub fn native_audio_pause(state: State<'_, AppCoreState>) {
    #[cfg(desktop)]
    state.native_audio.pause();
    #[cfg(not(desktop))]
    let _ = state;
}

#[tauri::command]
pub fn native_audio_stop(state: State<'_, AppCoreState>) {
    #[cfg(desktop)]
    state.native_audio.stop();
    #[cfg(not(desktop))]
    let _ = state;
}

#[tauri::command]
pub fn native_audio_seek(state: State<'_, AppCoreState>, secs: f64) {
    #[cfg(desktop)]
    state.native_audio.seek(secs);
    #[cfg(not(desktop))]
    let _ = (state, secs);
}

#[tauri::command]
pub fn native_audio_set_volume(state: State<'_, AppCoreState>, volume: f32) {
    #[cfg(desktop)]
    state.native_audio.set_volume(volume);
    #[cfg(not(desktop))]
    let _ = (state, volume);
}

#[tauri::command]
pub fn native_audio_set_crossfade(state: State<'_, AppCoreState>, secs: f64) {
    #[cfg(desktop)]
    state.native_audio.set_crossfade_secs(secs);
    #[cfg(not(desktop))]
    let _ = (state, secs);
}

#[tauri::command]
pub fn native_audio_set_eq(
    state: State<'_, AppCoreState>,
    enabled: bool,
    normalize: bool,
    bands: [f32; 5],
) {
    #[cfg(desktop)]
    state.native_audio.set_eq(
        gyrotube_core::native_audio::NativeEqConfig {
            enabled,
            normalize,
            bands,
        },
    );
    #[cfg(not(desktop))]
    let _ = (state, enabled, normalize, bands);
}
