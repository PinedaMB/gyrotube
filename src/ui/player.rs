use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Image, Label, ListBoxRow, MenuButton, Orientation, Scale};

use crate::backend::runtime;
use crate::models::{LyricsData, format_duration, is_ytm_square_cover};
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{
    clamped_cover, clear_box, clear_list_box, load_cover_into_picture, load_player_cover,
    notify_track_changed, record_recent_track,
};

pub fn refresh_queue_ui(state: Rc<RefCell<AppState>>) {
    let (queue_list, queue_empty_label, queue_title_label, queue, current_index) = {
        let s = state.borrow();
        (
            s.queue_list.clone(),
            s.queue_empty_label.clone(),
            s.queue_title_label.clone(),
            s.queue.clone(),
            s.current_index,
        )
    };

    clear_list_box(&queue_list);
    let is_empty = queue.is_empty();
    queue_empty_label.set_visible(is_empty);
    queue_list.set_visible(!is_empty);
    if is_empty {
        queue_title_label.set_label("Cola");
    } else {
        queue_title_label.set_label(&format!("Cola • {} pistas", queue.len()));
    }

    for (idx, track) in queue.iter().enumerate() {
        let is_current = current_index == Some(idx);
        let row_box = GtkBox::new(Orientation::Horizontal, 8);
        row_box.add_css_class("track-row");

        let (cover_frame, cover) = clamped_cover(36, 36, "cover-art");
        if let Some(url) = &track.cover_url {
            load_cover_into_picture(&cover, url, Rc::clone(&state));
        }

        let title_label = Label::builder()
            .label(&track.title)
            .xalign(0.0)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();
        if is_current {
            title_label.add_css_class("playing-title");
        } else {
            title_label.add_css_class("heading");
        }

        let artist_label = Label::builder()
            .label(&track.artist)
            .xalign(0.0)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["dim-label", "caption"])
            .build();

        let text_box = GtkBox::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(Align::Center);
        text_box.append(&title_label);
        text_box.append(&artist_label);

        let duration_str = track
            .duration_secs
            .map(format_duration)
            .unwrap_or_else(|| String::from("--:--"));
        let duration_label = Label::builder()
            .label(&duration_str)
            .valign(Align::Center)
            .css_classes(["dim-label", "caption", "numeric"])
            .build();

        let remove_btn = Button::builder()
            .icon_name("list-remove-symbolic")
            .tooltip_text("Quitar de la cola")
            .valign(Align::Center)
            .css_classes(["flat", "circular"])
            .build();
        {
            let state = Rc::clone(&state);
            remove_btn.connect_clicked(move |_| {
                {
                    let mut s = state.borrow_mut();
                    if idx < s.queue.len() {
                        s.queue.remove(idx);
                        if let Some(cur) = s.current_index {
                            if idx < cur {
                                s.current_index = Some(cur - 1);
                            } else if idx == cur {
                                if s.queue.is_empty() {
                                    s.current_index = None;
                                } else {
                                    s.current_index = Some(cur.min(s.queue.len() - 1));
                                }
                            }
                        }
                        s.update_nav_buttons();
                    }
                }
                refresh_queue_ui(Rc::clone(&state));
            });
        }

        if is_current {
            let playing_icon = Image::builder()
                .icon_name("audio-volume-high-symbolic")
                .pixel_size(14)
                .valign(Align::Center)
                .css_classes(["playing-title"])
                .build();
            row_box.append(&playing_icon);
        }
        row_box.append(&cover_frame);
        row_box.append(&text_box);
        row_box.append(&duration_label);
        row_box.append(&remove_btn);

        let row = ListBoxRow::builder()
            .child(&row_box)
            .activatable(true)
            .build();
        if is_current {
            row.add_css_class("chart-active-row");
        }
        row.set_cursor_from_name(Some("pointer"));
        {
            let state = Rc::clone(&state);
            row.connect_activate(move |_| {
                play_queue_index(Rc::clone(&state), idx);
            });
        }
        queue_list.append(&row);
    }
}

