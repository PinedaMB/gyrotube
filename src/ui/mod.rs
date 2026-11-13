pub mod collection;
pub mod explore;
pub mod header_search;
pub mod home;
pub mod player;
pub mod responsive;
pub mod shortcuts;
pub mod sidebar_player;
pub mod state;
pub mod widgets;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, CssProvider, Image, Label, ListBox, MediaFile, Orientation,
    SelectionMode, ToggleButton,
};
use libadwaita as adw;

use crate::backend::Backend;
use crate::models::{
    load_favorites, load_recent_tracks, load_search_history, load_session_state,
    load_user_playlists, save_user_playlists,
};
use crate::mpris::{MprisCommand, MprisController};
use crate::ui::collection::{
    build_collection_page, refresh_favorites_ui, refresh_playlists_ui, refresh_recent_ui,
};
use crate::ui::explore::build_explore_page;
use crate::ui::header_search::{build_header_bar, install_unfocus_on_click, wire_search_popover};
use crate::ui::home::build_home_page;
use crate::ui::player::{
    load_current_lyrics, play_queue_index, refresh_queue_ui, restore_last_track,
    scroll_to_active_lyric, wire_media_signals,
};
use crate::ui::responsive::{ResponsiveTargets, build_main_overlay, setup_responsive};
use crate::ui::shortcuts::{ShortcutTargets, install_shortcuts};
use crate::ui::sidebar_player::{build_mini_bar, build_sidebar_player};
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{empty_state_card, set_track_favorite, start_track_radio};

const APP_ID: &str = "com.github.gyrotube.GyroTube";

pub fn run() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

