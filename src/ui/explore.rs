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
use crate::models::{Artist, ArtistDetails, Genre, Track, format_subscribers};
use crate::ui::player::play_queue_index;
use crate::ui::state::AppState;
use crate::ui::widgets::{
    PagedCarousel, build_album_card, build_carousel_section, build_paged_carousel_section,
    build_playlist_card, clamped_cover, clear_box, load_cover_into_picture,
    populate_paged_carousel, populate_track_list, show_collection_preview_dialog, spawn_bg,
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
            .label("Explorar")
            .xalign(0.0)
            .css_classes(["title-1"])
            .build(),
    );
    explore_header_box.append(
        &Label::builder()
            .label("Estados de ánimo, momentos y géneros musicales de YouTube Music")
            .xalign(0.0)
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
    play_btn_content.append(&gtk4::Image::from_icon_name("media-playback-start-symbolic"));
    play_btn_content.append(&Label::new(Some("Reproducir")));
    artist_play_btn.set_child(Some(&play_btn_content));
    artist_play_btn.set_cursor_from_name(Some("pointer"));

    let artist_shuffle_btn = Button::builder()
        .css_classes(["pill", "artist-secondary-btn"])
        .build();
    let shuffle_btn_content = GtkBox::new(Orientation::Horizontal, 6);
    shuffle_btn_content.append(&gtk4::Image::from_icon_name("media-playlist-shuffle-symbolic"));
    shuffle_btn_content.append(&Label::new(Some("Aleatorio")));
    artist_shuffle_btn.set_child(Some(&shuffle_btn_content));
    artist_shuffle_btn.set_cursor_from_name(Some("pointer"));

    let artist_actions_row = GtkBox::new(Orientation::Horizontal, 10);
    artist_actions_row.set_margin_top(6);
    artist_actions_row.append(&artist_play_btn);
    artist_actions_row.append(&artist_shuffle_btn);

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

    let artist_hero = gtk4::Overlay::builder()
        .height_request(250)
        .overflow(Overflow::Hidden)
        .css_classes(["artist-hero"])
        .child(&artist_banner)
        .build();
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
            let len = tracks.len();
            let start_idx = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| (d.subsec_nanos() as usize) % len)
                .unwrap_or(0);
            {
                let mut s = state.borrow_mut();
                s.shuffle = true;
                s.queue = tracks;
                s.source_name = name;
            }
            play_queue_index(Rc::clone(&state), start_idx);
        });
    }

    let artist_desc_label = Label::builder()
        .xalign(0.0)
        .wrap(true)
        .lines(3)
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

    let (artist_albums_section, artist_albums_carousel) =
        build_paged_carousel_section("Álbumes");
    let (artist_playlists_section, artist_playlists_carousel) =
        build_paged_carousel_section("Playlists");
    let (artist_similar_section, artist_similar_carousel) =
        build_paged_carousel_section("Artistas similares");

    let artist_section = GtkBox::new(Orientation::Vertical, 18);
    artist_section.set_visible(false);
    artist_section.append(&artist_hero);
    artist_section.append(&artist_desc_label);
    artist_section.append(&artist_tracks_heading);
    artist_section.append(&artist_tracks_list);
    artist_section.append(&artist_tracks_toggle_btn);
    artist_section.append(&artist_albums_section);
    artist_section.append(&artist_playlists_section);
    artist_section.append(&artist_similar_section);

    // Genres & moods grid
    let moods_title = Label::builder()
        .label("Estados de ánimo y momentos")
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
        .label("Géneros musicales")
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
        .tooltip_text("Volver a géneros")
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
    container.append(&search_results_section);
    container.append(&artist_section);
    container.append(&genres_section);
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
        let genre_playlists_section = genre_playlists_section.clone();
        let genre_heading = genre_heading.clone();
        let playlists_flow = playlists_flow.clone();
        let spinner = spinner.clone();

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
                        &genre_playlists_section,
                        &genre_heading,
                        &playlists_flow,
                        &spinner,
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
        play_btn: artist_play_btn,
        shuffle_btn: artist_shuffle_btn,
        current_tracks: current_artist_tracks,
        desc_label: artist_desc_label,
        tracks_heading: artist_tracks_heading,
        tracks_list: artist_tracks_list,
        tracks_toggle_btn: artist_tracks_toggle_btn,
        tracks_expanded: artist_tracks_expanded,
        albums_section: artist_albums_section,
        albums_carousel: artist_albums_carousel,
        playlists_section: artist_playlists_section,
        playlists_carousel: artist_playlists_carousel,
        similar_section: artist_similar_section,
        similar_carousel: artist_similar_carousel,
    };

    {
        let top_search_entry = top_search_entry.clone();
        let artist_section = artist_section.clone();
        let search_results_section = search_results_section.clone();
        let genres_section = genres_section.clone();
        artist_back_btn.connect_clicked(move |_| {
            artist_section.set_visible(false);
            if top_search_entry.text().trim().is_empty() {
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
        let search_results_section = search_results_section.clone();
        let artist_section = artist_section.clone();
        let status_label = status_label.clone();
        let genres_section = genres_section.clone();
        let genre_playlists_section = genre_playlists_section.clone();
        top_search_entry.connect_search_changed(move |entry| {
            if entry.text().trim().is_empty() {
                search_results_section.set_visible(false);
                artist_section.set_visible(false);
                status_label.set_visible(false);
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
            artist_widgets.spinner.set_visible(true);
            artist_widgets.spinner.set_spinning(true);

            let backend = state.borrow().backend.clone();
            let query = name.clone();
            let state_cb = Rc::clone(&state);
            let aw = artist_widgets.clone();
            let fallback_search = Rc::clone(&run_search);
            spawn_bg(
                move || async move { backend.search_all(&query).await },
                move |res| {
                    aw.spinner.set_spinning(false);
                    aw.spinner.set_visible(false);
                    if let Ok(sr) = res
                        && let Some(artist) = sr
                            .artists
                            .iter()
                            .find(|a| a.name.eq_ignore_ascii_case(&name))
                            .or_else(|| sr.artists.first())
                    {
                        open_artist_page(&artist.id, &artist.name, aw, state_cb);
                    } else {
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
            state
                .borrow()
                .show_toast(&format!("Buscando álbum: {album}"));
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
                            show_collection_preview_dialog(title, cover, tracks, state_cb);
                        }
                        Ok(_) => fallback_search(&album_fallback),
                        Err(err) => state_cb.borrow().show_toast(&err),
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
    name_label: Label,
    subs_label: Label,
    banner: Picture,
    play_btn: Button,
    shuffle_btn: Button,
    current_tracks: Rc<RefCell<Option<(String, Vec<Track>)>>>,
    desc_label: Label,
    tracks_heading: Label,
    tracks_list: ListBox,
    tracks_toggle_btn: Button,
    tracks_expanded: Rc<Cell<bool>>,
    albums_section: GtkBox,
    albums_carousel: PagedCarousel,
    playlists_section: GtkBox,
    playlists_carousel: PagedCarousel,
    similar_section: GtkBox,
    similar_carousel: PagedCarousel,
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

    let album_cards = details
        .albums
        .iter()
        .map(|album| build_album_card(album, Rc::clone(&state)).upcast())
        .collect();
    populate_paged_carousel(&ui.albums_carousel, &ui.albums_section, album_cards);

    let playlist_cards = details
        .playlists
        .iter()
        .map(|playlist| build_playlist_card(playlist, Rc::clone(&state)).upcast())
        .collect();
    populate_paged_carousel(
        &ui.playlists_carousel,
        &ui.playlists_section,
        playlist_cards,
    );

    let similar_cards = details
        .similar_artists
        .iter()
        .map(|similar| build_artist_card(similar, Rc::clone(&state), ui.clone()).upcast())
        .collect();
    populate_paged_carousel(&ui.similar_carousel, &ui.similar_section, similar_cards);

    ui.artist_section.set_visible(true);
}

fn apply_artist_tracks_collapse(list: &ListBox, toggle: &Button, expanded: bool) {
    const INITIAL_TRACKS: usize = 10;
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
            content.append(&Label::new(Some("Mostrar menos")));
        } else {
            content.append(&gtk4::Image::from_icon_name("pan-down-symbolic"));
            content.append(&Label::new(Some(&format!(
                "Ver {} más",
                total - INITIAL_TRACKS
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
    genre_playlists_section: &GtkBox,
    genre_heading: &Label,
    playlists_flow: &FlowBox,
    spinner: &Spinner,
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