pub fn load_current_lyrics(state: Rc<RefCell<AppState>>) {
    let (track, backend) = {
        let mut s = state.borrow_mut();
        let Some(track) = s.current_track().cloned() else {
            clear_box(&s.lyrics_box);
            s.synced_lyrics.clear();
            s.active_lyric_idx = None;
            s.lyrics_auto_scroll = true;
            s.lyrics_resync_btn.set_visible(false);
            s.lyrics_loaded_for = None;
            s.lyrics_title_label.set_label("Letras");
            s.lyrics_artist_label.set_visible(false);
            s.lyrics_sync_badge.set_visible(false);
            s.lyrics_body_label
                .set_label("Reproduce una canción para ver su letra.");
            s.lyrics_box.append(&s.lyrics_body_label);
            return;
        };
        if s.lyrics_loaded_for.as_deref() == Some(&track.id) {
            return;
        }
        clear_box(&s.lyrics_box);
        s.synced_lyrics.clear();
        s.active_lyric_idx = None;
        s.lyrics_auto_scroll = true;
        s.lyrics_resync_btn.set_visible(false);
        s.lyrics_title_label.set_label(&track.title);
        s.lyrics_artist_label.set_label(&track.artist);
        s.lyrics_artist_label.set_visible(!track.artist.is_empty());
        s.lyrics_sync_badge.set_visible(false);
        s.lyrics_body_label.set_label("Cargando letra…");
        s.lyrics_box.append(&s.lyrics_body_label);
        (track, s.backend.clone())
    };

    let (tx, rx) = async_channel::bounded(1);
    let track_for_req = track.clone();
    runtime().spawn(async move {
        let res = backend.fetch_synced_lyrics(&track_for_req).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        if let Ok(result) = rx.recv().await {
            let mut s = state.borrow_mut();
            if s.current_track().is_some_and(|cur| cur.id == track.id) {
                clear_box(&s.lyrics_box);
                s.synced_lyrics.clear();
                s.active_lyric_idx = None;
                match result {
                    Ok(LyricsData::Synced(lines)) => {
                        s.lyrics_sync_badge.set_label("● Sincronizada");
                        s.lyrics_sync_badge.remove_css_class("lyrics-badge-plain");
                        s.lyrics_sync_badge.add_css_class("lyrics-badge-synced");
                        s.lyrics_sync_badge.set_visible(true);
                        for line in lines {
                            let lbl = Label::builder()
                                .label(&line.text)
                                .xalign(0.0)
                                .wrap(true)
                                .build();
                            let btn = Button::builder()
                                .child(&lbl)
                                .css_classes(["flat", "lyric-line"])
                                .build();
                            let state_click = Rc::clone(&state);
                            let time_secs = line.time_secs;
                            btn.connect_clicked(move |_| {
                                let mut s = state_click.borrow_mut();
                                s.lyrics_auto_scroll = true;
                                s.lyrics_resync_btn.set_visible(false);
                                let media = s.active_media().clone();
                                let target_us = (time_secs * 1_000_000.0) as i64;
                                media.seek(target_us);
                                s.seek_scale.set_value(time_secs);
                                s.time_label.set_label(&format_duration(time_secs as u32));
                            });
                            s.lyrics_box.append(&btn);
                            s.synced_lyrics.push((time_secs, btn));
                        }
                        s.lyrics_loaded_for = Some(track.id);
                    }
                    Ok(LyricsData::Plain(text)) => {
                        s.lyrics_sync_badge.set_label("Estática");
                        s.lyrics_sync_badge.remove_css_class("lyrics-badge-synced");
                        s.lyrics_sync_badge.add_css_class("lyrics-badge-plain");
                        s.lyrics_sync_badge.set_visible(true);
                        s.lyrics_body_label.set_label(&text);
                        s.lyrics_box.append(&s.lyrics_body_label);
                        s.lyrics_loaded_for = Some(track.id);
                    }
                    Err(err) => {
                        s.lyrics_sync_badge.set_visible(false);
                        s.lyrics_body_label.set_label(&err);
                        s.lyrics_box.append(&s.lyrics_body_label);
                    }
                }
            }
        }
    });
}

pub fn scroll_to_active_lyric(s: &AppState) {
    if let Some(idx) = s.active_lyric_idx
        && let Some((_, btn)) = s.synced_lyrics.get(idx)
        && let Some(bounds) = btn.compute_bounds(&s.lyrics_box)
    {
        let adj = s.lyrics_scroll.vadjustment();
        let target = (bounds.y() as f64 - adj.page_size() * 0.35)
            .clamp(0.0, (adj.upper() - adj.page_size()).max(0.0));
        s.lyrics_programmatic_scroll.set(true);
        adj.set_value(target);
        s.lyrics_programmatic_scroll.set(false);
    }
}

