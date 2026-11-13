use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, Image, Label, ListBox, ListBoxRow, MenuButton,
    Orientation, Overflow, Picture, PolicyType, Popover, ScrolledWindow, ToggleButton,
};
use libadwaita as adw;

use crate::backend::runtime;
use crate::models::{
    Album, Playlist, Track, format_duration, save_favorites, save_recent_tracks,
    save_user_playlists,
};
use crate::ui::collection::{refresh_favorites_ui, refresh_playlists_ui, refresh_recent_ui};
use crate::ui::player::{play_queue_index, refresh_queue_ui};
use crate::ui::state::AppState;

pub fn clear_box(container: &GtkBox) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

pub fn clear_list_box(list: &ListBox) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
}

pub fn build_skeleton_box(width: i32, height: i32) -> GtkBox {
    let b = GtkBox::builder()
        .width_request(width)
        .height_request(height)
        .css_classes(["skeleton-box"])
        .build();
    if width <= 0 {
        b.set_hexpand(true);
    }
    b
}

pub fn notify_track_changed(state: Rc<RefCell<AppState>>) {
    let callbacks = state.borrow().on_track_changed.clone();
    for cb in callbacks {
        cb();
    }
}

pub fn record_recent_track(state: Rc<RefCell<AppState>>, track: &Track) {
    {
        let mut s = state.borrow_mut();
        s.recent_tracks.retain(|t| t.id != track.id);
        s.recent_tracks.insert(0, track.clone());
        s.recent_tracks.truncate(30);
        save_recent_tracks(&s.recent_tracks);
    }
    refresh_recent_ui(state);
}

pub fn build_carousel_section(title: &str) -> (GtkBox, GtkBox, ScrolledWindow) {
    let header_box = GtkBox::new(Orientation::Horizontal, 8);
    let title_label = Label::builder()
        .label(title)
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-3"])
        .build();

    let prev_scroll_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .css_classes(["circular", "flat"])
        .build();
    let next_scroll_btn = Button::builder()
        .icon_name("go-next-symbolic")
        .css_classes(["circular", "flat"])
        .build();

    header_box.append(&title_label);
    header_box.append(&prev_scroll_btn);
    header_box.append(&next_scroll_btn);

    let cards_box = GtkBox::new(Orientation::Horizontal, 12);
    cards_box.set_margin_bottom(6);

    let scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Never)
        .child(&cards_box)
        .build();

    {
        let scroller = scroller.clone();
        prev_scroll_btn.connect_clicked(move |_| {
            let adj = scroller.hadjustment();
            adj.set_value((adj.value() - 360.0).max(adj.lower()));
        });
    }
    {
        let scroller = scroller.clone();
        next_scroll_btn.connect_clicked(move |_| {
            let adj = scroller.hadjustment();
            adj.set_value((adj.value() + 360.0).min(adj.upper() - adj.page_size()));
        });
    }

    let section = GtkBox::new(Orientation::Vertical, 10);
    section.append(&header_box);
    section.append(&scroller);
    section.set_visible(false);

    (section, cards_box, scroller)
}

pub fn build_album_card(album: &Album, state: Rc<RefCell<AppState>>) -> Button {
    let (cover_frame, cover) = clamped_cover(156, 156, "cover-art");
    cover_frame.set_halign(Align::Center);

    if let Some(url) = &album.cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(&state));
    }

    let play_icon = Image::builder()
        .icon_name("media-playback-start-symbolic")
        .pixel_size(18)
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(true)
        .build();
    let play_overlay_badge = GtkBox::builder()
        .halign(Align::End)
        .valign(Align::End)
        .margin_end(8)
        .margin_bottom(8)
        .width_request(40)
        .height_request(40)
        .css_classes(["album-play-overlay"])
        .build();
    play_overlay_badge.append(&play_icon);

    let cover_overlay = gtk4::Overlay::new();
    cover_overlay.set_halign(Align::Center);
    cover_overlay.set_child(Some(&cover_frame));
    cover_overlay.add_overlay(&play_overlay_badge);

    if let Some(year) = album.year {
        let year_badge = Label::builder()
            .label(&year.to_string())
            .halign(Align::Start)
            .valign(Align::Start)
            .margin_start(8)
            .margin_top(8)
            .css_classes(["album-year-badge", "numeric"])
            .build();
        cover_overlay.add_overlay(&year_badge);
    }

    let title_label = Label::builder()
        .label(&album.title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(18)
        .css_classes(["heading", "album-title"])
        .build();

    let artist_label = Label::builder()
        .label(&album.artist)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(18)
        .css_classes(["dim-label", "caption"])
        .build();

    let meta_text = match album.year {
        Some(y) => format!("Álbum • {y}"),
        None => String::from("Álbum"),
    };
    let meta_label = Label::builder()
        .label(&meta_text)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["dim-label", "caption"])
        .build();
    meta_label.set_opacity(0.75);

    let text_box = GtkBox::new(Orientation::Vertical, 2);
    text_box.append(&title_label);
    text_box.append(&artist_label);
    text_box.append(&meta_label);

    let card_box = GtkBox::new(Orientation::Vertical, 10);
    card_box.append(&cover_overlay);
    card_box.append(&text_box);

    let btn = Button::builder()
        .child(&card_box)
        .css_classes(["flat", "album-featured-card"])
        .build();
    btn.set_cursor_from_name(Some("pointer"));

    let album_id = album.id.clone();
    let album_title = album.title.clone();
    btn.connect_clicked(move |_| {
        let backend = state.borrow().backend.clone();
        state
            .borrow()
            .show_toast(&format!("Cargando álbum: {album_title}"));

        let (tx, rx) = async_channel::bounded(1);
        let id = album_id.clone();
        runtime().spawn(async move {
            let res = backend.fetch_album_tracks(&id).await;
            let _ = tx.send(res).await;
        });

        let state = Rc::clone(&state);
        glib::spawn_future_local(async move {
            if let Ok(result) = rx.recv().await {
                match result {
                    Ok((title, tracks)) if !tracks.is_empty() => {
                        {
                            let mut s = state.borrow_mut();
                            s.queue = tracks;
                            s.source_name = title;
                        }
                        play_queue_index(Rc::clone(&state), 0);
                    }
                    Ok(_) => state.borrow().show_toast("El álbum no tiene pistas disponibles."),
                    Err(err) => state.borrow().show_toast(&err),
                }
            }
        });
    });

    btn
}

