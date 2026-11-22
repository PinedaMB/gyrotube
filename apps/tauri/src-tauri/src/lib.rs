mod commands;

#[cfg(desktop)]
use std::sync::atomic::Ordering;

use gyrotube_core::backend::Backend;
use gyrotube_core::jam::JamCommand;
use gyrotube_core::models::init_storage_dirs;
use gyrotube_core::spin_remote::RemoteRole;
#[cfg(desktop)]
use tauri::WindowEvent;
use tauri::{Emitter, Manager};

use commands::{AppCoreState, SpinCommandEvent, SpinRemoteSyncEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    gyrotube_core::tls::ensure_crypto_provider();

    tauri::Builder::default()
        .setup(move |app| {
            if let (Ok(data_dir), Ok(cache_dir)) =
                (app.path().app_data_dir(), app.path().app_cache_dir())
            {
                init_storage_dirs(data_dir, cache_dir);
            }
            Backend::trim_audio_cache_on_startup();

            // Construct core state AFTER init_storage_dirs so Android has valid writable dirs
            let (core_state, jam_cmd_rx, remote_rx) = AppCoreState::new();
            app.manage(core_state);

            // Bridge SPIN LAN & Remote host commands to the frontend via Tauri events
            let handle_for_jam = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Ok(req) = jam_cmd_rx.recv().await {
                    let is_valid = {
                        let core = handle_for_jam.state::<AppCoreState>();
                        core.spin
                            .lock()
                            .map(|spin| {
                                let lan_session = spin.server.as_ref().map(|s| s.session_id);
                                let host_room = match &spin.remote_role {
                                    RemoteRole::Host(code) => Some(code.as_str()),
                                    _ => None,
                                };
                                req.source.is_current(lan_session, host_room)
                            })
                            .unwrap_or(false)
                    };
                    if !is_valid {
                        continue;
                    }
                    let event = match req.command {
                        JamCommand::AddTrack(video_id, guest_name) => {
                            SpinCommandEvent::AddTrack { video_id, guest_name }
                        }
                        JamCommand::TogglePlayPause => SpinCommandEvent::TogglePlayPause,
                        JamCommand::NextTrack => SpinCommandEvent::NextTrack,
                        JamCommand::PrevTrack => SpinCommandEvent::PrevTrack,
                        JamCommand::MoveTrack(from, to) => SpinCommandEvent::MoveTrack { from, to },
                    };
                    let _ = handle_for_jam.emit("spin:command", event);
                }
            });

            // Bridge SPIN Remote guest sync updates to the frontend
            let handle_for_remote = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Ok(update) = remote_rx.recv().await {
                    let state = handle_for_remote.state::<AppCoreState>();
                    let current = state.spin.lock().is_ok_and(|spin| {
                        matches!(&spin.remote_role, RemoteRole::Guest(target) if *target == update.target)
                    });
                    if !current {
                        continue;
                    }
                    let event = match update.result {
                        Ok(payload) => SpinRemoteSyncEvent {
                            target: update.target,
                            payload: Some(payload),
                            error: None,
                        },
                        Err(err) => SpinRemoteSyncEvent {
                            target: update.target,
                            payload: None,
                            error: Some(err),
                        },
                    };
                    let _ = handle_for_remote.emit("spin:remote-sync", event);
                }
            });

            #[cfg(desktop)]
            {
                use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

                let show_i = MenuItem::with_id(app, "show", "Show GyroTube", true, None::<&str>)?;
                let play_i = MenuItem::with_id(app, "toggle_play", "Play / Pause", true, None::<&str>)?;
                let prev_i = MenuItem::with_id(app, "prev", "Previous", true, None::<&str>)?;
                let next_i = MenuItem::with_id(app, "next", "Next", true, None::<&str>)?;
                let sep_i = PredefinedMenuItem::separator(app)?;
                let quit_i = MenuItem::with_id(app, "quit", "Quit GyroTube", true, None::<&str>)?;

                let menu = Menu::with_items(
                    app,
                    &[&show_i, &play_i, &prev_i, &next_i, &sep_i, &quit_i],
                )?;

                let mut tray_builder = TrayIconBuilder::with_id("gyrotube-tray")
                    .menu(&menu)
                    .tooltip("GyroTube")
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "show" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.unminimize();
                                let _ = win.set_focus();
                            }
                        }
                        "toggle_play" | "prev" | "next" => {
                            let _ = app.emit("tray:action", event.id.as_ref());
                        }
                        "quit" => {
                            let core = app.state::<AppCoreState>();
                            core.quitting.store(true, Ordering::SeqCst);
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.unminimize();
                                let _ = win.set_focus();
                            }
                        }
                    });

                if let Some(icon) = app.default_window_icon() {
                    tray_builder = tray_builder.icon(icon.clone());
                }
                let _ = tray_builder.build(app);
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            #[cfg(desktop)]
            if let WindowEvent::CloseRequested { api, .. } = event {
                let core = window.state::<AppCoreState>();
                let quitting = core.quitting.load(Ordering::SeqCst);
                let bg = core.background_playback.load(Ordering::Relaxed);
                if !quitting && bg {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
            #[cfg(not(desktop))]
            let _ = (window, event);
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_initial_state,
            commands::save_session,
            commands::save_favorites_cmd,
            commands::save_recent_tracks_cmd,
            commands::save_user_playlists_cmd,
            commands::save_search_history_cmd,
            commands::save_followed_artists_cmd,
            commands::get_home_feed,
            commands::get_genres,
            commands::get_genre_playlists,
            commands::search_all,
            commands::get_search_suggestions,
            commands::get_artist,
            commands::get_album_tracks,
            commands::get_playlist_tracks,
            commands::get_radio_tracks,
            commands::get_playlist_radio,
            commands::get_related_for_track,
            commands::get_lyrics,
            commands::resolve_audio_stream,
            commands::resolve_video_stream,
            commands::find_counterpart_track,
            commands::prefetch_audio,
            commands::cancel_stale_streams,
            commands::get_cache_bytes,
            commands::clear_audio_cache,
            commands::list_downloaded_tracks,
            commands::download_track_offline,
            commands::remove_downloaded_tracks,
            commands::get_downloads_bytes,
            commands::spin_status,
            commands::spin_start_local,
            commands::spin_stop_local,
            commands::spin_set_allow_controls,
            commands::spin_start_remote_host,
            commands::spin_stop_remote,
            commands::spin_join_remote,
            commands::spin_sync_snapshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running GyroTube Tauri application");
}
