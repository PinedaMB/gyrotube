use std::cell::RefCell;
use std::rc::Rc;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Image, Label, ListBoxRow, MenuButton, Orientation, Scale,
};

use crate::backend::runtime;
use crate::i18n::tr;
use crate::models::{LyricsData, format_duration};
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{
    clamped_cover, clear_box, clear_list_box, load_cover_into_picture, load_track_player_cover,
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
        queue_title_label.set_label(tr("Cola", "Queue"));
    } else {
        let unit = if queue.len() == 1 {
            tr("pista", "track")
        } else {
            tr("pistas", "tracks")
        };
        queue_title_label.set_label(&format!("{} • {} {unit}", tr("Cola", "Queue"), queue.len()));
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

        let move_up_btn = Button::builder()
            .icon_name("go-up-symbolic")
            .tooltip_text(tr("Subir en la cola", "Move up in queue"))
            .valign(Align::Center)
            .sensitive(idx > 0)
            .css_classes(["flat", "circular"])
            .build();
        {
            let state = Rc::clone(&state);
            move_up_btn.connect_clicked(move |_| {
                if idx > 0 {
                    state.borrow_mut().move_queue_item(idx, idx - 1);
                    refresh_queue_ui(Rc::clone(&state));
                }
            });
        }

        let move_down_btn = Button::builder()
            .icon_name("go-down-symbolic")
            .tooltip_text(tr("Bajar en la cola", "Move down in queue"))
            .valign(Align::Center)
            .sensitive(idx + 1 < queue.len())
            .css_classes(["flat", "circular"])
            .build();
        {
            let state = Rc::clone(&state);
            let queue_len = queue.len();
            move_down_btn.connect_clicked(move |_| {
                if idx + 1 < queue_len {
                    state.borrow_mut().move_queue_item(idx, idx + 1);
                    refresh_queue_ui(Rc::clone(&state));
                }
            });
        }

        let remove_btn = Button::builder()
            .icon_name("list-remove-symbolic")
            .tooltip_text(tr("Quitar de la cola", "Remove from queue"))
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
                .icon_name("speaker-2-regular")
                .pixel_size(14)
                .valign(Align::Center)
                .css_classes(["playing-title"])
                .build();
            row_box.append(&playing_icon);
        }
        row_box.append(&cover_frame);
        row_box.append(&text_box);
        row_box.append(&duration_label);
        row_box.append(&move_up_btn);
        row_box.append(&move_down_btn);
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

        let drag_source = gtk4::DragSource::builder()
            .actions(gtk4::gdk::DragAction::MOVE)
            .build();
        drag_source.connect_prepare(move |_, _, _| {
            Some(gtk4::gdk::ContentProvider::for_value(&idx.to_string().to_value()))
        });
        row.add_controller(drag_source);

        let drop_target = gtk4::DropTarget::new(String::static_type(), gtk4::gdk::DragAction::MOVE);
        {
            let state = Rc::clone(&state);
            drop_target.connect_drop(move |_, value, _, _| {
                if let Ok(from_str) = value.get::<String>() {
                    if let Ok(from) = from_str.parse::<usize>() {
                        if from != idx {
                            state.borrow_mut().move_queue_item(from, idx);
                            let state_for_refresh = Rc::clone(&state);
                            glib::idle_add_local_once(move || {
                                refresh_queue_ui(state_for_refresh);
                            });
                            return true;
                        }
                    }
                }
                false
            });
        }
        row.add_controller(drop_target);

        queue_list.append(&row);
    }
    crate::ui::party_mode::refresh_party_next_up(&state);
}

fn make_lyric_line_button(
    text: &str,
    time_secs: f64,
    state: Rc<RefCell<AppState>>,
    css_class: &str,
) -> Button {
    let lbl = Label::builder()
        .label(text)
        .xalign(0.0)
        .wrap(true)
        .build();
    let btn = Button::builder()
        .child(&lbl)
        .css_classes(["flat", css_class])
        .build();
    btn.connect_clicked(move |_| {
        let mut s = state.borrow_mut();
        s.lyrics_auto_scroll = true;
        s.lyrics_resync_btn.set_visible(false);
        let media = s.active_media().clone();
        let target_us = (time_secs * 1_000_000.0) as i64;
        media.seek(target_us);
        s.seek_scale.set_value(time_secs);
        s.time_label.set_label(&format_duration(time_secs as u32));
        update_synced_lyrics_position(&mut s, time_secs);
    });
    btn
}