pub fn build_playlist_card(playlist: &Playlist, state: Rc<RefCell<AppState>>) -> Button {
    let (cover_frame, cover) = clamped_cover(140, 140, "cover-art");

    if let Some(url) = &playlist.cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(&state));
    }

    let title_label = Label::builder()
        .label(&playlist.name)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["heading"])
        .build();

    let sub_label = Label::builder()
        .label(&playlist.subtitle)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["dim-label", "caption"])
        .build();

    let card_box = GtkBox::new(Orientation::Vertical, 6);
    card_box.set_width_request(140);
    card_box.append(&cover_frame);
    card_box.append(&title_label);
    card_box.append(&sub_label);

    let btn = Button::builder()
        .child(&card_box)
        .css_classes(["flat", "media-card"])
        .build();

    let playlist_id = playlist.id.clone();
    let playlist_name = playlist.name.clone();
    btn.connect_clicked(move |_| {
        let backend = state.borrow().backend.clone();
        state
            .borrow()
            .show_toast(&format!("Cargando playlist: {playlist_name}"));

        let (tx, rx) = async_channel::bounded(1);
        let id = playlist_id.clone();
        runtime().spawn(async move {
            let res = backend.fetch_playlist_tracks(&id).await;
            let _ = tx.send(res).await;
        });

        let state = Rc::clone(&state);
        glib::spawn_future_local(async move {
            if let Ok(result) = rx.recv().await {
                match result {
                    Ok((name, tracks)) if !tracks.is_empty() => {
                        {
                            let mut s = state.borrow_mut();
                            s.queue = tracks;
                            s.source_name = name;
                        }
                        play_queue_index(Rc::clone(&state), 0);
                    }
                    Ok(_) => state.borrow().show_toast("La playlist está vacía."),
                    Err(err) => state.borrow().show_toast(&err),
                }
            }
        });
    });

    btn
}

pub fn populate_track_list(
    list: &ListBox,
    tracks: &[Track],
    source_name: &str,
    state: Rc<RefCell<AppState>>,
) {
    clear_list_box(list);
    for (idx, track) in tracks.iter().enumerate() {
        let row = build_track_row(track, tracks, idx, source_name, Rc::clone(&state));
        list.append(&row);
    }
}