pub fn build_ui(app: &adw::Application) {
    load_css();

    let backend = Backend::new();
    let decks = [MediaFile::new(), MediaFile::new()];
    let toast_overlay = adw::ToastOverlay::new();
    let saved_session = load_session_state();
    let initial_sidebar_collapsed = saved_session.sidebar_collapsed;
    let initial_source_name = if saved_session.source_name.is_empty() {
        String::from("Sin reproducción")
    } else {
        saved_session.source_name
    };

    let sb = build_sidebar_player();

    // Right pane: navigation tabs + view stack
    let content_view_stack = adw::ViewStack::new();
    let make_nav_tab = |icon_name: &str, text: &str, active: bool| -> ToggleButton {
        let inner = GtkBox::new(Orientation::Horizontal, 6);
        inner.append(&Image::from_icon_name(icon_name));
        inner.append(&Label::new(Some(text)));
        let btn = ToggleButton::builder()
            .child(&inner)
            .active(active)
            .css_classes(["flat", "pill-switch"])
            .build();
        btn.set_cursor_from_name(Some("pointer"));
        btn
    };
    let nav_tab_home = make_nav_tab("go-home-symbolic", "Inicio", true);
    let nav_tab_explore = make_nav_tab("system-search-symbolic", "Explorar", false);
    let nav_tab_collection = make_nav_tab("library-music-symbolic", "Colección", false);
    nav_tab_explore.set_group(Some(&nav_tab_home));
    nav_tab_collection.set_group(Some(&nav_tab_home));

    let nav_pill_group = GtkBox::new(Orientation::Horizontal, 4);
    nav_pill_group.add_css_class("nav-pill-group");
    nav_pill_group.set_halign(Align::Center);
    nav_pill_group.append(&nav_tab_home);
    nav_pill_group.append(&nav_tab_explore);
    nav_pill_group.append(&nav_tab_collection);

    let top_nav_bar = GtkBox::new(Orientation::Horizontal, 0);
    top_nav_bar.add_css_class("top-nav-bar");
    top_nav_bar.set_halign(Align::Center);
    top_nav_bar.append(&nav_pill_group);

    for (tab, page) in [
        (&nav_tab_home, "home"),
        (&nav_tab_explore, "explore"),
        (&nav_tab_collection, "collection"),
    ] {
        let stack = content_view_stack.clone();
        tab.connect_toggled(move |btn| {
            if btn.is_active() {
                stack.set_visible_child_name(page);
            }
        });
    }
    {
        let nav_tab_home = nav_tab_home.clone();
        let nav_tab_explore = nav_tab_explore.clone();
        let nav_tab_collection = nav_tab_collection.clone();
        content_view_stack.connect_visible_child_name_notify(move |stack| {
            match stack.visible_child_name().as_deref() {
                Some("home") => nav_tab_home.set_active(true),
                Some("explore") => nav_tab_explore.set_active(true),
                Some("collection") => nav_tab_collection.set_active(true),
                _ => {}
            }
        });
    }

    let hdr = build_header_bar(&content_view_stack);

    // Collection lists
    let favorites_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .visible(false)
        .build();
    let favorites_empty_label = empty_state_card(
        "starred-symbolic",
        "Aún no tienes canciones favoritas",
        "Pulsa el ícono de estrella en cualquier canción para guardarla.",
    );
    let recent_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    let recent_empty_label = empty_state_card(
        "document-open-recent-symbolic",
        "Sin historial reciente",
        "Las canciones que reproduzcas aparecerán aquí.",
    );
    let playlists_box = GtkBox::new(Orientation::Vertical, 8);
    let playlists_empty_label = empty_state_card(
        "folder-music-symbolic",
        "No tienes playlists locales",
        "Escribe un nombre arriba y pulsa Crear para organizar tus canciones.",
    );

    let state = Rc::new(RefCell::new(AppState {
        backend: backend.clone(),
        decks: decks.clone(),
        active_deck: 0,
        user_volume: saved_session.user_volume.clamp(0.0, 1.0),
        crossfade_enabled: true,
        crossfade_active: false,
        crossfade_generation: 0,
        crossfade_triggered_for: None,
        queue: saved_session.queue,
        favorites: load_favorites(),
        recent_tracks: load_recent_tracks(),
        user_playlists: load_user_playlists(),
        search_history: load_search_history(),
        current_index: saved_session.current_index,
        source_name: initial_source_name,
        shuffle: saved_session.shuffle,
        repeat: saved_session.repeat,
        cover_cache: HashMap::new(),
        toast_overlay: toast_overlay.clone(),
        player_source_label: sb.player_source_label.clone(),
        player_cover: sb.player_cover.clone(),
        player_cover_spinner: sb.player_cover_spinner.clone(),
        player_title: sb.player_title.clone(),
        player_artist: sb.player_artist.clone(),
        player_fav_btn: sb.player_fav_btn.clone(),
        player_radio_btn: sb.player_radio_btn.clone(),
        quality_label: sb.quality_label.clone(),
        play_pause_btn: sb.play_pause_btn.clone(),
        play_spinner: sb.play_spinner.clone(),
        prev_btn: sb.prev_btn.clone(),
        next_btn: sb.next_btn.clone(),
        seek_scale: sb.seek_scale.clone(),
        time_label: sb.time_label.clone(),
        duration_label: sb.duration_label.clone(),
        lyrics_title_label: sb.lyrics_title_label.clone(),
        lyrics_artist_label: sb.lyrics_artist_label.clone(),
        lyrics_sync_badge: sb.lyrics_sync_badge.clone(),
        lyrics_body_label: sb.lyrics_body_label.clone(),
        lyrics_box: sb.lyrics_lines_box.clone(),
        lyrics_scroll: sb.lyrics_scroll.clone(),
        lyrics_resync_btn: sb.lyrics_resync_btn.clone(),
        synced_lyrics: Vec::new(),
        active_lyric_idx: None,
        lyrics_auto_scroll: true,
        lyrics_programmatic_scroll: Rc::new(Cell::new(false)),
        lyrics_loaded_for: None,
        queue_list: sb.queue_list.clone(),
        queue_empty_label: sb.queue_empty_label.clone(),
        queue_title_label: sb.queue_title_label.clone(),
        favorites_list,
        favorites_empty_label,
        recent_list,
        recent_empty_label,
        playlists_box,
        playlists_empty_label,
        mpris: None,
        pending_autoplay: false,
        sidebar_collapsed: initial_sidebar_collapsed,
        loading_generation: 0,
        preloading_id: None,
        on_track_changed: Vec::new(),
    }));

    sb.volume_scale
        .set_value(saved_session.user_volume.clamp(0.0, 1.0));
    sb.shuffle_btn.set_active(saved_session.shuffle);
    sb.repeat_btn.set_active(saved_session.repeat);

    // Lyrics scroll & resync
    {
        let state = Rc::clone(&state);
        sb.lyrics_scroll
            .vadjustment()
            .connect_value_changed(move |_| {
                if let Ok(mut s) = state.try_borrow_mut() {
                    if s.lyrics_programmatic_scroll.get() || s.synced_lyrics.is_empty() {
                        return;
                    }
                    if s.lyrics_auto_scroll {
                        s.lyrics_auto_scroll = false;
                        s.lyrics_resync_btn.set_visible(true);
                    }
                }
            });
    }
    {
        let state = Rc::clone(&state);
        sb.lyrics_resync_btn.connect_clicked(move |_| {
            let mut s = state.borrow_mut();
            s.lyrics_auto_scroll = true;
            s.lyrics_resync_btn.set_visible(false);
            scroll_to_active_lyric(&s);
        });
    }

    // Build right-pane pages
    let (home_page, quick_grid) = build_home_page(Rc::clone(&state), content_view_stack.clone());
    let (explore_page, trigger_explore_search) =
        build_explore_page(Rc::clone(&state), hdr.search_entry.clone());
    let collection_page = build_collection_page(Rc::clone(&state));
    refresh_favorites_ui(Rc::clone(&state));
    refresh_recent_ui(Rc::clone(&state));
    refresh_playlists_ui(Rc::clone(&state));

    content_view_stack
        .add_titled_with_icon(&home_page, Some("home"), "Inicio", "go-home-symbolic");
    content_view_stack.add_titled_with_icon(
        &explore_page,
        Some("explore"),
        "Explorar",
        "system-search-symbolic",
    );
    content_view_stack.add_titled_with_icon(
        &collection_page,
        Some("collection"),
        "Colección",
        "library-music-symbolic",
    );

    let (search_focus_ctrl, search_popover, allow_search_focus) = wire_search_popover(
        Rc::clone(&state),
        &hdr.search_entry,
        &content_view_stack,
        trigger_explore_search,
    );

    // Wire sidebar actions (queue save/clear, radio, lyrics, shuffle/repeat, fav)
    wire_sidebar_actions(&state, &sb);
    wire_media_signals(Rc::clone(&state), &sb.volume_scale, &sb.volume_btn);

    // Mini players (docked on mobile, floating on tablet/PC)
    let (mobile_mini_bar, mobile_open_btn) =
        build_mini_bar(&state, &sb.play_pause_btn, &sb.next_btn, false);
    mobile_mini_bar.add_css_class("mobile-mini-player");

    let content_toolbar = adw::ToolbarView::new();
    content_toolbar.add_top_bar(&top_nav_bar);
    content_toolbar.set_content(Some(&content_view_stack));
    content_toolbar.add_bottom_bar(&mobile_mini_bar);

    let (mini_player_bar, mini_open_btn) =
        build_mini_bar(&state, &sb.play_pause_btn, &sb.next_btn, true);
    mini_player_bar.set_halign(Align::End);
    mini_player_bar.set_valign(Align::End);
    mini_player_bar.set_margin_end(18);
    mini_player_bar.set_margin_bottom(18);
    mini_player_bar.add_css_class("floating-mini-player");

    let split_view = adw::OverlaySplitView::builder()
        .sidebar(&sb.sidebar_toolbar)
        .content(&content_toolbar)
        .min_sidebar_width(280.0)
        .max_sidebar_width(360.0)
        .sidebar_width_fraction(0.32)
        .show_sidebar(!initial_sidebar_collapsed)
        .build();

    let (main_overlay, bottom_drawer_revealer) =
        build_main_overlay(&split_view, &mini_player_bar);

    restore_last_track(Rc::clone(&state), saved_session.position_secs);

    let main_toolbar = adw::ToolbarView::new();
    main_toolbar.add_top_bar(&hdr.header_bar);
    main_toolbar.set_content(Some(&main_overlay));
    toast_overlay.set_child(Some(&main_toolbar));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("GyroTube")
        .default_width(1100)
        .default_height(700)
        .width_request(360)   // Ancho mínimo para pantallas móviles
        .height_request(294)  // Alto mínimo requerido por Libadwaita
        .content(&toast_overlay)
        .build();

    setup_responsive(ResponsiveTargets {
        window: window.clone(),
        split_view,
        sidebar_toolbar: sb.sidebar_toolbar,
        bottom_drawer_revealer,
        mini_player_bar,
        mobile_mini_bar,
        mini_open_btn,
        mobile_open_btn,
        sidebar_collapse_btn: sb.sidebar_collapse_btn,
        brand_title: hdr.brand_title,
        brand_badge: hdr.brand_badge,
        top_search_entry: hdr.search_entry.clone(),
        search_focus_ctrl: search_focus_ctrl.clone(),
        quick_grid,
        state: Rc::clone(&state),
    });

    install_unfocus_on_click(
        &window,
        &hdr.search_entry,
        &search_focus_ctrl,
        &search_popover,
        &allow_search_focus,
    );

    wire_mpris(&state, &window);

    {
        let state_close = Rc::clone(&state);
        window.connect_close_request(move |_| {
            state_close.borrow().save_session();
            glib::Propagation::Proceed
        });
    }

    install_shortcuts(ShortcutTargets {
        window: window.clone(),
        search_entry: hdr.search_entry,
        allow_search_focus,
        tab_player: sb.tab_player,
        tab_lyrics: sb.tab_lyrics,
        play_pause_btn: sb.play_pause_btn,
        prev_btn: sb.prev_btn,
        next_btn: sb.next_btn,
        content_stack: content_view_stack,
    });

    window.present();
}

