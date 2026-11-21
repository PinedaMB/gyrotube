use gyrotube_core::backend::HomeFeed;
use gyrotube_core::models::{
    ArtistDetails, Genre, LyricsData, Playlist, RelatedFeed, SearchResults, SyncedLyricLine, Track,
};
use serde::Serialize;
use tauri::State;

use super::AppCoreState;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum LyricsPayload {
    Synced { lines: Vec<SyncedLyricLine> },
    Plain { text: String },
    None,
}

#[tauri::command]
pub async fn get_home_feed(
    state: State<'_, AppCoreState>,
    force_refresh: bool,
    country: Option<String>,
) -> Result<HomeFeed, String> {
    state
        .backend
        .fetch_home_feed_for_country(country.as_deref(), force_refresh)
        .await
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
pub async fn get_playlist_radio(
    state: State<'_, AppCoreState>,
    playlist_id: String,
) -> Result<Vec<Track>, String> {
    state.backend.fetch_playlist_radio(&playlist_id).await
}

#[tauri::command]
pub async fn get_related_for_track(
    state: State<'_, AppCoreState>,
    track_id: String,
) -> Result<RelatedFeed, String> {
    state.backend.fetch_related_for_track(&track_id).await
}

#[tauri::command]
pub async fn get_lyrics(
    state: State<'_, AppCoreState>,
    track_id: Option<String>,
    title: String,
    artist: String,
    duration_secs: Option<u32>,
) -> Result<LyricsPayload, String> {
    let stub = Track {
        is_music: true,
        id: track_id.unwrap_or_default(),
        title,
        artist,
        artist_id: None,
        album: None,
        album_id: None,
        duration_secs,
        view_count: None,
        cover_url: None,
        cover_large_url: None,
    };
    match state.backend.fetch_synced_lyrics(&stub).await {
        Ok(LyricsData::Synced(lines)) => Ok(LyricsPayload::Synced { lines }),
        Ok(LyricsData::Plain(text)) => Ok(LyricsPayload::Plain { text }),
        Err(_) => Ok(LyricsPayload::None),
    }
}