pub fn build_track_row(
    track: &Track,
    section_tracks: &[Track],
    section_index: usize,
    source_name: &str,
    state: Rc<RefCell<AppState>>,
) -> ListBoxRow {
    let row_box = GtkBox::new(Orientation::Horizontal, 12);
    row_box.add_css_class("track-row");

    let (cover_frame, cover) = clamped_cover(44, 44, "cover-art");

    if let Some(url) = &track.cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(&state));
    }

    let subtitle = match &track.album {
        Some(album) if !album.is_empty() => format!("{} • {album}", track.artist),
        _ => track.artist.clone(),
    };

    let title_label = Label::builder()
        .label(&track.title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"])
        .build();

    let subtitle_label = Label::builder()
        .label(&subtitle)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["dim-label", "caption"])
        .build();

    let text_box = GtkBox::new(Orientation::Vertical, 2);
    text_box.set_hexpand(true);
    text_box.set_valign(Align::Center);
    text_box.append(&title_label);
    text_box.append(&subtitle_label);

    let duration_str = track
        .duration_secs
        .map(format_duration)
        .unwrap_or_else(|| String::from("--:--"));
    let duration_label = Label::builder()
        .label(&duration_str)
        .valign(Align::Center)
        .css_classes(["dim-label", "caption", "numeric"])
        .build();

    let is_fav = state.borrow().is_favorite(&track.id);
    let fav_btn = ToggleButton::builder()
        .icon_name(if is_fav {
            "starred-symbolic"
        } else {
            "non-starred-symbolic"
        })
        .active(is_fav)
        .tooltip_text("Favorito")
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        let track = track.clone();
        fav_btn.connect_toggled(move |btn| {
            btn.set_icon_name(if btn.is_active() {
                "starred-symbolic"
            } else {
                "non-starred-symbolic"
            });
            set_track_favorite(Rc::clone(&state), &track, btn.is_active());
        });
    }

    let queue_btn = Button::builder()
        .icon_name("list-add-symbolic")
        .tooltip_text("Agregar a la cola")
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        let track = track.clone();
        queue_btn.connect_clicked(move |_| {
            let should_start = {
                let mut s = state.borrow_mut();
                s.queue.push(track.clone());
                s.update_nav_buttons();
                s.show_toast(&format!("Agregado a la cola: {}", track.title));
                s.current_index.is_none()
            };
            refresh_queue_ui(Rc::clone(&state));
            if should_start {
                play_queue_index(Rc::clone(&state), 0);
            }
        });
    }

    let add_pl_popover = Popover::new();
    let add_pl_btn = MenuButton::builder()
        .icon_name("playlist-add-symbolic")
        .tooltip_text("Agregar a playlist")
        .valign(Align::Center)
        .popover(&add_pl_popover)
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        let track = track.clone();
        let pop = add_pl_popover.clone();
        add_pl_popover.connect_show(move |_| {
            let box_pl = GtkBox::new(Orientation::Vertical, 4);
            box_pl.set_margin_top(6);
            box_pl.set_margin_bottom(6);
            box_pl.set_margin_start(6);
            box_pl.set_margin_end(6);

            let playlists = state.borrow().user_playlists.clone();
            if playlists.is_empty() {
                box_pl.append(
                    &Label::builder()
                        .label("Crea una playlist en Colección")
                        .css_classes(["dim-label", "caption"])
                        .margin_top(4)
                        .margin_bottom(4)
                        .margin_start(6)
                        .margin_end(6)
                        .build(),
                );
            } else {
                for pl in playlists {
                    let item = Button::builder()
                        .label(&pl.name)
                        .css_classes(["flat"])
                        .build();
                    let state = Rc::clone(&state);
                    let track = track.clone();
                    let pl_id = pl.id.clone();
                    let pop_inner = pop.clone();
                    item.connect_clicked(move |_| {
                        {
                            let mut s = state.borrow_mut();
                            if let Some(target) =
                                s.user_playlists.iter_mut().find(|p| p.id == pl_id)
                            {
                                if !target.tracks.iter().any(|t| t.id == track.id) {
                                    target.tracks.push(track.clone());
                                    let name = target.name.clone();
                                    save_user_playlists(&s.user_playlists);
                                    s.show_toast(&format!("Agregado a {name}"));
                                } else {
                                    s.show_toast("Ya está en la playlist.");
                                }
                            }
                        }
                        refresh_playlists_ui(Rc::clone(&state));
                        pop_inner.popdown();
                    });
                    box_pl.append(&item);
                }
            }
            pop.set_child(Some(&box_pl));
        });
    }

    let radio_btn = Button::builder()
        .icon_name("media-playlist-consecutive-symbolic")
        .tooltip_text("Iniciar radio")
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        let track = track.clone();
        radio_btn.connect_clicked(move |_| {
            start_track_radio(Rc::clone(&state), &track);
        });
    }

    let play_btn = Button::builder()
        .icon_name("media-playback-start-symbolic")
        .tooltip_text("Reproducir ahora")
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        let section = section_tracks.to_vec();
        let source = source_name.to_string();
        play_btn.connect_clicked(move |_| {
            {
                let mut s = state.borrow_mut();
                s.queue = section.clone();
                s.source_name = source.clone();
            }
            play_queue_index(Rc::clone(&state), section_index);
        });
    }

    row_box.append(&cover_frame);
    row_box.append(&text_box);
    row_box.append(&duration_label);
    row_box.append(&fav_btn);
    row_box.append(&add_pl_btn);
    row_box.append(&radio_btn);
    row_box.append(&queue_btn);
    row_box.append(&play_btn);

    ListBoxRow::builder().child(&row_box).build()
}