fn update_synced_lyrics_position(s: &mut AppState, pos_secs_exact: f64) {
    if s.synced_lyrics.is_empty() {
        return;
    }
    let new_idx = s
        .synced_lyrics
        .iter()
        .rposition(|(t, _)| *t <= pos_secs_exact + 0.15);
    if new_idx != s.active_lyric_idx {
        if let Some(old) = s.active_lyric_idx
            && let Some((_, btn)) = s.synced_lyrics.get(old)
        {
            btn.remove_css_class("lyric-line-active");
        }
        if let Some(idx) = new_idx
            && let Some((_, btn)) = s.synced_lyrics.get(idx)
        {
            btn.add_css_class("lyric-line-active");
        }
        s.active_lyric_idx = new_idx;
        if s.lyrics_auto_scroll {
            scroll_to_active_lyric(s);
        }
    }
}

pub fn play_queue_index(state: Rc<RefCell<AppState>>, index: usize) {
    let (track, backend, current_gen, decks, player_cover, player_fav_btn, is_fav) = {
        let mut s = state.borrow_mut();
        let Some(track) = s.queue.get(index).cloned() else {
            return;
        };
        s.current_index = Some(index);
        s.loading_generation = s.loading_generation.wrapping_add(1);
        let current_gen = s.loading_generation;
        s.crossfade_active = false;
        s.crossfade_generation = s.crossfade_generation.wrapping_add(1);
        s.crossfade_triggered_for = None;
        s.lyrics_loaded_for = None;

        let source_label = track
            .album
            .clone()
            .unwrap_or_else(|| s.source_name.clone());
        s.player_source_label.set_label(&source_label);
        s.player_title.set_label(&track.title);
        s.player_artist
            .set_label(&format!("Cargando • {}", track.artist));
        s.play_pause_btn.set_sensitive(false);
        s.play_spinner.set_spinning(true);
        s.play_pause_btn.set_child(Some(&s.play_spinner));
        s.player_cover.set_paintable(None::<&gdk::Paintable>);
        s.player_cover_spinner.set_visible(true);
        s.player_cover_spinner.set_spinning(true);
        s.seek_scale.set_sensitive(false);
        s.seek_scale.set_value(0.0);
        s.time_label.set_label("0:00");
        s.duration_label.set_label(
            &track
                .duration_secs
                .map(format_duration)
                .unwrap_or_else(|| String::from("0:00")),
        );
        s.quality_label.set_label("...");

        let is_fav = s.is_favorite(&track.id);
        s.player_fav_btn.set_sensitive(true);
        s.player_radio_btn.set_sensitive(true);
        s.player_fav_btn.set_icon_name(if is_fav {
            "starred-symbolic"
        } else {
            "non-starred-symbolic"
        });
        s.update_nav_buttons();

        if let Some(mpris) = &s.mpris {
            mpris.update_track(&track);
        }

        (
            track,
            s.backend.clone(),
            current_gen,
            s.decks.clone(),
            s.player_cover.clone(),
            s.player_fav_btn.clone(),
            is_fav,
        )
    };

    decks[0].pause();
    decks[1].pause();

    if player_fav_btn.is_active() != is_fav {
        player_fav_btn.set_active(is_fav);
    }

    if let Some(url) = track.cover_large_url.as_deref().or(track.cover_url.as_deref()) {
        if is_ytm_square_cover(url) {
            load_player_cover(&player_cover, url, current_gen, Rc::clone(&state));
        } else {
            let (cov_tx, cov_rx) = async_channel::bounded(1);
            let backend_cov = backend.clone();
            let track_cov = track.clone();
            runtime().spawn(async move {
                let _ = cov_tx.send(backend_cov.resolve_ytm_cover(&track_cov).await).await;
            });

            let state_cov = Rc::clone(&state);
            let player_cover_cov = player_cover.clone();
            let fallback_url = url.to_string();
            glib::spawn_future_local(async move {
                if let Ok(resolved) = cov_rx.recv().await
                    && state_cov.borrow().loading_generation == current_gen
                {
                    let final_url = resolved.as_deref().unwrap_or(&fallback_url);
                    load_player_cover(&player_cover_cov, final_url, current_gen, state_cov);
                }
            });
        }
    } else {
        let s = state.borrow();
        s.player_cover_spinner.set_spinning(false);
        s.player_cover_spinner.set_visible(false);
    }

    refresh_queue_ui(Rc::clone(&state));
    record_recent_track(Rc::clone(&state), &track);
    notify_track_changed(Rc::clone(&state));

    let (tx, rx) = async_channel::bounded(1);
    let video_id = track.id.clone();
    runtime().spawn(async move {
        let res = backend.resolve_audio_stream(&video_id).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        let recv_res = rx.recv().await;
        let stream_to_play = {
            let mut s = state.borrow_mut();
            let Ok(result) = recv_res else {
                if s.loading_generation == current_gen {
                    s.play_spinner.set_spinning(false);
                    s.play_pause_btn.set_icon_name("media-playback-start-symbolic");
                    s.player_artist.set_label(&track.artist);
                    s.quality_label.set_label("ERR");
                    s.show_toast("No se pudo resolver el flujo de audio.");
                }
                return;
            };
            if s.loading_generation != current_gen {
                return;
            }
            s.play_spinner.set_spinning(false);
            match result {
                Ok(stream) => {
                    s.player_artist.set_label(&track.artist);
                    s.quality_label.set_label(&stream.quality_label);
                    s.play_pause_btn.set_icon_name("media-playback-start-symbolic");
                    s.play_pause_btn.set_sensitive(true);
                    s.seek_scale.set_sensitive(true);
                    s.pending_autoplay = true;
                    let next_id = if !s.shuffle {
                        s.queue.get(index + 1).map(|t| t.id.clone())
                    } else {
                        None
                    };
                    Some((
                        s.active_media().clone(),
                        stream.url,
                        s.backend.clone(),
                        next_id,
                    ))
                }
                Err(err) => {
                    s.play_pause_btn.set_icon_name("media-playback-start-symbolic");
                    s.player_artist.set_label(&track.artist);
                    s.quality_label.set_label("ERR");
                    s.show_toast(&err);
                    None
                }
            }
        };

        if let Some((media_file, path, backend, next_id)) = stream_to_play {
            let file = gio::File::for_path(&path);
            media_file.pause();
            media_file.set_file(Some(&file));
            if media_file.is_prepared() {
                state.borrow_mut().pending_autoplay = false;
                media_file.play();
            }
            if let Some(next_video_id) = next_id {
                runtime().spawn(async move {
                    backend.prefetch_audio(&next_video_id).await;
                });
            }
        }
    });
}