pub fn load_current_lyrics(state: Rc<RefCell<AppState>>) {
    let (track, backend) = {
        let mut s = state.borrow_mut();
        let Some(track) = s.current_track().cloned() else {
            clear_box(&s.lyrics_box);
            s.synced_lyrics.clear();
            clear_box(&s.party_lyrics_box);
            s.party_synced_lyrics.clear();
            s.active_lyric_idx = None;
            s.lyrics_auto_scroll = true;
            s.lyrics_resync_btn.set_visible(false);
            s.lyrics_loaded_for = None;
            s.lyrics_title_label.set_label("Letras");
            s.lyrics_artist_label.set_visible(false);
            s.lyrics_sync_badge.set_visible(false);
            let empty_msg = "Reproduce una canción para ver su letra.";
            s.lyrics_body_label.set_label(empty_msg);
            s.lyrics_box.append(&s.lyrics_body_label);
            s.party_lyrics_body_label.set_label(empty_msg);
            s.party_lyrics_box.append(&s.party_lyrics_body_label);
            return;
        };
        if s.lyrics_loaded_for.as_deref() == Some(&track.id) {
            return;
        }
        clear_box(&s.lyrics_box);
        s.synced_lyrics.clear();
        clear_box(&s.party_lyrics_box);
        s.party_synced_lyrics.clear();
        s.active_lyric_idx = None;
        s.lyrics_auto_scroll = true;
        s.lyrics_resync_btn.set_visible(false);
        s.lyrics_title_label.set_label(&track.title);
        s.lyrics_artist_label.set_label(&track.artist);
        s.lyrics_artist_label.set_visible(!track.artist.is_empty());
        s.lyrics_sync_badge.set_visible(false);
        s.lyrics_body_label.set_label("Cargando letra…");
        s.lyrics_box.append(&s.lyrics_body_label);
        s.party_lyrics_body_label.set_label("Cargando letra…");
        s.party_lyrics_box.append(&s.party_lyrics_body_label);
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
                clear_box(&s.party_lyrics_box);
                s.party_synced_lyrics.clear();
                s.active_lyric_idx = None;
                match result {
                    Ok(LyricsData::Synced(lines)) => {
                        s.lyrics_sync_badge.set_label("● Sincronizada");
                        s.lyrics_sync_badge.remove_css_class("lyrics-badge-plain");
                        s.lyrics_sync_badge.add_css_class("lyrics-badge-synced");
                        s.lyrics_sync_badge.set_visible(true);
                        for line in lines {
                            let btn = make_lyric_line_button(
                                &line.text,
                                line.time_secs,
                                Rc::clone(&state),
                                "lyric-line",
                            );
                            let party_btn = make_lyric_line_button(
                                &line.text,
                                line.time_secs,
                                Rc::clone(&state),
                                "party-lyric-line",
                            );
                            s.lyrics_box.append(&btn);
                            s.synced_lyrics.push((line.time_secs, btn));
                            s.party_lyrics_box.append(&party_btn);
                            s.party_synced_lyrics.push((line.time_secs, party_btn));
                        }
                        s.lyrics_loaded_for = Some(track.id);
                        let state_fade = Rc::clone(&state);
                        glib::idle_add_local_once(move || {
                            let s = state_fade.borrow();
                            if s.party_active {
                                scroll_party_lyric_centered(&s, false);
                            }
                            update_party_lyrics_fade(&s);
                        });
                    }
                    Ok(LyricsData::Plain(text)) => {
                        s.lyrics_sync_badge.set_label("Estática");
                        s.lyrics_sync_badge.remove_css_class("lyrics-badge-synced");
                        s.lyrics_sync_badge.add_css_class("lyrics-badge-plain");
                        s.lyrics_sync_badge.set_visible(true);
                        s.lyrics_body_label.set_label(&text);
                        s.lyrics_box.append(&s.lyrics_body_label);
                        s.party_lyrics_body_label.set_label(&text);
                        s.party_lyrics_box.append(&s.party_lyrics_body_label);
                        s.lyrics_loaded_for = Some(track.id);
                        let state_fade = Rc::clone(&state);
                        glib::idle_add_local_once(move || {
                            let s = state_fade.borrow();
                            if s.party_active {
                                scroll_party_lyric_centered(&s, false);
                            }
                            update_party_lyrics_fade(&s);
                        });
                    }
                    Err(err) => {
                        s.lyrics_sync_badge.set_visible(false);
                        s.lyrics_body_label.set_label(&err);
                        s.lyrics_box.append(&s.lyrics_body_label);
                        s.party_lyrics_body_label.set_label(&err);
                        s.party_lyrics_box.append(&s.party_lyrics_body_label);
                    }
                }
            }
        }
    });
}

