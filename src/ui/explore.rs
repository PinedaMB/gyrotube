use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use tokio::sync::watch;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, CssProvider, FlowBox, Label, ListBox,
    Orientation, Overflow, Picture, PolicyType, ScrolledWindow, SearchEntry, SelectionMode,
    Spinner, ToggleButton,
};
use libadwaita as adw;

use crate::backend::runtime;
use crate::i18n::tr;
use crate::models::{Artist, ArtistDetails, Genre, Track, format_subscribers};
use crate::ui::player::play_queue_index;
use crate::ui::state::{AppState, ViewRequests};
use crate::ui::widgets::{
    begin_collection_loading, build_album_card, build_carousel_section, build_playlist_card,
    build_skeleton_box, cancel_collection_loading, clear_box,
    highlight_active_track_in_list, load_cover_into_picture,
    load_cover_into_picture_with_focal_if,
    open_collection_page, populate_track_list, rail_art, spawn_bg, start_track_radio,
};

#[derive(Clone)]
pub struct ExploreActions {
    pub search: Rc<dyn Fn(&str)>,
    pub open_artist: Rc<dyn Fn(&str)>,
    pub open_album: Rc<dyn Fn(&str, &str)>,
}

fn build_search_filters(
    sections: [GtkBox; 4],
    scroll: gtk4::Adjustment,
) -> (GtkBox, impl Fn([bool; 4]) + Clone) {
    let row = FlowBox::builder()
        .halign(Align::Start)
        .selection_mode(SelectionMode::None)
        .activate_on_single_click(false)
        .column_spacing(6)
        .row_spacing(6)
        .min_children_per_line(1)
        .max_children_per_line(5)
        .build();
    row.update_property(&[gtk4::accessible::Property::Label(tr(
        "Filtrar resultados",
        "Filter results",
    ))]);

    let empty = Label::builder()
        .label(tr("No hay resultados de este tipo.", "No results of this type."))
        .xalign(0.0)
        .wrap(true)
        .css_classes(["dim-label"])
        .visible(false)
        .build();
    let available = Rc::new(Cell::new([false; 4]));
    let apply = {
        let available = available.clone();
        let empty = empty.clone();
        move |selected: usize| {
            let mut shown = false;
            for (index, (section, present)) in sections.iter().zip(available.get()).enumerate() {
                let visible = present && (selected == 0 || selected == index + 1);
                section.set_visible(visible);
                shown |= visible;
            }
            empty.set_visible(!shown);
        }
    };

    let labels = [
        tr("Todo", "All"),
        tr("Canciones", "Songs"),
        tr("Álbumes", "Albums"),
        tr("Artistas", "Artists"),
        tr("Playlists", "Playlists"),
    ];
    let buttons = labels.map(|label| {
        ToggleButton::builder()
            .label(label)
            .css_classes(["flat", "pill-switch"])
            .build()
    });
    let all = buttons[0].clone();
    for (index, button) in buttons.iter().enumerate() {
        if index != 0 {
            button.set_group(Some(&all));
        }
        let apply = apply.clone();
        let scroll = scroll.clone();
        button.connect_toggled(move |button| {
            if button.is_active() {
                apply(index);
                scroll.set_value(scroll.lower());
            }
        });
        let cell = gtk4::FlowBoxChild::builder().child(button).focusable(false).build();
        row.insert(&cell, -1);
    }

    let filters = GtkBox::new(Orientation::Vertical, 12);
    filters.append(&row);
    filters.append(&empty);
    let new_results = move |present| {
        available.set(present);
        all.set_active(true);
        apply(0);
        scroll.set_value(scroll.lower());
    };
    (filters, new_results)
}

const SEARCH_PRELOAD_DELAY: Duration = Duration::from_millis(220);

async fn next_search_preload(intents: &mut watch::Receiver<Option<Track>>) -> Option<Track> {
    while intents.changed().await.is_ok() {
        while intents.borrow().is_some() {
            tokio::select! {
                biased;
                change = intents.changed() => {
                    change.ok()?;
                }
                _ = tokio::time::sleep(SEARCH_PRELOAD_DELAY) => {
                    if let Some(track) = intents.borrow_and_update().clone() {
                        return Some(track);
                    }
                }
            }
        }
    }
    None
}

