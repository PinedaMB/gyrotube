use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use adw::prelude::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, CssProvider, FlowBox, Image, Justification, Label,
    ListBox, ListBoxRow, MediaFile, MenuButton, Orientation, Overflow, Picture, PolicyType, Popover,
    Scale, ScrolledWindow, SearchBar, SearchEntry, SelectionMode, Spinner, Stack,
    StackTransitionType, ToggleButton, gio,
};
use libadwaita as adw;

use crate::backend::{Backend, runtime};
use crate::models::{
    Album, Artist, ArtistDetails, Genre, Playlist, Track, format_duration, format_subscribers,
    load_favorites, save_favorites,
};
use crate::mpris::{MprisCommand, MprisController};

const APP_CSS: &str = r#"
.cover-art {
    border-radius: 8px;
    background-color: alpha(@window_fg_color, 0.08);
}

.avatar-art {
    border-radius: 999px;
    background-color: alpha(@window_fg_color, 0.08);
}

.artist-banner {
    border-radius: 14px;
    background-color: alpha(@window_fg_color, 0.06);
}

.cover-art-large {
    border-radius: 16px;
    background-color: alpha(@window_fg_color, 0.08);
    box-shadow: 0 10px 28px alpha(black, 0.34);
}

.sidebar-pane {
    background-color: @sidebar_bg_color;
    border-right: 1px solid alpha(@window_fg_color, 0.10);
}

.quality-pill {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 6px;
    background-color: alpha(@accent_bg_color, 0.18);
    color: @accent_color;
}

.play-pill {
    min-width: 56px;
    min-height: 44px;
    border-radius: 999px;
}

.media-card {
    padding: 6px;
    border-radius: 12px;
    transition: background-color 150ms ease;
}

.media-card:hover {
    background-color: alpha(@window_fg_color, 0.06);
}

.track-row {
    padding: 6px 8px;
}

.playing-title {
    color: @accent_color;
    font-weight: bold;
}

.sidebar-tabs {
    padding: 8px 12px 10px 12px;
    border-top: 1px solid alpha(@window_fg_color, 0.08);
}

.genre-chip {
    padding: 10px 14px;
    border-radius: 10px;
    font-weight: 600;
    background-color: alpha(@window_fg_color, 0.07);
}
"#;

struct AppState {
    backend: Backend,
    media_file: MediaFile,
    queue: Vec<Track>,
    favorites: Vec<Track>,
    current_index: Option<usize>,
    source_name: String,
    shuffle: bool,
    repeat: bool,
    cover_cache: HashMap<String, gdk::Texture>,
    toast_overlay: adw::ToastOverlay,
    // Sidebar player widgets
    player_source_label: Label,
    player_cover: Picture,
    player_title: Label,
    player_artist: Label,
    player_fav_btn: ToggleButton,
    player_radio_btn: Button,
    quality_label: Label,
    play_pause_btn: Button,
    prev_btn: Button,
    next_btn: Button,
    seek_scale: Scale,
    time_label: Label,
    duration_label: Label,
    // Sidebar lyrics & queue widgets
    lyrics_title_label: Label,
    lyrics_body_label: Label,
    lyrics_loaded_for: Option<String>,
    queue_list: ListBox,
    queue_empty_label: Label,
    // Collection (favorites) widgets
    favorites_list: ListBox,
    favorites_empty_label: Label,
    mpris: Option<MprisController>,
    pending_autoplay: bool,
    loading_generation: u64,
}

impl AppState {
    fn show_toast(&self, msg: &str) {
        let toast = adw::Toast::new(msg);
        toast.set_timeout(3);
        self.toast_overlay.add_toast(toast);
    }

    fn current_track(&self) -> Option<&Track> {
        self.current_index.and_then(|i| self.queue.get(i))
    }

    fn is_favorite(&self, id: &str) -> bool {
        self.favorites.iter().any(|t| t.id == id)
    }

    fn update_nav_buttons(&self) {
        let has_prev = self.current_index.is_some_and(|idx| idx > 0);
        let has_next = self
            .current_index
            .is_some_and(|idx| idx + 1 < self.queue.len() || (self.repeat && !self.queue.is_empty()));
        self.prev_btn.set_sensitive(has_prev);
        self.next_btn.set_sensitive(has_next);
    }
}

