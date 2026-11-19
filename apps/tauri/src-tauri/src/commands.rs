use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use gyrotube_core::audio_cache::LocalAudio;
use gyrotube_core::backend::{Backend, HomeFeed};
use gyrotube_core::jam::{
    JamRequest, JamServerHandle, JamSnapshot, JamTrackInfo, generate_qr_svg, start_jam_server,
};
use gyrotube_core::models::{
    ArtistDetails, Genre, LyricsData, Playlist, SearchResults, SessionState, SyncedLyricLine,
    Track, UserPlaylist, cache_dir, load_favorites, load_recent_tracks, load_search_history,
    load_session_state, load_user_playlists, save_favorites, save_recent_tracks,
    save_search_history, save_session_state, save_user_playlists,
};
use gyrotube_core::spin_remote::{
    GuestSession, HostPublisher, RemoteRole, RemoteSyncPayload, RemoteUpdate, local_display_name,
    normalize_room_code, now_ms, start_guest_sync, ticket_qr_svg,
};
use serde::Serialize;
use tauri::State;

pub struct SpinState {
    pub snapshot: Arc<RwLock<JamSnapshot>>,
    pub server: Option<JamServerHandle>,
    pub cmd_tx: async_channel::Sender<JamRequest>,
    pub remote_role: RemoteRole,
    pub host_publisher: Option<HostPublisher>,
    pub guest_session: Option<GuestSession>,
    pub remote_tx: async_channel::Sender<RemoteUpdate>,
}

pub struct AppCoreState {
    pub backend: Backend,
    pub active_audio_lease: Mutex<Option<Arc<LocalAudio>>>,
    pub spin: Mutex<SpinState>,
    pub background_playback: Arc<AtomicBool>,
    pub quitting: Arc<AtomicBool>,
}