fn watch_track_intent(row: &gtk4::ListBoxRow, track: Track, intents: &watch::Sender<Option<Track>>) {
    let motion = gtk4::EventControllerMotion::new();
    let focus = gtk4::EventControllerFocus::new();
    let hovered = Rc::new(Cell::new(false));
    let focused = Rc::new(Cell::new(false));
    let update = {
        let intents = intents.clone();
        let hovered = hovered.clone();
        let focused = focused.clone();
        move || {
            intents.send_if_modified(|current| {
                let is_current = current.as_ref().is_some_and(|entry| entry.id == track.id);
                match (hovered.get() || focused.get(), is_current) {
                    (true, false) => *current = Some(track.clone()),
                    (false, true) => *current = None,
                    _ => return false,
                }
                true
            });
        }
    };
    let enter = update.clone();
    let entering = hovered.clone();
    motion.connect_enter(move |_, _, _| {
        entering.set(true);
        enter();
    });
    let leave = update.clone();
    motion.connect_leave(move |_| {
        hovered.set(false);
        leave();
    });
    let enter = update.clone();
    let entering = focused.clone();
    focus.connect_enter(move |_| {
        entering.set(true);
        enter();
    });
    focus.connect_leave(move |_| {
        focused.set(false);
        update();
    });
    row.add_controller(motion);
    row.add_controller(focus);
}