fn wire_sidebar_actions(
    state: &Rc<RefCell<AppState>>,
    sb: &sidebar_player::SidebarPlayerWidgets,
) {
    {
        let state = Rc::clone(state);
        sb.queue_save_btn.connect_clicked(move |_| {
            let saved = {
                let mut s = state.borrow_mut();
                if s.queue.is_empty() {
                    s.show_toast("La cola está vacía.");
                    false
                } else {
                    let id = format!(
                        "local-{}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_millis())
                            .unwrap_or(0)
                    );
                    let name = if s.source_name.trim().is_empty() {
                        String::from("Cola guardada")
                    } else {
                        s.source_name.clone()
                    };
                    let tracks = s.queue.clone();
                    s.user_playlists.push(crate::models::UserPlaylist {
                        id,
                        name: name.clone(),
                        tracks,
                    });
                    save_user_playlists(&s.user_playlists);
                    s.show_toast(&format!("Playlist guardada: {name}"));
                    true
                }
            };
            if saved {
                refresh_playlists_ui(Rc::clone(&state));
            }
        });
    }
    {
        let state = Rc::clone(state);
        sb.player_radio_btn.connect_clicked(move |_| {
            if let Some(track) = state.borrow().current_track().cloned() {
                start_track_radio(Rc::clone(&state), &track);
            }
        });
    }
    {
        let state = Rc::clone(state);
        sb.tab_lyrics.connect_toggled(move |btn| {
            if btn.is_active() {
                load_current_lyrics(Rc::clone(&state));
            }
        });
    }
    {
        let state_cb = Rc::clone(state);
        let tab_lyrics = sb.tab_lyrics.clone();
        state
            .borrow_mut()
            .on_track_changed
            .push(Rc::new(move || {
                if tab_lyrics.is_active() {
                    load_current_lyrics(Rc::clone(&state_cb));
                } else {
                    let mut s = state_cb.borrow_mut();
                    s.synced_lyrics.clear();
                    s.active_lyric_idx = None;
                    s.lyrics_loaded_for = None;
                    s.lyrics_resync_btn.set_visible(false);
                }
            }));
    }
    {
        let state = Rc::clone(state);
        sb.crossfade_btn.connect_toggled(move |btn| {
            state.borrow_mut().crossfade_enabled = btn.is_active();
        });
    }
    {
        let state = Rc::clone(state);
        sb.queue_clear_btn.connect_clicked(move |_| {
            let decks = {
                let mut s = state.borrow_mut();
                s.queue.clear();
                s.current_index = None;
                s.crossfade_active = false;
                s.crossfade_generation = s.crossfade_generation.wrapping_add(1);
                s.crossfade_triggered_for = None;
                s.play_pause_btn.set_sensitive(false);
                s.seek_scale.set_sensitive(false);
                s.player_fav_btn.set_sensitive(false);
                s.player_radio_btn.set_sensitive(false);
                s.player_title.set_label("Nada reproduciéndose");
                s.player_artist.set_label("Selecciona una canción");
                s.player_source_label.set_label("Sin reproducción");
                s.decks.clone()
            };
            decks[0].pause();
            decks[1].pause();
            {
                let s = state.borrow();
                s.player_cover.set_paintable(None::<&gdk::Paintable>);
                crate::ui::widgets::update_ambient_color(None);
                s.time_label.set_label("0:00");
                s.duration_label.set_label("0:00");
                s.quality_label.set_label("YTM");
                s.update_nav_buttons();
                s.save_session();
            }
            refresh_queue_ui(Rc::clone(&state));
            crate::ui::widgets::notify_track_changed(Rc::clone(&state));
        });
    }
    {
        let state = Rc::clone(state);
        sb.shuffle_btn.connect_toggled(move |btn| {
            let mut s = state.borrow_mut();
            s.shuffle = btn.is_active();
            s.save_session();
        });
    }
    {
        let state = Rc::clone(state);
        sb.repeat_btn.connect_toggled(move |btn| {
            let mut s = state.borrow_mut();
            s.repeat = btn.is_active();
            s.update_nav_buttons();
            s.save_session();
        });
    }
    {
        let state = Rc::clone(state);
        sb.player_fav_btn.connect_toggled(move |btn| {
            if let Some(track) = state.borrow().current_track().cloned() {
                set_track_favorite(Rc::clone(&state), &track, btn.is_active());
            }
        });
    }
}

