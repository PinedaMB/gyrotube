use std::path::PathBuf;

use gyrotube_core::audio_streamer::AudioStreamer;
use gyrotube_core::mix_cache::{self, TrackMixProfile};
use gyrotube_core::models::{Track, VideoStreamInfo, cache_dir};
use gyrotube_core::native_audio::analyzer::DEFAULT_WAVEFORM_PEAKS;
use gyrotube_core::native_audio::{
    DeckId, DecoderSource, EngineCommand, EqParams, TrackWaveformAnalysis, analyze_audio_file,
    spawn_deck_decoder,
};
use serde::Serialize;
use tauri::State;

use super::AppCoreState;

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
        "[gyrotube-tauri] audio stream resolved: id={} quality={} url={}",
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
        "[gyrotube-tauri] video stream resolved: id={} quality={} url={}",
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
pub async fn prefetch_audio(
    state: State<'_, AppCoreState>,
    track: Track,
) -> Result<Option<String>, String> {
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
    Ok(Some(stream_url))
}

#[tauri::command]
pub async fn get_mix_profile(track_id: String) -> Option<TrackMixProfile> {
    tokio::task::spawn_blocking(move || mix_cache::get_profile(&track_id))
        .await
        .ok()
        .flatten()
}

#[tauri::command]
pub async fn save_mix_profile(
    track_id: String,
    profile: TrackMixProfile,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || mix_cache::save_profile(&track_id, profile))
        .await
        .map_err(|e| e.to_string())?
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

#[derive(Serialize)]
pub struct NativeLoadResult {
    pub stream_url: String,
    pub quality_label: String,
    pub duration_secs: f64,
}

async fn resolve_decoder_source_for_url(
    url: &str,
    local_path_fallback: Option<PathBuf>,
) -> Result<(DecoderSource, Option<PathBuf>), String> {
    if let Some(streamer) = AudioStreamer::try_global()
        && let Some((path, mime, progress_rx)) = streamer.lookup_session_source(url)
    {
        let ext_hint = if mime.contains("mp4") || mime.contains("m4a") {
            Some("m4a".to_string())
        } else if mime.contains("mpeg") || mime.contains("mp3") {
            Some("mp3".to_string())
        } else if mime.contains("ogg") {
            Some("ogg".to_string())
        } else if mime.contains("flac") {
            Some("flac".to_string())
        } else {
            path.extension()
                .and_then(|s| s.to_str())
                .map(str::to_ascii_lowercase)
        };
        let is_done = progress_rx.borrow().done;
        let source = if is_done {
            DecoderSource::LocalFile {
                path: path.clone(),
                ext_hint,
            }
        } else {
            DecoderSource::ProgressiveFile {
                path: path.clone(),
                ext_hint,
                progress_rx,
            }
        };
        return Ok((source, Some(path)));
    }

    if let Some(path) = local_path_fallback {
        let ext_hint = path
            .extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase);
        return Ok((
            DecoderSource::LocalFile {
                path: path.clone(),
                ext_hint,
            },
            Some(path),
        ));
    }

    Err(format!("No local or progressive source available for {url}"))
}

fn file_url_to_path(file_url: &str) -> Option<PathBuf> {
    file_url.strip_prefix("file://").map(PathBuf::from)
}