fn scroll_lyric_list_to_idx(
    lyrics: &[(f64, Button)],
    lyrics_box: &gtk4::Box,
    scroll: &gtk4::ScrolledWindow,
    programmatic_flag: Option<&std::cell::Cell<bool>>,
    idx: Option<usize>,
) {
    if let Some(idx) = idx
        && let Some((_, btn)) = lyrics.get(idx)
        && let Some(bounds) = btn.compute_bounds(lyrics_box)
    {
        let adj = scroll.vadjustment();
        let target = (bounds.y() as f64 - adj.page_size() * 0.35)
            .clamp(0.0, (adj.upper() - adj.page_size()).max(0.0));
        if let Some(flag) = programmatic_flag {
            flag.set(true);
        }
        adj.set_value(target);
        if let Some(flag) = programmatic_flag {
            flag.set(false);
        }
    }
}

pub fn scroll_to_active_lyric(s: &AppState) {
    scroll_lyric_list_to_idx(
        &s.synced_lyrics,
        &s.lyrics_box,
        &s.lyrics_scroll,
        Some(&s.lyrics_programmatic_scroll),
        s.active_lyric_idx,
    );
}

pub fn update_party_lyrics_fade(s: &AppState) {
    if !s.party_active || s.party_synced_lyrics.is_empty() {
        return;
    }
    let adj = s.party_lyrics_scroll.vadjustment();
    let scroll_top = adj.value();
    let page_h = adj.page_size();
    let fade_zone = 130.0_f64;

    for (i, (_, btn)) in s.party_synced_lyrics.iter().enumerate() {
        let base_opacity = match s.active_lyric_idx {
            Some(active) if i == active => 1.0,
            Some(active) if i.abs_diff(active) == 1 => 0.62,
            Some(active) if i.abs_diff(active) == 2 => 0.38,
            Some(_) => 0.20,
            None => 0.45,
        };
        let edge_fade = if page_h > 0.0
            && let Some(bounds) = btn.compute_bounds(&s.party_lyrics_box)
        {
            let center_y = (bounds.y() as f64 + bounds.height() as f64 * 0.5) - scroll_top;
            if center_y < fade_zone {
                (center_y / fade_zone).clamp(0.08, 1.0)
            } else if center_y > page_h - fade_zone {
                ((page_h - center_y) / fade_zone).clamp(0.08, 1.0)
            } else {
                1.0
            }
        } else {
            1.0
        };
        btn.set_opacity(base_opacity * edge_fade);
    }
}

