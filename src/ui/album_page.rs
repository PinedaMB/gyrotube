use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Image, Justification, Label, ListBox, ListBoxRow, MenuButton,
    Orientation, PolicyType, Popover, ScrolledWindow, SelectionMode, Separator,
};
use libadwaita as adw;

use crate::i18n::tr;
use crate::models::{Track, format_duration};
use crate::ui::collection::refresh_playlists_ui;
use crate::ui::player::{play_queue_index, refresh_queue_ui};
use crate::ui::state::AppState;
use crate::ui::widgets::{
    build_skeleton_box, clamped_cover, clear_box, clear_list_box, highlight_active_track_in_list,
    load_cover_into_picture, set_artist_links, set_track_favorite, start_track_radio,
    update_album_ambient_color,
};

pub fn build_album_page(
    state: Rc<RefCell<AppState>>,
    content_stack: adw::ViewStack,
    open_artist: Rc<dyn Fn(&str)>,
) -> (
    ScrolledWindow,
    Rc<dyn Fn(String, Option<String>, Option<Vec<Track>>)>,
    Rc<dyn Fn()>,
) {
    let previous_page: Rc<RefCell<String>> = Rc::new(RefCell::new(String::from("explore")));
    let current_tracks: Rc<RefCell<Vec<Track>>> = Rc::new(RefCell::new(Vec::new()));
    let current_title: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let loading_active: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Volver")
        .halign(Align::Start)
        .valign(Align::Start)
        .css_classes(["circular", "artist-hero-back"])
        .build();
    back_btn.set_cursor_from_name(Some("pointer"));
    {
        let stack = content_stack.clone();
        let prev = Rc::clone(&previous_page);
        let loading = Rc::clone(&loading_active);
        back_btn.connect_clicked(move |_| {
            loading.set(false);
            let target = prev.borrow().clone();
            stack.set_visible_child_name(&target);
        });
    }

    let (cover_frame, cover_pic) = clamped_cover(192, 192, "cover-art-large");
    cover_frame.set_halign(Align::Center);

    let cover_skeleton = build_skeleton_box(192, 192);
    cover_skeleton.set_halign(Align::Center);
    cover_skeleton.add_css_class("cover-art-large");
    cover_skeleton.set_visible(false);

    let badge_label = Label::builder()
        .label("ÁLBUM")
        .halign(Align::Center)
        .css_classes(["album-type-label"])
        .build();

    let title_label = Label::builder()
        .label("")
        .xalign(0.5)
        .justify(Justification::Center)
        .wrap(true)
        .max_width_chars(24)
        .css_classes(["album-hero-title"])
        .build();

    let artist_label = Label::builder()
        .label("")
        .xalign(0.5)
        .justify(gtk4::Justification::Center)
        .use_markup(true)
        .wrap(true)
        .max_width_chars(28)
        .css_classes(["heading", "player-link"])
        .visible(false)
        .build();
    {
        let stack = content_stack.clone();
        let open_artist = Rc::clone(&open_artist);
        artist_label.connect_activate_link(move |_, uri| {
            stack.set_visible_child_name("explore");
            open_artist(uri);
            glib::Propagation::Stop
        });
    }

    let meta_label = Label::builder()
        .label("")
        .xalign(0.5)
        .css_classes(["dim-label", "caption"])
        .build();

    let shuffle_btn = Button::builder()
        .icon_name("arrow-shuffle-regular")
        .tooltip_text(tr("Reproducción aleatoria", "Shuffle play"))
        .valign(Align::Center)
        .css_classes(["circular", "album-action-btn"])
        .build();
    shuffle_btn.set_cursor_from_name(Some("pointer"));

    let play_all_btn = Button::builder()
        .icon_name("play-filled")
        .tooltip_text(tr("Reproducir todo", "Play all"))
        .valign(Align::Center)
        .css_classes(["circular", "suggested-action", "album-play-fab"])
        .build();
    play_all_btn.set_cursor_from_name(Some("pointer"));

    let add_queue_btn = Button::builder()
        .icon_name("add-regular")
        .tooltip_text(tr("Añadir a la cola", "Add to queue"))
        .valign(Align::Center)
        .css_classes(["circular", "album-action-btn"])
        .build();
    add_queue_btn.set_cursor_from_name(Some("pointer"));

    let save_fav_btn = Button::builder()
        .icon_name("heart-regular")
        .tooltip_text(tr(
            "Guardar canciones en favoritos",
            "Save songs to favorites",
        ))
        .valign(Align::Center)
        .css_classes(["circular", "album-action-btn"])
        .build();
    save_fav_btn.set_cursor_from_name(Some("pointer"));

    let radio_btn = Button::builder()
        .icon_name("sound-wave-circle-regular")
        .tooltip_text(tr("Iniciar radio", "Start radio"))
        .valign(Align::Center)
        .css_classes(["circular", "album-action-btn"])
        .build();
    radio_btn.set_cursor_from_name(Some("pointer"));

    let actions_row = GtkBox::new(Orientation::Horizontal, 14);
    actions_row.set_halign(Align::Center);
    actions_row.set_margin_top(6);
    actions_row.append(&shuffle_btn);
    actions_row.append(&play_all_btn);
    actions_row.append(&add_queue_btn);
    actions_row.append(&save_fav_btn);
    actions_row.append(&radio_btn);

    let cover_wrap = GtkBox::new(Orientation::Vertical, 0);
    cover_wrap.set_halign(Align::Center);
    cover_wrap.set_valign(Align::Center);
    cover_wrap.append(&cover_frame);
    cover_wrap.append(&cover_skeleton);

    let info_box = GtkBox::new(Orientation::Vertical, 6);
    info_box.set_hexpand(true);
    info_box.append(&badge_label);
    info_box.append(&title_label);
    info_box.append(&artist_label);
    info_box.append(&meta_label);

    let hero_content = GtkBox::new(Orientation::Vertical, 10);
    hero_content.set_halign(Align::Center);
    hero_content.append(&cover_wrap);
    hero_content.append(&info_box);

    let hero_overlay = gtk4::Overlay::new();
    hero_overlay.add_css_class("album-page-hero");
    hero_overlay.set_child(Some(&hero_content));
    hero_overlay.add_overlay(&back_btn);

    let meta_skeleton = build_skeleton_box(160, 14);
    meta_skeleton.set_halign(Align::Center);
    let actions_skeleton = GtkBox::new(Orientation::Horizontal, 14);
    actions_skeleton.set_halign(Align::Center);
    for size in [40, 52, 40, 40] {
        let dot = build_skeleton_box(size, size);
        dot.add_css_class("circular");
        actions_skeleton.append(&dot);
    }
    let tracks_skeleton_list = GtkBox::new(Orientation::Vertical, 8);
    for _ in 0..6 {
        let row = GtkBox::new(Orientation::Horizontal, 10);
        row.set_margin_start(10);
        row.set_margin_end(10);
        row.set_margin_top(6);
        row.set_margin_bottom(6);
        row.append(&build_skeleton_box(20, 16));
        let col = GtkBox::new(Orientation::Vertical, 6);
        col.set_hexpand(true);
        col.append(&build_skeleton_box(0, 14));
        col.append(&build_skeleton_box(120, 11));
        row.append(&col);
        row.append(&build_skeleton_box(36, 14));
        tracks_skeleton_list.append(&row);
    }
    let skeleton_box = GtkBox::new(Orientation::Vertical, 12);
    skeleton_box.set_visible(false);
    skeleton_box.append(&meta_skeleton);
    skeleton_box.append(&actions_skeleton);
    skeleton_box.append(&tracks_skeleton_list);

    // Spotify-style column header above track list
    let list_header = GtkBox::new(Orientation::Horizontal, 10);
    list_header.add_css_class("album-list-header");
    list_header.append(
        &Label::builder()
            .label("#")
            .width_chars(2)
            .xalign(1.0)
            .css_classes(["dim-label", "caption", "numeric"])
            .build(),
    );
    list_header.append(
        &Label::builder()
            .label(tr("Título", "Title"))
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["dim-label", "caption"])
            .build(),
    );
    list_header.append(
        &Image::builder()
            .icon_name("document-open-recent-symbolic")
            .pixel_size(14)
            .margin_end(36)
            .css_classes(["dim-label"])
            .build(),
    );

    let track_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    cover_pic.connect_paintable_notify(|pic| {
        let tex = pic
            .paintable()
            .and_then(|p| p.downcast::<gtk4::gdk::Texture>().ok());
        update_album_ambient_color(tex.as_ref());
    });

    {
        let state_cb = Rc::clone(&state);
        let list = track_list.clone();
        state.borrow_mut().on_track_changed.push(Rc::new(move || {
            let active_id = state_cb.borrow().current_track().map(|t| t.id.clone());
            highlight_active_track_in_list(&list, active_id.as_deref());
        }));
    }

    let container = GtkBox::new(Orientation::Vertical, 12);
    container.set_margin_top(14);
    container.set_margin_bottom(88);
    container.set_margin_start(16);
    container.set_margin_end(16);
    container.append(&hero_overlay);
    container.append(&actions_row);
    container.append(&skeleton_box);
    container.append(&list_header);
    container.append(&track_list);

    let clamp = adw::Clamp::builder()
        .maximum_size(860)
        .child(&container)
        .build();

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&clamp)
        .build();

    // Adapt hero layout: horizontal Spotify-style on tablet/desktop (>= 540px), centered on mobile
    {
        let last_wide: Cell<Option<bool>> = Cell::new(None);
        let hero_content = hero_content.clone();
        let info_box = info_box.clone();
        let badge_label = badge_label.clone();
        let title_label = title_label.clone();
        let artist_label = artist_label.clone();
        let meta_label = meta_label.clone();
        let actions_row = actions_row.clone();
        let play_all_btn = play_all_btn.clone();
        let shuffle_btn = shuffle_btn.clone();
        scrolled.add_tick_callback(move |s, _| {
            let w = s.width();
            if w > 0 {
                let is_wide = w >= 540;
                if last_wide.get() != Some(is_wide) {
                    last_wide.set(Some(is_wide));
                    if is_wide {
                        hero_content.set_orientation(Orientation::Horizontal);
                        hero_content.set_spacing(22);
                        hero_content.set_halign(Align::Fill);
                        hero_content.set_margin_top(42);
                        info_box.set_valign(Align::End);
                        info_box.set_margin_bottom(4);
                        badge_label.set_halign(Align::Start);
                        title_label.set_xalign(0.0);
                        title_label.set_justify(Justification::Left);
                        artist_label.set_xalign(0.0);
                        artist_label.set_justify(Justification::Left);
                        meta_label.set_xalign(0.0);
                        actions_row.set_halign(Align::Start);
                        actions_row.set_margin_start(4);
                        actions_row.reorder_child_after(&play_all_btn, None::<&Button>);
                        actions_row.reorder_child_after(&shuffle_btn, Some(&play_all_btn));
                    } else {
                        hero_content.set_orientation(Orientation::Vertical);
                        hero_content.set_spacing(10);
                        hero_content.set_halign(Align::Center);
                        hero_content.set_margin_top(0);
                        info_box.set_valign(Align::Center);
                        info_box.set_margin_bottom(0);
                        badge_label.set_halign(Align::Center);
                        title_label.set_xalign(0.5);
                        title_label.set_justify(Justification::Center);
                        artist_label.set_xalign(0.5);
                        artist_label.set_justify(Justification::Center);
                        meta_label.set_xalign(0.5);
                        actions_row.set_halign(Align::Center);
                        actions_row.set_margin_start(0);
                        actions_row.reorder_child_after(&shuffle_btn, None::<&Button>);
                        actions_row.reorder_child_after(&play_all_btn, Some(&shuffle_btn));
                    }
                }
            }
            glib::ControlFlow::Continue
        });
    }

    // Wire action buttons
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_tracks);
        let title_ref = Rc::clone(&current_title);
        play_all_btn.connect_clicked(move |_| {
            let tracks = tracks_ref.borrow().clone();
            if tracks.is_empty() {
                return;
            }
            {
                let mut s = state.borrow_mut();
                s.replace_queue(tracks, title_ref.borrow().clone());
            }
            play_queue_index(Rc::clone(&state), 0);
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_tracks);
        let title_ref = Rc::clone(&current_title);
        shuffle_btn.connect_clicked(move |_| {
            let tracks = tracks_ref.borrow().clone();
            if tracks.is_empty() {
                return;
            }
            {
                let mut s = state.borrow_mut();
                s.set_shuffled_queue(tracks, title_ref.borrow().clone());
            }
            play_queue_index(Rc::clone(&state), 0);
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_tracks);
        add_queue_btn.connect_clicked(move |_| {
            let tracks = tracks_ref.borrow().clone();
            if tracks.is_empty() {
                return;
            }
            let count = tracks.len();
            {
                let mut s = state.borrow_mut();
                s.queue.extend(tracks);
                s.update_nav_buttons();
                s.save_session();
                s.show_toast(&format!("{count} canciones añadidas a la cola"));
            }
            refresh_queue_ui(Rc::clone(&state));
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_tracks);
        save_fav_btn.connect_clicked(move |btn| {
            let tracks = tracks_ref.borrow().clone();
            if tracks.is_empty() {
                return;
            }
            let all_saved = {
                let s = state.borrow();
                tracks.iter().all(|t| s.is_favorite(&t.id))
            };
            let target = !all_saved;
            for track in &tracks {
                set_track_favorite(Rc::clone(&state), track, target);
            }
            btn.set_icon_name(if target {
                "heart-filled"
            } else {
                "heart-regular"
            });
            let msg = if target {
                "Guardado en favoritos"
            } else {
                "Eliminado de favoritos"
            };
            state.borrow().show_toast(msg);
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_tracks);
        radio_btn.connect_clicked(move |_| {
            let first = tracks_ref.borrow().first().cloned();
            if let Some(track) = first {
                start_track_radio(Rc::clone(&state), &track);
            }
        });
    }

    let open_page: Rc<dyn Fn(String, Option<String>, Option<Vec<Track>>)> = {
        let state = Rc::clone(&state);
        let stack = content_stack.clone();
        let prev_ref = Rc::clone(&previous_page);
        let tracks_ref = Rc::clone(&current_tracks);
        let title_ref = Rc::clone(&current_title);
        let scrolled = scrolled.clone();
        let loading = Rc::clone(&loading_active);
        Rc::new(
            move |title: String, cover_url: Option<String>, maybe_tracks: Option<Vec<Track>>| {
                let Some(tracks) = maybe_tracks else {
                    if let Some(name) = stack.visible_child_name()
                        && name != "album"
                    {
                        *prev_ref.borrow_mut() = name.to_string();
                    }
                    loading.set(true);
                    tracks_ref.borrow_mut().clear();
                    *title_ref.borrow_mut() = title.clone();
                    cover_pic.set_paintable(None::<&gtk4::gdk::Paintable>);
                    if let Some(url) = cover_url.as_deref() {
                        load_cover_into_picture(&cover_pic, url, Rc::clone(&state));
                        cover_frame.set_visible(true);
                        cover_skeleton.set_visible(false);
                    } else {
                        cover_frame.set_visible(false);
                        cover_skeleton.set_visible(true);
                    }
                    badge_label.set_label("ÁLBUM");
                    title_label.set_label(&title);
                    artist_label.set_visible(false);
                    meta_label.set_visible(false);
                    actions_row.set_visible(false);
                    list_header.set_visible(false);
                    track_list.set_visible(false);
                    skeleton_box.set_visible(true);
                    scrolled.vadjustment().set_value(0.0);
                    stack.set_visible_child_name("album");
                    return;
                };

                if !loading.get() && stack.visible_child_name().as_deref() == Some("album") {
                    // User navigated away while loading; ignore stale response if not already on album.
                }
                let was_loading = loading.replace(false);
                if was_loading && stack.visible_child_name().as_deref() != Some("album") {
                    return;
                }
                if let Some(name) = stack.visible_child_name()
                    && name != "album"
                {
                    *prev_ref.borrow_mut() = name.to_string();
                }

                *tracks_ref.borrow_mut() = tracks.clone();
                *title_ref.borrow_mut() = title.clone();

                cover_skeleton.set_visible(false);
                cover_frame.set_visible(true);
                cover_pic.set_paintable(None::<&gtk4::gdk::Paintable>);
                if let Some(url) = cover_url.as_deref().or_else(|| {
                    tracks
                        .first()
                        .and_then(|t| t.cover_large_url.as_deref().or(t.cover_url.as_deref()))
                }) {
                    load_cover_into_picture(&cover_pic, url, Rc::clone(&state));
                }

                let is_album = tracks.iter().any(|t| {
                    t.album
                        .as_deref()
                        .is_some_and(|a| !a.is_empty() && a.eq_ignore_ascii_case(&title))
                });
                badge_label.set_label(if is_album { "ÁLBUM" } else { "PLAYLIST" });
                title_label.set_label(&title);

                let first_artist = tracks
                    .first()
                    .map(|t| t.artist.trim())
                    .filter(|a| !a.is_empty());
                if let Some(artist) = first_artist {
                    set_artist_links(&artist_label, artist);
                    artist_label.set_visible(true);
                } else {
                    artist_label.set_visible(false);
                }

                let total_secs: u64 = tracks
                    .iter()
                    .filter_map(|t| t.duration_secs.map(u64::from))
                    .sum();
                let meta = if total_secs > 0 {
                    format!(
                        "{} canciones • {}",
                        tracks.len(),
                        format_total_duration(total_secs)
                    )
                } else {
                    format!("{} canciones", tracks.len())
                };
                meta_label.set_label(&meta);
                meta_label.set_visible(true);

                let all_saved = {
                    let s = state.borrow();
                    !tracks.is_empty() && tracks.iter().all(|t| s.is_favorite(&t.id))
                };
                save_fav_btn.set_icon_name(if all_saved {
                    "heart-filled"
                } else {
                    "heart-regular"
                });

                populate_album_track_list(&track_list, &tracks, &title, Rc::clone(&state));

                skeleton_box.set_visible(false);
                actions_row.set_visible(true);
                list_header.set_visible(true);
                track_list.set_visible(true);

                scrolled.vadjustment().set_value(0.0);
                stack.set_visible_child_name("album");
            },
        )
    };

    let cancel_loading: Rc<dyn Fn()> = {
        let stack = content_stack;
        let prev_ref = Rc::clone(&previous_page);
        let loading = Rc::clone(&loading_active);
        Rc::new(move || {
            if loading.replace(false) && stack.visible_child_name().as_deref() == Some("album") {
                let target = prev_ref.borrow().clone();
                stack.set_visible_child_name(&target);
            }
        })
    };

    (scrolled, open_page, cancel_loading)
}

fn format_total_duration(total_secs: u64) -> String {
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    if hours > 0 {
        format!("{hours} h {mins} min")
    } else if mins > 0 {
        format!("{mins} min {secs} s")
    } else {
        format!("{secs} s")
    }
}

fn populate_album_track_list(
    list: &ListBox,
    tracks: &[Track],
    source_name: &str,
    state: Rc<RefCell<AppState>>,
) {
    clear_list_box(list);
    for (idx, track) in tracks.iter().enumerate() {
        let row = build_album_track_row(track, tracks, idx, source_name, Rc::clone(&state));
        list.append(&row);
    }
}

fn build_album_track_row(
    track: &Track,
    section_tracks: &[Track],
    section_index: usize,
    source_name: &str,
    state: Rc<RefCell<AppState>>,
) -> ListBoxRow {
    let row_box = GtkBox::new(Orientation::Horizontal, 10);
    row_box.add_css_class("track-row");

    let num_label = Label::builder()
        .label(format!("{}", section_index + 1))
        .width_chars(2)
        .xalign(1.0)
        .valign(Align::Center)
        .css_classes(["dim-label", "caption", "numeric"])
        .build();

    let title_label = Label::builder()
        .label(&track.title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"])
        .build();

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
        .unwrap_or_default();
    let duration_label = Label::builder()
        .label(&duration_str)
        .valign(Align::Center)
        .visible(track.duration_secs.is_some())
        .css_classes(["dim-label", "caption", "numeric"])
        .build();

    let menu_popover = Popover::new();
    let menu_btn = MenuButton::builder()
        .icon_name("view-more-symbolic")
        .tooltip_text("Opciones")
        .valign(Align::Center)
        .popover(&menu_popover)
        .css_classes(["flat", "circular"])
        .build();

    {
        let state = Rc::clone(&state);
        let track = track.clone();
        let pop = menu_popover.clone();
        menu_popover.connect_show(move |_| {
            let menu_box = GtkBox::new(Orientation::Vertical, 4);
            menu_box.set_margin_top(6);
            menu_box.set_margin_bottom(6);
            menu_box.set_margin_start(6);
            menu_box.set_margin_end(6);

            let play_next_btn = Button::builder()
                .label("Reproducir siguiente")
                .css_classes(["flat"])
                .build();
            if let Some(child) = play_next_btn.child().and_downcast::<Label>() {
                child.set_xalign(0.0);
            }
            {
                let state = Rc::clone(&state);
                let track = track.clone();
                let pop = pop.clone();
                play_next_btn.connect_clicked(move |_| {
                    let should_start = {
                        let mut s = state.borrow_mut();
                        if let Some(cur) = s.current_index {
                            let insert_at = (cur + 1).min(s.queue.len());
                            s.queue.insert(insert_at, track.clone());
                        } else {
                            s.queue.push(track.clone());
                        }
                        s.update_nav_buttons();
                        s.save_session();
                        s.show_toast(&format!("Siguiente: {}", track.title));
                        s.current_index.is_none()
                    };
                    refresh_queue_ui(Rc::clone(&state));
                    if should_start {
                        play_queue_index(Rc::clone(&state), 0);
                    }
                    pop.popdown();
                });
            }
            menu_box.append(&play_next_btn);

            let add_q_btn = Button::builder()
                .label("Añadir a la cola")
                .css_classes(["flat"])
                .build();
            if let Some(child) = add_q_btn.child().and_downcast::<Label>() {
                child.set_xalign(0.0);
            }
            {
                let state = Rc::clone(&state);
                let track = track.clone();
                let pop = pop.clone();
                add_q_btn.connect_clicked(move |_| {
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
                    pop.popdown();
                });
            }
            menu_box.append(&add_q_btn);

            let is_fav = state.borrow().is_favorite(&track.id);
            let fav_btn = Button::builder()
                .label(if is_fav {
                    "Quitar de favoritos"
                } else {
                    "Añadir a favoritos"
                })
                .css_classes(["flat"])
                .build();
            if let Some(child) = fav_btn.child().and_downcast::<Label>() {
                child.set_xalign(0.0);
            }
            {
                let state = Rc::clone(&state);
                let track = track.clone();
                let pop = pop.clone();
                fav_btn.connect_clicked(move |_| {
                    set_track_favorite(Rc::clone(&state), &track, !is_fav);
                    pop.popdown();
                });
            }
            menu_box.append(&fav_btn);

            let playlists = state.borrow().user_playlists.clone();
            if !playlists.is_empty() {
                menu_box.append(&Separator::new(Orientation::Horizontal));
                let pl_header = Label::builder()
                    .label("Añadir a playlist")
                    .xalign(0.0)
                    .margin_start(8)
                    .margin_top(2)
                    .css_classes(["dim-label", "caption"])
                    .build();
                menu_box.append(&pl_header);

                for pl in playlists {
                    let b = Button::builder()
                        .label(&pl.name)
                        .css_classes(["flat"])
                        .build();
                    if let Some(child) = b.child().and_downcast::<Label>() {
                        child.set_xalign(0.0);
                    }
                    let state = Rc::clone(&state);
                    let track = track.clone();
                    let pl_id = pl.id.clone();
                    let pl_name = pl.name.clone();
                    let pop = pop.clone();
                    b.connect_clicked(move |_| {
                        let added = {
                            let mut s = state.borrow_mut();
                            if let Some(p) = s.user_playlists.iter_mut().find(|p| p.id == pl_id) {
                                if !p.tracks.iter().any(|t| t.id == track.id) {
                                    p.tracks.push(track.clone());
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };
                        if added {
                            refresh_playlists_ui(Rc::clone(&state));
                            state.borrow().show_toast(&format!("Agregada a {pl_name}"));
                        } else {
                            state.borrow().show_toast("La canción ya está en esta playlist");
                        }
                        pop.popdown();
                    });
                    menu_box.append(&b);
                }
            }

            if let Some(old) = pop.child() {
                if let Ok(old_box) = old.downcast::<GtkBox>() {
                    clear_box(&old_box);
                }
            }
            pop.set_child(Some(&menu_box));
        });
    }

    row_box.append(&num_label);
    row_box.append(&text_box);
    row_box.append(&duration_label);
    row_box.append(&menu_btn);

    let row = ListBoxRow::new();
    row.set_child(Some(&row_box));
    row.set_widget_name(&track.id);
    if state
        .borrow()
        .current_track()
        .is_some_and(|t| t.id == track.id)
    {
        row.add_css_class("chart-active-row");
    }
    row.set_cursor_from_name(Some("pointer"));

    let click = gtk4::GestureClick::new();
    {
        let state = Rc::clone(&state);
        let tracks_vec = section_tracks.to_vec();
        let src = source_name.to_string();
        click.connect_released(move |_, _, _, _| {
            {
                let mut s = state.borrow_mut();
                s.replace_queue(tracks_vec.clone(), src.clone());
            }
            play_queue_index(Rc::clone(&state), section_index);
        });
    }
    row.add_controller(click);

    row
}