impl AppCoreState {
    pub fn new() -> (
        Self,
        async_channel::Receiver<JamRequest>,
        async_channel::Receiver<RemoteUpdate>,
    ) {
        let (cmd_tx, cmd_rx) = async_channel::unbounded();
        let (remote_tx, remote_rx) = async_channel::unbounded();
        let state = Self {
            backend: Backend::new(),
            active_audio_lease: Mutex::new(None),
            spin: Mutex::new(SpinState {
                snapshot: Arc::new(RwLock::new(JamSnapshot::default())),
                server: None,
                cmd_tx,
                remote_role: RemoteRole::Idle,
                host_publisher: None,
                guest_session: None,
                remote_tx,
            }),
            background_playback: Arc::new(AtomicBool::new(true)),
            quitting: Arc::new(AtomicBool::new(false)),
        };
        (state, cmd_rx, remote_rx)
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind")]
pub enum SpinCommandEvent {
    #[serde(rename = "add_track")]
    AddTrack { video_id: String, guest_name: String },
    #[serde(rename = "toggle_play_pause")]
    TogglePlayPause,
    #[serde(rename = "next_track")]
    NextTrack,
    #[serde(rename = "prev_track")]
    PrevTrack,
    #[serde(rename = "move_track")]
    MoveTrack { from: usize, to: usize },
}

#[derive(Clone, Serialize)]
pub struct SpinRemoteSyncEvent {
    pub target: String,
    pub payload: Option<RemoteSyncPayload>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct InitialAppData {
    pub session: SessionState,
    pub favorites: Vec<Track>,
    pub recent_tracks: Vec<Track>,
    pub user_playlists: Vec<UserPlaylist>,
    pub search_history: Vec<String>,
}

#[derive(Serialize)]
pub struct StreamPayload {
    pub url: String,
    pub quality_label: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum LyricsPayload {
    Synced { lines: Vec<SyncedLyricLine> },
    Plain { text: String },
    None,
}

#[derive(Serialize)]
pub struct SpinStatusPayload {
    pub local_active: bool,
    pub local_url: String,
    pub local_qr_svg: Option<String>,
    pub allow_controls: bool,
    pub guest_count: usize,
    pub added_count: usize,
    pub remote_role: &'static str,
    pub remote_ticket: Option<String>,
    pub remote_qr_svg: Option<String>,
}

fn build_spin_payload(spin: &SpinState) -> SpinStatusPayload {
    let snap = spin.snapshot.read().ok();
    let local_active = spin.server.is_some();
    let local_url = snap.as_ref().map(|n| n.url.clone()).unwrap_or_default();
    let local_qr_svg = if local_active && !local_url.is_empty() {
        generate_qr_svg(&local_url)
    } else {
        None
    };
    let (remote_role, remote_ticket, remote_qr_svg) = match &spin.remote_role {
        RemoteRole::Idle => ("idle", None, None),
        RemoteRole::Host(code) => {
            let ticket = format!("GYRO-{code}");
            let qr = ticket_qr_svg(&ticket);
            ("host", Some(ticket), qr)
        }
        RemoteRole::Guest(target) => ("guest", Some(target.clone()), None),
    };
    SpinStatusPayload {
        local_active,
        local_url,
        local_qr_svg,
        allow_controls: snap.as_ref().map(|n| n.allow_controls).unwrap_or(false),
        guest_count: snap.as_ref().map(|n| n.guest_count).unwrap_or(0),
        added_count: snap.as_ref().map(|n| n.added_count).unwrap_or(0),
        remote_role,
        remote_ticket,
        remote_qr_svg,
    }
}

#[tauri::command]
pub fn load_initial_state(state: State<'_, AppCoreState>) -> InitialAppData {
    let session = load_session_state();
    state
        .background_playback
        .store(session.background_playback, Ordering::Relaxed);
    InitialAppData {
        session,
        favorites: load_favorites(),
        recent_tracks: load_recent_tracks(),
        user_playlists: load_user_playlists(),
        search_history: load_search_history(),
    }
}

#[tauri::command]
pub fn save_session(state: State<'_, AppCoreState>, session: SessionState) -> Result<(), String> {
    state
        .background_playback
        .store(session.background_playback, Ordering::Relaxed);
    save_session_state(&session).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_favorites_cmd(favorites: Vec<Track>) -> Result<(), String> {
    save_favorites(&favorites).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_recent_tracks_cmd(tracks: Vec<Track>) -> Result<(), String> {
    save_recent_tracks(&tracks).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_user_playlists_cmd(playlists: Vec<UserPlaylist>) -> Result<(), String> {
    save_user_playlists(&playlists).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_search_history_cmd(history: Vec<String>) -> Result<(), String> {
    save_search_history(&history).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_home_feed(
    state: State<'_, AppCoreState>,
    force_refresh: bool,
) -> Result<HomeFeed, String> {
    state.backend.fetch_home_feed_cached(force_refresh).await
}

#[tauri::command]
pub async fn get_genres(
    state: State<'_, AppCoreState>,
    force_refresh: bool,
) -> Result<Vec<Genre>, String> {
    state.backend.fetch_genres_cached(force_refresh).await
}

#[tauri::command]
pub async fn get_genre_playlists(
    state: State<'_, AppCoreState>,
    genre_id: String,
) -> Result<(String, Vec<Playlist>), String> {
    state.backend.fetch_genre_playlists(&genre_id).await
}

#[tauri::command]
pub async fn search_all(
    state: State<'_, AppCoreState>,
    query: String,
) -> Result<SearchResults, String> {
    state.backend.search_all(&query).await
}

#[tauri::command]
pub async fn get_search_suggestions(
    state: State<'_, AppCoreState>,
    query: String,
) -> Result<Vec<String>, String> {
    state.backend.search_suggestions(&query).await
}

#[tauri::command]
pub async fn get_artist(
    state: State<'_, AppCoreState>,
    artist_id: String,
    fallback_name: String,
) -> Result<ArtistDetails, String> {
    match state.backend.fetch_artist(&artist_id).await {
        Ok(mut details) => {
            if details.name.trim().is_empty() {
                details.name = fallback_name;
            }
            Ok(details)
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn get_album_tracks(
    state: State<'_, AppCoreState>,
    album_id: String,
) -> Result<(String, Vec<Track>), String> {
    state.backend.fetch_album_tracks(&album_id).await
}

#[tauri::command]
pub async fn get_playlist_tracks(
    state: State<'_, AppCoreState>,
    playlist_id: String,
) -> Result<(String, Vec<Track>), String> {
    state.backend.fetch_playlist_tracks(&playlist_id).await
}

#[tauri::command]
pub async fn get_radio_tracks(
    state: State<'_, AppCoreState>,
    track_id: String,
) -> Result<Vec<Track>, String> {
    state.backend.fetch_radio_tracks(&track_id).await
}

#[tauri::command]
pub async fn get_lyrics(
    state: State<'_, AppCoreState>,
    title: String,
    artist: String,
    duration_secs: Option<u32>,
) -> Result<LyricsPayload, String> {
    let stub = Track {
        is_music: true,
        id: String::new(),
        title,
        artist,
        album: None,
        duration_secs,
        cover_url: None,
        cover_large_url: None,
    };
    match state.backend.fetch_synced_lyrics(&stub).await {
        Ok(LyricsData::Synced(lines)) => Ok(LyricsPayload::Synced { lines }),
        Ok(LyricsData::Plain(text)) => Ok(LyricsPayload::Plain { text }),
        Err(_) => Ok(LyricsPayload::None),
    }
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
        let streamer = gyrotube_core::audio_streamer::AudioStreamer::global().await?;
        streamer.serve_file_url(&track.id, &info.url)?
    } else {
        let streamer = gyrotube_core::audio_streamer::AudioStreamer::global().await?;
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
pub async fn prefetch_audio(state: State<'_, AppCoreState>, track: Track) -> Result<(), String> {
    state.backend.prefetch_audio(&track).await;
    Ok(())
}

#[tauri::command]
pub fn cancel_stale_streams(state: State<'_, AppCoreState>, keep_ids: Vec<String>) {
    let refs: Vec<&str> = keep_ids.iter().map(String::as_str).collect();
    state.backend.cancel_stale_streams(&refs);
}

#[tauri::command]
pub fn get_cache_bytes() -> u64 {
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
}

#[tauri::command]
pub fn clear_audio_cache() -> Result<(), String> {
    if let Some(dir) = cache_dir().map(|d| d.join("streams"))
        && let Ok(entries) = std::fs::read_dir(dir)
    {
        for entry in entries.flatten() {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    Ok(())
}

#[tauri::command]
pub fn spin_status(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let spin = state.spin.lock().map_err(|e| e.to_string())?;
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_start_local(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    if spin.server.is_none() {
        let handle = start_jam_server(
            state.backend.clone(),
            Arc::clone(&spin.snapshot),
            spin.cmd_tx.clone(),
        )?;
        let url = handle.url.clone();
        if let Ok(mut sn) = spin.snapshot.write() {
            sn.active = true;
            sn.url = url;
            sn.host_name = local_display_name();
            sn.guests.clear();
            sn.guest_list.clear();
            sn.guest_count = 0;
            sn.added_count = 0;
        }
        spin.server = Some(handle);
    }
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_stop_local(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    if let Some(mut srv) = spin.server.take() {
        srv.stop();
    }
    if let Ok(mut sn) = spin.snapshot.write() {
        sn.active = false;
    }
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_set_allow_controls(
    state: State<'_, AppCoreState>,
    allow: bool,
) -> Result<SpinStatusPayload, String> {
    let spin = state.spin.lock().map_err(|e| e.to_string())?;
    if let Ok(mut sn) = spin.snapshot.write() {
        sn.allow_controls = allow;
    }
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub async fn spin_start_remote_host(
    state: State<'_, AppCoreState>,
) -> Result<SpinStatusPayload, String> {
    let cmd_tx = {
        let spin = state.spin.lock().map_err(|e| e.to_string())?;
        spin.cmd_tx.clone()
    };
    let publisher = HostPublisher::new(cmd_tx)
        .await
        .map_err(|e| e.to_string())?;
    let code = publisher.invitation.clone();
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    spin.guest_session = None;
    spin.host_publisher = Some(publisher);
    spin.remote_role = RemoteRole::Host(code);
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_stop_remote(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    spin.host_publisher = None;
    spin.guest_session = None;
    spin.remote_role = RemoteRole::Idle;
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_join_remote(
    state: State<'_, AppCoreState>,
    code: String,
) -> Result<SpinStatusPayload, String> {
    let target = normalize_room_code(&code)?;
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    spin.host_publisher = None;
    spin.remote_role = RemoteRole::Guest(target.clone());
    let tx = spin.remote_tx.clone();
    spin.guest_session = Some(start_guest_sync(target, tx));
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_sync_snapshot(
    state: State<'_, AppCoreState>,
    is_playing: bool,
    position_secs: f64,
    source_name: String,
    current_index: Option<usize>,
    queue: Vec<Track>,
) -> Result<(), String> {
    let spin = state.spin.lock().map_err(|e| e.to_string())?;
    let current_track = current_index.and_then(|idx| queue.get(idx)).cloned();
    let jam_queue: Vec<JamTrackInfo> = queue
        .iter()
        .enumerate()
        .map(|(i, t)| JamTrackInfo {
            id: t.id.clone(),
            title: t.title.clone(),
            artist: t.artist.clone(),
            album: t.album.clone(),
            duration_secs: t.duration_secs,
            cover_url: t.cover_url.clone(),
            is_current: current_index == Some(i),
        })
        .collect();
    let now_playing = current_index.and_then(|idx| jam_queue.get(idx).cloned());

    let allow_controls = if let Ok(mut sn) = spin.snapshot.write() {
        sn.is_playing = is_playing;
        sn.position_secs = position_secs;
        sn.now_playing = now_playing;
        sn.queue = jam_queue;
        sn.allow_controls
    } else {
        false
    };

    if let Some(publisher) = &spin.host_publisher {
        publisher.publish(RemoteSyncPayload {
            track: current_track,
            position_secs,
            is_playing,
            sent_at_ms: now_ms(),
            allow_controls,
            source_name,
            host_name: local_display_name(),
        });
    }
    Ok(())
}