pub fn scroll_party_lyric_centered(s: &AppState, animate: bool) {
    use std::cell::Cell;
    use std::time::{Duration, Instant};
    thread_local! {
        static PARTY_SCROLL_GEN: Cell<u64> = const { Cell::new(0) };
    }

    let target_idx = s.active_lyric_idx.or_else(|| {
        if s.party_synced_lyrics.is_empty() {
            None
        } else {
            Some(0)
        }
    });
    let Some(idx) = target_idx else {
        return;
    };
    let Some((_, btn)) = s.party_synced_lyrics.get(idx).cloned() else {
        return;
    };

    let scroll_gen = PARTY_SCROLL_GEN.with(|g| {
        let next = g.get().wrapping_add(1);
        g.set(next);
        next
    });

    let lyrics_box = s.party_lyrics_box.clone();
    let scroll = s.party_lyrics_scroll.clone();

    let compute_target = move |b: &Button, lb: &gtk4::Box, sc: &gtk4::ScrolledWindow| -> Option<f64> {
        let bounds = b.compute_bounds(lb)?;
        let adj = sc.vadjustment();
        let max_val = (adj.upper() - adj.page_size()).max(0.0);
        Some(
            (bounds.y() as f64 - adj.page_size() * 0.35)
                .clamp(0.0, max_val),
        )
    };

    if !animate {
        if let Some(target) = compute_target(&btn, &lyrics_box, &scroll) {
            scroll.vadjustment().set_value(target);
            update_party_lyrics_fade(s);
        }
        return;
    }

    let start = scroll.vadjustment().value();
    let started = Instant::now();
    let duration_secs = 0.36_f64;

    glib::timeout_add_local(Duration::from_millis(16), move || {
        let still_current = PARTY_SCROLL_GEN.with(|g| g.get() == scroll_gen);
        if !still_current {
            return glib::ControlFlow::Break;
        }
        let Some(target) = compute_target(&btn, &lyrics_box, &scroll) else {
            return glib::ControlFlow::Break;
        };
        let adj = scroll.vadjustment();
        if (target - start).abs() < 1.0 {
            adj.set_value(target);
            return glib::ControlFlow::Break;
        }
        let progress = (started.elapsed().as_secs_f64() / duration_secs).clamp(0.0, 1.0);
        let ease = 1.0 - (1.0 - progress).powi(3);
        adj.set_value(start + (target - start) * ease);
        if progress >= 1.0 {
            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });
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
        if let Some(old) = s.active_lyric_idx {
            if let Some((_, btn)) = s.synced_lyrics.get(old) {
                btn.remove_css_class("lyric-line-active");
            }
            if let Some((_, btn)) = s.party_synced_lyrics.get(old) {
                btn.remove_css_class("lyric-line-active");
            }
        }
        if let Some(idx) = new_idx {
            if let Some((_, btn)) = s.synced_lyrics.get(idx) {
                btn.add_css_class("lyric-line-active");
            }
            if let Some((_, btn)) = s.party_synced_lyrics.get(idx) {
                btn.add_css_class("lyric-line-active");
            }
        }
        s.active_lyric_idx = new_idx;
        if s.lyrics_auto_scroll {
            scroll_to_active_lyric(s);
        }
        if s.party_active {
            scroll_party_lyric_centered(s, true);
            update_party_lyrics_fade(s);
        }
    }
}

fn set_stream_on_media(media: &crate::gst_deck::GStreamerDeck, stream: crate::models::StreamInfo) {
    media.set_stream(&stream.url, stream.user_agent.as_deref(), &stream.headers);
}

