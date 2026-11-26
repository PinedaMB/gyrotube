use std::sync::Arc;
use std::sync::atomic::Ordering;

use gyrotube_core::i18n::set_language_preference;
use gyrotube_core::models::{
    Artist, SessionState, Track, UserPlaylist, load_favorites, load_followed_artists,
    load_recent_tracks, load_search_history, load_session_state, load_user_playlists,
    save_favorites, save_followed_artists, save_recent_tracks, save_search_history,
    save_session_state, save_user_playlists,
};
use serde::Serialize;
use tauri::State;

use super::{AppCoreState, sanitize_export_filename};

const MAX_BACKUP_JSON_BYTES: usize = 50 * 1024 * 1024;

#[derive(Serialize)]
pub struct InitialAppData {
    pub session: SessionState,
    pub favorites: Vec<Track>,
    pub recent_tracks: Vec<Track>,
    pub user_playlists: Vec<UserPlaylist>,
    pub search_history: Vec<String>,
    pub followed_artists: Vec<Artist>,
}

#[tauri::command]
pub async fn load_initial_state(
    app: tauri::AppHandle,
    state: State<'_, AppCoreState>,
) -> Result<InitialAppData, String> {
    let bg_flag = Arc::clone(&state.background_playback);
    let backend = state.backend.clone();
    let data = tokio::task::spawn_blocking(move || {
        let session = load_session_state();
        set_language_preference(session.language);
        bg_flag.store(session.background_playback, Ordering::Relaxed);
        backend.set_audio_quality(session.audio_quality);
        InitialAppData {
            session,
            favorites: load_favorites(),
            recent_tracks: load_recent_tracks(),
            user_playlists: load_user_playlists(),
            search_history: load_search_history(),
            followed_artists: load_followed_artists(),
        }
    })
    .await
    .map_err(|e| e.to_string())?;
    crate::refresh_tray_menu(&app);
    Ok(data)
}

#[tauri::command]
pub async fn save_session(
    app: tauri::AppHandle,
    state: State<'_, AppCoreState>,
    session: SessionState,
) -> Result<(), String> {
    set_language_preference(session.language);
    crate::refresh_tray_menu(&app);
    state
        .background_playback
        .store(session.background_playback, Ordering::Relaxed);
    state.backend.set_audio_quality(session.audio_quality);
    tokio::task::spawn_blocking(move || save_session_state(&session).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn save_favorites_cmd(favorites: Vec<Track>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || save_favorites(&favorites).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn save_recent_tracks_cmd(tracks: Vec<Track>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || save_recent_tracks(&tracks).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn save_user_playlists_cmd(playlists: Vec<UserPlaylist>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || save_user_playlists(&playlists).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn save_search_history_cmd(history: Vec<String>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || save_search_history(&history).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn save_followed_artists_cmd(artists: Vec<Artist>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || save_followed_artists(&artists).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn save_backup_json(
    app: tauri::AppHandle,
    filename: String,
    json_content: String,
) -> Result<String, String> {
    use tauri::Manager;

    if json_content.is_empty() {
        return Err("Empty backup payload".to_string());
    }
    if json_content.len() > MAX_BACKUP_JSON_BYTES {
        return Err("Backup payload exceeds 50 MiB limit".to_string());
    }
    let final_name = sanitize_export_filename(&filename, "gyro-library.json", ".json");

    let base_dir = app
        .path()
        .download_dir()
        .or_else(|_| app.path().document_dir())
        .or_else(|_| app.path().home_dir())
        .map_err(|e| format!("No se pudo localizar la carpeta de descargas: {e}"))?;
    tokio::fs::create_dir_all(&base_dir)
        .await
        .map_err(|e| format!("No se pudo crear la carpeta de descargas: {e}"))?;
    let target_path = base_dir.join(&final_name);
    tokio::fs::write(&target_path, json_content.as_bytes())
        .await
        .map_err(|e| format!("No se pudo guardar el respaldo: {e}"))?;
    Ok(target_path.to_string_lossy().to_string())
}