pub fn build_explore_page(
    state: Rc<RefCell<AppState>>,
    top_search_entry: SearchEntry,
    content_stack: &libadwaita::ViewStack,
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

    let (search_artists_section, search_artists_cards, _) = build_carousel_section(tr("Artistas", "Artists"));
    let (search_albums_section, search_albums_cards, _) = build_carousel_section(tr("Álbumes", "Albums"));
    let (search_playlists_section, search_playlists_cards, _) = build_carousel_section("Playlists");

    let search_tracks_heading = Label::builder()
        .label(tr("Canciones", "Songs"))
        .xalign(0.0)
        .css_classes(["title-3"])
        .build();

    let results_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    let search_tracks_section = GtkBox::new(Orientation::Vertical, 10);
    search_tracks_section.append(&search_tracks_heading);
    search_tracks_section.append(&results_list);

    let search_results_section = GtkBox::new(Orientation::Vertical, 18);
    search_results_section.set_visible(false);
    search_results_section.append(&search_artists_section);
    search_results_section.append(&search_albums_section);
    search_results_section.append(&search_playlists_section);
    search_results_section.append(&search_tracks_section);

    // Artist details sub-view (Spotify-inspired hero + tracks + discography)
    let artist_back_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text(tr("Volver", "Back"))
        .halign(Align::Start)
        .valign(Align::Start)
        .margin_top(14)
        .margin_start(14)
        .css_classes(["circular", "artist-hero-back"])
        .build();
    artist_back_btn.set_cursor_from_name(Some("pointer"));

    let artist_banner = Picture::builder()
        .height_request(340)
        .hexpand(true)
        .can_shrink(true)
        .content_fit(ContentFit::Fill)
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
        .label(tr("Artista", "Artist"))
        .css_classes(["artist-verified-badge"])
        .build();
    verified_row.append(&verified_icon);
    verified_row.append(&verified_label);

    let artist_name_label = Label::builder()
        .label(tr("Artista", "Artist"))
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
    play_btn_content.append(&Label::new(Some(tr("Reproducir", "Play"))));
    artist_play_btn.set_child(Some(&play_btn_content));
    artist_play_btn.set_cursor_from_name(Some("pointer"));

    let artist_shuffle_btn = Button::builder()
        .icon_name("arrow-shuffle-regular")
        .tooltip_text(tr("Reproducción aleatoria", "Shuffle"))
        .valign(Align::Center)
        .css_classes(["circular", "artist-secondary-btn"])
        .build();
    artist_shuffle_btn.set_cursor_from_name(Some("pointer"));

    let artist_radio_btn = Button::builder()
        .icon_name("sound-wave-circle-regular")
        .tooltip_text(tr("Iniciar radio del artista", "Start artist radio"))
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
        .height_request(340)
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
                s.replace_queue(tracks, name);
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
            .label(tr("Sobre el artista", "About the artist"))
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
    let hero_skel = build_skeleton_box(0, 340);
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

    let (filters, show_new_results) = build_search_filters(
        [
            search_tracks_section,
            search_albums_section,
            search_artists_section,
            search_playlists_section,
        ],
        scrolled.vadjustment(),
    );
    search_results_section.prepend(&filters);
    let has_search_results = Rc::new(Cell::new(false));

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
        let requests = state.borrow().view_requests.clone();
        genre_back_btn.connect_clicked(move |_| {
            requests.begin();
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
        search_skeleton: search_skeleton.clone(),
        genre_skeleton: genre_playlists_skeleton.clone(),
        banners: ViewRequests::default(),
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
        let has_search_results = has_search_results.clone();
        let artist_section = artist_section.clone();
        let artist_skeleton_ref = artist_skeleton.clone();
        let search_skeleton_ref = search_skeleton.clone();
        let search_results_section = search_results_section.clone();
        let genres_section = genres_section.clone();
        let aw_back = artist_widgets.clone();
        let state_back = Rc::clone(&state);
        artist_back_btn.connect_clicked(move |_| {
            state_back.borrow().view_requests.begin();
            if let Some((prev_id, prev_name)) = aw_back.artist_history.borrow_mut().pop() {
                *aw_back.current_artist.borrow_mut() = None;
                open_artist_page(
                    &prev_id,
                    &prev_name,
                    aw_back.clone(),
                    Rc::clone(&state_back),
                );
                return;
            }
            *aw_back.current_artist.borrow_mut() = None;
            artist_section.set_visible(false);
            artist_skeleton_ref.set_visible(false);
            search_skeleton_ref.set_visible(false);
            if has_search_results.get() {
                search_results_section.set_visible(true);
            } else {
                genres_section.set_visible(true);
            }
        });
    }

    let (hover_intents, mut queued_intents) = watch::channel(None);
    let backend = state.borrow().backend.clone();
    runtime().spawn(async move {
        while let Some(track) = next_search_preload(&mut queued_intents).await {
            let _ = backend.resolve_track_stream(&track).await;
        }
    });
    let pending = hover_intents.clone();
    results_list.connect_unmap(move |_| {
        pending.send_replace(None);
    });

    let run_search: Rc<dyn Fn(&str)> = {
        let state = Rc::clone(&state);
        let status_label = status_label.clone();
        let search_skeleton_ref = search_skeleton.clone();
        let search_results_section = search_results_section.clone();
        let search_artists_cards = search_artists_cards.clone();
        let search_albums_cards = search_albums_cards.clone();
        let search_playlists_cards = search_playlists_cards.clone();
        let results_list = results_list.clone();
        let artist_section = artist_section.clone();
        let artist_skeleton_ref = artist_skeleton.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_skeleton_ref = genre_playlists_skeleton.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        let artist_widgets = artist_widgets.clone();

        let has_search_results = has_search_results.clone();
        let scroll = scrolled.vadjustment();
        let stack = content_stack.clone();
        Rc::new(move |raw_query: &str| {
            hover_intents.send_replace(None);
            let query = raw_query.trim().to_string();
            if !query.is_empty() {
                stack.set_visible_child_name("explore");
            }
            let request = state.borrow().view_requests.begin();
            has_search_results.set(false);
            scroll.set_value(scroll.lower());
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
            let search_artists_cards = search_artists_cards.clone();
            let search_albums_cards = search_albums_cards.clone();
            let search_playlists_cards = search_playlists_cards.clone();
            let results_list = results_list.clone();
            let artist_widgets = artist_widgets.clone();

            let hover_intents = hover_intents.clone();
            let show_new_results = show_new_results.clone();
            let has_search_results = has_search_results.clone();
            glib::spawn_future_local(async move {
                if let Ok(result) = rx.recv().await
                    && state.borrow().view_requests.is_current(request)
                {
                    search_skeleton_done.set_visible(false);
                    match result {
                        Ok(res)
                            if res.artists.is_empty()
                                && res.albums.is_empty()
                                && res.playlists.is_empty()
                                && res.tracks.is_empty() =>
                        {
                            status_label.set_label(tr(
                                "No se encontraron resultados.",
                                "No results found.",
                            ));
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

                            clear_box(&search_albums_cards);
                            for album in &res.albums {
                                search_albums_cards
                                    .append(&build_album_card(album, Rc::clone(&state)));
                            }

                            clear_box(&search_playlists_cards);
                            for playlist in &res.playlists {
                                search_playlists_cards
                                    .append(&build_playlist_card(playlist, Rc::clone(&state)));
                            }

                            populate_track_list(
                                &results_list,
                                &res.tracks,
                                &format!("Búsqueda: {query_for_source}"),
                                Rc::clone(&state),
                            );

                            has_search_results.set(true);
                            search_results_section.set_visible(true);
                            show_new_results([
                                !res.tracks.is_empty(),
                                !res.albums.is_empty(),
                                !res.artists.is_empty(),
                                !res.playlists.is_empty(),
                            ]);
                            status_label.set_visible(false);
                            for (index, track) in res.tracks.iter().enumerate() {
                                if let Some(row) = results_list.row_at_index(index as i32) {
                                    watch_track_intent(&row, track.clone(), &hover_intents);
                                }
                            }
                            if let Some(first) = res.tracks.first().cloned() {
                                hover_intents.send_if_modified(|current| {
                                    if current.is_some() {
                                        return false;
                                    }
                                    *current = Some(first);
                                    true
                                });
                            }
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
        let search_skeleton = search_skeleton.clone();
        let requests = state.borrow().view_requests.clone();
        let stack = content_stack.clone();
        let has_search_results = has_search_results.clone();
        top_search_entry.connect_changed(move |entry| {
            let exploring = stack.visible_child_name().as_deref() == Some("explore");
            if entry.text().trim().is_empty() || (exploring && search_skeleton.is_visible()) {
                has_search_results.set(false);
                if exploring {
                    requests.begin();
                }
                search_skeleton.set_visible(false);
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

    {
        let loading = [
            search_skeleton,
            artist_widgets.skeleton.clone(),
            genre_playlists_skeleton,
        ];
        let genres_section = genres_section.clone();
        content_stack.connect_visible_child_name_notify(move |stack| {
            if stack.visible_child_name().as_deref() != Some("explore")
                && loading.iter().any(|widget| widget.is_visible())
            {
                for widget in &loading {
                    widget.set_visible(false);
                }
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
            let request = state.borrow().view_requests.begin();
            artist_widgets.search_skeleton.set_visible(false);
            artist_widgets.genre_skeleton.set_visible(false);
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
                    if !state_cb.borrow().view_requests.is_current(request) {
                        return;
                    }
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
            let request = begin_collection_loading(album.clone(), None, &state);
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
                move |res: Result<
                    Option<(String, Option<String>, Vec<crate::models::Track>)>,
                    String,
                >| {
                    if !state_cb.borrow().view_requests.is_current(request) {
                        return;
                    }
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
    search_skeleton: GtkBox,
    genre_skeleton: GtkBox,
    banners: ViewRequests,
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
    let request = state.borrow().view_requests.begin();
    ui.search_skeleton.set_visible(false);
    ui.genre_skeleton.set_visible(false);
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
        if let Ok(result) = rx.recv().await
            && state.borrow().view_requests.is_current(request)
        {
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

    let banner_request = ui.banners.begin();
    if let Some(url) = &details.header_url {
        ui.banner.set_paintable(None::<&gdk::Paintable>);
        let banners = ui.banners.clone();
        load_cover_into_picture_with_focal_if(
            &ui.banner,
            url,
            0.22,
            Rc::clone(&state),
            move || banners.is_current(banner_request),
        );
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
    let (avatar_frame, avatar) = rail_art(128, "avatar-art");
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
        .unwrap_or_else(|| String::from(tr("Artista", "Artist")));
    let subs_label = Label::builder()
        .label(&subs_text)
        .halign(Align::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(16)
        .css_classes(["dim-label", "caption"])
        .build();

    let card_box = GtkBox::new(Orientation::Vertical, 6);
    card_box.append(&avatar_frame);
    card_box.append(&name_label);
    card_box.append(&subs_label);

    let btn = Button::builder()
        .child(&card_box)
        .tooltip_text(&artist.name)
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
        let request = state.borrow().view_requests.begin();
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
            if let Ok(result) = rx.recv().await
                && state.borrow().view_requests.is_current(request)
            {
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

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[tokio::test]
    async fn speculative_preload_waits_for_the_latest_non_cancelled_row() {
        let track = |id: &str| -> Track {
            serde_json::from_value(serde_json::json!({"id":id,"title":"Song","artist":"Artist"})).unwrap()
        };
        let (tx, mut rx) = watch::channel(None);
        let updates = tx.clone();
        let sender = tokio::spawn(async move {
            updates.send_replace(Some(track("abcdefghijk")));
            tokio::time::sleep(Duration::from_millis(25)).await;
            updates.send_replace(None);
            tokio::time::sleep(Duration::from_millis(25)).await;
            updates.send_replace(Some(track("bcdefghijkl")));
        });
        assert_eq!(next_search_preload(&mut rx).await.unwrap().id, "bcdefghijkl");
        sender.await.unwrap();
        tx.send_replace(Some(track("cdefghijklm")));
        tx.send_replace(None);
        drop(tx);
        assert!(next_search_preload(&mut rx).await.is_none());
    }

    #[test]
    #[ignore = "Requires a GTK display"]
    fn filters_switch_loaded_results_and_reset_for_a_new_search() {
        adw::init().unwrap();
        let provider = CssProvider::new();
        provider.load_from_string(include_str!("../style.css"));
        gtk4::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let scroll = gtk4::Adjustment::new(0.0, 0.0, 1500.0, 20.0, 100.0, 250.0);
        let sections = std::array::from_fn(|_| GtkBox::new(Orientation::Vertical, 0));
        let (filters, new_results) = build_search_filters(sections.clone(), scroll.clone());
        let row = filters.first_child().and_downcast::<FlowBox>().unwrap();
        let empty = filters.last_child().unwrap();
        let button = |index| {
            row.child_at_index(index)
                .and_then(|cell| cell.child())
                .and_downcast::<ToggleButton>()
                .unwrap()
        };

        new_results([true, true, true, false]);
        assert!(button(0).is_active());
        assert_eq!(sections.each_ref().map(|s| s.is_visible()), [true, true, true, false]);

        scroll.set_value(320.0);
        button(1).emit_clicked();
        assert!(button(1).is_active() && !button(0).is_active());
        assert_eq!(sections.each_ref().map(|s| s.is_visible()), [true, false, false, false]);
        assert_eq!(scroll.value(), 0.0);
        assert!(!empty.is_visible());

        button(4).emit_clicked();
        assert!(empty.is_visible());
        assert!(!sections.iter().any(WidgetExt::is_visible));

        new_results([false, true, false, true]);
        assert!(button(0).is_active() && !button(4).is_active());
        assert_eq!(sections.each_ref().map(|s| s.is_visible()), [false, true, false, true]);

        let window = gtk4::Window::builder().default_width(310).child(&filters).build();
        window.present();
        let context = glib::MainContext::default();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while button(4).width() == 0 && std::time::Instant::now() < deadline {
            context.iteration(false);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        for index in 0..5 {
            let bounds = button(index).compute_bounds(&filters).unwrap();
            assert!(bounds.width() > 0.0);
            assert!(bounds.x() + bounds.width() <= filters.width() as f32);
        }
        window.close();
    }
}