pub fn build_ui(app: &adw::Application) {
    load_css();

    let backend = Backend::new();
    let media_file = MediaFile::new();
    let toast_overlay = adw::ToastOverlay::new();

    // --- LEFT PANE: Dedicated Player / Lyrics / Queue Sidebar ---
    let sidebar_header = adw::HeaderBar::builder()
        .show_start_title_buttons(false)
        .show_end_title_buttons(false)
        .title_widget(&adw::WindowTitle::new("GyroTube", "YouTube Music"))
        .build();

    let sidebar_icon = Image::from_icon_name("audio-headphones-symbolic");
    sidebar_icon.set_margin_start(6);
    sidebar_header.pack_start(&sidebar_icon);

    let search_toggle_btn = ToggleButton::builder()
        .icon_name("system-search-symbolic")
        .tooltip_text("Buscar (Ctrl+F)")
        .build();
    sidebar_header.pack_end(&search_toggle_btn);

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
    player_cover_frame.set_halign(Align::Center);
    player_cover_frame.set_valign(Align::Center);
    player_cover_frame.set_vexpand(true);

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

    let action_icons_row = GtkBox::new(Orientation::Horizontal, 18);
    action_icons_row.set_halign(Align::Center);
    action_icons_row.append(&volume_btn);
    action_icons_row.append(&player_fav_btn);
    action_icons_row.append(&player_radio_btn);

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
    player_view.append(&player_cover_frame);
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
    sidebar_toolbar.add_top_bar(&sidebar_header);
    sidebar_toolbar.set_content(Some(&sidebar_stack));
    sidebar_toolbar.add_bottom_bar(&sidebar_tabs);
    sidebar_toolbar.add_css_class("sidebar-pane");

    // --- RIGHT PANE: Main Content ---
    let content_view_stack = adw::ViewStack::new();
    let content_switcher = adw::ViewSwitcher::builder()
        .stack(&content_view_stack)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build();

    let sidebar_toggle_btn = ToggleButton::builder()
        .icon_name("sidebar-show-symbolic")
        .active(true)
        .tooltip_text("Mostrar u ocultar reproductor")
        .build();

    let content_header = adw::HeaderBar::builder()
        .show_start_title_buttons(false)
        .title_widget(&content_switcher)
        .build();
    content_header.pack_start(&sidebar_toggle_btn);

    let quick_search_entry = SearchEntry::builder()
        .placeholder_text("Buscar canciones en YouTube Music…")
        .hexpand(true)
        .build();
    let search_bar = SearchBar::builder()
        .child(&quick_search_entry)
        .key_capture_widget(&content_view_stack)
        .build();
    search_toggle_btn
        .bind_property("active", &search_bar, "search-mode-enabled")
        .bidirectional()
        .sync_create()
        .build();

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

    let state = Rc::new(RefCell::new(AppState {
        backend: backend.clone(),
        media_file: media_file.clone(),
        queue: Vec::new(),
        favorites: load_favorites(),
        current_index: None,
        source_name: String::from("YouTube Music"),
        shuffle: false,
        repeat: false,
        cover_cache: HashMap::new(),
        toast_overlay: toast_overlay.clone(),
        player_source_label,
        player_cover,
        player_title,
        player_artist,
        player_fav_btn: player_fav_btn.clone(),
        player_radio_btn: player_radio_btn.clone(),
        quality_label,
        play_pause_btn,
        prev_btn,
        next_btn,
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
        mpris: None,
        pending_autoplay: false,
        loading_generation: 0,
    }));

    // Build right-pane pages
    let home_page = build_home_page(Rc::clone(&state));
    let (explore_page, explore_search_entry, trigger_explore_search) =
        build_explore_page(Rc::clone(&state));
    let collection_page = build_collection_page(Rc::clone(&state));
    refresh_favorites_ui(Rc::clone(&state));

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

    // Quick search bar in header forwards to Explore search
    {
        let content_view_stack = content_view_stack.clone();
        let explore_search_entry = explore_search_entry.clone();
        let search_bar = search_bar.clone();
        quick_search_entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if !text.trim().is_empty() {
                explore_search_entry.set_text(&text);
                content_view_stack.set_visible_child_name("explore");
                search_bar.set_search_mode(false);
                trigger_explore_search(&text);
            }
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
        queue_clear_btn.connect_clicked(move |_| {
            {
                let mut s = state.borrow_mut();
                s.queue.clear();
                s.current_index = None;
                s.media_file.pause();
                s.play_pause_btn.set_sensitive(false);
                s.seek_scale.set_sensitive(false);
                s.player_fav_btn.set_sensitive(false);
                s.player_radio_btn.set_sensitive(false);
                s.player_title.set_label("Nada reproduciéndose");
                s.player_artist.set_label("Selecciona una canción");
                s.player_source_label.set_label("Sin reproducción");
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
    content_toolbar.add_top_bar(&content_header);
    content_toolbar.add_top_bar(&search_bar);
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

    toast_overlay.set_child(Some(&split_view));

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
            MprisCommand::Play => state_mpris.borrow().media_file.play(),
            MprisCommand::Pause => state_mpris.borrow().media_file.pause(),
            MprisCommand::PlayPause => {
                let media = &state_mpris.borrow().media_file;
                if media.is_playing() {
                    media.pause();
                } else {
                    media.play();
                }
            }
            MprisCommand::Stop => {
                let s = state_mpris.borrow();
                s.media_file.pause();
                s.media_file.seek(0);
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
                let s = state_mpris.borrow();
                let cur = s.media_file.timestamp().max(0);
                let target = (cur + offset_us).max(0);
                s.media_file.seek(target);
                if let Some(m) = &s.mpris {
                    m.emit_seeked(target);
                }
            }
            MprisCommand::SetPositionUs(pos_us) => {
                let s = state_mpris.borrow();
                let target = pos_us.max(0);
                s.media_file.seek(target);
                if let Some(m) = &s.mpris {
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
    provider.load_from_string(APP_CSS);
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn build_carousel_section(title: &str) -> (GtkBox, GtkBox, ScrolledWindow) {
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

fn build_home_page(state: Rc<RefCell<AppState>>) -> ScrolledWindow {
    let container = GtkBox::new(Orientation::Vertical, 22);
    container.set_margin_top(16);
    container.set_margin_bottom(24);
    container.set_margin_start(20);
    container.set_margin_end(20);

    let spinner = Spinner::builder()
        .halign(Align::Center)
        .margin_top(32)
        .spinning(true)
        .build();

    let (trending_section, trending_cards, _) = build_carousel_section("Tendencias");
    let (albums_section, albums_cards, _) = build_carousel_section("Álbumes nuevos");

    let top_header = GtkBox::new(Orientation::Horizontal, 8);
    top_header.append(
        &Label::builder()
            .label("Top Música")
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["title-3"])
            .build(),
    );
    let refresh_btn = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Recargar inicio")
        .css_classes(["flat", "circular"])
        .build();
    top_header.append(&refresh_btn);

    let top_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    let top_section = GtkBox::new(Orientation::Vertical, 10);
    top_section.set_visible(false);
    top_section.append(&top_header);
    top_section.append(&top_list);

    container.append(&spinner);
    container.append(&trending_section);
    container.append(&albums_section);
    container.append(&top_section);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&container)
        .build();

    let load_home = {
        let state = Rc::clone(&state);
        let spinner = spinner.clone();
        let trending_section = trending_section.clone();
        let trending_cards = trending_cards.clone();
        let albums_section = albums_section.clone();
        let albums_cards = albums_cards.clone();
        let top_section = top_section.clone();
        let top_list = top_list.clone();

        move || {
            spinner.set_visible(true);
            spinner.set_spinning(true);
            let backend = state.borrow().backend.clone();
            let (tx, rx) = async_channel::bounded(1);

            runtime().spawn(async move {
                let res = backend.fetch_home_feed().await;
                let _ = tx.send(res).await;
            });

            let state = Rc::clone(&state);
            let spinner = spinner.clone();
            let trending_section = trending_section.clone();
            let trending_cards = trending_cards.clone();
            let albums_section = albums_section.clone();
            let albums_cards = albums_cards.clone();
            let top_section = top_section.clone();
            let top_list = top_list.clone();

            glib::spawn_future_local(async move {
                if let Ok(result) = rx.recv().await {
                    spinner.set_spinning(false);
                    spinner.set_visible(false);
                    match result {
                        Ok(feed) => {
                            clear_box(&trending_cards);
                            for (idx, track) in feed.trending.iter().enumerate() {
                                let card = build_track_card(
                                    track,
                                    &feed.trending,
                                    idx,
                                    "Tendencias",
                                    Rc::clone(&state),
                                );
                                trending_cards.append(&card);
                            }
                            trending_section.set_visible(!feed.trending.is_empty());

                            clear_box(&albums_cards);
                            for album in &feed.new_albums {
                                let card = build_album_card(album, Rc::clone(&state));
                                albums_cards.append(&card);
                            }
                            albums_section.set_visible(!feed.new_albums.is_empty());

                            populate_track_list(&top_list, &feed.top, "Top Música", Rc::clone(&state));
                            top_section.set_visible(!feed.top.is_empty());
                        }
                        Err(err) => {
                            state.borrow().show_toast(&err);
                        }
                    }
                }
            });
        }
    };

    load_home();
    refresh_btn.connect_clicked(move |_| {
        load_home();
    });

    scrolled
}

fn build_track_card(
    track: &Track,
    section_tracks: &[Track],
    section_index: usize,
    source_name: &str,
    state: Rc<RefCell<AppState>>,
) -> Button {
    let (cover_frame, cover) = clamped_cover(146, 146, "cover-art");

    if let Some(url) = track.cover_large_url.as_deref().or(track.cover_url.as_deref()) {
        load_cover_into_picture(&cover, url, Rc::clone(&state));
    }

    let title_label = Label::builder()
        .label(&track.title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["heading"])
        .build();

    let artist_label = Label::builder()
        .label(&track.artist)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["dim-label", "caption"])
        .build();

    let card_box = GtkBox::new(Orientation::Vertical, 6);
    card_box.set_width_request(146);
    card_box.append(&cover_frame);
    card_box.append(&title_label);
    card_box.append(&artist_label);

    let btn = Button::builder()
        .child(&card_box)
        .css_classes(["flat", "media-card"])
        .build();

    let section = section_tracks.to_vec();
    let source = source_name.to_string();
    btn.connect_clicked(move |_| {
        {
            let mut s = state.borrow_mut();
            s.queue = section.clone();
            s.source_name = source.clone();
        }
        play_queue_index(Rc::clone(&state), section_index);
    });

    btn
}

fn build_album_card(album: &Album, state: Rc<RefCell<AppState>>) -> Button {
    let (cover_frame, cover) = clamped_cover(146, 146, "cover-art");

    if let Some(url) = &album.cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(&state));
    }

    let title_label = Label::builder()
        .label(&album.title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["heading"])
        .build();

    let sub = match album.year {
        Some(y) => format!("{} • {y}", album.artist),
        None => album.artist.clone(),
    };
    let artist_label = Label::builder()
        .label(&sub)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["dim-label", "caption"])
        .build();

    let card_box = GtkBox::new(Orientation::Vertical, 6);
    card_box.set_width_request(146);
    card_box.append(&cover_frame);
    card_box.append(&title_label);
    card_box.append(&artist_label);

    let btn = Button::builder()
        .child(&card_box)
        .css_classes(["flat", "media-card"])
        .build();

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
                    Ok((name, tracks)) if !tracks.is_empty() => {
                        {
                            let mut s = state.borrow_mut();
                            s.queue = tracks;
                            s.source_name = name;
                        }
                        play_queue_index(Rc::clone(&state), 0);
                    }
                    Ok(_) => state.borrow().show_toast("El álbum no tiene pistas."),
                    Err(err) => state.borrow().show_toast(&err),
                }
            }
        });
    });

    btn
}