fn start_crossfade_to(state: Rc<RefCell<AppState>>, next_index: usize) {
    let (track, backend, cf_gen) = {
        let s = state.borrow();
        let Some(track) = s.queue.get(next_index).cloned() else {
            return;
        };
        (track, s.backend.clone(), s.crossfade_generation)
    };

    let (tx, rx) = async_channel::bounded(1);
    let video_id = track.id.clone();
    runtime().spawn(async move {
        let res = backend.resolve_audio_stream(&video_id).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        let Ok(Ok(stream)) = rx.recv().await else {
            return;
        };

        let setup = {
            let mut s = state.borrow_mut();
            if s.crossfade_generation != cf_gen || !s.crossfade_enabled || !s.active_media().is_playing() {
                return;
            }
            if s.queue.get(next_index).is_none_or(|t| t.id != track.id) {
                return;
            }

            let old_deck = s.active_deck;
            let new_deck = 1 - old_deck;
            s.active_deck = new_deck;
            s.crossfade_active = true;
            s.current_index = Some(next_index);
            s.loading_generation = s.loading_generation.wrapping_add(1);
            let current_gen = s.loading_generation;
            s.crossfade_triggered_for = None;
            s.lyrics_loaded_for = None;
            s.pending_autoplay = true;

            let source_label = track
                .album
                .clone()
                .unwrap_or_else(|| s.source_name.clone());
            s.player_source_label.set_label(&source_label);
            s.player_title.set_label(&track.title);
            s.player_artist.set_label(&track.artist);
            s.quality_label.set_label(&stream.quality_label);
            s.seek_scale.set_value(0.0);
            s.time_label.set_label("0:00");
            s.duration_label.set_label(
                &track
                    .duration_secs
                    .map(format_duration)
                    .unwrap_or_else(|| String::from("0:00")),
            );
            let is_fav = s.is_favorite(&track.id);
            s.player_fav_btn.set_icon_name(if is_fav {
                "starred-symbolic"
            } else {
                "non-starred-symbolic"
            });
            s.update_nav_buttons();
            if let Some(mpris) = &s.mpris {
                mpris.update_track(&track);
            }

            let next_prefetch = if !s.shuffle {
                s.queue.get(next_index + 1).map(|t| t.id.clone())
            } else {
                None
            };

            (
                old_deck,
                new_deck,
                s.decks[new_deck].clone(),
                s.player_cover.clone(),
                s.player_fav_btn.clone(),
                is_fav,
                current_gen,
                s.backend.clone(),
                next_prefetch,
            )
        };

        let (
            old_deck,
            new_deck,
            new_media,
            player_cover,
            player_fav_btn,
            is_fav,
            current_gen,
            backend,
            next_prefetch,
        ) = setup;

        if player_fav_btn.is_active() != is_fav {
            player_fav_btn.set_active(is_fav);
        }

        if let Some(url) = track.cover_large_url.as_deref().or(track.cover_url.as_deref()) {
            if is_ytm_square_cover(url) {
                load_player_cover(&player_cover, url, current_gen, Rc::clone(&state));
            } else {
                let (cov_tx, cov_rx) = async_channel::bounded(1);
                let backend_cov = backend.clone();
                let track_cov = track.clone();
                runtime().spawn(async move {
                    let _ = cov_tx.send(backend_cov.resolve_ytm_cover(&track_cov).await).await;
                });
                let state_cov = Rc::clone(&state);
                let player_cover_cov = player_cover.clone();
                let fallback_url = url.to_string();
                glib::spawn_future_local(async move {
                    if let Ok(resolved) = cov_rx.recv().await
                        && state_cov.borrow().loading_generation == current_gen
                    {
                        let final_url = resolved.as_deref().unwrap_or(&fallback_url);
                        load_player_cover(&player_cover_cov, final_url, current_gen, state_cov);
                    }
                });
            }
        }

        refresh_queue_ui(Rc::clone(&state));
        record_recent_track(Rc::clone(&state), &track);
        notify_track_changed(Rc::clone(&state));

        let file = gio::File::for_path(&stream.url);
        new_media.pause();
        new_media.set_file(Some(&file));
        new_media.set_volume(0.0);
        if new_media.is_prepared() {
            state.borrow_mut().pending_autoplay = false;
            new_media.play();
        }

        run_crossfade_ramp(Rc::clone(&state), cf_gen, old_deck, new_deck);

        if let Some(next_video_id) = next_prefetch {
            runtime().spawn(async move {
                backend.prefetch_audio(&next_video_id).await;
            });
        }
    });
}