fn wire_mpris(state: &Rc<RefCell<AppState>>, window: &adw::ApplicationWindow) {
    let state_mpris = Rc::clone(state);
    let win_weak = window.downgrade();
    let mpris = MprisController::spawn(move |cmd| match cmd {
        MprisCommand::Play => state_mpris.borrow().active_media().play(),
        MprisCommand::Pause => state_mpris.borrow().active_media().pause(),
        MprisCommand::PlayPause => {
            let media = state_mpris.borrow().active_media().clone();
            if media.is_playing() {
                media.pause();
            } else {
                media.play();
            }
        }
        MprisCommand::Next => {
            let next_idx = next_queue_index(&state_mpris.borrow());
            if let Some(idx) = next_idx {
                play_queue_index(Rc::clone(&state_mpris), idx);
            }
        }
        MprisCommand::Previous => {
            let prev_idx = {
                let s = state_mpris.borrow();
                s.current_index.filter(|&i| i > 0).map(|i| i - 1)
            };
            if let Some(idx) = prev_idx {
                play_queue_index(Rc::clone(&state_mpris), idx);
            }
        }
        MprisCommand::Stop => {
            let decks = state_mpris.borrow().decks.clone();
            decks[0].pause();
            decks[1].pause();
        }
        MprisCommand::SeekOffsetUs(offset_us) => {
            let (media, mpris) = {
                let s = state_mpris.borrow();
                (s.active_media().clone(), s.mpris.clone())
            };
            let target = (media.timestamp() + offset_us).max(0);
            media.seek(target);
            if let Some(m) = mpris {
                m.emit_seeked(target);
            }
        }
        MprisCommand::SetPositionUs(pos_us) => {
            let (media, mpris) = {
                let s = state_mpris.borrow();
                (s.active_media().clone(), s.mpris.clone())
            };
            let target = pos_us.max(0);
            media.seek(target);
            if let Some(m) = mpris {
                m.emit_seeked(target);
            }
        }
        MprisCommand::Raise => {
            if let Some(w) = win_weak.upgrade() {
                w.present();
            }
        }
    });
    state.borrow_mut().mpris = Some(mpris);
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../style.css"));
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
