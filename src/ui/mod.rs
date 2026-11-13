pub mod collection;
pub mod explore;
pub mod home;
pub mod player;
pub mod state;
pub mod widgets;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, CssProvider, Image, Justification, Label, ListBox, MediaFile,
    MenuButton, Orientation, PolicyType, Popover, Scale, ScrolledWindow, SearchEntry,
    SelectionMode, Spinner, Stack, StackTransitionType, ToggleButton,
};
use libadwaita as adw;

use crate::backend::Backend;
use crate::models::{
    load_favorites, load_recent_tracks, load_search_history, load_user_playlists,
    save_search_history,
};
use crate::mpris::{MprisCommand, MprisController};
use crate::ui::collection::{
    build_collection_page, refresh_favorites_ui, refresh_playlists_ui, refresh_recent_ui,
};
use crate::ui::explore::build_explore_page;
use crate::ui::home::build_home_page;
use crate::ui::player::{
    load_current_lyrics, play_queue_index, refresh_queue_ui, wire_media_signals,
};
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{clamped_cover, set_track_favorite, start_track_radio};

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

    // --- LEFT PANE: Dedicated Player / Lyrics / Queue Sidebar ---
    // Context header ("Reproduciendo desde ...")
    let player_source_label = Label::builder()
        .label("Sin reproducción")
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading", "caption"])
        .build();

    let context_text = GtkBox::new(Orientation::Vertical, 1);
    context_text.set_hexpand(true);
    context_text.append(
        &Label::builder()
            .label("Reproduciendo desde")
            .xalign(0.0)
            .css_classes(["dim-label", "caption"])
            .build(),
    );
    context_text.append(&player_source_label);

    let context_row = GtkBox::new(Orientation::Horizontal, 10);
    context_row.append(&context_text);

    // Large Album Art
    let (player_cover_frame, player_cover) = clamped_cover(250, 280, "cover-art-large");
    let player_cover_spinner = Spinner::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(36)
        .height_request(36)
        .visible(false)
        .build();
    let player_cover_overlay = gtk4::Overlay::builder()
        .child(&player_cover_frame)
        .halign(Align::Center)
        .valign(Align::Center)
        .css_classes(["player-cover-glow"])
        .vexpand(true)
        .build();
    player_cover_overlay.add_overlay(&player_cover_spinner);

    // Track title & artist
    let player_title = Label::builder()
        .label("Nada reproduciéndose")
        .halign(Align::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(28)
        .css_classes(["title-3"])
        .build();

    let player_artist = Label::builder()
        .label("Selecciona una canción")
        .halign(Align::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(30)
        .css_classes(["dim-label"])
        .build();

    // Volume + Favorite row
    let volume_scale = Scale::with_range(Orientation::Vertical, 0.0, 1.0, 0.05);
    volume_scale.set_inverted(true);
    volume_scale.set_draw_value(false);
    volume_scale.set_height_request(120);
    volume_scale.set_value(1.0);

    let volume_popover = Popover::builder().child(&volume_scale).build();
    let volume_btn = MenuButton::builder()
        .icon_name("audio-volume-high-symbolic")
        .tooltip_text("Volumen")
        .popover(&volume_popover)
        .css_classes(["flat", "circular"])
        .build();

    let player_fav_btn = ToggleButton::builder()
        .icon_name("non-starred-symbolic")
        .tooltip_text("Agregar a favoritos")
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let player_radio_btn = Button::builder()
        .icon_name("media-playlist-consecutive-symbolic")
        .tooltip_text("Iniciar radio desde esta canción")
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let crossfade_btn = ToggleButton::builder()
        .label("CF")
        .active(true)
        .tooltip_text("Crossfade automático (4 s)")
        .css_classes(["flat", "circular", "caption"])
        .build();

    let action_icons_row = GtkBox::new(Orientation::Horizontal, 16);
    action_icons_row.set_halign(Align::Center);
    action_icons_row.append(&volume_btn);
    action_icons_row.append(&player_fav_btn);
    action_icons_row.append(&player_radio_btn);
    action_icons_row.append(&crossfade_btn);

    // Seek bar + time / quality row
    let seek_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    seek_scale.set_draw_value(false);
    seek_scale.set_hexpand(true);
    seek_scale.set_sensitive(false);

    let time_label = Label::builder()
        .label("0:00")
        .css_classes(["caption", "numeric", "dim-label"])
        .build();

    let quality_label = Label::builder()
        .label("YTM")
        .halign(Align::Center)
        .hexpand(true)
        .css_classes(["quality-pill"])
        .build();

    let duration_label = Label::builder()
        .label("0:00")
        .css_classes(["caption", "numeric", "dim-label"])
        .build();

    let time_row = GtkBox::new(Orientation::Horizontal, 8);
    time_row.append(&time_label);
    time_row.append(&quality_label);
    time_row.append(&duration_label);

    // Transport buttons
    let shuffle_btn = ToggleButton::builder()
        .icon_name("media-playlist-shuffle-symbolic")
        .tooltip_text("Aleatorio")
        .css_classes(["flat", "circular"])
        .build();

    let prev_btn = Button::builder()
        .icon_name("media-skip-backward-symbolic")
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let play_spinner = Spinner::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    let play_pause_btn = Button::builder()
        .icon_name("media-playback-start-symbolic")
        .sensitive(false)
        .css_classes(["suggested-action", "play-pill"])
        .build();

    let next_btn = Button::builder()
        .icon_name("media-skip-forward-symbolic")
        .sensitive(false)
        .css_classes(["flat", "circular"])
        .build();

    let repeat_btn = ToggleButton::builder()
        .icon_name("media-playlist-repeat-symbolic")
        .tooltip_text("Repetir")
        .css_classes(["flat", "circular"])
        .build();

    let transport_row = GtkBox::new(Orientation::Horizontal, 10);
    transport_row.set_halign(Align::Center);
    transport_row.append(&shuffle_btn);
    transport_row.append(&prev_btn);
    transport_row.append(&play_pause_btn);
    transport_row.append(&next_btn);
    transport_row.append(&repeat_btn);

    let player_view = GtkBox::new(Orientation::Vertical, 12);
    player_view.set_margin_top(10);
    player_view.set_margin_bottom(12);
    player_view.set_margin_start(18);
    player_view.set_margin_end(18);
    player_view.set_vexpand(true);
    player_view.append(&context_row);
    player_view.append(&player_cover_overlay);
    player_view.append(&player_title);
    player_view.append(&player_artist);
    player_view.append(&action_icons_row);
    player_view.append(&seek_scale);
    player_view.append(&time_row);
    player_view.append(&transport_row);

    // Sidebar Lyrics View
    let lyrics_title_label = Label::builder()
        .label("Letras")
        .xalign(0.0)
        .css_classes(["title-4"])
        .build();
    let lyrics_body_label = Label::builder()
        .label("Reproduce una canción para ver su letra.")
        .xalign(0.0)
        .yalign(0.0)
        .wrap(true)
        .selectable(true)
        .css_classes(["body"])
        .build();
    let lyrics_box = GtkBox::new(Orientation::Vertical, 12);
    lyrics_box.set_margin_top(14);
    lyrics_box.set_margin_bottom(14);
    lyrics_box.set_margin_start(16);
    lyrics_box.set_margin_end(16);
    lyrics_box.append(&lyrics_title_label);
    lyrics_box.append(&lyrics_body_label);

    let lyrics_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&lyrics_box)
        .build();

    // Sidebar Queue View
    let queue_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    let queue_empty_label = Label::builder()
        .label("La cola está vacía.")
        .justify(Justification::Center)
        .margin_top(36)
        .css_classes(["dim-label"])
        .build();

    let queue_clear_btn = Button::builder()
        .label("Limpiar")
        .css_classes(["flat"])
        .build();
    let queue_header = GtkBox::new(Orientation::Horizontal, 8);
    queue_header.append(
        &Label::builder()
            .label("Cola")
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["title-4"])
            .build(),
    );
    queue_header.append(&queue_clear_btn);

    let queue_box = GtkBox::new(Orientation::Vertical, 10);
    queue_box.set_margin_top(12);
    queue_box.set_margin_bottom(12);
    queue_box.set_margin_start(12);
    queue_box.set_margin_end(12);
    queue_box.append(&queue_header);
    queue_box.append(&queue_empty_label);
    queue_box.append(&queue_list);

    let queue_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&queue_box)
        .build();

    let sidebar_stack = Stack::builder()
        .transition_type(StackTransitionType::Crossfade)
        .vexpand(true)
        .build();
    sidebar_stack.add_named(&player_view, Some("player"));
    sidebar_stack.add_named(&lyrics_scroll, Some("lyrics"));
    sidebar_stack.add_named(&queue_scroll, Some("queue"));

    // Bottom 3-tab switcher in sidebar
    let tab_player = make_sidebar_tab("audio-x-generic-symbolic", "Reproductor", true);
    let tab_lyrics = make_sidebar_tab("chat-message-new-symbolic", "Letras", false);
    let tab_queue = make_sidebar_tab("view-list-symbolic", "Cola", false);
    tab_lyrics.set_group(Some(&tab_player));
    tab_queue.set_group(Some(&tab_player));

    let sidebar_tabs = GtkBox::new(Orientation::Horizontal, 6);
    sidebar_tabs.set_homogeneous(true);
    sidebar_tabs.add_css_class("sidebar-tabs");
    sidebar_tabs.append(&tab_player);
    sidebar_tabs.append(&tab_lyrics);
    sidebar_tabs.append(&tab_queue);

    let sidebar_toolbar = adw::ToolbarView::new();
    sidebar_toolbar.set_content(Some(&sidebar_stack));
    sidebar_toolbar.add_bottom_bar(&sidebar_tabs);
    sidebar_toolbar.add_css_class("sidebar-pane");

    // --- RIGHT PANE: Main Content ---
    let content_view_stack = adw::ViewStack::new();
    let content_switcher = adw::ViewSwitcher::builder()
        .stack(&content_view_stack)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .halign(Align::Center)
        .build();

    let top_nav_bar = GtkBox::new(Orientation::Horizontal, 8);
    top_nav_bar.add_css_class("top-nav-bar");
    top_nav_bar.set_halign(Align::Fill);
    content_switcher.set_hexpand(true);
    top_nav_bar.append(&content_switcher);

    // --- UNIFIED CUSTOM TOP HEADER BAR ---
    let sidebar_toggle_btn = ToggleButton::builder()
        .icon_name("sidebar-show-symbolic")
        .active(true)
        .tooltip_text("Mostrar u ocultar reproductor")
        .css_classes(["flat", "circular"])
        .build();

    let brand_icon = Image::builder()
        .icon_name("media-playback-start-symbolic")
        .pixel_size(14)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let brand_icon_box = GtkBox::new(Orientation::Horizontal, 0);
    brand_icon_box.add_css_class("brand-icon");
    brand_icon_box.set_halign(Align::Center);
    brand_icon_box.set_valign(Align::Center);
    brand_icon_box.append(&brand_icon);

    let brand_title = Label::builder()
        .label("GyroTube")
        .css_classes(["brand-title"])
        .build();
    let brand_badge = Label::builder()
        .label("Música")
        .css_classes(["brand-badge"])
        .build();

    let brand_box = GtkBox::new(Orientation::Horizontal, 8);
    brand_box.set_valign(Align::Center);
    brand_box.append(&sidebar_toggle_btn);
    brand_box.append(&brand_icon_box);
    brand_box.append(&brand_title);
    brand_box.append(&brand_badge);

    let top_search_entry = SearchEntry::builder()
        .placeholder_text("Buscar canciones, álbumes, artistas…")
        .css_classes(["header-search"])
        .build();

    let nav_home_btn = Button::builder()
        .label("Inicio")
        .css_classes(["flat"])
        .build();
    let nav_explore_btn = Button::builder()
        .label("Explorar")
        .css_classes(["flat"])
        .build();
    let nav_collection_btn = Button::builder()
        .label("Colección")
        .css_classes(["flat"])
        .build();

    let menu_box = GtkBox::new(Orientation::Vertical, 4);
    menu_box.set_margin_top(6);
    menu_box.set_margin_bottom(6);
    menu_box.set_margin_start(6);
    menu_box.set_margin_end(6);
    menu_box.append(&nav_home_btn);
    menu_box.append(&nav_explore_btn);
    menu_box.append(&nav_collection_btn);

    let profile_popover = Popover::builder().child(&menu_box).build();
    let profile_btn = MenuButton::builder()
        .icon_name("avatar-default-symbolic")
        .tooltip_text("Menú de GyroTube")
        .popover(&profile_popover)
        .css_classes(["profile-pill"])
        .build();

    let main_header = adw::HeaderBar::builder()
        .title_widget(&top_search_entry)
        .build();
    main_header.pack_start(&brand_box);
    main_header.pack_end(&profile_btn);

    // Favorites / Collection widgets
    let favorites_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .visible(false)
        .build();
    let favorites_empty_label = Label::builder()
        .label("Aún no tienes canciones favoritas.\nPulsa el ícono de estrella en cualquier canción para guardarla.")
        .justify(Justification::Center)
        .margin_top(48)
        .css_classes(["dim-label"])
        .build();

    let recent_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    let recent_empty_label = Label::builder()
        .label("Aún no has reproducido canciones.")
        .justify(Justification::Center)
        .margin_top(16)
        .margin_bottom(16)
        .css_classes(["dim-label"])
        .build();

    let playlists_box = GtkBox::new(Orientation::Vertical, 8);
    let playlists_empty_label = Label::builder()
        .label("No tienes playlists locales. Crea una arriba para organizar tus canciones.")
        .justify(Justification::Center)
        .margin_top(16)
        .margin_bottom(16)
        .css_classes(["dim-label"])
        .build();

    let state = Rc::new(RefCell::new(AppState {
        backend: backend.clone(),
        decks: decks.clone(),
        active_deck: 0,
        user_volume: 1.0,
        crossfade_enabled: true,
        crossfade_active: false,
        crossfade_generation: 0,
        crossfade_triggered_for: None,
        queue: Vec::new(),
        favorites: load_favorites(),
        recent_tracks: load_recent_tracks(),
        user_playlists: load_user_playlists(),
        search_history: load_search_history(),
        current_index: None,
        source_name: String::from("YouTube Music"),
        shuffle: false,
        repeat: false,
        cover_cache: HashMap::new(),
        toast_overlay: toast_overlay.clone(),
        player_source_label,
        player_cover,
        player_cover_spinner,
        player_title,
        player_artist,
        player_fav_btn: player_fav_btn.clone(),
        player_radio_btn: player_radio_btn.clone(),
        quality_label,
        play_pause_btn: play_pause_btn.clone(),
        play_spinner,
        prev_btn: prev_btn.clone(),
        next_btn: next_btn.clone(),
        seek_scale,
        time_label,
        duration_label,
        lyrics_title_label,
        lyrics_body_label,
        lyrics_loaded_for: None,
        queue_list,
        queue_empty_label,
        favorites_list,
        favorites_empty_label,
        recent_list,
        recent_empty_label,
        playlists_box,
        playlists_empty_label,
        mpris: None,
        pending_autoplay: false,
        loading_generation: 0,
        on_track_changed: Vec::new(),
    }));

    // Build right-pane pages
    let home_page = build_home_page(Rc::clone(&state), content_view_stack.clone());
    let (explore_page, explore_search_entry, trigger_explore_search) =
        build_explore_page(Rc::clone(&state));
    let collection_page = build_collection_page(Rc::clone(&state));
    refresh_favorites_ui(Rc::clone(&state));
    refresh_recent_ui(Rc::clone(&state));
    refresh_playlists_ui(Rc::clone(&state));

    {
        let state = Rc::clone(&state);
        player_radio_btn.connect_clicked(move |_| {
            let track_opt = state.borrow().current_track().cloned();
            if let Some(track) = track_opt {
                start_track_radio(Rc::clone(&state), &track);
            }
        });
    }

    content_view_stack
        .add_titled_with_icon(&home_page, Some("home"), "Inicio", "go-home-symbolic");
    content_view_stack.add_titled_with_icon(
        &explore_page,
        Some("explore"),
        "Explorar",
        "compass2-symbolic",
    );
    content_view_stack.add_titled_with_icon(
        &collection_page,
        Some("collection"),
        "Colección",
        "library-music-symbolic",
    );

    // Quick search bar in header forwards to Explore search + shows recent searches popover
    let search_popover = Popover::builder().autohide(false).build();
    search_popover.set_parent(&top_search_entry);
    let run_header_search: Rc<dyn Fn(&str)> = {
        let content_view_stack = content_view_stack.clone();
        let explore_search_entry = explore_search_entry.clone();
        let state = Rc::clone(&state);
        let search_popover = search_popover.clone();
        Rc::new(move |query: &str| {
            let trimmed = query.trim();
            if trimmed.is_empty() {
                return;
            }
            {
                let mut s = state.borrow_mut();
                s.search_history.retain(|q| !q.eq_ignore_ascii_case(trimmed));
                s.search_history.insert(0, trimmed.to_string());
                s.search_history.truncate(8);
                save_search_history(&s.search_history);
            }
            search_popover.popdown();
            explore_search_entry.set_text(trimmed);
            content_view_stack.set_visible_child_name("explore");
            trigger_explore_search(trimmed);
        })
    };
    {
        let run_header_search = Rc::clone(&run_header_search);
        top_search_entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            run_header_search(&text);
        });
    }
    {
        let state = Rc::clone(&state);
        let search_popover = search_popover.clone();
        let top_entry_ref = top_search_entry.clone();
        let run_header_search = Rc::clone(&run_header_search);
        let focus_ctrl = gtk4::EventControllerFocus::new();
        focus_ctrl.connect_enter(move |_| {
            let history = state.borrow().search_history.clone();
            if history.is_empty() {
                return;
            }
            let box_hist = GtkBox::new(Orientation::Vertical, 4);
            box_hist.set_margin_top(6);
            box_hist.set_margin_bottom(6);
            box_hist.set_margin_start(6);
            box_hist.set_margin_end(6);
            box_hist.append(
                &Label::builder()
                    .label("Búsquedas recientes")
                    .xalign(0.0)
                    .margin_start(6)
                    .css_classes(["dim-label", "caption"])
                    .build(),
            );
            for term in history {
                let item = Button::builder()
                    .label(&term)
                    .css_classes(["flat", "search-popover-row"])
                    .build();
                let top_entry = top_entry_ref.clone();
                let run_fn = Rc::clone(&run_header_search);
                let term_clone = term.clone();
                item.connect_clicked(move |_| {
                    top_entry.set_text(&term_clone);
                    run_fn(&term_clone);
                });
                box_hist.append(&item);
            }
            search_popover.set_child(Some(&box_hist));
            search_popover.popup();
        });
        top_search_entry.add_controller(focus_ctrl);
    }
    for (btn, page) in [
        (&nav_home_btn, "home"),
        (&nav_explore_btn, "explore"),
        (&nav_collection_btn, "collection"),
    ] {
        let content_view_stack = content_view_stack.clone();
        let profile_popover = profile_popover.clone();
        btn.connect_clicked(move |_| {
            content_view_stack.set_visible_child_name(page);
            profile_popover.popdown();
        });
    }

    // Sidebar tab switching + lazy lyrics loading
    {
        let sidebar_stack = sidebar_stack.clone();
        tab_player.connect_toggled(move |btn| {
            if btn.is_active() {
                sidebar_stack.set_visible_child_name("player");
            }
        });
    }
    {
        let sidebar_stack = sidebar_stack.clone();
        let state = Rc::clone(&state);
        tab_lyrics.connect_toggled(move |btn| {
            if btn.is_active() {
                sidebar_stack.set_visible_child_name("lyrics");
                load_current_lyrics(Rc::clone(&state));
            }
        });
    }
    {
        let sidebar_stack = sidebar_stack.clone();
        tab_queue.connect_toggled(move |btn| {
            if btn.is_active() {
                sidebar_stack.set_visible_child_name("queue");
            }
        });
    }

    // Clear queue
    {
        let state = Rc::clone(&state);
        crossfade_btn.connect_toggled(move |btn| {
            let mut s = state.borrow_mut();
            s.crossfade_enabled = btn.is_active();
        });
    }
    {
        let state = Rc::clone(&state);
        queue_clear_btn.connect_clicked(move |_| {
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
                s.time_label.set_label("0:00");
                s.duration_label.set_label("0:00");
                s.quality_label.set_label("YTM");
                s.update_nav_buttons();
            }
            refresh_queue_ui(Rc::clone(&state));
        });
    }

    // Shuffle & Repeat toggles
    {
        let state = Rc::clone(&state);
        shuffle_btn.connect_toggled(move |btn| {
            state.borrow_mut().shuffle = btn.is_active();
        });
    }
    {
        let state = Rc::clone(&state);
        repeat_btn.connect_toggled(move |btn| {
            let mut s = state.borrow_mut();
            s.repeat = btn.is_active();
            s.update_nav_buttons();
        });
    }

    // Favorite toggle in player sidebar
    {
        let state = Rc::clone(&state);
        player_fav_btn.connect_toggled(move |btn| {
            let track_opt = state.borrow().current_track().cloned();
            if let Some(track) = track_opt {
                set_track_favorite(Rc::clone(&state), &track, btn.is_active());
            }
        });
    }

    wire_media_signals(Rc::clone(&state), &volume_scale, &volume_btn);

    let content_toolbar = adw::ToolbarView::new();
    content_toolbar.add_top_bar(&top_nav_bar);
    content_toolbar.set_content(Some(&content_view_stack));

    let split_view = adw::OverlaySplitView::builder()
        .sidebar(&sidebar_toolbar)
        .content(&content_toolbar)
        .min_sidebar_width(320.0)
        .max_sidebar_width(370.0)
        .sidebar_width_fraction(0.32)
        .show_sidebar(true)
        .build();

    sidebar_toggle_btn
        .bind_property("active", &split_view, "show-sidebar")
        .bidirectional()
        .sync_create()
        .build();

    let main_toolbar = adw::ToolbarView::new();
    main_toolbar.add_top_bar(&main_header);
    main_toolbar.set_content(Some(&split_view));

    toast_overlay.set_child(Some(&main_toolbar));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("GyroTube")
        .default_width(1100)
        .default_height(700)
        .content(&toast_overlay)
        .build();

    let mpris = {
        let state_mpris = Rc::clone(&state);
        let win_weak = window.downgrade();
        MprisController::spawn(move |cmd| match cmd {
            MprisCommand::Play => {
                let media = state_mpris.borrow().active_media().clone();
                media.play();
            }
            MprisCommand::Pause => {
                let media = state_mpris.borrow().active_media().clone();
                media.pause();
            }
            MprisCommand::PlayPause => {
                let media = state_mpris.borrow().active_media().clone();
                if media.is_playing() {
                    media.pause();
                } else {
                    media.play();
                }
            }
            MprisCommand::Stop => {
                let decks = state_mpris.borrow().decks.clone();
                decks[0].pause();
                decks[1].pause();
                state_mpris.borrow().active_media().seek(0);
            }
            MprisCommand::Next => {
                let target = next_queue_index(&state_mpris.borrow());
                if let Some(idx) = target {
                    play_queue_index(Rc::clone(&state_mpris), idx);
                }
            }
            MprisCommand::Previous => {
                let target = state_mpris
                    .borrow()
                    .current_index
                    .and_then(|idx| idx.checked_sub(1));
                if let Some(idx) = target {
                    play_queue_index(Rc::clone(&state_mpris), idx);
                }
            }
            MprisCommand::SeekOffsetUs(offset_us) => {
                let (media, mpris) = {
                    let s = state_mpris.borrow();
                    (s.active_media().clone(), s.mpris.clone())
                };
                let cur = media.timestamp().max(0);
                let target = (cur + offset_us).max(0);
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
        })
    };
    state.borrow_mut().mpris = Some(mpris);

    // Global keyboard shortcuts (Ctrl+F, Ctrl+L, Ctrl+Left/Right, Space)
    {
        let key_ctrl = gtk4::EventControllerKey::new();
        let top_search_entry = top_search_entry.clone();
        let tab_lyrics = tab_lyrics.clone();
        let tab_player = tab_player.clone();
        let play_pause_btn = play_pause_btn.clone();
        let prev_btn = prev_btn.clone();
        let next_btn = next_btn.clone();
        let win_ref = window.clone();
        key_ctrl.connect_key_pressed(move |_, key, _, mods| {
            let ctrl = mods.contains(gdk::ModifierType::CONTROL_MASK);
            if ctrl && (key == gdk::Key::f || key == gdk::Key::F) {
                top_search_entry.grab_focus();
                return glib::Propagation::Stop;
            }
            if ctrl && (key == gdk::Key::l || key == gdk::Key::L) {
                if tab_lyrics.is_active() {
                    tab_player.set_active(true);
                } else {
                    tab_lyrics.set_active(true);
                }
                return glib::Propagation::Stop;
            }
            if ctrl && key == gdk::Key::Right {
                next_btn.emit_clicked();
                return glib::Propagation::Stop;
            }
            if ctrl && key == gdk::Key::Left {
                prev_btn.emit_clicked();
                return glib::Propagation::Stop;
            }
            if key == gdk::Key::space {
                let in_text = gtk4::prelude::GtkWindowExt::focus(&win_ref)
                    .is_some_and(|w| w.is::<gtk4::Text>() || w.is::<gtk4::Entry>() || w.is::<SearchEntry>());
                if !in_text {
                    play_pause_btn.emit_clicked();
                    return glib::Propagation::Stop;
                }
            }
            glib::Propagation::Proceed
        });
        window.add_controller(key_ctrl);
    }

    window.present();
}

fn make_sidebar_tab(icon_name: &str, label_text: &str, active: bool) -> ToggleButton {
    let content = GtkBox::new(Orientation::Horizontal, 6);
    content.set_halign(Align::Center);
    content.append(&Image::from_icon_name(icon_name));
    content.append(&Label::new(Some(label_text)));

    ToggleButton::builder()
        .child(&content)
        .active(active)
        .css_classes(["flat"])
        .build()
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