fn run_crossfade_ramp(
    state: Rc<RefCell<AppState>>,
    cf_gen: u64,
    old_deck: usize,
    new_deck: usize,
) {
    const TOTAL_STEPS: u32 = 80; // 80 * 50ms = 4.0 seconds
    let mut step: u32 = 0;

    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        step += 1;
        let t = (step as f64 / TOTAL_STEPS as f64).min(1.0);

        let (active, user_vol, old_media, new_media) = {
            let mut s = state.borrow_mut();
            if s.crossfade_generation != cf_gen || !s.crossfade_active {
                return glib::ControlFlow::Break;
            }
            let done = step >= TOTAL_STEPS;
            if done {
                s.crossfade_active = false;
            }
            (
                !done,
                s.user_volume,
                s.decks[old_deck].clone(),
                s.decks[new_deck].clone(),
            )
        };

        old_media.set_volume(user_vol * (1.0 - t));
        new_media.set_volume(user_vol * t);

        if !active {
            old_media.pause();
            new_media.set_volume(user_vol);
            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });
}


pub fn wire_media_signals(
    state: Rc<RefCell<AppState>>,
    volume_scale: &Scale,
    volume_btn: &MenuButton,
) {
    let decks = state.borrow().decks.clone();

    {
        let state = Rc::clone(&state);
        let volume_btn = volume_btn.clone();
        volume_scale.connect_value_changed(move |scale| {
            let val = scale.value();
            let target_media = {
                let mut s = state.borrow_mut();
                s.user_volume = val;
                if s.crossfade_active {
                    None
                } else {
                    Some(s.active_media().clone())
                }
            };
            if let Some(media) = target_media
                && media.file().is_some()
            {
                media.set_volume(val);
            }
            let icon = if val <= 0.01 {
                "audio-volume-muted-symbolic"
            } else if val < 0.35 {
                "audio-volume-low-symbolic"
            } else if val < 0.7 {
                "audio-volume-medium-symbolic"
            } else {
                "audio-volume-high-symbolic"
            };
            volume_btn.set_icon_name(icon);
        });
    }

    {
        let state = Rc::clone(&state);
        let play_pause_btn = state.borrow().play_pause_btn.clone();
        play_pause_btn.connect_clicked(move |_| {
            let (active, standby, current_idx) = {
                let s = state.borrow();
                (
                    s.active_media().clone(),
                    s.standby_media().clone(),
                    s.current_index,
                )
            };
            if active.is_playing() {
                active.pause();
                standby.pause();
            } else if active.file().is_none() {
                if let Some(idx) = current_idx {
                    play_queue_index(Rc::clone(&state), idx);
                }
            } else {
                active.play();
            }
        });
    }

    {
        let state = Rc::clone(&state);
        let prev_btn = state.borrow().prev_btn.clone();
        prev_btn.connect_clicked(move |_| {
            let target = state
                .borrow()
                .current_index
                .and_then(|idx| idx.checked_sub(1));
            if let Some(idx) = target {
                play_queue_index(Rc::clone(&state), idx);
            }
        });
    }

    {
        let state = Rc::clone(&state);
        let next_btn = state.borrow().next_btn.clone();
        next_btn.connect_clicked(move |_| {
            let target = next_queue_index(&state.borrow());
            if let Some(idx) = target {
                play_queue_index(Rc::clone(&state), idx);
            }
        });
    }

    for (deck_idx, media_file) in decks.into_iter().enumerate() {
        {
            let state = Rc::clone(&state);
            media_file.connect_prepared_notify(move |media| {
                if media.is_prepared() {
                    let (should_play, target_vol) = {
                        let mut s = state.borrow_mut();
                        if s.active_deck == deck_idx {
                            let vol = if s.crossfade_active { None } else { Some(s.user_volume) };
                            let play = if s.pending_autoplay {
                                s.pending_autoplay = false;
                                true
                            } else {
                                false
                            };
                            (play, vol)
                        } else {
                            (false, None)
                        }
                    };
                    if let Some(vol) = target_vol {
                        media.set_volume(vol);
                    }
                    if should_play {
                        media.play();
                    }
                }
            });
        }

        {
            let state = Rc::clone(&state);
            media_file.connect_playing_notify(move |media| {
                let s = state.borrow();
                if s.active_deck != deck_idx {
                    return;
                }
                let playing = media.is_playing();
                let icon = if playing {
                    "media-playback-pause-symbolic"
                } else {
                    "media-playback-start-symbolic"
                };
                s.play_pause_btn.set_icon_name(icon);
                if let Some(mpris) = &s.mpris {
                    mpris.set_status(playing, s.current_index.is_none());
                }
            });
        }

        {
            let state = Rc::clone(&state);
            media_file.connect_duration_notify(move |media| {
                let s = state.borrow();
                if s.active_deck != deck_idx {
                    return;
                }
                let dur_us = media.duration();
                if dur_us > 0 {
                    let dur_secs = (dur_us / 1_000_000) as f64;
                    s.seek_scale.set_range(0.0, dur_secs);
                    s.duration_label.set_label(&format_duration(dur_secs as u32));
                }
            });
        }

        {
            let state = Rc::clone(&state);
            media_file.connect_timestamp_notify(move |media| {
                let s = state.borrow();
                if s.active_deck != deck_idx {
                    return;
                }
                let pos_us = media.timestamp();
                if pos_us >= 0 {
                    let pos_secs = (pos_us / 1_000_000) as f64;
                    s.seek_scale.set_value(pos_secs);
                    s.time_label.set_label(&format_duration(pos_secs as u32));
                }
            });
        }

        {
            let state = Rc::clone(&state);
            media_file.connect_ended_notify(move |media| {
                if !media.is_ended() {
                    return;
                }
                let target = {
                    let s = state.borrow();
                    if s.active_deck != deck_idx {
                        None
                    } else {
                        next_queue_index(&s)
                    }
                };
                if let Some(idx) = target {
                    play_queue_index(Rc::clone(&state), idx);
                }
            });
        }

        {
            let state = Rc::clone(&state);
            media_file.connect_error_notify(move |media| {
                if let Some(err) = media.error() {
                    let s = state.borrow();
                    if s.active_deck == deck_idx {
                        s.show_toast(&format!("Error de reproducción: {err}"));
                    }
                }
            });
        }
    }

    {
        let state = Rc::clone(&state);
        let seek_scale = state.borrow().seek_scale.clone();
        seek_scale.connect_change_value(move |_, _, val| {
            if val >= 0.0 {
                let target_us = (val * 1_000_000.0) as i64;
                let (media, mpris) = {
                    let mut s = state.borrow_mut();
                    s.crossfade_triggered_for = None;
                    (s.active_media().clone(), s.mpris.clone())
                };
                media.seek(target_us);
                if let Some(mpris) = mpris {
                    mpris.set_position_us(target_us);
                    mpris.emit_seeked(target_us);
                }
            }
            glib::Propagation::Proceed
        });
    }

    // Periodic 250ms sync with GStreamer position, duration, and crossfade trigger
    {
        let state = Rc::clone(&state);
        glib::timeout_add_local(std::time::Duration::from_millis(250), move || {
            let crossfade_target = {
                let mut s = state.borrow_mut();
                let media = s.active_media().clone();
                if media.is_playing() {
                    let dur_us = media.duration();
                    if dur_us > 0 {
                        let dur_secs = (dur_us / 1_000_000) as f64;
                        s.seek_scale.set_range(0.0, dur_secs);
                        s.duration_label.set_label(&format_duration(dur_secs as u32));
                    }
                    let pos_us = media.timestamp();
                    if pos_us >= 0 {
                        let pos_secs = (pos_us / 1_000_000) as f64;
                        s.seek_scale.set_value(pos_secs);
                        s.time_label.set_label(&format_duration(pos_secs as u32));
                        update_synced_lyrics_position(&mut s, pos_us as f64 / 1_000_000.0);
                        if let Some(mpris) = &s.mpris {
                            mpris.set_position_us(pos_us);
                        }
                    }

                    if s.crossfade_enabled
                        && !s.crossfade_active
                        && dur_us > 12_000_000
                        && pos_us > 0
                        && s.crossfade_triggered_for != Some(s.loading_generation)
                    {
                        let remaining_us = dur_us - pos_us;
                        if (500_000..=4_500_000).contains(&remaining_us) {
                            s.crossfade_triggered_for = Some(s.loading_generation);
                            next_queue_index(&s)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some(next_idx) = crossfade_target {
                start_crossfade_to(Rc::clone(&state), next_idx);
            }
            glib::ControlFlow::Continue
        });
    }
}

pub fn restore_last_track(state: Rc<RefCell<AppState>>) {
    let (track, player_cover, cached_stream, current_gen) = {
        let mut s = state.borrow_mut();
        let Some(idx) = s.current_index else {
            return;
        };
        let Some(track) = s.queue.get(idx).cloned() else {
            s.current_index = None;
            return;
        };
        let is_fav = s.favorites.iter().any(|t| t.id == track.id);
        let cached = s.backend.cached_stream(&track.id);
        s.player_source_label.set_label(&s.source_name);
        s.player_title.set_label(&track.title);
        s.player_artist.set_label(&track.artist);
        s.player_fav_btn.set_sensitive(true);
        s.player_fav_btn.set_active(is_fav);
        s.player_fav_btn.set_icon_name(if is_fav {
            "starred-symbolic"
        } else {
            "non-starred-symbolic"
        });
        s.player_radio_btn.set_sensitive(true);
        s.prev_btn.set_sensitive(idx > 0);
        s.next_btn.set_sensitive(idx + 1 < s.queue.len());
        if let Some(dur) = track.duration_secs {
            s.duration_label.set_label(&format_duration(dur));
            s.seek_scale.set_range(0.0, dur as f64);
        }
        s.pending_autoplay = false;
        (track, s.player_cover.clone(), cached, s.loading_generation)
    };

    if let Some(cover_url) = track.cover_url.as_deref() {
        load_player_cover(&player_cover, cover_url, current_gen, Rc::clone(&state));
    }
    if let Some(stream) = cached_stream {
        let s = state.borrow();
        s.quality_label.set_label(&stream.quality_label);
        let media = s.active_media().clone();
        media.set_file(Some(&gio::File::for_path(&stream.url)));
    }
    refresh_queue_ui(Rc::clone(&state));
    load_current_lyrics(state);
}