fn build_explore_page(
    state: Rc<RefCell<AppState>>,
) -> (ScrolledWindow, SearchEntry, Rc<dyn Fn(&str)>) {
    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_margin_top(18);
    container.set_margin_bottom(24);
    container.set_margin_start(20);
    container.set_margin_end(20);

    let search_entry = SearchEntry::builder()
        .placeholder_text("Buscar canciones, artistas o álbumes en YouTube Music…")
        .hexpand(true)
        .build();

    let spinner = Spinner::builder()
        .halign(Align::Center)
        .visible(false)
        .build();

    let status_label = Label::builder()
        .visible(false)
        .margin_top(18)
        .css_classes(["dim-label"])
        .build();

    let (search_artists_section, search_artists_cards, _) = build_carousel_section("Artistas");
    let (search_albums_section, search_albums_cards, _) = build_carousel_section("Álbumes");
    let (search_playlists_section, search_playlists_cards, _) = build_carousel_section("Playlists");

    let search_tracks_heading = Label::builder()
        .label("Canciones")
        .xalign(0.0)
        .css_classes(["title-3"])
        .visible(false)
        .build();

    let results_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .visible(false)
        .build();

    let search_results_section = GtkBox::new(Orientation::Vertical, 18);
    search_results_section.set_visible(false);
    search_results_section.append(&search_artists_section);
    search_results_section.append(&search_albums_section);
    search_results_section.append(&search_playlists_section);
    search_results_section.append(&search_tracks_heading);
    search_results_section.append(&results_list);

    // Artist details sub-view
    let artist_back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .css_classes(["flat", "circular"])
        .build();
    let artist_name_label = Label::builder()
        .label("Artista")
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["title-2"])
        .build();
    let artist_subs_label = Label::builder()
        .label("")
        .xalign(0.0)
        .css_classes(["dim-label", "caption"])
        .visible(false)
        .build();
    let artist_title_box = GtkBox::new(Orientation::Vertical, 2);
    artist_title_box.set_hexpand(true);
    artist_title_box.append(&artist_name_label);
    artist_title_box.append(&artist_subs_label);

    let artist_top_bar = GtkBox::new(Orientation::Horizontal, 10);
    artist_top_bar.append(&artist_back_btn);
    artist_top_bar.append(&artist_title_box);

    let artist_banner = Picture::builder()
        .height_request(180)
        .content_fit(ContentFit::Cover)
        .overflow(Overflow::Hidden)
        .visible(false)
        .build();
    artist_banner.add_css_class("artist-banner");

    let artist_desc_label = Label::builder()
        .xalign(0.0)
        .wrap(true)
        .lines(4)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["body", "dim-label"])
        .visible(false)
        .build();

    let artist_tracks_heading = Label::builder()
        .label("Canciones populares")
        .xalign(0.0)
        .css_classes(["title-3"])
        .visible(false)
        .build();
    let artist_tracks_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .visible(false)
        .build();

    let (artist_albums_section, artist_albums_cards, _) = build_carousel_section("Álbumes");
    let (artist_playlists_section, artist_playlists_cards, _) = build_carousel_section("Playlists");
    let (artist_similar_section, artist_similar_cards, _) =
        build_carousel_section("Artistas similares");

    let artist_section = GtkBox::new(Orientation::Vertical, 16);
    artist_section.set_visible(false);
    artist_section.append(&artist_top_bar);
    artist_section.append(&artist_banner);
    artist_section.append(&artist_desc_label);
    artist_section.append(&artist_tracks_heading);
    artist_section.append(&artist_tracks_list);
    artist_section.append(&artist_albums_section);
    artist_section.append(&artist_playlists_section);
    artist_section.append(&artist_similar_section);

    // Genres & moods grid
    let genres_title = Label::builder()
        .label("Géneros y estados de ánimo")
        .xalign(0.0)
        .css_classes(["title-3"])
        .build();
    let genres_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .column_spacing(10)
        .row_spacing(10)
        .min_children_per_line(2)
        .max_children_per_line(5)
        .homogeneous(true)
        .build();

    let genres_section = GtkBox::new(Orientation::Vertical, 12);
    genres_section.append(&genres_title);
    genres_section.append(&genres_flow);

    // Genre playlists sub-view
    let genre_back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .css_classes(["flat", "circular"])
        .build();
    let genre_heading = Label::builder()
        .label("Playlists")
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-3"])
        .build();
    let genre_header = GtkBox::new(Orientation::Horizontal, 8);
    genre_header.append(&genre_back_btn);
    genre_header.append(&genre_heading);

    let playlists_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .column_spacing(12)
        .row_spacing(12)
        .min_children_per_line(2)
        .max_children_per_line(4)
        .build();

    let genre_playlists_section = GtkBox::new(Orientation::Vertical, 12);
    genre_playlists_section.set_visible(false);
    genre_playlists_section.append(&genre_header);
    genre_playlists_section.append(&playlists_flow);

    container.append(&search_entry);
    container.append(&spinner);
    container.append(&status_label);
    container.append(&search_results_section);
    container.append(&artist_section);
    container.append(&genres_section);
    container.append(&genre_playlists_section);

    let clamp = adw::Clamp::builder()
        .maximum_size(900)
        .child(&container)
        .build();

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&clamp)
        .build();

    // Load genres on startup
    {
        let backend = state.borrow().backend.clone();
        let (tx, rx) = async_channel::bounded(1);
        runtime().spawn(async move {
            let res = backend.fetch_genres().await;
            let _ = tx.send(res).await;
        });

        let state = Rc::clone(&state);
        let genres_flow = genres_flow.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        let genre_heading = genre_heading.clone();
        let playlists_flow = playlists_flow.clone();
        let spinner = spinner.clone();

        glib::spawn_future_local(async move {
            if let Ok(Ok(genres)) = rx.recv().await {
                for genre in genres {
                    let btn = build_genre_button(
                        &genre,
                        Rc::clone(&state),
                        &genres_section,
                        &genre_playlists_section,
                        &genre_heading,
                        &playlists_flow,
                        &spinner,
                    );
                    genres_flow.insert(&btn, -1);
                }
            }
        });
    }

    {
        let genres_section = genres_section.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        genre_back_btn.connect_clicked(move |_| {
            genre_playlists_section.set_visible(false);
            genres_section.set_visible(true);
        });
    }

    let artist_widgets = ArtistPageWidgets {
        spinner: spinner.clone(),
        status_label: status_label.clone(),
        search_results_section: search_results_section.clone(),
        genres_section: genres_section.clone(),
        genre_playlists_section: genre_playlists_section.clone(),
        artist_section: artist_section.clone(),
        name_label: artist_name_label,
        subs_label: artist_subs_label,
        banner: artist_banner,
        desc_label: artist_desc_label,
        tracks_heading: artist_tracks_heading,
        tracks_list: artist_tracks_list,
        albums_section: artist_albums_section,
        albums_cards: artist_albums_cards,
        playlists_section: artist_playlists_section,
        playlists_cards: artist_playlists_cards,
        similar_section: artist_similar_section,
        similar_cards: artist_similar_cards,
    };

    {
        let search_entry = search_entry.clone();
        let artist_section = artist_section.clone();
        let search_results_section = search_results_section.clone();
        let genres_section = genres_section.clone();
        artist_back_btn.connect_clicked(move |_| {
            artist_section.set_visible(false);
            if search_entry.text().trim().is_empty() {
                genres_section.set_visible(true);
            } else {
                search_results_section.set_visible(true);
            }
        });
    }

    let run_search: Rc<dyn Fn(&str)> = {
        let state = Rc::clone(&state);
        let spinner = spinner.clone();
        let status_label = status_label.clone();
        let search_results_section = search_results_section.clone();
        let search_artists_section = search_artists_section.clone();
        let search_artists_cards = search_artists_cards.clone();
        let search_albums_section = search_albums_section.clone();
        let search_albums_cards = search_albums_cards.clone();
        let search_playlists_section = search_playlists_section.clone();
        let search_playlists_cards = search_playlists_cards.clone();
        let search_tracks_heading = search_tracks_heading.clone();
        let results_list = results_list.clone();
        let artist_section = artist_section.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        let artist_widgets = artist_widgets.clone();

        Rc::new(move |raw_query: &str| {
            let query = raw_query.trim().to_string();
            if query.is_empty() {
                search_results_section.set_visible(false);
                artist_section.set_visible(false);
                status_label.set_visible(false);
                genre_playlists_section.set_visible(false);
                genres_section.set_visible(true);
                return;
            }

            genres_section.set_visible(false);
            genre_playlists_section.set_visible(false);
            artist_section.set_visible(false);
            spinner.set_visible(true);
            spinner.set_spinning(true);
            status_label.set_visible(false);
            search_results_section.set_visible(false);

            let backend = state.borrow().backend.clone();
            let (tx, rx) = async_channel::bounded(1);

            let query_for_source = query.clone();
            runtime().spawn(async move {
                let res = backend.search_all(&query).await;
                let _ = tx.send(res).await;
            });

            let state = Rc::clone(&state);
            let spinner = spinner.clone();
            let status_label = status_label.clone();
            let search_results_section = search_results_section.clone();
            let search_artists_section = search_artists_section.clone();
            let search_artists_cards = search_artists_cards.clone();
            let search_albums_section = search_albums_section.clone();
            let search_albums_cards = search_albums_cards.clone();
            let search_playlists_section = search_playlists_section.clone();
            let search_playlists_cards = search_playlists_cards.clone();
            let search_tracks_heading = search_tracks_heading.clone();
            let results_list = results_list.clone();
            let artist_widgets = artist_widgets.clone();

            glib::spawn_future_local(async move {
                if let Ok(result) = rx.recv().await {
                    spinner.set_spinning(false);
                    spinner.set_visible(false);
                    match result {
                        Ok(res)
                            if res.artists.is_empty()
                                && res.albums.is_empty()
                                && res.playlists.is_empty()
                                && res.tracks.is_empty() =>
                        {
                            status_label.set_label("No se encontraron resultados.");
                            status_label.set_visible(true);
                            search_results_section.set_visible(false);
                        }
                        Ok(res) => {
                            clear_box(&search_artists_cards);
                            for artist in &res.artists {
                                search_artists_cards.append(&build_artist_card(
                                    artist,
                                    Rc::clone(&state),
                                    artist_widgets.clone(),
                                ));
                            }
                            search_artists_section.set_visible(!res.artists.is_empty());

                            clear_box(&search_albums_cards);
                            for album in &res.albums {
                                search_albums_cards
                                    .append(&build_album_card(album, Rc::clone(&state)));
                            }
                            search_albums_section.set_visible(!res.albums.is_empty());

                            clear_box(&search_playlists_cards);
                            for playlist in &res.playlists {
                                search_playlists_cards
                                    .append(&build_playlist_card(playlist, Rc::clone(&state)));
                            }
                            search_playlists_section.set_visible(!res.playlists.is_empty());

                            if res.tracks.is_empty() {
                                search_tracks_heading.set_visible(false);
                                results_list.set_visible(false);
                            } else {
                                populate_track_list(
                                    &results_list,
                                    &res.tracks,
                                    &format!("Búsqueda: {query_for_source}"),
                                    Rc::clone(&state),
                                );
                                search_tracks_heading.set_visible(true);
                                results_list.set_visible(true);
                            }

                            search_results_section.set_visible(true);
                            status_label.set_visible(false);
                        }
                        Err(err) => {
                            status_label.set_label(&err);
                            status_label.set_visible(true);
                            state.borrow().show_toast(&err);
                        }
                    }
                }
            });
        })
    };

    {
        let run_search = Rc::clone(&run_search);
        search_entry.connect_activate(move |entry| {
            run_search(&entry.text());
        });
    }
    {
        let search_results_section = search_results_section.clone();
        let artist_section = artist_section.clone();
        let status_label = status_label.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        search_entry.connect_search_changed(move |entry| {
            if entry.text().trim().is_empty() {
                search_results_section.set_visible(false);
                artist_section.set_visible(false);
                status_label.set_visible(false);
                genre_playlists_section.set_visible(false);
                genres_section.set_visible(true);
            }
        });
    }

    (scrolled, search_entry, run_search)
}

