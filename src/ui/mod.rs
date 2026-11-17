pub mod album_page;
pub mod collection;
pub mod explore;
pub mod header_search;
pub mod home;
pub mod jam_dialog;
pub mod party_mode;
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
    Align, Box as GtkBox, CssProvider, Image, Label, ListBox, Orientation, SelectionMode,
    ToggleButton,
};
use libadwaita as adw;

use crate::backend::Backend;
use crate::gst_deck::GStreamerDeck;
use crate::i18n::{set_language_preference, tr};
use crate::models::{
    load_favorites, load_recent_tracks, load_search_history, load_session_state,
    load_user_playlists, save_user_playlists,
};
use crate::mpris::{MprisCommand, MprisController};
use crate::ui::collection::{
    build_collection_page, refresh_favorites_ui, refresh_playlists_ui, refresh_recent_ui,
};
use crate::ui::explore::build_explore_page;
use crate::ui::header_search::{
    build_header_bar, install_unfocus_on_click, wire_search_popover, wire_settings_dialog,
};
use crate::ui::home::build_home_page;
use crate::ui::jam_dialog::{JamRuntime, open_jam_dialog, sync_jam_snapshot, wire_jam_commands};
use crate::ui::party_mode::{
    build_party_mode, enter_party_mode, exit_party_mode, refresh_party_next_up,
};
use crate::ui::player::{
    load_current_lyrics, play_queue_index, refresh_queue_ui, restore_last_track,
    scroll_to_active_lyric, wire_media_signals,
};
use crate::ui::responsive::{ResponsiveTargets, build_main_overlay, setup_responsive};
use crate::ui::shortcuts::{ShortcutTargets, install_shortcuts};
use crate::ui::sidebar_player::{build_mini_bar, build_sidebar_player};
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{empty_state_card, set_track_favorite, start_track_radio};

pub const APP_ID: &str = "com.github.gyrotube.GyroTube";
const APP_ICON_SVG: &[u8] = include_bytes!("../../gyrotube.svg");

fn register_app_icon() {
    if let Some(cache) = crate::models::cache_dir() {
        let icons_root = cache.join("icons");
        let apps_dir = icons_root.join("hicolor/scalable/apps");
        if std::fs::create_dir_all(&apps_dir).is_ok() {
            let icon_path = apps_dir.join(format!("{APP_ID}.svg"));
            let needs_write = std::fs::metadata(&icon_path)
                .map(|m| m.len() != APP_ICON_SVG.len() as u64)
                .unwrap_or(true);
            if needs_write {
                let _ = std::fs::write(&icon_path, APP_ICON_SVG);
            }
            if let Some(display) = gdk::Display::default() {
                gtk4::IconTheme::for_display(&display).add_search_path(&icons_root);
            }
        }
    }
    gtk4::Window::set_default_icon_name(APP_ID);
}

pub fn run() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