#[tauri::command]
pub async fn native_load_deck(
    state: State<'_, AppCoreState>,
    deck: DeckId,
    track: Track,
    start_secs: Option<f64>,
    autoplay: Option<bool>,
) -> Result<NativeLoadResult, String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard
            .controller
            .clone()
            .ok_or_else(|| "Native audio output device unavailable".to_string())?
    };

    let info = state.backend.resolve_track_stream(&track).await?;
    let (stream_url, local_path_fallback): (String, Option<PathBuf>) =
        if info.url.starts_with("http://") || info.url.starts_with("https://") {
            (info.url.clone(), None)
        } else if info.url.starts_with("file://") {
            let streamer = AudioStreamer::global().await?;
            let url = streamer.serve_file_url(&track.id, &info.url)?;
            (url, file_url_to_path(&info.url))
        } else {
            let path = PathBuf::from(&info.url);
            let streamer = AudioStreamer::global().await?;
            let url = streamer.serve_local_file(&track.id, path.clone())?;
            (url, Some(path))
        };

    let (source, resolved_path) =
        resolve_decoder_source_for_url(&stream_url, local_path_fallback).await?;
    let device_sr = controller.sample_rate();
    let seek_secs = start_secs.unwrap_or(0.0).max(0.0);
    let want_autoplay = autoplay.unwrap_or(true);
    let bpm_hint = mix_cache::get_profile(&track.id)
        .and_then(|p| p.bpm)
        .unwrap_or(120.0) as f32;

    let decoded = tokio::task::spawn_blocking(move || {
        spawn_deck_decoder(deck, source, device_sr, seek_secs)
    })
    .await
    .map_err(|e| e.to_string())??;

    {
        let mut guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        let slot = guard.slot_mut(deck);
        if let Some(old) = slot.decoder.take() {
            old.cancel();
        }
        slot.decoder = Some(decoded.handle);
        slot.track_id = Some(track.id.clone());
        slot.stream_url = Some(stream_url.clone());
        slot.local_path = resolved_path;
        slot.lease = info.local_audio;
        slot.bpm = bpm_hint;
    }

    controller
        .send(EngineCommand::AttachDeckConsumer {
            deck,
            consumer: decoded.consumer,
            total_frames: decoded.total_frames,
            start_frame: decoded.start_frame,
            bpm: bpm_hint,
            autoplay: want_autoplay,
        })
        .map_err(|e| e.to_string())?;

    if want_autoplay {
        state.tick_notify.notify_one();
    }

    Ok(NativeLoadResult {
        stream_url,
        quality_label: info.quality_label,
        duration_secs: decoded.duration_secs,
    })
}

#[tauri::command]
pub async fn native_play(state: State<'_, AppCoreState>, deck: DeckId) -> Result<(), String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard
            .controller
            .clone()
            .ok_or_else(|| "Native audio output device unavailable".to_string())?
    };
    controller
        .send(EngineCommand::Play(deck))
        .map_err(|e| e.to_string())?;
    state.tick_notify.notify_one();
    Ok(())
}