#[derive(Clone)]
struct ArtistPageWidgets {
    spinner: Spinner,
    status_label: Label,
    search_results_section: GtkBox,
    genres_section: GtkBox,
    genre_playlists_section: GtkBox,
    artist_section: GtkBox,
    name_label: Label,
    subs_label: Label,
    banner: Picture,
    desc_label: Label,
    tracks_heading: Label,
    tracks_list: ListBox,
    albums_section: GtkBox,
    albums_cards: GtkBox,
    playlists_section: GtkBox,
    playlists_cards: GtkBox,
    similar_section: GtkBox,
    similar_cards: GtkBox,
}

fn open_artist_page(
    artist_id: &str,
    artist_name: &str,
    ui: ArtistPageWidgets,
    state: Rc<RefCell<AppState>>,
) {
    ui.search_results_section.set_visible(false);
    ui.genres_section.set_visible(false);
    ui.genre_playlists_section.set_visible(false);
    ui.artist_section.set_visible(false);
    ui.status_label.set_visible(false);
    ui.spinner.set_visible(true);
    ui.spinner.set_spinning(true);

    let backend = state.borrow().backend.clone();
    let (tx, rx) = async_channel::bounded(1);
    let id = artist_id.to_string();
    runtime().spawn(async move {
        let res = backend.fetch_artist(&id).await;
        let _ = tx.send(res).await;
    });

    let fallback_name = artist_name.to_string();
    glib::spawn_future_local(async move {
        if let Ok(result) = rx.recv().await {
            ui.spinner.set_spinning(false);
            ui.spinner.set_visible(false);
            match result {
                Ok(details) => {
                    populate_artist_page(details, ui, state);
                }
                Err(err) => {
                    ui.name_label.set_label(&fallback_name);
                    ui.status_label.set_label(&err);
                    ui.status_label.set_visible(true);
                    state.borrow().show_toast(&err);
                }
            }
        }
    });
}

