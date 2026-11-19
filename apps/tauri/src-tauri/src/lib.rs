mod commands;

use gyrotube_core::backend::Backend;
use gyrotube_core::models::init_storage_dirs;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (core_state, _jam_cmd_rx, _remote_rx) = commands::AppCoreState::new();

    tauri::Builder::default()
        .setup(|app| {
            if let (Ok(data_dir), Ok(cache_dir)) =
                (app.path().app_data_dir(), app.path().app_cache_dir())
            {
                init_storage_dirs(data_dir, cache_dir);
            }
            Backend::trim_audio_cache_on_startup();
            Ok(())
        })
        .manage(core_state)
        .invoke_handler(tauri::generate_handler![
            commands::load_initial_state,
            commands::save_session,
            commands::save_favorites_cmd,
            commands::save_recent_tracks_cmd,
            commands::save_user_playlists_cmd,
            commands::save_search_history_cmd,
            commands::get_home_feed,
            commands::get_genres,
            commands::get_genre_playlists,
            commands::search_all,
            commands::get_search_suggestions,
            commands::get_artist,
            commands::get_album_tracks,
            commands::get_playlist_tracks,
            commands::get_radio_tracks,
            commands::get_lyrics,
            commands::resolve_audio_stream,
            commands::prefetch_audio,
            commands::cancel_stale_streams,
            commands::get_cache_bytes,
            commands::clear_audio_cache,
            commands::spin_status,
            commands::spin_start_local,
            commands::spin_stop_local,
            commands::spin_set_allow_controls,
            commands::spin_start_remote_host,
            commands::spin_stop_remote,
            commands::spin_join_remote,
        ])
        .run(tauri::generate_context!())
        .expect("error while running GyroTube Tauri application");
}