pub fn show_player_loading(state: &Rc<RefCell<AppState>>, title: &str) {
    let s = state.borrow();
    s.player_title.set_label(title);
    s.player_artist.set_text("Cargando...");
    s.play_pause_btn.set_sensitive(false);
    s.play_spinner.set_spinning(true);
    s.play_pause_btn.set_child(Some(&s.play_spinner));
    s.quality_label.set_label("...");
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

        crate::ui::widgets::set_source_link(
            &s.player_source_label,
            track.album.as_deref(),
            &s.source_name,
        );
        s.player_title.set_label(&track.title);
        s.player_artist
            .set_text(&format!("Cargando • {}", track.artist));
        crate::ui::widgets::set_album_link(&s.player_album_label, track.album.as_deref());
        s.play_pause_btn.set_sensitive(false);
        s.play_spinner.set_spinning(true);
        s.play_pause_btn.set_child(Some(&s.play_spinner));
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
        s.pending_restore_seek = None;

        let is_fav = s.is_favorite(&track.id);
        s.player_fav_btn.set_sensitive(true);
        s.player_radio_btn.set_sensitive(true);
        s.player_fav_btn.set_icon_name(if is_fav {
            "heart-filled"
        } else {
            "heart-regular"
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

    load_track_player_cover(&player_cover, &track, current_gen, Rc::clone(&state));

    refresh_queue_ui(Rc::clone(&state));
    record_recent_track(Rc::clone(&state), &track);
    notify_track_changed(Rc::clone(&state));

    let (tx, rx) = async_channel::bounded(1);
    let track_res = track.clone();
    runtime().spawn(async move {
        let res = backend.resolve_track_stream(&track_res).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        let recv_res = rx.recv().await;
        let stream_to_play = {
            let mut s = state.borrow_mut();
            let Ok(result) = recv_res else {
                if s.loading_generation == current_gen {
                    s.play_spinner.set_spinning(false);
                    s.play_pause_btn
                        .set_icon_name("play-filled");
                    crate::ui::widgets::set_artist_links(&s.player_artist, &track.artist);
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
                    crate::ui::widgets::set_artist_links(&s.player_artist, &track.artist);
                    s.quality_label.set_label(&stream.quality_label);
                    s.play_pause_btn
                        .set_icon_name("play-filled");
                    s.play_pause_btn.set_sensitive(true);
                    s.seek_scale.set_sensitive(true);
                    s.pending_autoplay = true;
                    let next_id = next_queue_index(&s)
                        .and_then(|ni| s.queue.get(ni))
                        .cloned()
                        .filter(|t| s.preloading_id.as_deref() != Some(t.id.as_str()));
                    if let Some(ref id) = next_id {
                        s.preloading_id = Some(id.id.clone());
                    }
                    Some((s.active_media().clone(), stream, s.backend.clone(), next_id))
                }
                Err(err) => {
                    s.play_pause_btn
                        .set_icon_name("play-filled");
                    crate::ui::widgets::set_artist_links(&s.player_artist, &track.artist);
                    s.quality_label.set_label("ERR");
                    s.show_toast(&err);
                    None
                }
            }
        };

        if let Some((media_file, stream, backend, next_id)) = stream_to_play {
            media_file.pause();
            set_stream_on_media(&media_file, stream);
            if media_file.is_prepared() {
                state.borrow_mut().pending_autoplay = false;
                media_file.play();
            }
            if let Some(next_track) = next_id {
                runtime().spawn(async move {
                    backend.prefetch_audio(&next_track).await;
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
    let track_res = track.clone();
    runtime().spawn(async move {
        let res = backend.resolve_track_stream(&track_res).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        let Ok(Ok(stream)) = rx.recv().await else {
            return;
        };

        let setup = {
            let mut s = state.borrow_mut();
            if s.crossfade_generation != cf_gen
                || !s.crossfade_enabled
                || !s.active_media().is_playing()
            {
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

            crate::ui::widgets::set_source_link(
                &s.player_source_label,
                track.album.as_deref(),
                &s.source_name,
            );
            s.player_title.set_label(&track.title);
            crate::ui::widgets::set_artist_links(&s.player_artist, &track.artist);
            crate::ui::widgets::set_album_link(&s.player_album_label, track.album.as_deref());
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
                "heart-filled"
            } else {
                "heart-regular"
            });
            s.update_nav_buttons();
            if let Some(mpris) = &s.mpris {
                mpris.update_track(&track);
            }

            let next_prefetch = next_queue_index(&s)
                .and_then(|ni| s.queue.get(ni))
                .cloned()
                .filter(|t| s.preloading_id.as_deref() != Some(t.id.as_str()));
            if let Some(ref id) = next_prefetch {
                s.preloading_id = Some(id.id.clone());
            }

            let cf_secs = s.crossfade_secs.clamp(1, 10);
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
                cf_secs,
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
            cf_secs,
        ) = setup;

        if player_fav_btn.is_active() != is_fav {
            player_fav_btn.set_active(is_fav);
        }

        load_track_player_cover(&player_cover, &track, current_gen, Rc::clone(&state));

        refresh_queue_ui(Rc::clone(&state));
        record_recent_track(Rc::clone(&state), &track);
        notify_track_changed(Rc::clone(&state));

        new_media.pause();
        set_stream_on_media(&new_media, stream);
        new_media.set_volume(0.0);
        if new_media.is_prepared() {
            state.borrow_mut().pending_autoplay = false;
            new_media.play();
        }

        run_crossfade_ramp(Rc::clone(&state), cf_gen, old_deck, new_deck, cf_secs);

        if let Some(next_track) = next_prefetch {
            runtime().spawn(async move {
                backend.prefetch_audio(&next_track).await;
            });
        }
    });
}

fn run_crossfade_ramp(
    state: Rc<RefCell<AppState>>,
    cf_gen: u64,
    old_deck: usize,
    new_deck: usize,
    cf_secs: u32,
) {
    let total_steps: u32 = cf_secs.clamp(1, 10) * 20;
    let mut step: u32 = 0;

    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        step += 1;
        let t = (step as f64 / total_steps as f64).min(1.0);

        let (active, user_vol, old_media, new_media) = {
            let mut s = state.borrow_mut();
            if s.crossfade_generation != cf_gen || !s.crossfade_active {
                return glib::ControlFlow::Break;
            }
            let done = step >= total_steps;
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
                s.save_session();
                if s.crossfade_active {
                    None
                } else {
                    Some(s.active_media().clone())
                }
            };
            if let Some(media) = target_media
                && media.has_stream()
            {
                media.set_volume(val);
            }
            let icon = if val <= 0.01 {
                "speaker-0-regular"
            } else if val < 0.35 {
                "audio-volume-low-symbolic"
            } else if val < 0.7 {
                "speaker-1-regular"
            } else {
                "speaker-2-regular"
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
            } else if !active.has_stream() {
                if let Some(idx) = current_idx {
                    let saved_seek = state.borrow().pending_restore_seek;
                    play_queue_index(Rc::clone(&state), idx);
                    state.borrow_mut().pending_restore_seek = saved_seek;
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
                    let (should_play, target_vol, restore_seek) = {
                        let mut s = state.borrow_mut();
                        if s.active_deck == deck_idx {
                            let vol = if s.crossfade_active {
                                None
                            } else {
                                Some(s.user_volume)
                            };
                            let play = if s.pending_autoplay {
                                s.pending_autoplay = false;
                                true
                            } else {
                                false
                            };
                            let restore = s.pending_restore_seek.take();
                            if let Some(pos) = restore {
                                s.seek_scale.set_value(pos);
                                s.time_label.set_label(&format_duration(pos as u32));
                            }
                            (play, vol, restore)
                        } else {
                            (false, None, None)
                        }
                    };
                    if let Some(vol) = target_vol {
                        media.set_volume(vol);
                    }
                    if let Some(pos) = restore_seek {
                        media.seek((pos * 1_000_000.0) as i64);
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
                    "pause-filled"
                } else {
                    "play-filled"
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
                    s.duration_label
                        .set_label(&format_duration(dur_secs as u32));
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
                if s.pending_restore_seek.is_some() && pos_us <= 1_000_000 {
                    return;
                }
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
                    let fallback_req = {
                        let mut s = state.borrow_mut();
                        if s.active_deck != deck_idx {
                            None
                        } else if media.is_remote_stream()
                            && let Some(track) =
                                s.current_index.and_then(|i| s.queue.get(i)).cloned()
                        {
                            s.play_spinner.set_visible(true);
                            s.play_spinner.set_spinning(true);
                            s.play_pause_btn.set_child(Some(&s.play_spinner));
                            s.play_pause_btn.set_sensitive(false);
                            s.pending_autoplay = true;
                            Some((track.id, s.backend.clone(), s.loading_generation))
                        } else {
                            s.show_toast(&format!("Error de reproducción: {err}"));
                            None
                        }
                    };

                    if let Some((video_id, backend, load_gen)) = fallback_req {
                        let (tx, rx) = async_channel::bounded(1);
                        runtime().spawn(async move {
                            let res = backend.download_with_ytdlp_fallback(&video_id).await;
                            let _ = tx.send(res).await;
                        });
                        let state_cb = Rc::clone(&state);
                        glib::spawn_future_local(async move {
                            let recv = rx.recv().await;
                            let media_opt = {
                                let mut s = state_cb.borrow_mut();
                                if s.loading_generation != load_gen {
                                    return;
                                }
                                s.play_spinner.set_spinning(false);
                                s.play_pause_btn
                                    .set_icon_name("play-filled");
                                s.play_pause_btn.set_sensitive(true);
                                match recv {
                                    Ok(Ok(stream)) => {
                                        s.quality_label.set_label(&stream.quality_label);
                                        s.pending_autoplay = true;
                                        Some((s.active_media().clone(), stream))
                                    }
                                    Ok(Err(e)) => {
                                        s.show_toast(&e);
                                        None
                                    }
                                    Err(_) => {
                                        s.show_toast("Error al descargar el audio de respaldo.");
                                        None
                                    }
                                }
                            };
                            if let Some((m, stream)) = media_opt {
                                set_stream_on_media(&m, stream);
                            }
                        });
                    }
                }
            });
        }
    }

    {
        let state = Rc::clone(&state);
        let seek_scale = state.borrow().seek_scale.clone();
        let pending_seek_source: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
        seek_scale.connect_change_value(move |scale, _, val| {
            let max_dur = scale.adjustment().upper();
            let clamped = if max_dur > 0.5 {
                val.clamp(0.0, max_dur - 0.5)
            } else {
                val.max(0.0)
            };
            let target_us = (clamped * 1_000_000.0) as i64;
            {
                let mut s = state.borrow_mut();
                s.crossfade_triggered_for = None;
                s.time_label.set_label(&format_duration(clamped as u32));
            }
            if let Some(id) = pending_seek_source.borrow_mut().take() {
                id.remove();
            }
            let state_cb = Rc::clone(&state);
            let pending_ref = Rc::clone(&pending_seek_source);
            let id =
                glib::timeout_add_local_once(std::time::Duration::from_millis(60), move || {
                    *pending_ref.borrow_mut() = None;
                    let (media, mpris) = {
                        let s = state_cb.borrow();
                        (s.active_media().clone(), s.mpris.clone())
                    };
                    media.seek(target_us);
                    if let Some(mpris) = mpris {
                        mpris.set_position_us(target_us);
                        mpris.emit_seeked(target_us);
                    }
                });
            *pending_seek_source.borrow_mut() = Some(id);
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
                        s.duration_label
                            .set_label(&format_duration(dur_secs as u32));
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

                    let cf_us = (s.crossfade_secs.min(10) as i64) * 1_000_000;
                    if s.crossfade_enabled
                        && cf_us > 0
                        && !s.crossfade_active
                        && dur_us > cf_us + 4_000_000
                        && pos_us > 0
                        && s.crossfade_triggered_for != Some(s.loading_generation)
                    {
                        let remaining_us = dur_us - pos_us;
                        if (500_000..=cf_us + 1_000_000).contains(&remaining_us) {
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

pub fn restore_last_track(state: Rc<RefCell<AppState>>, position_secs: f64) {
    let (track, player_cover, player_fav_btn, is_fav, cached_stream, current_gen) = {
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
        s.pending_restore_seek = if position_secs > 1.0 {
            Some(position_secs)
        } else {
            None
        };
        crate::ui::widgets::set_source_link(
            &s.player_source_label,
            track.album.as_deref(),
            &s.source_name,
        );
        s.player_title.set_label(&track.title);
        crate::ui::widgets::set_artist_links(&s.player_artist, &track.artist);
        crate::ui::widgets::set_album_link(&s.player_album_label, track.album.as_deref());
        s.player_fav_btn.set_sensitive(true);
        s.player_fav_btn.set_icon_name(if is_fav {
            "heart-filled"
        } else {
            "heart-regular"
        });
        s.player_radio_btn.set_sensitive(true);
        s.play_pause_btn.set_sensitive(true);
        s.seek_scale.set_sensitive(cached.is_some());
        s.prev_btn.set_sensitive(idx > 0);
        s.next_btn
            .set_sensitive(idx + 1 < s.queue.len() || (s.repeat && !s.queue.is_empty()));
        if let Some(dur) = track.duration_secs {
            s.duration_label.set_label(&format_duration(dur));
            s.seek_scale.set_range(0.0, dur as f64);
        }
        if position_secs > 1.0 {
            s.seek_scale.set_value(position_secs);
            s.time_label
                .set_label(&format_duration(position_secs as u32));
        }
        s.pending_autoplay = false;
        (
            track,
            s.player_cover.clone(),
            s.player_fav_btn.clone(),
            is_fav,
            cached,
            s.loading_generation,
        )
    };

    if player_fav_btn.is_active() != is_fav {
        player_fav_btn.set_active(is_fav);
    }

    load_track_player_cover(&player_cover, &track, current_gen, Rc::clone(&state));
    if let Some(stream) = cached_stream {
        let s = state.borrow();
        s.quality_label.set_label(&stream.quality_label);
        let media = s.active_media().clone();
        set_stream_on_media(&media, stream);
    }
    refresh_queue_ui(Rc::clone(&state));
    load_current_lyrics(state);
}