fn populate_artist_page(
    details: ArtistDetails,
    ui: ArtistPageWidgets,
    state: Rc<RefCell<AppState>>,
) {
    ui.name_label.set_label(&details.name);
    if let Some(subs) = details.subscriber_count {
        ui.subs_label.set_label(&format_subscribers(subs));
        ui.subs_label.set_visible(true);
    } else {
        ui.subs_label.set_visible(false);
    }

    if let Some(url) = &details.header_url {
        ui.banner.set_paintable(None::<&gdk::Paintable>);
        load_cover_into_picture(&ui.banner, url, Rc::clone(&state));
        ui.banner.set_visible(true);
    } else {
        ui.banner.set_visible(false);
    }

    if let Some(desc) = details.description.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        ui.desc_label.set_label(desc);
        ui.desc_label.set_visible(true);
    } else {
        ui.desc_label.set_visible(false);
    }

    if details.tracks.is_empty() {
        ui.tracks_heading.set_visible(false);
        ui.tracks_list.set_visible(false);
    } else {
        populate_track_list(
            &ui.tracks_list,
            &details.tracks,
            &details.name,
            Rc::clone(&state),
        );
        ui.tracks_heading.set_visible(true);
        ui.tracks_list.set_visible(true);
    }

    clear_box(&ui.albums_cards);
    for album in &details.albums {
        ui.albums_cards
            .append(&build_album_card(album, Rc::clone(&state)));
    }
    ui.albums_section.set_visible(!details.albums.is_empty());

    clear_box(&ui.playlists_cards);
    for playlist in &details.playlists {
        ui.playlists_cards
            .append(&build_playlist_card(playlist, Rc::clone(&state)));
    }
    ui.playlists_section.set_visible(!details.playlists.is_empty());

    clear_box(&ui.similar_cards);
    for similar in &details.similar_artists {
        ui.similar_cards.append(&build_artist_card(
            similar,
            Rc::clone(&state),
            ui.clone(),
        ));
    }
    ui.similar_section
        .set_visible(!details.similar_artists.is_empty());

    ui.artist_section.set_visible(true);
}