pub fn set_track_favorite(state: Rc<RefCell<AppState>>, track: &Track, fav: bool) {
    {
        let mut s = state.borrow_mut();
        if fav {
            if !s.is_favorite(&track.id) {
                s.favorites.push(track.clone());
            }
        } else {
            s.favorites.retain(|t| t.id != track.id);
        }

        if s.current_track().is_some_and(|cur| cur.id == track.id) {
            s.player_fav_btn.set_icon_name(if fav {
                "starred-symbolic"
            } else {
                "non-starred-symbolic"
            });
        }
        save_favorites(&s.favorites);
    }
    refresh_favorites_ui(state);
}

pub fn start_track_radio(state: Rc<RefCell<AppState>>, track: &Track) {
    let backend = state.borrow().backend.clone();
    state
        .borrow()
        .show_toast(&format!("Generando radio para: {}", track.title));

    let (tx, rx) = async_channel::bounded(1);
    let id = track.id.clone();
    let title = track.title.clone();
    runtime().spawn(async move {
        let res = backend.fetch_radio_tracks(&id).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        if let Ok(result) = rx.recv().await {
            match result {
                Ok(tracks) if !tracks.is_empty() => {
                    {
                        let mut s = state.borrow_mut();
                        s.queue = tracks;
                        s.source_name = format!("Radio • {title}");
                    }
                    play_queue_index(Rc::clone(&state), 0);
                }
                Ok(_) => state.borrow().show_toast("No se encontraron canciones para la radio."),
                Err(err) => state.borrow().show_toast(&err),
            }
        }
    });
}

pub fn clamped_cover(min_size: i32, max_size: i32, css_class: &str) -> (adw::Clamp, Picture) {
    let picture = Picture::builder()
        .width_request(min_size)
        .height_request(min_size)
        .content_fit(ContentFit::Cover)
        .overflow(Overflow::Hidden)
        .build();
    picture.add_css_class(css_class);

    let v_clamp = adw::Clamp::builder()
        .orientation(Orientation::Vertical)
        .maximum_size(max_size)
        .tightening_threshold(max_size)
        .child(&picture)
        .build();

    let h_clamp = adw::Clamp::builder()
        .orientation(Orientation::Horizontal)
        .maximum_size(max_size)
        .tightening_threshold(max_size)
        .child(&v_clamp)
        .build();

    (h_clamp, picture)
}

pub fn load_cover_into_picture(picture: &Picture, url: &str, state: Rc<RefCell<AppState>>) {
    if let Some(texture) = state.borrow().cover_cache.get(url).cloned() {
        picture.set_paintable(Some(&texture));
        return;
    }

    let backend = state.borrow().backend.clone();
    let url_owned = url.to_string();
    let (tx, rx) = async_channel::bounded(1);

    runtime().spawn(async move {
        let res = backend.fetch_image_bytes(&url_owned).await;
        let _ = tx.send(res).await;
    });

    let picture = picture.clone();
    let url_key = url.to_string();
    glib::spawn_future_local(async move {
        if let Ok(Ok(bytes)) = rx.recv().await {
            let glib_bytes = glib::Bytes::from_owned(bytes);
            if let Ok(texture) = gdk::Texture::from_bytes(&glib_bytes) {
                state.borrow_mut().cover_cache.insert(url_key, texture.clone());
                picture.set_paintable(Some(&texture));
            }
        }
    });
}

pub fn load_player_cover(
    picture: &Picture,
    url: &str,
    expected_gen: u64,
    state: Rc<RefCell<AppState>>,
) {
    let cached = state.borrow().cover_cache.get(url).cloned();
    if let Some(texture) = cached {
        let s = state.borrow();
        if s.loading_generation == expected_gen {
            picture.set_paintable(Some(&texture));
            s.player_cover_spinner.set_spinning(false);
            s.player_cover_spinner.set_visible(false);
        }
        return;
    }

    let backend = state.borrow().backend.clone();
    let url_owned = url.to_string();
    let (tx, rx) = async_channel::bounded(1);

    runtime().spawn(async move {
        let res = backend.fetch_image_bytes(&url_owned).await;
        let _ = tx.send(res).await;
    });

    let picture = picture.clone();
    let url_key = url.to_string();
    glib::spawn_future_local(async move {
        let maybe_tex = match rx.recv().await {
            Ok(Ok(bytes)) => gdk::Texture::from_bytes(&glib::Bytes::from_owned(bytes)).ok(),
            _ => None,
        };
        if let Some(texture) = &maybe_tex {
            state
                .borrow_mut()
                .cover_cache
                .insert(url_key, texture.clone());
        }
        let s = state.borrow();
        if s.loading_generation == expected_gen {
            if let Some(texture) = maybe_tex {
                picture.set_paintable(Some(&texture));
            }
            s.player_cover_spinner.set_spinning(false);
            s.player_cover_spinner.set_visible(false);
        }
    });
}