pub fn build_ui(app: &adw::Application) {
    load_css();
    relm4_icons::initialize_icons(
        crate::icon_names::GRESOURCE_BYTES,
        crate::icon_names::RESOURCE_PREFIX,
    );
    register_app_icon();

    let backend = Backend::new();
    let decks = [GStreamerDeck::new(), GStreamerDeck::new()];
    let toast_overlay = adw::ToastOverlay::new();
    let saved_session = load_session_state();
    set_language_preference(saved_session.language);
    let initial_sidebar_collapsed = saved_session.sidebar_collapsed;
    let initial_source_name = if saved_session.source_name.is_empty() {
        String::from(tr("Sin reproducción", "Not playing"))
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
    let nav_tab_home = make_nav_tab("home-regular", tr("Inicio", "Home"), true);
    let nav_tab_explore = make_nav_tab("search-regular", tr("Explorar", "Explore"), false);
    let nav_tab_collection = make_nav_tab("library-regular", tr("Colección", "Collection"), false);
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
        "heart-filled",
        tr("Aún no tienes canciones favoritas", "No favorite songs yet"),
        tr(
            "Pulsa el ícono de estrella en cualquier canción para guardarla.",
            "Click the heart icon on any song to save it.",
        ),
    );
    let recent_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    let recent_empty_label = empty_state_card(
        "document-open-recent-symbolic",
        tr("Sin historial reciente", "No recent history"),
        tr(
            "Las canciones que reproduzcas aparecerán aquí.",
            "Songs you play will appear here.",
        ),
    );
    let playlists_box = GtkBox::new(Orientation::Vertical, 8);
    let playlists_empty_label = empty_state_card(
        "music-note-2-regular",
        tr("No tienes playlists locales", "No local playlists yet"),
        tr(
            "Escribe un nombre arriba y pulsa Crear para organizar tus canciones.",
            "Enter a name above and click Create to organize your songs.",
        ),
    );

    let (jam_cmd_tx, jam_cmd_rx) = async_channel::unbounded();
    let initial_cf_secs = saved_session.crossfade_secs.min(10);
    let state = Rc::new(RefCell::new(AppState {
        backend: backend.clone(),
        decks: decks.clone(),
        active_deck: 0,
        user_volume: saved_session.user_volume.clamp(0.0, 1.0),
        crossfade_enabled: initial_cf_secs > 0,
        crossfade_secs: initial_cf_secs,
        crossfade_active: false,
        crossfade_generation: 0,
        crossfade_triggered_for: None,
        queue: saved_session.queue,
        unshuffled_queue: Vec::new(),
        favorites: load_favorites(),
        recent_tracks: load_recent_tracks(),
        user_playlists: load_user_playlists(),
        search_history: load_search_history(),
        current_index: saved_session.current_index,
        source_name: initial_source_name,
        shuffle: saved_session.shuffle,
        repeat: saved_session.repeat,
        language: saved_session.language,
        cover_cache: HashMap::new(),
        toast_overlay: toast_overlay.clone(),
        player_source_label: sb.player_source_label.clone(),
        player_cover: sb.player_cover.clone(),
        player_cover_spinner: sb.player_cover_spinner.clone(),
        player_title: sb.player_title.clone(),
        player_artist: sb.player_artist.clone(),
        player_album_label: sb.player_album_label.clone(),
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
        party_active: false,
        party_lyrics_box: GtkBox::new(Orientation::Vertical, 0),
        party_lyrics_scroll: gtk4::ScrolledWindow::new(),
        party_lyrics_body_label: Label::new(None),
        party_synced_lyrics: Vec::new(),
        party_next_box: GtkBox::new(Orientation::Vertical, 0),
        party_next_cover: gtk4::Picture::new(),
        party_next_title: Label::new(None),
        party_next_artist: Label::new(None),
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
        pending_restore_seek: None,
        jam: JamRuntime::new(jam_cmd_tx),
        on_track_changed: Vec::new(),
        on_open_collection: None,
        on_cancel_collection_loading: None,
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
    let (explore_page, explore_actions) =
        build_explore_page(Rc::clone(&state), hdr.search_entry.clone());
    let collection_page = build_collection_page(Rc::clone(&state));
    refresh_favorites_ui(Rc::clone(&state));
    refresh_recent_ui(Rc::clone(&state));
    refresh_playlists_ui(Rc::clone(&state));

    content_view_stack
        .add_titled_with_icon(&home_page, Some("home"), tr("Inicio", "Home"), "home-regular");
    content_view_stack.add_titled_with_icon(
        &explore_page,
        Some("explore"),
        tr("Explorar", "Explore"),
        "search-regular",
    );
    content_view_stack.add_titled_with_icon(
        &collection_page,
        Some("collection"),
        tr("Colección", "Collection"),
        "library-regular",
    );

    let (album_page, open_album_page, cancel_album_loading) = album_page::build_album_page(
        Rc::clone(&state),
        content_view_stack.clone(),
        Rc::clone(&explore_actions.open_artist),
    );
    content_view_stack.add_named(&album_page, Some("album"));
    {
        let mut s = state.borrow_mut();
        s.on_open_collection = Some(open_album_page);
        s.on_cancel_collection_loading = Some(cancel_album_loading);
    }

    let (search_focus_ctrl, search_popover, allow_search_focus) = wire_search_popover(
        Rc::clone(&state),
        &hdr.search_entry,
        &content_view_stack,
        Rc::clone(&explore_actions.search),
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
        .min_sidebar_width(320.0)
        .max_sidebar_width(430.0)
        .sidebar_width_fraction(0.35)
        .show_sidebar(!initial_sidebar_collapsed)
        .build();

    let (main_overlay, bottom_drawer_revealer) =
        build_main_overlay(&split_view, &mini_player_bar);

    {
        let open_artist = Rc::clone(&explore_actions.open_artist);
        let stack = content_view_stack.clone();
        let drawer = bottom_drawer_revealer.clone();
        let sv = split_view.clone();
        sb.player_artist.connect_activate_link(move |_, uri| {
            drawer.set_reveal_child(false);
            if sv.is_collapsed() {
                sv.set_show_sidebar(false);
            }
            stack.set_visible_child_name("explore");
            open_artist(uri);
            glib::Propagation::Stop
        });

        for album_label in [&sb.player_album_label, &sb.player_source_label] {
            let open_album = Rc::clone(&explore_actions.open_album);
            let state_cb = Rc::clone(&state);
            let drawer = bottom_drawer_revealer.clone();
            let sv = split_view.clone();
            album_label.connect_activate_link(move |_, uri| {
                drawer.set_reveal_child(false);
                if sv.is_collapsed() {
                    sv.set_show_sidebar(false);
                }
                let artist = {
                    let s = state_cb.borrow();
                    s.current_index
                        .and_then(|i| s.queue.get(i))
                        .map(|t| t.artist.clone())
                        .unwrap_or_default()
                };
                open_album(uri, &artist);
                glib::Propagation::Stop
            });
        }
    }

    restore_last_track(Rc::clone(&state), saved_session.position_secs);

    let main_toolbar = adw::ToolbarView::new();
    main_toolbar.add_top_bar(&hdr.header_bar);
    wire_settings_dialog(
        &hdr.settings_btn,
        &hdr.menu_popover,
        Rc::clone(&state),
    );
    let party = build_party_mode(&state, &sb.shuffle_btn, &sb.repeat_btn);
    {
        let mut s = state.borrow_mut();
        s.party_lyrics_box = party.lyrics_box.clone();
        s.party_lyrics_scroll = party.lyrics_scroll.clone();
        s.party_lyrics_body_label = party.lyrics_body_label.clone();
        s.party_next_box = party.next_box.clone();
        s.party_next_cover = party.next_cover.clone();
        s.party_next_title = party.next_title.clone();
        s.party_next_artist = party.next_artist.clone();
    }

    main_toolbar.set_content(Some(&main_overlay));
    let root_stack = gtk4::Stack::builder()
        .transition_type(gtk4::StackTransitionType::Crossfade)
        .build();
    root_stack.add_named(&main_toolbar, Some("main"));
    root_stack.add_named(&party.root, Some("party"));
    toast_overlay.set_child(Some(&root_stack));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("GyroTube")
        .default_width(1100)
        .default_height(700)
        .width_request(360)   // Ancho mínimo para pantallas móviles
        .height_request(294)  // Alto mínimo requerido por Libadwaita
        .content(&toast_overlay)
        .build();

    let enter_party = {
        let state = Rc::clone(&state);
        let root_stack = root_stack.clone();
        let window = window.clone();
        let fs_btn = party.fullscreen_btn.clone();
        Rc::new(move || {
            enter_party_mode(&state, &root_stack, &window, &fs_btn);
        })
    };
    let exit_party = {
        let state = Rc::clone(&state);
        let root_stack = root_stack.clone();
        let window = window.clone();
        Rc::new(move || {
            if state.borrow().party_active {
                exit_party_mode(&state, &root_stack, &window);
                true
            } else {
                false
            }
        })
    };
    {
        let ep = Rc::clone(&enter_party);
        sb.party_btn.connect_clicked(move |_| ep());
    }
    {
        let ep = Rc::clone(&enter_party);
        hdr.party_btn.connect_clicked(move |_| ep());
    }
    {
        let st_jam = Rc::clone(&state);
        hdr.jam_btn.connect_clicked(move |btn| {
            open_jam_dialog(btn, Rc::clone(&st_jam), btn.clone());
        });
    }
    wire_jam_commands(Rc::clone(&state), jam_cmd_rx);
    {
        let ex = Rc::clone(&exit_party);
        party.exit_btn.connect_clicked(move |_| {
            ex();
        });
    }
    {
        let win = window.clone();
        party.fullscreen_btn.connect_clicked(move |btn| {
            if win.is_fullscreen() {
                win.unfullscreen();
                btn.set_icon_name("full-screen-maximize-regular");
            } else {
                win.fullscreen();
                btn.set_icon_name("full-screen-minimize-regular");
            }
        });
    }

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

    let on_toggle_party: Rc<dyn Fn()> = {
        let state = Rc::clone(&state);
        let ep = Rc::clone(&enter_party);
        let ex = Rc::clone(&exit_party);
        Rc::new(move || {
            if state.borrow().party_active {
                ex();
            } else {
                ep();
            }
        })
    };

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
        on_toggle_party,
        on_exit_party: exit_party,
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
                refresh_party_next_up(&state_cb);
                sync_jam_snapshot(&state_cb.borrow());
                let party_active = state_cb.borrow().party_active;
                if tab_lyrics.is_active() || party_active {
                    load_current_lyrics(Rc::clone(&state_cb));
                } else {
                    let mut s = state_cb.borrow_mut();
                    s.synced_lyrics.clear();
                    s.party_synced_lyrics.clear();
                    s.active_lyric_idx = None;
                    s.lyrics_loaded_for = None;
                    s.lyrics_resync_btn.set_visible(false);
                }
            }));
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
                s.player_artist.set_text("Selecciona una canción");
                s.player_album_label.set_visible(false);
                s.player_source_label.set_text("Sin reproducción");
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
            let active = btn.is_active();
            {
                let mut s = state.borrow_mut();
                if s.shuffle == active {
                    return;
                }
                s.set_shuffle_mode(active);
            }
            refresh_queue_ui(Rc::clone(&state));
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
            let track = state.borrow().current_track().cloned();
            if let Some(track) = track {
                set_track_favorite(Rc::clone(&state), &track, btn.is_active());
            }
        });
    }
    wire_sleep_timer(state, &sb.sleep_timer_btn);
}

