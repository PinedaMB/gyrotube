use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, CssProvider, FlowBox, Label, ListBox,
    Orientation, Overflow, Picture, PolicyType, ScrolledWindow, SearchEntry, SelectionMode,
    Spinner,
};
use libadwaita as adw;

use crate::backend::runtime;
use crate::i18n::tr;
use crate::models::{Artist, ArtistDetails, Genre, Track, format_subscribers};
use crate::ui::player::play_queue_index;
use crate::ui::state::AppState;
use crate::ui::widgets::{
    build_album_card, build_carousel_section, build_playlist_card, build_skeleton_box,
    cancel_collection_loading, clamped_cover, clear_box, highlight_active_track_in_list,
    load_cover_into_picture, open_collection_page, populate_track_list, spawn_bg,
    start_track_radio,
};

#[derive(Clone)]
pub struct ExploreActions {
    pub search: Rc<dyn Fn(&str)>,
    pub open_artist: Rc<dyn Fn(&str)>,
    pub open_album: Rc<dyn Fn(&str, &str)>,
}

pub fn build_explore_page(
    state: Rc<RefCell<AppState>>,
    top_search_entry: SearchEntry,
) -> (ScrolledWindow, ExploreActions) {
    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_margin_top(18);
    container.set_margin_bottom(88);
    container.set_margin_start(20);
    container.set_margin_end(20);

    let explore_header_box = GtkBox::new(Orientation::Vertical, 2);
    explore_header_box.append(
        &Label::builder()
            .label(tr("Explorar", "Explore"))
            .xalign(0.0)
            .css_classes(["title-1"])
            .build(),
    );
    explore_header_box.append(
        &Label::builder()
            .label(tr(
                "Estados de ánimo, momentos y géneros musicales de YouTube Music",
                "Moods, moments, and music genres from YouTube Music",
            ))
            .xalign(0.0)
            .wrap(true)
            .max_width_chars(32)
            .css_classes(["dim-label", "caption"])
            .build(),
    );

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

    // Artist details sub-view (Spotify-inspired hero + tracks + discography)
    let artist_back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Volver")
        .halign(Align::Start)
        .valign(Align::Start)
        .margin_top(14)
        .margin_start(14)
        .css_classes(["circular", "artist-hero-back"])
        .build();
    artist_back_btn.set_cursor_from_name(Some("pointer"));

    let artist_banner = Picture::builder()
        .height_request(250)
        .hexpand(true)
        .can_shrink(true)
        .content_fit(ContentFit::Cover)
        .visible(false)
        .build();

    let artist_hero_scrim = GtkBox::new(Orientation::Vertical, 0);
    artist_hero_scrim.set_hexpand(true);
    artist_hero_scrim.set_vexpand(true);
    artist_hero_scrim.set_can_target(false);
    artist_hero_scrim.add_css_class("artist-hero-scrim");

    let verified_row = GtkBox::new(Orientation::Horizontal, 6);
    let verified_icon = gtk4::Image::builder()
        .icon_name("emblem-ok-symbolic")
        .pixel_size(14)
        .css_classes(["artist-verified-icon"])
        .build();
    let verified_label = Label::builder()
        .label("Artista")
        .css_classes(["artist-verified-badge"])
        .build();
    verified_row.append(&verified_icon);
    verified_row.append(&verified_label);

    let artist_name_label = Label::builder()
        .label("Artista")
        .xalign(0.0)
        .max_width_chars(20)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["artist-hero-title"])
        .build();
    let artist_subs_label = Label::builder()
        .label("")
        .xalign(0.0)
        .css_classes(["artist-hero-subs"])
        .visible(false)
        .build();

    let artist_play_btn = Button::builder()
        .css_classes(["suggested-action", "pill"])
        .build();
    let play_btn_content = GtkBox::new(Orientation::Horizontal, 6);
    play_btn_content.append(&gtk4::Image::from_icon_name("play-filled"));
    play_btn_content.append(&Label::new(Some("Reproducir")));
    artist_play_btn.set_child(Some(&play_btn_content));
    artist_play_btn.set_cursor_from_name(Some("pointer"));

    let artist_shuffle_btn = Button::builder()
        .icon_name("arrow-shuffle-regular")
        .tooltip_text("Reproducción aleatoria")
        .valign(Align::Center)
        .css_classes(["circular", "artist-secondary-btn"])
        .build();
    artist_shuffle_btn.set_cursor_from_name(Some("pointer"));

    let artist_radio_btn = Button::builder()
        .icon_name("sound-wave-circle-regular")
        .tooltip_text("Iniciar radio del artista")
        .valign(Align::Center)
        .css_classes(["circular", "artist-secondary-btn"])
        .build();
    artist_radio_btn.set_cursor_from_name(Some("pointer"));

    let artist_actions_row = GtkBox::new(Orientation::Horizontal, 10);
    artist_actions_row.set_margin_top(6);
    artist_actions_row.append(&artist_play_btn);
    artist_actions_row.append(&artist_shuffle_btn);
    artist_actions_row.append(&artist_radio_btn);

    let artist_hero_info = GtkBox::new(Orientation::Vertical, 4);
    artist_hero_info.set_halign(Align::Start);
    artist_hero_info.set_valign(Align::End);
    artist_hero_info.set_margin_bottom(20);
    artist_hero_info.set_margin_start(22);
    artist_hero_info.set_margin_end(22);
    artist_hero_info.append(&verified_row);
    artist_hero_info.append(&artist_name_label);
    artist_hero_info.append(&artist_subs_label);
    artist_hero_info.append(&artist_actions_row);

    let artist_hero_spacer = GtkBox::new(Orientation::Vertical, 0);
    let artist_hero = gtk4::Overlay::builder()
        .height_request(250)
        .overflow(Overflow::Hidden)
        .css_classes(["artist-hero"])
        .child(&artist_hero_spacer)
        .build();
    artist_hero.add_overlay(&artist_banner);
    artist_hero.add_overlay(&artist_hero_scrim);
    artist_hero.add_overlay(&artist_back_btn);
    artist_hero.add_overlay(&artist_hero_info);

    let current_artist_tracks: Rc<RefCell<Option<(String, Vec<Track>)>>> =
        Rc::new(RefCell::new(None));
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_artist_tracks);
        artist_play_btn.connect_clicked(move |_| {
            let Some((name, tracks)) = tracks_ref.borrow().clone() else {
                return;
            };
            if tracks.is_empty() {
                return;
            }
            {
                let mut s = state.borrow_mut();
                s.shuffle = false;
                s.unshuffled_queue.clear();
                s.queue = tracks;
                s.source_name = name;
            }
            play_queue_index(Rc::clone(&state), 0);
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_artist_tracks);
        artist_shuffle_btn.connect_clicked(move |_| {
            let Some((name, tracks)) = tracks_ref.borrow().clone() else {
                return;
            };
            if tracks.is_empty() {
                return;
            }
            {
                let mut s = state.borrow_mut();
                s.set_shuffled_queue(tracks, name);
            }
            play_queue_index(Rc::clone(&state), 0);
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks_ref = Rc::clone(&current_artist_tracks);
        artist_radio_btn.connect_clicked(move |_| {
            let first = tracks_ref
                .borrow()
                .as_ref()
                .and_then(|(_, t)| t.first().cloned());
            if let Some(track) = first {
                start_track_radio(Rc::clone(&state), &track);
            }
        });
    }

    let artist_desc_label = Label::builder()
        .xalign(0.0)
        .wrap(true)
        .selectable(true)
        .css_classes(["body", "artist-bio-text"])
        .build();
    let artist_desc_header = GtkBox::new(Orientation::Horizontal, 8);
    artist_desc_header.append(&gtk4::Image::from_icon_name("person-account-regular"));
    artist_desc_header.append(
        &Label::builder()
            .label("Sobre el artista")
            .xalign(0.0)
            .css_classes(["title-4"])
            .build(),
    );
    let artist_desc_card = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .css_classes(["artist-bio-card"])
        .visible(false)
        .build();
    artist_desc_card.append(&artist_desc_header);
    artist_desc_card.append(&artist_desc_label);

    let artist_tracks_heading = Label::builder()
        .label(tr("Canciones populares", "Popular songs"))
        .xalign(0.0)
        .css_classes(["title-3"])
        .visible(false)
        .build();
    let artist_tracks_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .visible(false)
        .build();
    {
        let state_cb = Rc::clone(&state);
        let a_list = artist_tracks_list.clone();
        let s_list = results_list.clone();
        state.borrow_mut().on_track_changed.push(Rc::new(move || {
            let active_id = state_cb.borrow().current_track().map(|t| t.id.clone());
            highlight_active_track_in_list(&a_list, active_id.as_deref());
            highlight_active_track_in_list(&s_list, active_id.as_deref());
        }));
    }
    let artist_tracks_expanded = Rc::new(Cell::new(false));
    let artist_tracks_toggle_btn = Button::builder()
        .halign(Align::Start)
        .css_classes(["flat", "pill-switch"])
        .visible(false)
        .build();
    artist_tracks_toggle_btn.set_cursor_from_name(Some("pointer"));
    {
        let list = artist_tracks_list.clone();
        let expanded = Rc::clone(&artist_tracks_expanded);
        artist_tracks_toggle_btn.connect_clicked(move |btn| {
            expanded.set(!expanded.get());
            apply_artist_tracks_collapse(&list, btn, expanded.get());
        });
    }

    let (artist_albums_section, artist_albums_cards, _) =
        build_carousel_section(tr("Álbumes", "Albums"));
    let (artist_playlists_section, artist_playlists_cards, _) = build_carousel_section("Playlists");
    let (artist_similar_section, artist_similar_cards, _) =
        build_carousel_section(tr("Artistas similares", "Similar artists"));

    let artist_section = GtkBox::new(Orientation::Vertical, 18);
    artist_section.set_visible(false);
    artist_section.append(&artist_hero);
    artist_section.append(&artist_tracks_heading);
    artist_section.append(&artist_tracks_list);
    artist_section.append(&artist_tracks_toggle_btn);
    artist_section.append(&artist_albums_section);
    artist_section.append(&artist_playlists_section);
    artist_section.append(&artist_similar_section);
    artist_section.append(&artist_desc_card);

    let artist_skeleton = GtkBox::new(Orientation::Vertical, 16);
    artist_skeleton.set_visible(false);
    let hero_skel = build_skeleton_box(0, 250);
    hero_skel.add_css_class("artist-hero");
    artist_skeleton.append(&hero_skel);
    artist_skeleton.append(&build_skeleton_box(190, 20));
    let artist_tracks_skel = GtkBox::new(Orientation::Vertical, 8);
    for _ in 0..6 {
        let row = GtkBox::new(Orientation::Horizontal, 12);
        row.set_margin_top(4);
        row.set_margin_bottom(4);
        row.append(&build_skeleton_box(44, 44));
        let col = GtkBox::new(Orientation::Vertical, 6);
        col.set_hexpand(true);
        col.set_valign(Align::Center);
        col.append(&build_skeleton_box(0, 14));
        col.append(&build_skeleton_box(130, 11));
        row.append(&col);
        row.append(&build_skeleton_box(36, 14));
        artist_tracks_skel.append(&row);
    }
    artist_skeleton.append(&artist_tracks_skel);

    let search_skeleton = GtkBox::new(Orientation::Vertical, 16);
    search_skeleton.set_visible(false);
    search_skeleton.append(&build_skeleton_box(150, 20));
    let search_cards_skel = GtkBox::new(Orientation::Horizontal, 12);
    for _ in 0..4 {
        let card = GtkBox::new(Orientation::Vertical, 6);
        card.append(&build_skeleton_box(140, 140));
        card.append(&build_skeleton_box(110, 13));
        card.append(&build_skeleton_box(80, 11));
        search_cards_skel.append(&card);
    }
    search_skeleton.append(&search_cards_skel);
    search_skeleton.append(&build_skeleton_box(130, 20));
    let search_tracks_skel = GtkBox::new(Orientation::Vertical, 8);
    for _ in 0..5 {
        let row = GtkBox::new(Orientation::Horizontal, 12);
        row.set_margin_top(4);
        row.set_margin_bottom(4);
        row.append(&build_skeleton_box(44, 44));
        let col = GtkBox::new(Orientation::Vertical, 6);
        col.set_hexpand(true);
        col.set_valign(Align::Center);
        col.append(&build_skeleton_box(0, 14));
        col.append(&build_skeleton_box(140, 11));
        row.append(&col);
        row.append(&build_skeleton_box(36, 14));
        search_tracks_skel.append(&row);
    }
    search_skeleton.append(&search_tracks_skel);

    // Genres & moods grid
    let moods_title = Label::builder()
        .label(tr("Estados de ánimo y momentos", "Moods & moments"))
        .xalign(0.0)
        .css_classes(["title-3"])
        .build();
    let moods_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .column_spacing(12)
        .row_spacing(10)
        .min_children_per_line(1)
        .max_children_per_line(4)
        .homogeneous(true)
        .build();

    let genres_title = Label::builder()
        .label(tr("Géneros musicales", "Music genres"))
        .xalign(0.0)
        .margin_top(8)
        .css_classes(["title-3"])
        .build();
    let genres_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .column_spacing(12)
        .row_spacing(10)
        .min_children_per_line(1)
        .max_children_per_line(4)
        .homogeneous(true)
        .build();

    let genres_section = GtkBox::new(Orientation::Vertical, 12);
    genres_section.append(&moods_title);
    genres_section.append(&moods_flow);
    genres_section.append(&genres_title);
    genres_section.append(&genres_flow);

    // Genre playlists sub-view
    let genre_back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text(tr("Volver a géneros", "Back to genres"))
        .css_classes(["flat", "circular"])
        .build();
    let genre_heading = Label::builder()
        .label("Playlists")
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-2"])
        .build();
    let genre_header = GtkBox::new(Orientation::Horizontal, 10);
    genre_header.append(&genre_back_btn);
    genre_header.append(&genre_heading);

    let playlists_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .homogeneous(true)
        .column_spacing(12)
        .row_spacing(12)
        .min_children_per_line(1)
        .max_children_per_line(4)
        .build();

    let genre_playlists_skeleton = GtkBox::new(Orientation::Vertical, 12);
    genre_playlists_skeleton.set_visible(false);
    let genre_skel_header = GtkBox::new(Orientation::Horizontal, 10);
    let genre_skel_back = build_skeleton_box(34, 34);
    genre_skel_back.add_css_class("circular");
    genre_skel_header.append(&genre_skel_back);
    genre_skel_header.append(&build_skeleton_box(180, 22));
    genre_playlists_skeleton.append(&genre_skel_header);

    let genre_skel_flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .min_children_per_line(2)
        .max_children_per_line(6)
        .row_spacing(14)
        .column_spacing(14)
        .homogeneous(true)
        .build();
    for _ in 0..10 {
        let card = GtkBox::new(Orientation::Vertical, 6);
        card.append(&build_skeleton_box(156, 156));
        card.append(&build_skeleton_box(128, 13));
        card.append(&build_skeleton_box(92, 11));
        genre_skel_flow.insert(&card, -1);
    }
    genre_playlists_skeleton.append(&genre_skel_flow);

    let genre_playlists_section = GtkBox::new(Orientation::Vertical, 12);
    genre_playlists_section.set_visible(false);
    genre_playlists_section.append(&genre_header);
    genre_playlists_section.append(&playlists_flow);

    genres_section
        .bind_property("visible", &explore_header_box, "visible")
        .sync_create()
        .build();

    container.append(&explore_header_box);
    container.append(&spinner);
    container.append(&status_label);
    container.append(&search_skeleton);
    container.append(&search_results_section);
    container.append(&artist_skeleton);
    container.append(&artist_section);
    container.append(&genres_section);
    container.append(&genre_playlists_skeleton);
    container.append(&genre_playlists_section);

    let clamp = adw::Clamp::builder()
        .maximum_size(1100)
        .child(&container)
        .build();

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&clamp)
        .build();

    // Load genres on startup (cached)
    {
        let backend = state.borrow().backend.clone();
        let (tx, rx) = async_channel::bounded(1);
        runtime().spawn(async move {
            let res = backend.fetch_genres_cached(false).await;
            let _ = tx.send(res).await;
        });

        let state = Rc::clone(&state);
        let moods_flow = moods_flow.clone();
        let genres_flow = genres_flow.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_skeleton = genre_playlists_skeleton.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        let genre_heading = genre_heading.clone();
        let playlists_flow = playlists_flow.clone();

        glib::spawn_future_local(async move {
            if let Ok(Ok(genres)) = rx.recv().await {
                let mood_names = [
                    "chill", "christmas", "commute", "energize", "feel good", "focus",
                    "gaming", "party", "romance", "sad", "sleep", "workout",
                ];
                for genre in genres {
                    let btn = build_genre_button(
                        &genre,
                        Rc::clone(&state),
                        &genres_section,
                        &genre_playlists_skeleton,
                        &genre_playlists_section,
                        &genre_heading,
                        &playlists_flow,
                    );
                    let lower = genre.name.to_lowercase();
                    if mood_names.iter().any(|m| lower.contains(m)) {
                        moods_flow.insert(&btn, -1);
                    } else {
                        genres_flow.insert(&btn, -1);
                    }
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

    let artist_history: Rc<RefCell<Vec<(String, String)>>> = Rc::new(RefCell::new(Vec::new()));
    let current_artist: Rc<RefCell<Option<(String, String)>>> = Rc::new(RefCell::new(None));

    let artist_widgets = ArtistPageWidgets {
        spinner: spinner.clone(),
        status_label: status_label.clone(),
        search_results_section: search_results_section.clone(),
        genres_section: genres_section.clone(),
        genre_playlists_section: genre_playlists_section.clone(),
        artist_section: artist_section.clone(),
        skeleton: artist_skeleton.clone(),
        artist_history: Rc::clone(&artist_history),
        current_artist: Rc::clone(&current_artist),
        name_label: artist_name_label,
        subs_label: artist_subs_label,
        banner: artist_banner,
        play_btn: artist_play_btn,
        shuffle_btn: artist_shuffle_btn,
        current_tracks: current_artist_tracks,
        desc_card: artist_desc_card,
        desc_label: artist_desc_label,
        tracks_heading: artist_tracks_heading,
        tracks_list: artist_tracks_list,
        tracks_toggle_btn: artist_tracks_toggle_btn,
        tracks_expanded: artist_tracks_expanded,
        albums_section: artist_albums_section,
        albums_cards: artist_albums_cards,
        playlists_section: artist_playlists_section,
        playlists_cards: artist_playlists_cards,
        similar_section: artist_similar_section,
        similar_cards: artist_similar_cards,
    };

    {
        let top_search_entry = top_search_entry.clone();
        let artist_section = artist_section.clone();
        let artist_skeleton_ref = artist_skeleton.clone();
        let search_skeleton_ref = search_skeleton.clone();
        let search_results_section = search_results_section.clone();
        let genres_section = genres_section.clone();
        let aw_back = artist_widgets.clone();
        let state_back = Rc::clone(&state);
        artist_back_btn.connect_clicked(move |_| {
            if let Some((prev_id, prev_name)) = aw_back.artist_history.borrow_mut().pop() {
                *aw_back.current_artist.borrow_mut() = None;
                open_artist_page(&prev_id, &prev_name, aw_back.clone(), Rc::clone(&state_back));
                return;
            }
            *aw_back.current_artist.borrow_mut() = None;
            artist_section.set_visible(false);
            artist_skeleton_ref.set_visible(false);
            search_skeleton_ref.set_visible(false);
            if top_search_entry.text().trim().is_empty() {
                genres_section.set_visible(true);
            } else {
                search_results_section.set_visible(true);
            }
        });
    }

    let run_search: Rc<dyn Fn(&str)> = {
        let state = Rc::clone(&state);
        let status_label = status_label.clone();
        let search_skeleton_ref = search_skeleton.clone();
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
        let artist_skeleton_ref = artist_skeleton.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_skeleton_ref = genre_playlists_skeleton.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        let artist_widgets = artist_widgets.clone();

        Rc::new(move |raw_query: &str| {
            let query = raw_query.trim().to_string();
            if query.is_empty() {
                search_results_section.set_visible(false);
                search_skeleton_ref.set_visible(false);
                artist_section.set_visible(false);
                artist_skeleton_ref.set_visible(false);
                status_label.set_visible(false);
                genre_playlists_skeleton_ref.set_visible(false);
                genre_playlists_section.set_visible(false);
                genres_section.set_visible(true);
                return;
            }

            genres_section.set_visible(false);
            genre_playlists_skeleton_ref.set_visible(false);
            genre_playlists_section.set_visible(false);
            artist_section.set_visible(false);
            artist_skeleton_ref.set_visible(false);
            search_skeleton_ref.set_visible(true);
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
            let search_skeleton_done = search_skeleton_ref.clone();
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
                    search_skeleton_done.set_visible(false);
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
        let search_results_section = search_results_section.clone();
        let artist_section = artist_section.clone();
        let status_label = status_label.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_skeleton_ref = genre_playlists_skeleton.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        top_search_entry.connect_search_changed(move |entry| {
            if entry.text().trim().is_empty() {
                search_results_section.set_visible(false);
                artist_section.set_visible(false);
                artist_skeleton.set_visible(false);
                status_label.set_visible(false);
                genre_playlists_skeleton_ref.set_visible(false);
                genre_playlists_section.set_visible(false);
                genres_section.set_visible(true);
            }
        });
    }

    let open_artist: Rc<dyn Fn(&str)> = {
        let state = Rc::clone(&state);
        let artist_widgets = artist_widgets.clone();
        let run_search = Rc::clone(&run_search);
        Rc::new(move |artist_name: &str| {
            let name = artist_name.trim().to_string();
            if name.is_empty() {
                return;
            }
            artist_widgets.search_results_section.set_visible(false);
            artist_widgets.genres_section.set_visible(false);
            artist_widgets.genre_playlists_section.set_visible(false);
            artist_widgets.artist_section.set_visible(false);
            artist_widgets.status_label.set_visible(false);
            artist_widgets.spinner.set_spinning(false);
            artist_widgets.spinner.set_visible(false);
            artist_widgets.skeleton.set_visible(true);

            let backend = state.borrow().backend.clone();
            let query = name.clone();
            let state_cb = Rc::clone(&state);
            let aw = artist_widgets.clone();
            let fallback_search = Rc::clone(&run_search);
            spawn_bg(
                move || async move { backend.search_all(&query).await },
                move |res| {
                    if let Ok(sr) = res
                        && let Some(artist) = sr
                            .artists
                            .iter()
                            .find(|a| a.name.eq_ignore_ascii_case(&name))
                            .or_else(|| sr.artists.first())
                    {
                        open_artist_page(&artist.id, &artist.name, aw, state_cb);
                    } else {
                        aw.skeleton.set_visible(false);
                        fallback_search(&name);
                    }
                },
            );
        })
    };

    let open_album: Rc<dyn Fn(&str, &str)> = {
        let state = Rc::clone(&state);
        let run_search = Rc::clone(&run_search);
        Rc::new(move |album_name: &str, artist_name: &str| {
            let album = album_name.trim().to_string();
            if album.is_empty() {
                return;
            }
            let query = if artist_name.trim().is_empty() {
                album.clone()
            } else {
                format!("{album} {artist_name}")
            };
            open_collection_page(album.clone(), None, None, Rc::clone(&state));
            let backend = state.borrow().backend.clone();
            let state_cb = Rc::clone(&state);
            let fallback_search = Rc::clone(&run_search);
            let album_fallback = album.clone();
            spawn_bg(
                move || async move {
                    let sr = backend.search_all(&query).await?;
                    let matched = sr
                        .albums
                        .iter()
                        .find(|a| a.title.eq_ignore_ascii_case(&album))
                        .or_else(|| sr.albums.first())
                        .cloned();
                    if let Some(a) = matched {
                        let (title, tracks) = backend.fetch_album_tracks(&a.id).await?;
                        Ok(Some((title, a.cover_url, tracks)))
                    } else {
                        Ok(None)
                    }
                },
                move |res: Result<Option<(String, Option<String>, Vec<crate::models::Track>)>, String>| {
                    match res {
                        Ok(Some((title, cover, tracks))) if !tracks.is_empty() => {
                            open_collection_page(title, cover, Some(tracks), state_cb);
                        }
                        Ok(_) => {
                            cancel_collection_loading(&state_cb);
                            fallback_search(&album_fallback);
                        }
                        Err(err) => {
                            cancel_collection_loading(&state_cb);
                            state_cb.borrow().show_toast(&err);
                        }
                    }
                },
            );
        })
    };

    (
        scrolled,
        ExploreActions {
            search: run_search,
            open_artist,
            open_album,
        },
    )
}

#[derive(Clone)]
struct ArtistPageWidgets {
    spinner: Spinner,
    status_label: Label,
    search_results_section: GtkBox,
    genres_section: GtkBox,
    genre_playlists_section: GtkBox,
    artist_section: GtkBox,
    skeleton: GtkBox,
    artist_history: Rc<RefCell<Vec<(String, String)>>>,
    current_artist: Rc<RefCell<Option<(String, String)>>>,
    name_label: Label,
    subs_label: Label,
    banner: Picture,
    play_btn: Button,
    shuffle_btn: Button,
    current_tracks: Rc<RefCell<Option<(String, Vec<Track>)>>>,
    desc_card: GtkBox,
    desc_label: Label,
    tracks_heading: Label,
    tracks_list: ListBox,
    tracks_toggle_btn: Button,
    tracks_expanded: Rc<Cell<bool>>,
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
    if ui.artist_section.is_visible()
        && let Some(prev) = ui.current_artist.borrow().clone()
        && prev.0 != artist_id
    {
        ui.artist_history.borrow_mut().push(prev);
    }
    *ui.current_artist.borrow_mut() = Some((artist_id.to_string(), artist_name.to_string()));

    ui.search_results_section.set_visible(false);
    ui.genres_section.set_visible(false);
    ui.genre_playlists_section.set_visible(false);
    ui.artist_section.set_visible(false);
    ui.status_label.set_visible(false);
    ui.spinner.set_spinning(false);
    ui.spinner.set_visible(false);
    ui.skeleton.set_visible(true);

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
            ui.skeleton.set_visible(false);
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
    mut details: ArtistDetails,
    ui: ArtistPageWidgets,
    state: Rc<RefCell<AppState>>,
) {
    details.tracks.truncate(25);
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
        ui.desc_card.set_visible(true);
    } else {
        ui.desc_card.set_visible(false);
    }

    let has_tracks = !details.tracks.is_empty();
    ui.play_btn.set_visible(has_tracks);
    ui.shuffle_btn.set_visible(has_tracks);
    *ui.current_tracks.borrow_mut() = if has_tracks {
        Some((details.name.clone(), details.tracks.clone()))
    } else {
        None
    };

    if !has_tracks {
        ui.tracks_heading.set_visible(false);
        ui.tracks_list.set_visible(false);
        ui.tracks_toggle_btn.set_visible(false);
    } else {
        populate_track_list(
            &ui.tracks_list,
            &details.tracks,
            &details.name,
            Rc::clone(&state),
        );
        ui.tracks_expanded.set(false);
        apply_artist_tracks_collapse(&ui.tracks_list, &ui.tracks_toggle_btn, false);
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
    ui.playlists_section
        .set_visible(!details.playlists.is_empty());

    clear_box(&ui.similar_cards);
    for similar in &details.similar_artists {
        ui.similar_cards
            .append(&build_artist_card(similar, Rc::clone(&state), ui.clone()));
    }
    ui.similar_section
        .set_visible(!details.similar_artists.is_empty());

    ui.artist_section.set_visible(true);
}

fn apply_artist_tracks_collapse(list: &ListBox, toggle: &Button, expanded: bool) {
    const INITIAL_TRACKS: usize = 5;
    let mut total = 0usize;
    let mut child = list.first_child();
    while let Some(row) = child {
        child = row.next_sibling();
        row.set_visible(expanded || total < INITIAL_TRACKS);
        total += 1;
    }
    if total <= INITIAL_TRACKS {
        toggle.set_visible(false);
    } else {
        let content = GtkBox::new(Orientation::Horizontal, 6);
        if expanded {
            content.append(&gtk4::Image::from_icon_name("pan-up-symbolic"));
            content.append(&Label::new(Some(tr("Mostrar menos", "Show less"))));
        } else {
            content.append(&gtk4::Image::from_icon_name("pan-down-symbolic"));
            content.append(&Label::new(Some(&format!(
                "{} {} {}",
                tr("Ver", "Show"),
                total - INITIAL_TRACKS,
                tr("más", "more")
            ))));
        }
        toggle.set_child(Some(&content));
        toggle.set_visible(true);
    }
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
    genre_playlists_skeleton: &GtkBox,
    genre_playlists_section: &GtkBox,
    genre_heading: &Label,
    playlists_flow: &FlowBox,
) -> Button {
    let color = &genre.color_hex;

    let accent_bar = GtkBox::builder()
        .valign(Align::Center)
        .css_classes(["genre-accent-bar"])
        .build();
    let bar_css = CssProvider::new();
    bar_css.load_from_string(&format!("box {{ background-color: {color}; }}"));
    #[allow(deprecated)]
    accent_bar
        .style_context()
        .add_provider(&bar_css, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let name_label = Label::builder()
        .label(&genre.name)
        .xalign(0.0)
        .hexpand(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"])
        .build();

    let arrow = gtk4::Image::builder()
        .icon_name("go-next-symbolic")
        .pixel_size(14)
        .valign(Align::Center)
        .css_classes(["genre-arrow"])
        .build();

    let inner = GtkBox::new(Orientation::Horizontal, 10);
    inner.append(&accent_bar);
    inner.append(&name_label);
    inner.append(&arrow);

    let btn = Button::builder()
        .child(&inner)
        .css_classes(["flat", "genre-chip"])
        .build();
    btn.set_cursor_from_name(Some("pointer"));
    let card_css = CssProvider::new();
    card_css.load_from_string(&format!(
        "button {{ background-image: linear-gradient(90deg, alpha({color}, 0.14) 0%, alpha(@window_fg_color, 0.04) 60%); }}
         button:hover {{ background-image: linear-gradient(90deg, alpha({color}, 0.24) 0%, alpha(@window_fg_color, 0.07) 65%); border-color: alpha({color}, 0.45); }}"
    ));
    #[allow(deprecated)]
    btn.style_context()
        .add_provider(&card_css, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let genre_id = genre.id.clone();
    let genre_name = genre.name.clone();
    let genres_section = genres_section.clone();
    let genre_playlists_skeleton = genre_playlists_skeleton.clone();
    let genre_playlists_section = genre_playlists_section.clone();
    let genre_heading = genre_heading.clone();
    let playlists_flow = playlists_flow.clone();

    btn.connect_clicked(move |_| {
        genres_section.set_visible(false);
        genre_playlists_section.set_visible(false);
        genre_playlists_skeleton.set_visible(true);

        let backend = state.borrow().backend.clone();
        let (tx, rx) = async_channel::bounded(1);
        let id = genre_id.clone();
        runtime().spawn(async move {
            let res = backend.fetch_genre_playlists(&id).await;
            let _ = tx.send(res).await;
        });

        let state = Rc::clone(&state);
        let genres_section = genres_section.clone();
        let genre_playlists_skeleton = genre_playlists_skeleton.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        let genre_heading = genre_heading.clone();
        let playlists_flow = playlists_flow.clone();
        let fallback_name = genre_name.clone();

        glib::spawn_future_local(async move {
            if let Ok(result) = rx.recv().await {
                genre_playlists_skeleton.set_visible(false);
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
                        genres_section.set_visible(true);
                        state.borrow().show_toast(&err);
                    }
                }
            }
        });
    });

    btn
}

