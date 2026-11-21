use std::sync::Arc;
use std::sync::atomic::Ordering;

use gyrotube_core::models::{
    SessionState, Track, UserPlaylist, load_favorites, load_recent_tracks, load_search_history,
    load_session_state, load_user_playlists, save_favorites, save_recent_tracks,
    save_search_history, save_session_state, save_user_playlists,
};
use serde::Serialize;
use tauri::State;

use super::AppCoreState;

#[derive(Serialize)]
pub struct InitialAppData {
    pub session: SessionState,
    pub favorites: Vec<Track>,
    pub recent_tracks: Vec<Track>,
    pub user_playlists: Vec<UserPlaylist>,
    pub search_history: Vec<String>,
}

#[tauri::command]
pub async fn load_initial_state(state: State<'_, AppCoreState>) -> Result<InitialAppData, String> {
    let bg_flag = Arc::clone(&state.background_playback);
    tokio::task::spawn_blocking(move || {
        let session = load_session_state();
        bg_flag.store(session.background_playback, Ordering::Relaxed);
        InitialAppData {
            session,
            favorites: load_favorites(),
            recent_tracks: load_recent_tracks(),
            user_playlists: load_user_playlists(),
            search_history: load_search_history(),
        }
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_session(
    state: State<'_, AppCoreState>,
    session: SessionState,
) -> Result<(), String> {
    state
        .background_playback
        .store(session.background_playback, Ordering::Relaxed);
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