#[tauri::command]
pub async fn native_pause(state: State<'_, AppCoreState>, deck: DeckId) -> Result<(), String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard
            .controller
            .clone()
            .ok_or_else(|| "Native audio output device unavailable".to_string())?
    };
    controller
        .send(EngineCommand::Pause(deck))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn native_stop(
    state: State<'_, AppCoreState>,
    deck: Option<DeckId>,
) -> Result<(), String> {
    let controller = {
        let mut guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        match deck {
            Some(d) => {
                let slot = guard.slot_mut(d);
                if let Some(old) = slot.decoder.take() {
                    old.cancel();
                }
            }
            None => {
                if let Some(old) = guard.deck_a.decoder.take() {
                    old.cancel();
                }
                if let Some(old) = guard.deck_b.decoder.take() {
                    old.cancel();
                }
            }
        }
        guard.controller.clone()
    };
    if let Some(ctrl) = controller {
        match deck {
            Some(d) => ctrl.send(EngineCommand::Stop(d)).map_err(|e| e.to_string())?,
            None => {
                let _ = ctrl.send(EngineCommand::Stop(DeckId::A));
                let _ = ctrl.send(EngineCommand::Stop(DeckId::B));
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn native_seek(
    state: State<'_, AppCoreState>,
    deck: DeckId,
    position_secs: f64,
) -> Result<(), String> {
    let (controller, stream_url, local_path, bpm, was_playing) = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        let ctrl = guard
            .controller
            .clone()
            .ok_or_else(|| "Native audio output device unavailable".to_string())?;
        let snap = ctrl.telemetry_snapshot();
        let playing = match deck {
            DeckId::A => snap.deck_a_playing,
            DeckId::B => snap.deck_b_playing,
        };
        let slot = match deck {
            DeckId::A => &guard.deck_a,
            DeckId::B => &guard.deck_b,
        };
        (
            ctrl,
            slot.stream_url.clone(),
            slot.local_path.clone(),
            slot.bpm,
            playing,
        )
    };

    let url = stream_url.ok_or_else(|| "No active stream loaded on deck".to_string())?;
    let (source, _) = resolve_decoder_source_for_url(&url, local_path).await?;
    let device_sr = controller.sample_rate();
    let target = position_secs.max(0.0);

    let decoded = tokio::task::spawn_blocking(move || {
        spawn_deck_decoder(deck, source, device_sr, target)
    })
    .await
    .map_err(|e| e.to_string())??;

    {
        let mut guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        let slot = guard.slot_mut(deck);
        if let Some(old) = slot.decoder.take() {
            old.cancel();
        }
        slot.decoder = Some(decoded.handle);
    }

    controller
        .send(EngineCommand::AttachDeckConsumer {
            deck,
            consumer: decoded.consumer,
            total_frames: decoded.total_frames,
            start_frame: decoded.start_frame,
            bpm,
            autoplay: was_playing,
        })
        .map_err(|e| e.to_string())?;
    state.tick_notify.notify_one();
    Ok(())
}

#[tauri::command]
pub async fn native_set_volume(state: State<'_, AppCoreState>, volume: f32) -> Result<(), String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard.controller.clone()
    };
    if let Some(ctrl) = controller {
        ctrl.set_volume(volume).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn native_set_eq(state: State<'_, AppCoreState>, params: EqParams) -> Result<(), String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard.controller.clone()
    };
    if let Some(ctrl) = controller {
        ctrl.set_eq(params).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn native_trigger_mix(
    state: State<'_, AppCoreState>,
    from: DeckId,
    to: DeckId,
    duration_secs: f32,
    from_target_rate: f64,
    to_initial_rate: f64,
    enable_bass_swap: bool,
    enable_echo: bool,
    bridge_bpm: f32,
) -> Result<(), String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard
            .controller
            .clone()
            .ok_or_else(|| "Native audio output device unavailable".to_string())?
    };
    controller
        .send(EngineCommand::StartTransition {
            from,
            to,
            duration_secs,
            from_target_rate,
            to_initial_rate,
            enable_bass_swap,
            enable_echo,
            bridge_bpm,
        })
        .map_err(|e| e.to_string())?;
    state.tick_notify.notify_one();
    Ok(())
}

#[tauri::command]
pub async fn native_cancel_mix(state: State<'_, AppCoreState>) -> Result<(), String> {
    let controller = {
        let guard = state
            .native_audio
            .lock()
            .map_err(|_| "Native audio lock poisoned".to_string())?;
        guard.controller.clone()
    };
    if let Some(ctrl) = controller {
        ctrl.send(EngineCommand::CancelTransition)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_track_waveform(
    state: State<'_, AppCoreState>,
    track: Track,
    num_peaks: Option<usize>,
) -> Result<TrackWaveformAnalysis, String> {
    let peaks_count = num_peaks.unwrap_or(DEFAULT_WAVEFORM_PEAKS);
    if let Some(cached) = mix_cache::get_profile(&track.id)
        && cached.version >= 3
        && !cached.peaks.is_empty()
    {
        return Ok(TrackWaveformAnalysis {
            track_id: track.id,
            duration_secs: cached.duration_secs,
            peaks: cached.peaks,
            bpm: cached.bpm,
            bpm_confidence: cached.bpm_confidence,
            beat_offset_secs: cached.beat_offset_secs,
            intro_cue_secs: cached.intro_end_secs,
            outro_cue_secs: cached.outro_start_secs,
        });
    }

    let info = state.backend.resolve_track_stream(&track).await?;
    let path_to_analyze = if info.url.starts_with("http://") || info.url.starts_with("https://") {
        let streamer = AudioStreamer::global().await?;
        let (path, _, mut rx) = streamer
            .lookup_session_source(&info.url)
            .ok_or_else(|| "Active stream session not found for waveform analysis".to_string())?;
        // Wait for progressive download to finish before full-track waveform/BPM scan
        while !rx.borrow().done {
            if rx.changed().await.is_err() {
                break;
            }
        }
        path
    } else if info.url.starts_with("file://") {
        file_url_to_path(&info.url)
            .ok_or_else(|| "Invalid local file URL for waveform analysis".to_string())?
    } else {
        PathBuf::from(&info.url)
    };

    let track_id = track.id.clone();
    let analysis = tokio::task::spawn_blocking(move || {
        analyze_audio_file(&track_id, &path_to_analyze, peaks_count)
    })
    .await
    .map_err(|e| e.to_string())??;

    let profile = TrackMixProfile {
        version: 3,
        duration_secs: analysis.duration_secs,
        intro_end_secs: analysis.intro_cue_secs,
        outro_start_secs: analysis.outro_cue_secs,
        bpm: analysis.bpm,
        bpm_confidence: analysis.bpm_confidence,
        beat_offset_secs: analysis.beat_offset_secs,
        peaks: analysis.peaks.clone(),
        analyzed_at: 0,
    };
    let _ = mix_cache::save_profile(&track.id, profile);

    Ok(analysis)
}