fn build_artist_card(
    artist: &Artist,
    state: Rc<RefCell<AppState>>,
    artist_widgets: ArtistPageWidgets,
) -> Button {
    let (avatar_frame, avatar) = clamped_cover(128, 128, "avatar-art");
    if let Some(url) = &artist.avatar_url {
        load_cover_into_picture(&avatar, url, Rc::clone(&state));
    }

    let name_label = Label::builder()
        .label(&artist.name)
        .halign(Align::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(15)
        .css_classes(["heading"])
        .build();

    let subs_text = artist
        .subscriber_count
        .map(format_subscribers)
        .unwrap_or_else(|| String::from("Artista"));
    let subs_label = Label::builder()
        .label(&subs_text)
        .halign(Align::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["dim-label", "caption"])
        .build();

    let card_box = GtkBox::new(Orientation::Vertical, 6);
    card_box.set_width_request(132);
    card_box.append(&avatar_frame);
    card_box.append(&name_label);
    card_box.append(&subs_label);

    let btn = Button::builder()
        .child(&card_box)
        .css_classes(["flat", "media-card"])
        .build();

    let artist_id = artist.id.clone();
    let artist_name = artist.name.clone();
    btn.connect_clicked(move |_| {
        open_artist_page(
            &artist_id,
            &artist_name,
            artist_widgets.clone(),
            Rc::clone(&state),
        );
    });

    btn
}

fn build_genre_button(
    genre: &Genre,
    state: Rc<RefCell<AppState>>,
    genres_section: &GtkBox,
    genre_playlists_section: &GtkBox,
    genre_heading: &Label,
    playlists_flow: &FlowBox,
    spinner: &Spinner,
) -> Button {
    let dot = Label::builder()
        .label("●")
        .build();
    let dot_css = CssProvider::new();
    dot_css.load_from_string(&format!("label {{ color: {}; }}", genre.color_hex));
    #[allow(deprecated)]
    dot.style_context()
        .add_provider(&dot_css, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let name_label = Label::builder()
        .label(&genre.name)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();

    let inner = GtkBox::new(Orientation::Horizontal, 8);
    inner.append(&dot);
    inner.append(&name_label);

    let btn = Button::builder()
        .child(&inner)
        .css_classes(["flat", "genre-chip"])
        .build();

    let genre_id = genre.id.clone();
    let genre_name = genre.name.clone();
    let genres_section = genres_section.clone();
    let genre_playlists_section = genre_playlists_section.clone();
    let genre_heading = genre_heading.clone();
    let playlists_flow = playlists_flow.clone();
    let spinner = spinner.clone();

    btn.connect_clicked(move |_| {
        genres_section.set_visible(false);
        genre_playlists_section.set_visible(false);
        spinner.set_visible(true);
        spinner.set_spinning(true);

        let backend = state.borrow().backend.clone();
        let (tx, rx) = async_channel::bounded(1);
        let id = genre_id.clone();
        runtime().spawn(async move {
            let res = backend.fetch_genre_playlists(&id).await;
            let _ = tx.send(res).await;
        });

        let state = Rc::clone(&state);
        let genre_playlists_section = genre_playlists_section.clone();
        let genre_heading = genre_heading.clone();
        let playlists_flow = playlists_flow.clone();
        let spinner = spinner.clone();
        let fallback_name = genre_name.clone();

        glib::spawn_future_local(async move {
            if let Ok(result) = rx.recv().await {
                spinner.set_spinning(false);
                spinner.set_visible(false);
                match result {
                    Ok((name, playlists)) => {
                        while let Some(child) = playlists_flow.first_child() {
                            playlists_flow.remove(&child);
                        }
                        genre_heading.set_label(if name.is_empty() {
                            &fallback_name
                        } else {
                            &name
                        });
                        for pl in playlists {
                            let card = build_playlist_card(&pl, Rc::clone(&state));
                            playlists_flow.insert(&card, -1);
                        }
                        genre_playlists_section.set_visible(true);
                    }
                    Err(err) => {
                        state.borrow().show_toast(&err);
                    }
                }
            }
        });
    });

    btn
}

fn build_playlist_card(playlist: &Playlist, state: Rc<RefCell<AppState>>) -> Button {
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

fn build_collection_page(state: Rc<RefCell<AppState>>) -> ScrolledWindow {
    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_margin_top(18);
    container.set_margin_bottom(24);
    container.set_margin_start(20);
    container.set_margin_end(20);

    container.append(
        &Label::builder()
            .label("Favoritos")
            .xalign(0.0)
            .css_classes(["title-2"])
            .build(),
    );

    let (favorites_list, favorites_empty_label) = {
        let s = state.borrow();
        (s.favorites_list.clone(), s.favorites_empty_label.clone())
    };
    container.append(&favorites_empty_label);
    container.append(&favorites_list);

    let clamp = adw::Clamp::builder()
        .maximum_size(860)
        .child(&container)
        .build();

    ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&clamp)
        .build()
}

fn clear_box(container: &GtkBox) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

fn clear_list_box(list: &ListBox) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
}

fn populate_track_list(
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

fn build_track_row(
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
    row_box.append(&radio_btn);
    row_box.append(&queue_btn);
    row_box.append(&play_btn);

    ListBoxRow::builder().child(&row_box).build()
}

fn set_track_favorite(state: Rc<RefCell<AppState>>, track: &Track, fav: bool) {
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

fn start_track_radio(state: Rc<RefCell<AppState>>, track: &Track) {
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

fn refresh_favorites_ui(state: Rc<RefCell<AppState>>) {
    let (favorites_list, favorites_empty_label, favorites) = {
        let s = state.borrow();
        (
            s.favorites_list.clone(),
            s.favorites_empty_label.clone(),
            s.favorites.clone(),
        )
    };

    let is_empty = favorites.is_empty();
    favorites_empty_label.set_visible(is_empty);
    favorites_list.set_visible(!is_empty);
    populate_track_list(&favorites_list, &favorites, "Favoritos", state);
}

fn refresh_queue_ui(state: Rc<RefCell<AppState>>) {
    let (queue_list, queue_empty_label, queue, current_index) = {
        let s = state.borrow();
        (
            s.queue_list.clone(),
            s.queue_empty_label.clone(),
            s.queue.clone(),
            s.current_index,
        )
    };

    clear_list_box(&queue_list);
    let is_empty = queue.is_empty();
    queue_empty_label.set_visible(is_empty);
    queue_list.set_visible(!is_empty);

    for (idx, track) in queue.iter().enumerate() {
        let is_current = current_index == Some(idx);
        let row_box = GtkBox::new(Orientation::Horizontal, 10);
        row_box.add_css_class("track-row");

        let icon_name = if is_current {
            "audio-volume-high-symbolic"
        } else {
            "audio-x-generic-symbolic"
        };
        let icon = Image::from_icon_name(icon_name);
        icon.set_valign(Align::Center);

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
        text_box.append(&title_label);
        text_box.append(&artist_label);

        let jump_btn = Button::builder()
            .icon_name("media-playback-start-symbolic")
            .valign(Align::Center)
            .css_classes(["flat", "circular"])
            .build();

        {
            let state = Rc::clone(&state);
            jump_btn.connect_clicked(move |_| {
                play_queue_index(Rc::clone(&state), idx);
            });
        }

        row_box.append(&icon);
        row_box.append(&text_box);
        row_box.append(&jump_btn);

        queue_list.append(&ListBoxRow::builder().child(&row_box).build());
    }
}

fn load_current_lyrics(state: Rc<RefCell<AppState>>) {
    let (track, backend) = {
        let s = state.borrow();
        let Some(track) = s.current_track().cloned() else {
            s.lyrics_title_label.set_label("Letras");
            s.lyrics_body_label
                .set_label("Reproduce una canción para ver su letra.");
            return;
        };
        if s.lyrics_loaded_for.as_deref() == Some(&track.id) {
            return;
        }
        s.lyrics_title_label.set_label(&track.title);
        s.lyrics_body_label.set_label("Cargando letra…");
        (track, s.backend.clone())
    };

    let (tx, rx) = async_channel::bounded(1);
    let id = track.id.clone();
    runtime().spawn(async move {
        let res = backend.fetch_lyrics(&id).await;
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        if let Ok(result) = rx.recv().await {
            let mut s = state.borrow_mut();
            if s.current_track().is_some_and(|cur| cur.id == track.id) {
                match result {
                    Ok(text) => {
                        s.lyrics_body_label.set_label(&text);
                        s.lyrics_loaded_for = Some(track.id);
                    }
                    Err(err) => {
                        s.lyrics_body_label.set_label(&err);
                    }
                }
            }
        }
    });
}

fn play_queue_index(state: Rc<RefCell<AppState>>, index: usize) {
    let (track, backend, current_gen, media_file, player_cover, player_fav_btn, is_fav) = {
        let mut s = state.borrow_mut();
        let Some(track) = s.queue.get(index).cloned() else {
            return;
        };
        s.current_index = Some(index);
        s.loading_generation = s.loading_generation.wrapping_add(1);
        let current_gen = s.loading_generation;
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
            s.media_file.clone(),
            s.player_cover.clone(),
            s.player_fav_btn.clone(),
            is_fav,
        )
    };

    media_file.pause();

    if player_fav_btn.is_active() != is_fav {
        player_fav_btn.set_active(is_fav);
    }

    if let Some(url) = track.cover_large_url.as_deref().or(track.cover_url.as_deref()) {
        load_cover_into_picture(&player_cover, url, Rc::clone(&state));
    } else {
        player_cover.set_paintable(None::<&gdk::Paintable>);
    }

    refresh_queue_ui(Rc::clone(&state));

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
                    s.player_artist.set_label(&track.artist);
                    s.quality_label.set_label("ERR");
                    s.show_toast("No se pudo resolver el flujo de audio.");
                }
                return;
            };
            if s.loading_generation != current_gen {
                return;
            }
            match result {
                Ok(stream) => {
                    s.player_artist.set_label(&track.artist);
                    s.quality_label.set_label(&stream.quality_label);
                    s.play_pause_btn.set_sensitive(true);
                    s.seek_scale.set_sensitive(true);
                    s.pending_autoplay = true;
                    Some((s.media_file.clone(), stream.url))
                }
                Err(err) => {
                    s.player_artist.set_label(&track.artist);
                    s.quality_label.set_label("ERR");
                    s.show_toast(&err);
                    None
                }
            }
        };

        if let Some((media_file, path)) = stream_to_play {
            let file = gio::File::for_path(&path);
            media_file.pause();
            media_file.set_file(Some(&file));
            if media_file.is_prepared() {
                state.borrow_mut().pending_autoplay = false;
                media_file.play();
            }
        }
    });
}