fn wire_sleep_timer(state: &Rc<RefCell<AppState>>, btn: &gtk4::MenuButton) {
    let timer_source: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
    let stop_on_end: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let pop = gtk4::Popover::new();
    let box_pop = GtkBox::new(Orientation::Vertical, 4);
    box_pop.set_margin_top(6);
    box_pop.set_margin_bottom(6);
    box_pop.set_margin_start(6);
    box_pop.set_margin_end(6);

    let clear_timer = {
        let timer_source = Rc::clone(&timer_source);
        let stop_on_end = Rc::clone(&stop_on_end);
        let btn = btn.clone();
        Rc::new(move || {
            if let Some(id) = timer_source.borrow_mut().take() {
                id.remove();
            }
            stop_on_end.set(false);
            btn.remove_css_class("accent");
            btn.set_tooltip_text(Some("Temporizador de apagado"));
        })
    };

    for (label, mins) in [
        ("Desactivado", 0u32),
        ("15 minutos", 15),
        ("30 minutos", 30),
        ("45 minutos", 45),
        ("60 minutos", 60),
    ] {
        let item = gtk4::Button::builder()
            .label(label)
            .css_classes(["flat"])
            .build();
        if let Some(lbl) = item.child().and_downcast::<Label>() {
            lbl.set_xalign(0.0);
        }
        let state = Rc::clone(state);
        let pop = pop.clone();
        let clear = Rc::clone(&clear_timer);
        let timer_source = Rc::clone(&timer_source);
        let btn = btn.clone();
        item.connect_clicked(move |_| {
            clear();
            if mins == 0 {
                state.borrow().show_toast("Temporizador desactivado");
            } else {
                btn.add_css_class("accent");
                btn.set_tooltip_text(Some(&format!("Temporizador activo: {mins} min")));
                state
                    .borrow()
                    .show_toast(&format!("Temporizador: {mins} minutos"));
                let state_cb = Rc::clone(&state);
                let timer_ref = Rc::clone(&timer_source);
                let btn_cb = btn.clone();
                let id = glib::timeout_add_seconds_local_once(mins * 60, move || {
                    timer_ref.borrow_mut().take();
                    btn_cb.remove_css_class("accent");
                    btn_cb.set_tooltip_text(Some("Temporizador de apagado"));
                    let s = state_cb.borrow();
                    s.decks[0].pause();
                    s.decks[1].pause();
                    s.show_toast("Temporizador: reproducción detenida");
                });
                *timer_source.borrow_mut() = Some(id);
            }
            pop.popdown();
        });
        box_pop.append(&item);
    }

    let end_track_btn = gtk4::Button::builder()
        .label("Al terminar la canción")
        .css_classes(["flat"])
        .build();
    if let Some(lbl) = end_track_btn.child().and_downcast::<Label>() {
        lbl.set_xalign(0.0);
    }
    {
        let state = Rc::clone(state);
        let pop = pop.clone();
        let clear = Rc::clone(&clear_timer);
        let stop_on_end = Rc::clone(&stop_on_end);
        let btn = btn.clone();
        end_track_btn.connect_clicked(move |_| {
            clear();
            stop_on_end.set(true);
            btn.add_css_class("accent");
            btn.set_tooltip_text(Some("Detener al terminar la canción"));
            state
                .borrow()
                .show_toast("Se detendrá al terminar la canción");
            pop.popdown();
        });
    }
    box_pop.append(&end_track_btn);
    pop.set_child(Some(&box_pop));
    btn.set_popover(Some(&pop));

    {
        let state_cb = Rc::clone(state);
        let stop_on_end = Rc::clone(&stop_on_end);
        let btn = btn.clone();
        state.borrow_mut().on_track_changed.push(Rc::new(move || {
            if stop_on_end.replace(false) {
                btn.remove_css_class("accent");
                btn.set_tooltip_text(Some("Temporizador de apagado"));
                let s = state_cb.borrow();
                s.decks[0].pause();
                s.decks[1].pause();
                s.show_toast("Temporizador: reproducción detenida");
            }
        }));
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