fn clamped_cover(min_size: i32, max_size: i32, css_class: &str) -> (adw::Clamp, Picture) {
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

fn load_cover_into_picture(picture: &Picture, url: &str, state: Rc<RefCell<AppState>>) {
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

fn wire_media_signals(
    state: Rc<RefCell<AppState>>,
    volume_scale: &Scale,
    volume_btn: &MenuButton,
) {
    let media_file = state.borrow().media_file.clone();

    {
        let media = media_file.clone();
        let volume_btn = volume_btn.clone();
        volume_scale.connect_value_changed(move |scale| {
            let val = scale.value();
            media.set_volume(val);
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
            let media = &state.borrow().media_file;
            if media.is_playing() {
                media.pause();
            } else {
                media.play();
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

    {
        let state = Rc::clone(&state);
        media_file.connect_prepared_notify(move |media| {
            if media.is_prepared() {
                let should_play = {
                    let mut s = state.borrow_mut();
                    if s.pending_autoplay {
                        s.pending_autoplay = false;
                        true
                    } else {
                        false
                    }
                };
                if should_play {
                    media.play();
                }
            }
        });
    }

    {
        let state = Rc::clone(&state);
        media_file.connect_playing_notify(move |media| {
            let playing = media.is_playing();
            let icon = if playing {
                "media-playback-pause-symbolic"
            } else {
                "media-playback-start-symbolic"
            };
            let s = state.borrow();
            s.play_pause_btn.set_icon_name(icon);
            if let Some(mpris) = &s.mpris {
                mpris.set_status(playing, s.current_index.is_none());
            }
        });
    }

    {
        let state = Rc::clone(&state);
        media_file.connect_duration_notify(move |media| {
            let dur_us = media.duration();
            if dur_us > 0 {
                let dur_secs = (dur_us / 1_000_000) as f64;
                let s = state.borrow();
                s.seek_scale.set_range(0.0, dur_secs);
                s.duration_label.set_label(&format_duration(dur_secs as u32));
            }
        });
    }

    {
        let state = Rc::clone(&state);
        media_file.connect_timestamp_notify(move |media| {
            let pos_us = media.timestamp();
            if pos_us >= 0 {
                let pos_secs = (pos_us / 1_000_000) as f64;
                let s = state.borrow();
                s.seek_scale.set_value(pos_secs);
                s.time_label.set_label(&format_duration(pos_secs as u32));
            }
        });
    }

    {
        let state = Rc::clone(&state);
        let media = media_file.clone();
        let seek_scale = state.borrow().seek_scale.clone();
        seek_scale.connect_change_value(move |_, _, val| {
            if val >= 0.0 {
                let target_us = (val * 1_000_000.0) as i64;
                media.seek(target_us);
                if let Some(mpris) = &state.borrow().mpris {
                    mpris.set_position_us(target_us);
                    mpris.emit_seeked(target_us);
                }
            }
            glib::Propagation::Proceed
        });
    }

    // Periodic 250ms sync with GStreamer position & duration
    {
        let state = Rc::clone(&state);
        glib::timeout_add_local(std::time::Duration::from_millis(250), move || {
            let s = state.borrow();
            if s.media_file.is_playing() {
                let dur_us = s.media_file.duration();
                if dur_us > 0 {
                    let dur_secs = (dur_us / 1_000_000) as f64;
                    s.seek_scale.set_range(0.0, dur_secs);
                    s.duration_label.set_label(&format_duration(dur_secs as u32));
                }
                let pos_us = s.media_file.timestamp();
                if pos_us >= 0 {
                    let pos_secs = (pos_us / 1_000_000) as f64;
                    s.seek_scale.set_value(pos_secs);
                    s.time_label.set_label(&format_duration(pos_secs as u32));
                    if let Some(mpris) = &s.mpris {
                        mpris.set_position_us(pos_us);
                    }
                }
            }
            glib::ControlFlow::Continue
        });
    }

    {
        let state = Rc::clone(&state);
        media_file.connect_ended_notify(move |media| {
            if !media.is_ended() {
                return;
            }
            let target = next_queue_index(&state.borrow());
            if let Some(idx) = target {
                play_queue_index(Rc::clone(&state), idx);
            }
        });
    }

    {
        let state = Rc::clone(&state);
        media_file.connect_error_notify(move |media| {
            if let Some(err) = media.error() {
                state
                    .borrow()
                    .show_toast(&format!("Error de reproducción: {err}"));
            }
        });
    }
}

fn next_queue_index(s: &AppState) -> Option<usize> {
    let len = s.queue.len();
    if len == 0 {
        return None;
    }
    let current = s.current_index?;
    if s.shuffle && len > 1 {
        let pseudo = (s.loading_generation as usize + current + 1) % len;
        return Some(if pseudo == current {
            (current + 1) % len
        } else {
            pseudo
        });
    }
    if current + 1 < len {
        Some(current + 1)
    } else if s.repeat {
        Some(0)
    } else {
        None
    }
}
