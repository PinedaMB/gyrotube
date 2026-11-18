use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, CssProvider, FlowBox, Image, Label, ListBox,
    ListBoxRow, Orientation, Overflow, Picture, PolicyType, ScrolledWindow, SelectionMode, Spinner,
    ToggleButton,
};
use libadwaita as adw;

use crate::backend::runtime;
use crate::i18n::tr;
use crate::models::{Track, format_duration};
use crate::ui::player::{play_queue_index, refresh_queue_ui};
use crate::ui::state::AppState;
use crate::ui::widgets::{
    build_album_card, build_playlist_card, build_skeleton_box, clamped_cover, clear_list_box,
    load_cover_into_picture, open_album_by_id, play_album_by_id, set_track_favorite,
};

fn build_quick_card(
    num: &str,
    title: &str,
    subtitle: &str,
    icon_name: &str,
    variant: u8,
) -> Button {
    let icon = Image::builder()
        .icon_name(icon_name)
        .pixel_size(18)
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(true)
        .build();
    let badge_class = format!("quick-badge-{variant}");
    let icon_badge = GtkBox::builder()
        .halign(Align::Start)
        .valign(Align::Center)
        .width_request(38)
        .height_request(38)
        .css_classes(["quick-icon-badge", &badge_class])
        .build();
    icon_badge.append(&icon);

    let num_label = Label::builder()
        .label(num)
        .xalign(1.0)
        .hexpand(true)
        .valign(Align::Start)
        .css_classes(["quick-num", "numeric"])
        .build();

    let top_row = GtkBox::new(Orientation::Horizontal, 8);
    top_row.append(&icon_badge);
    top_row.append(&num_label);

    let title_label = Label::builder()
        .label(title)
        .xalign(0.0)
        .max_width_chars(20)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"])
        .build();

    let sub_label = Label::builder()
        .label(subtitle)
        .xalign(0.0)
        .max_width_chars(22)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["dim-label", "caption"])
        .build();

    let card_box = GtkBox::new(Orientation::Vertical, 12);
    card_box.append(&top_row);
    let text_box = GtkBox::new(Orientation::Vertical, 3);
    text_box.append(&title_label);
    text_box.append(&sub_label);
    card_box.append(&text_box);

    let card_class = format!("quick-card-{variant}");
    let btn = Button::builder()
        .child(&card_box)
        .hexpand(true)
        .css_classes(["flat", "quick-card", &card_class])
        .build();
    btn.set_cursor_from_name(Some("pointer"));
    btn
}

fn build_chart_track_row(
    track: &Track,
    section_tracks: &[Track],
    section_index: usize,
    source_name: &str,
    is_playing_now: bool,
    state: Rc<RefCell<AppState>>,
) -> ListBoxRow {
    let row_box = GtkBox::new(Orientation::Horizontal, 12);
    row_box.add_css_class("track-row");

    let rank_widget: gtk4::Widget = if is_playing_now {
        Image::builder()
            .icon_name("speaker-2-regular")
            .pixel_size(16)
            .halign(Align::Center)
            .valign(Align::Center)
            .width_request(24)
            .css_classes(["playing-title"])
            .build()
            .upcast()
    } else {
        Label::builder()
            .label(&format!("{:02}", section_index + 1))
            .xalign(0.5)
            .valign(Align::Center)
            .css_classes(["chart-rank", "numeric", "caption"])
            .build()
            .upcast()
    };

    let (cover_frame, cover) = clamped_cover(40, 40, "cover-art");
    if let Some(url) = &track.cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(&state));
    }

    let title_label = Label::builder()
        .label(&track.title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(if is_playing_now {
            vec!["heading", "playing-title"]
        } else {
            vec!["heading"]
        })
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

    let album_text = track
        .album
        .as_deref()
        .filter(|a| !a.is_empty())
        .unwrap_or("Sencillo");
    let album_label = Label::builder()
        .label(album_text)
        .xalign(0.0)
        .width_chars(16)
        .max_width_chars(20)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .valign(Align::Center)
        .css_classes(["dim-label", "caption"])
        .build();

    let duration_str = track
        .duration_secs
        .map(format_duration)
        .unwrap_or_else(|| String::from("--:--"));
    let duration_label = Label::builder()
        .label(&duration_str)
        .width_chars(5)
        .xalign(1.0)
        .valign(Align::Center)
        .css_classes(["dim-label", "caption", "numeric"])
        .build();

    let is_fav = state.borrow().is_favorite(&track.id);
    let fav_btn = ToggleButton::builder()
        .icon_name(if is_fav {
            "heart-filled"
        } else {
            "heart-regular"
        })
        .active(is_fav)
        .tooltip_text(tr("Favorito", "Favorite"))
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        let track = track.clone();
        fav_btn.connect_toggled(move |btn| {
            btn.set_icon_name(if btn.is_active() {
                "heart-filled"
            } else {
                "heart-regular"
            });
            set_track_favorite(Rc::clone(&state), &track, btn.is_active());
        });
    }

    let queue_btn = Button::builder()
        .icon_name("add-regular")
        .tooltip_text(tr("Agregar a la cola", "Add to queue"))
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

    let play_btn = Button::builder()
        .icon_name("play-filled")
        .tooltip_text(tr("Reproducir ahora", "Play now"))
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
                s.replace_queue(section.clone(), source.clone());
            }
            play_queue_index(Rc::clone(&state), section_index);
        });
    }

    row_box.append(&rank_widget);
    row_box.append(&cover_frame);
    row_box.append(&text_box);
    row_box.append(&album_label);
    row_box.append(&duration_label);
    row_box.append(&fav_btn);
    row_box.append(&queue_btn);
    row_box.append(&play_btn);

    let row = ListBoxRow::builder()
        .child(&row_box)
        .activatable(true)
        .build();
    if is_playing_now {
        row.add_css_class("chart-active-row");
    }
    row.set_cursor_from_name(Some("pointer"));
    {
        let state = Rc::clone(&state);
        let section = section_tracks.to_vec();
        let source = source_name.to_string();
        row.connect_activate(move |_| {
            {
                let mut s = state.borrow_mut();
                s.replace_queue(section.clone(), source.clone());
            }
            play_queue_index(Rc::clone(&state), section_index);
        });
    }
    {
        let backend = state.borrow().backend.clone();
        let track_for_hover = track.clone();
        let pending_hover: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
        let pending_leave = Rc::clone(&pending_hover);
        let motion = gtk4::EventControllerMotion::new();
        motion.connect_enter(move |_, _, _| {
            if let Some(id) = pending_hover.borrow_mut().take() {
                id.remove();
            }
            let backend = backend.clone();
            let track = track_for_hover.clone();
            let slot = Rc::clone(&pending_hover);
            let id = glib::timeout_add_local_once(std::time::Duration::from_millis(180), move || {
                *slot.borrow_mut() = None;
                runtime().spawn(async move {
                    let _ = backend.resolve_track_stream(&track).await;
                });
            });
            *pending_hover.borrow_mut() = Some(id);
        });
        motion.connect_leave(move |_| {
            if let Some(id) = pending_leave.borrow_mut().take() {
                id.remove();
            }
        });
        row.add_controller(motion);
    }
    row
}

fn open_genre_page(
    genre_id: String,
    genre_name: String,
    parent: gtk4::Widget,
    state: Rc<RefCell<AppState>>,
) {
    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_margin_top(16);
    container.set_margin_bottom(24);
    container.set_margin_start(20);
    container.set_margin_end(20);

    let heading = Label::builder()
        .label(&genre_name)
        .xalign(0.0)
        .css_classes(["title-2"])
        .build();

    let spinner = Spinner::builder()
        .halign(Align::Center)
        .margin_top(24)
        .spinning(true)
        .build();

    let playlists_flow = FlowBox::builder()
        .valign(Align::Start)
        .homogeneous(true)
        .max_children_per_line(4)
        .min_children_per_line(1)
        .row_spacing(12)
        .column_spacing(12)
        .selection_mode(SelectionMode::None)
        .build();

    container.append(&heading);
    container.append(&spinner);
    container.append(&playlists_flow);

    let scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&container)
        .build();

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    toolbar.set_content(Some(&scroller));

    let dialog = adw::Dialog::builder()
        .title(&genre_name)
        .content_width(760)
        .content_height(560)
        .child(&toolbar)
        .build();

    dialog.present(Some(&parent));

    let backend = state.borrow().backend.clone();
    let (tx, rx) = async_channel::bounded(1);
    runtime().spawn(async move {
        let res = backend.fetch_genre_playlists(&genre_id).await;
        let _ = tx.send(res).await;
    });

    let state = Rc::clone(&state);
    glib::spawn_future_local(async move {
        if let Ok(result) = rx.recv().await {
            spinner.set_spinning(false);
            spinner.set_visible(false);
            match result {
                Ok((name, playlists)) => {
                    if !name.is_empty() {
                        heading.set_label(&name);
                        dialog.set_title(&name);
                    }
                    for pl in playlists {
                        let card = build_playlist_card(&pl, Rc::clone(&state));
                        playlists_flow.insert(&card, -1);
                    }
                }
                Err(err) => state.borrow().show_toast(&err),
            }
        }
    });
}

fn quick_play_genre(genre_id: String, genre_name: String, state: Rc<RefCell<AppState>>) {
    let backend = state.borrow().backend.clone();
    state
        .borrow()
        .show_toast(&format!("Iniciando estación: {genre_name}"));

    let (tx, rx) = async_channel::bounded(1);
    runtime().spawn(async move {
        let playlists_res = backend.fetch_genre_playlists(&genre_id).await;
        let res = match playlists_res {
            Ok((name, playlists)) => {
                if let Some(first) = playlists.into_iter().next() {
                    backend
                        .fetch_playlist_tracks(&first.id)
                        .await
                        .map(|(pl_name, tracks)| {
                            (if pl_name.is_empty() { name } else { pl_name }, tracks)
                        })
                } else {
                    Err("No hay playlists en esta estación.".to_string())
                }
            }
            Err(e) => Err(e),
        };
        let _ = tx.send(res).await;
    });

    glib::spawn_future_local(async move {
        if let Ok(result) = rx.recv().await {
            match result {
                Ok((name, tracks)) if !tracks.is_empty() => {
                    {
                        let mut s = state.borrow_mut();
                        s.replace_queue(tracks, name);
                    }
                    play_queue_index(Rc::clone(&state), 0);
                }
                Ok(_) => state.borrow().show_toast("La estación no tiene pistas."),
                Err(err) => state.borrow().show_toast(&err),
            }
        }
    });
}

fn apply_genre_color_css(widget: &impl IsA<gtk4::Widget>, css: &str) {
    let provider = CssProvider::new();
    provider.load_from_string(css);
    #[allow(deprecated)]
    widget
        .style_context()
        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn build_home_skeleton() -> GtkBox {
    let skel = GtkBox::new(Orientation::Vertical, 16);
    for _ in 0..5 {
        skel.append(&build_skeleton_box(0, 56));
    }
    let row = GtkBox::new(Orientation::Horizontal, 12);
    row.set_homogeneous(true);
    for _ in 0..4 {
        row.append(&build_skeleton_box(0, 200));
    }
    skel.append(&row);
    skel
}

pub fn build_home_page(
    state: Rc<RefCell<AppState>>,
    content_view_stack: adw::ViewStack,
) -> (ScrolledWindow, FlowBox) {
    let container = GtkBox::new(Orientation::Vertical, 24);
    container.set_margin_top(16);
    container.set_margin_bottom(88);
    container.set_margin_start(20);
    container.set_margin_end(20);

    // 0. Contextual greeting header + refresh button
    let greeting_text = match glib::DateTime::now_local()
        .map(|dt| dt.hour())
        .unwrap_or(12)
    {
        5..=11 => tr("Buenos días", "Good morning"),
        12..=18 => tr("Buenas tardes", "Good afternoon"),
        _ => tr("Buenas noches", "Good evening"),
    };
    let greeting_col = GtkBox::new(Orientation::Vertical, 2);
    greeting_col.set_hexpand(true);
    greeting_col.append(
        &Label::builder()
            .label(greeting_text)
            .xalign(0.0)
            .css_classes(["home-greeting-title"])
            .build(),
    );
    greeting_col.append(
        &Label::builder()
            .label(tr(
                "Tu música lista para reproducir",
                "Your music ready to play",
            ))
            .xalign(0.0)
            .css_classes(["dim-label", "caption"])
            .build(),
    );

    let refresh_btn = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text(tr("Recargar inicio", "Reload home"))
        .valign(Align::Center)
        .css_classes(["flat", "circular"])
        .build();
    refresh_btn.set_cursor_from_name(Some("pointer"));

    let greeting_header = GtkBox::new(Orientation::Horizontal, 12);
    greeting_header.append(&greeting_col);
    greeting_header.append(&refresh_btn);

    // 1. Quick-category 4-card adaptive grid (4 in 1 row on desktop -> 2x2 on tablet/mobile)
    let quick_grid = FlowBox::builder()
        .valign(Align::Start)
        .homogeneous(true)
        .min_children_per_line(4)
        .max_children_per_line(4)
        .row_spacing(10)
        .column_spacing(10)
        .selection_mode(SelectionMode::None)
        .build();
    let card_new = build_quick_card(
        "01",
        tr("Nuevos lanzamientos", "New releases"),
        tr("Álbumes y sencillos recientes", "Recent albums & singles"),
        "media-optical-symbolic",
        1,
    );
    let card_charts = build_quick_card(
        "02",
        tr("Listas de éxitos", "Top charts"),
        tr("Tendencias y Top Música", "Trending & Top Music"),
        "view-list-ordered-symbolic",
        2,
    );
    let card_moods = build_quick_card(
        "03",
        tr("Estados de ánimo", "Moods"),
        tr("Explorar géneros y ritmos", "Explore genres & vibes"),
        "search-regular",
        3,
    );
    let card_collection = build_quick_card(
        "04",
        tr("Colección", "Collection"),
        tr("Tus canciones favoritas", "Your favorite songs"),
        "heart-filled",
        4,
    );
    quick_grid.insert(&card_new, -1);
    quick_grid.insert(&card_charts, -1);
    quick_grid.insert(&card_moods, -1);
    quick_grid.insert(&card_collection, -1);

    let skeleton = build_home_skeleton();

    // 1b. "Escuchar de nuevo" (Listen again) from recent_tracks
    let listen_again_grid = FlowBox::builder()
        .valign(Align::Start)
        .homogeneous(true)
        .min_children_per_line(1)
        .max_children_per_line(3)
        .row_spacing(10)
        .column_spacing(10)
        .selection_mode(SelectionMode::None)
        .build();
    let listen_again_section = GtkBox::new(Orientation::Vertical, 10);
    listen_again_section.set_visible(false);
    listen_again_section.append(
        &Label::builder()
            .label(tr("Escuchar de nuevo", "Listen again"))
            .xalign(0.0)
            .css_classes(["title-2"])
            .build(),
    );
    listen_again_section.append(&listen_again_grid);

    let render_listen_again: Rc<dyn Fn()> = {
        let listen_again_grid = listen_again_grid.clone();
        let listen_again_section = listen_again_section.clone();
        let state = Rc::clone(&state);
        Rc::new(move || {
            while let Some(child) = listen_again_grid.first_child() {
                listen_again_grid.remove(&child);
            }
            let recents: Vec<Track> = state.borrow().recent_tracks.iter().take(6).cloned().collect();
            listen_again_section.set_visible(!recents.is_empty());
            for (idx, track) in recents.iter().enumerate() {
                let (cover_frame, cover_pic) = clamped_cover(46, 46, "cover-art");
                if let Some(url) = &track.cover_url {
                    load_cover_into_picture(&cover_pic, url, Rc::clone(&state));
                }
                let text_col = GtkBox::new(Orientation::Vertical, 2);
                text_col.set_hexpand(true);
                text_col.set_valign(Align::Center);
                text_col.append(
                    &Label::builder()
                        .label(&track.title)
                        .xalign(0.0)
                        .ellipsize(gtk4::pango::EllipsizeMode::End)
                        .css_classes(["heading"])
                        .build(),
                );
                text_col.append(
                    &Label::builder()
                        .label(&track.artist)
                        .xalign(0.0)
                        .ellipsize(gtk4::pango::EllipsizeMode::End)
                        .css_classes(["caption", "dim-label"])
                        .build(),
                );
                let row_box = GtkBox::new(Orientation::Horizontal, 10);
                row_box.append(&cover_frame);
                row_box.append(&text_col);
                row_box.append(
                    &Image::builder()
                        .icon_name("play-filled")
                        .pixel_size(14)
                        .valign(Align::Center)
                        .css_classes(["dim-label"])
                        .build(),
                );
                let tile_btn = Button::builder()
                    .child(&row_box)
                    .css_classes(["flat", "listen-again-tile"])
                    .build();
                tile_btn.set_cursor_from_name(Some("pointer"));
                let state_click = Rc::clone(&state);
                let queue_tracks = recents.clone();
                tile_btn.connect_clicked(move |_| {
                    {
                        let mut s = state_click.borrow_mut();
                        s.replace_queue(
                            queue_tracks.clone(),
                            String::from(tr("Escuchar de nuevo", "Listen again")),
                        );
                    }
                    play_queue_index(Rc::clone(&state_click), idx);
                });
                listen_again_grid.insert(&tile_btn, -1);
            }
        })
    };
    render_listen_again();

    // 1c. Featured Hero card
    let hero_section = GtkBox::new(Orientation::Vertical, 0);
    hero_section.set_visible(false);

    // 2. "Éxitos en tendencia" numbered chart with pill switcher
    let chart_header = GtkBox::new(Orientation::Horizontal, 10);
    chart_header.append(
        &Label::builder()
            .label(tr("Éxitos en tendencia", "Trending hits"))
            .xalign(0.0)
            .hexpand(true)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["title-2"])
            .build(),
    );

    let pill_box = GtkBox::new(Orientation::Horizontal, 2);
    pill_box.add_css_class("pill-switch-group");
    let trending_pill = ToggleButton::builder()
        .label(tr("Tendencias", "Trending"))
        .active(true)
        .css_classes(["flat", "pill-switch"])
        .build();
    let top_pill = ToggleButton::builder()
        .label(tr("Top Música", "Top Music"))
        .group(&trending_pill)
        .css_classes(["flat", "pill-switch"])
        .build();
    pill_box.append(&trending_pill);
    pill_box.append(&top_pill);
    chart_header.append(&pill_box);

    let chart_cols = GtkBox::new(Orientation::Horizontal, 12);
    chart_cols.add_css_class("chart-header-row");
    chart_cols.append(
        &Label::builder()
            .label("#")
            .width_chars(3)
            .xalign(0.5)
            .build(),
    );
    chart_cols.append(
        &Label::builder()
            .label(tr("TÍTULO Y ARTISTA", "TITLE & ARTIST"))
            .xalign(0.0)
            .hexpand(true)
            .build(),
    );
    chart_cols.append(
        &Label::builder()
            .label(tr("ÁLBUM", "ALBUM"))
            .xalign(0.0)
            .width_chars(16)
            .build(),
    );
    chart_cols.append(
        &Label::builder()
            .label(tr("DURACIÓN", "DURATION"))
            .xalign(1.0)
            .width_chars(8)
            .build(),
    );

    let chart_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .activate_on_single_click(true)
        .css_classes(["boxed-list"])
        .build();

    let chart_card = GtkBox::new(Orientation::Vertical, 0);
    chart_card.append(&chart_cols);
    chart_card.append(&chart_list);

    let chart_section = GtkBox::new(Orientation::Vertical, 10);
    chart_section.set_visible(false);
    chart_section.append(&chart_header);
    chart_section.append(&chart_card);

    // 3. "Nuevos lanzamientos" adw::Carousel
    let albums_header_text = GtkBox::new(Orientation::Vertical, 2);
    albums_header_text.set_hexpand(true);
    albums_header_text.append(
        &Label::builder()
            .label(tr("Nuevos lanzamientos", "New releases"))
            .xalign(0.0)
            .css_classes(["title-2"])
            .build(),
    );
    albums_header_text.append(
        &Label::builder()
            .label(tr(
                "Álbumes y sencillos recién publicados",
                "Recently released albums & singles",
            ))
            .xalign(0.0)
            .css_classes(["dim-label", "caption"])
            .build(),
    );

    let albums_prev_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text(tr("Página anterior", "Previous page"))
        .valign(Align::Center)
        .css_classes(["circular", "flat"])
        .build();
    let albums_next_btn = Button::builder()
        .icon_name("go-next-symbolic")
        .tooltip_text(tr("Página siguiente", "Next page"))
        .valign(Align::Center)
        .css_classes(["circular", "flat"])
        .build();

    let albums_header = GtkBox::new(Orientation::Horizontal, 8);
    albums_header.append(&albums_header_text);
    albums_header.append(&albums_prev_btn);
    albums_header.append(&albums_next_btn);

    let albums_carousel = adw::Carousel::builder()
        .spacing(12)
        .allow_mouse_drag(true)
        .allow_scroll_wheel(false)
        .build();
    let albums_dots = adw::CarouselIndicatorDots::builder()
        .carousel(&albums_carousel)
        .build();

    {
        let carousel = albums_carousel.clone();
        albums_prev_btn.connect_clicked(move |_| {
            let n = carousel.n_pages();
            if n == 0 {
                return;
            }
            let pos = (carousel.position().round() as u32).saturating_sub(1);
            let page = carousel.nth_page(pos);
            carousel.scroll_to(&page, true);
        });
    }
    {
        let carousel = albums_carousel.clone();
        albums_next_btn.connect_clicked(move |_| {
            let n = carousel.n_pages();
            if n == 0 {
                return;
            }
            let pos = ((carousel.position().round() as u32) + 1).min(n - 1);
            let page = carousel.nth_page(pos);
            carousel.scroll_to(&page, true);
        });
    }

    let albums_section = GtkBox::new(Orientation::Vertical, 10);
    albums_section.set_visible(false);
    albums_section.append(&albums_header);
    albums_section.append(&albums_carousel);
    albums_section.append(&albums_dots);

    // 4. "Estaciones y géneros" adaptive bento grid (4 -> 2 -> 1 columns)
    let stations_header = Label::builder()
        .label(tr("Estaciones y géneros", "Stations & genres"))
        .xalign(0.0)
        .css_classes(["title-2"])
        .build();
    let stations_grid = FlowBox::builder()
        .valign(Align::Start)
        .homogeneous(true)
        .min_children_per_line(1)
        .max_children_per_line(3)
        .row_spacing(12)
        .column_spacing(12)
        .selection_mode(SelectionMode::None)
        .build();
    let stations_section = GtkBox::new(Orientation::Vertical, 10);
    stations_section.set_visible(false);
    stations_section.append(&stations_header);
    stations_section.append(&stations_grid);

    container.append(&greeting_header);
    container.append(&quick_grid);
    container.append(&listen_again_section);
    container.append(&skeleton);
    container.append(&hero_section);
    container.append(&albums_section);
    container.append(&chart_section);
    container.append(&stations_section);

    let clamp = adw::Clamp::builder()
        .maximum_size(1100)
        .child(&container)
        .build();

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&clamp)
        .build();

    let cached_trending: Rc<RefCell<Vec<Track>>> = Rc::new(RefCell::new(Vec::new()));
    let cached_top: Rc<RefCell<Vec<Track>>> = Rc::new(RefCell::new(Vec::new()));

    let render_chart: Rc<dyn Fn(bool)> = {
        let chart_list = chart_list.clone();
        let state = Rc::clone(&state);
        let cached_trending = Rc::clone(&cached_trending);
        let cached_top = Rc::clone(&cached_top);
        Rc::new(move |use_top: bool| {
            clear_list_box(&chart_list);
            let tracks = if use_top {
                cached_top.borrow()
            } else {
                cached_trending.borrow()
            };
            let current_id = state.borrow().current_track().map(|t| t.id.clone());
            let source = if use_top { "Top Música" } else { "Tendencias" };
            for (idx, track) in tracks.iter().take(15).enumerate() {
                let is_playing = current_id.as_deref() == Some(track.id.as_str());
                let row = build_chart_track_row(
                    track,
                    &tracks,
                    idx,
                    source,
                    is_playing,
                    Rc::clone(&state),
                );
                chart_list.append(&row);
            }
        })
    };

    // Re-highlight active track in chart & refresh Listen Again when playback changes
    {
        let render_chart = Rc::clone(&render_chart);
        let render_listen_again = Rc::clone(&render_listen_again);
        let top_pill = top_pill.clone();
        state.borrow_mut().on_track_changed.push(Rc::new(move || {
            render_chart(top_pill.is_active());
            render_listen_again();
        }));
    }

    {
        let render_chart = Rc::clone(&render_chart);
        trending_pill.connect_toggled(move |btn| {
            if btn.is_active() {
                render_chart(false);
            }
        });
    }
    {
        let render_chart = Rc::clone(&render_chart);
        top_pill.connect_toggled(move |btn| {
            if btn.is_active() {
                render_chart(true);
            }
        });
    }

    // Wire quick-category cards
    {
        let scrolled = scrolled.clone();
        let container = container.clone();
        let albums_section = albums_section.clone();
        card_new.connect_clicked(move |_| {
            let y = albums_section
                .compute_bounds(&container)
                .map(|b| b.y() as f64)
                .unwrap_or(0.0);
            let adj = scrolled.vadjustment();
            adj.set_value(y.clamp(adj.lower(), (adj.upper() - adj.page_size()).max(0.0)));
        });
    }
    {
        let top_pill = top_pill.clone();
        let scrolled = scrolled.clone();
        let container = container.clone();
        let chart_section = chart_section.clone();
        card_charts.connect_clicked(move |_| {
            top_pill.set_active(true);
            let y = chart_section
                .compute_bounds(&container)
                .map(|b| b.y() as f64)
                .unwrap_or(0.0);
            let adj = scrolled.vadjustment();
            adj.set_value(y.clamp(adj.lower(), (adj.upper() - adj.page_size()).max(0.0)));
        });
    }
    {
        let stack = content_view_stack.clone();
        card_moods.connect_clicked(move |_| {
            stack.set_visible_child_name("explore");
        });
    }
    {
        let stack = content_view_stack.clone();
        card_collection.connect_clicked(move |_| {
            stack.set_visible_child_name("collection");
        });
    }

    let load_home = {
        let state = Rc::clone(&state);
        let skeleton = skeleton.clone();
        let hero_section = hero_section.clone();
        let chart_section = chart_section.clone();
        let albums_section = albums_section.clone();
        let albums_carousel = albums_carousel.clone();
        let stations_section = stations_section.clone();
        let stations_grid = stations_grid.clone();
        let cached_trending = Rc::clone(&cached_trending);
        let cached_top = Rc::clone(&cached_top);
        let render_chart = Rc::clone(&render_chart);
        let top_pill = top_pill.clone();

        move |force_refresh: bool| {
            skeleton.set_visible(true);
            let backend = state.borrow().backend.clone();
            let (tx, rx) = async_channel::bounded(1);

            let (genres_tx, genres_rx) = async_channel::bounded(1);
            let genres_backend = backend.clone();
            runtime().spawn(async move {
                let _ = genres_tx
                    .send(genres_backend.fetch_genres_cached(force_refresh).await)
                    .await;
            });
            runtime().spawn(async move {
                let _ = tx
                    .send(backend.fetch_home_feed_cached(force_refresh).await)
                    .await;
            });

            let state = Rc::clone(&state);
            let skeleton = skeleton.clone();
            let hero_section = hero_section.clone();
            let chart_section = chart_section.clone();
            let albums_section = albums_section.clone();
            let albums_carousel = albums_carousel.clone();
            let stations_section = stations_section.clone();
            let stations_grid = stations_grid.clone();
            let cached_trending = Rc::clone(&cached_trending);
            let cached_top = Rc::clone(&cached_top);
            let render_chart = Rc::clone(&render_chart);
            let top_pill = top_pill.clone();

            let feed_state = Rc::clone(&state);
            glib::spawn_future_local(async move {
                let state = feed_state;
                if let Ok(feed_res) = rx.recv().await {
                    skeleton.set_visible(false);
                    match feed_res {
                        Ok(feed) => {
                            *cached_trending.borrow_mut() = feed.trending.clone();
                            *cached_top.borrow_mut() = feed.top.clone();
                            render_chart(top_pill.is_active());
                            chart_section
                                .set_visible(!feed.trending.is_empty() || !feed.top.is_empty());

                            while let Some(child) = hero_section.first_child() {
                                hero_section.remove(&child);
                            }
                            if let Some(featured) = feed.new_albums.first() {
                                let (cover_frame, cover_pic) =
                                    clamped_cover(128, 128, "cover-art-large");
                                cover_frame.add_css_class("home-hero-cover");
                                if let Some(url) = &featured.cover_url {
                                    load_cover_into_picture(&cover_pic, url, Rc::clone(&state));
                                }
                                let badge = Label::builder()
                                    .label(tr("DESTACADO", "FEATURED"))
                                    .halign(Align::Start)
                                    .css_classes(["home-hero-badge"])
                                    .build();
                                let title_lbl = Label::builder()
                                    .label(&featured.title)
                                    .xalign(0.0)
                                    .wrap(true)
                                    .max_width_chars(34)
                                    .css_classes(["title-1"])
                                    .build();
                                let subtitle = match featured.year {
                                    Some(y) => format!("{} • {y}", featured.artist),
                                    None => featured.artist.clone(),
                                };
                                let sub_lbl = Label::builder()
                                    .label(&subtitle)
                                    .xalign(0.0)
                                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                                    .css_classes(["dim-label"])
                                    .build();

                                let play_content = GtkBox::new(Orientation::Horizontal, 8);
                                play_content.append(&Image::from_icon_name("play-filled"));
                                play_content.append(&Label::new(Some(tr("Reproducir", "Play"))));
                                let hero_play_btn = Button::builder()
                                    .child(&play_content)
                                    .css_classes(["suggested-action", "pill"])
                                    .build();
                                hero_play_btn.set_cursor_from_name(Some("pointer"));
                                {
                                    let st = Rc::clone(&state);
                                    let id = featured.id.clone();
                                    hero_play_btn.connect_clicked(move |_| {
                                        play_album_by_id(id.clone(), Rc::clone(&st));
                                    });
                                }

                                let hero_open_btn = Button::builder()
                                    .label(tr("Abrir álbum", "Open album"))
                                    .css_classes(["pill"])
                                    .build();
                                hero_open_btn.set_cursor_from_name(Some("pointer"));
                                {
                                    let st = Rc::clone(&state);
                                    let id = featured.id.clone();
                                    let title = featured.title.clone();
                                    let cover = featured.cover_url.clone();
                                    hero_open_btn.connect_clicked(move |_| {
                                        open_album_by_id(
                                            id.clone(),
                                            title.clone(),
                                            cover.clone(),
                                            Rc::clone(&st),
                                        );
                                    });
                                }

                                let btns_row = GtkBox::new(Orientation::Horizontal, 10);
                                btns_row.set_margin_top(6);
                                btns_row.append(&hero_play_btn);
                                btns_row.append(&hero_open_btn);

                                let info_col = GtkBox::new(Orientation::Vertical, 6);
                                info_col.set_hexpand(true);
                                info_col.set_valign(Align::Center);
                                info_col.append(&badge);
                                info_col.append(&title_lbl);
                                info_col.append(&sub_lbl);
                                info_col.append(&btns_row);

                                let hero_card = GtkBox::new(Orientation::Horizontal, 20);
                                hero_card.add_css_class("home-hero-card");
                                hero_card.append(&cover_frame);
                                hero_card.append(&info_col);
                                hero_section.append(&hero_card);
                                hero_section.set_visible(true);
                            } else {
                                hero_section.set_visible(false);
                            }

                            while albums_carousel.n_pages() > 0 {
                                let page = albums_carousel.nth_page(0);
                                albums_carousel.remove(&page);
                            }
                            let limited_albums: Vec<_> = feed.new_albums.iter().take(16).collect();
                            for chunk in limited_albums.chunks(4) {
                                let page_flow = FlowBox::builder()
                                    .valign(Align::Start)
                                    .hexpand(true)
                                    .homogeneous(true)
                                    .min_children_per_line(1)
                                    .max_children_per_line(4)
                                    .row_spacing(12)
                                    .column_spacing(12)
                                    .selection_mode(SelectionMode::None)
                                    .build();
                                for album in chunk {
                                    let card = build_album_card(album, Rc::clone(&state));
                                    page_flow.insert(&card, -1);
                                }
                                albums_carousel.append(&page_flow);
                            }
                            albums_section.set_visible(!feed.new_albums.is_empty());
                        }
                        Err(err) => {
                            state.borrow().show_toast(&err);
                        }
                    }
                }
            });
            let state = Rc::clone(&state);
            glib::spawn_future_local(async move {
                if let Ok(Ok(genres)) = genres_rx.recv().await {
                    while let Some(child) = stations_grid.first_child() {
                        stations_grid.remove(&child);
                    }
                    let icons = [
                        "weather-clear-night-symbolic",
                        "audio-headphones-symbolic",
                        "media-optical-symbolic",
                        "heart-filled",
                        "emblem-music-symbolic",
                        "radio-symbolic",
                    ];
                    let mut count = 0usize;
                    for (idx, genre) in genres.into_iter().take(6).enumerate() {
                        let color = &genre.color_hex;

                        let bg_picture = Picture::builder()
                            .content_fit(ContentFit::Cover)
                            .can_shrink(true)
                            .hexpand(true)
                            .vexpand(true)
                            .css_classes(["bento-card-bg"])
                            .build();

                        let pill = Label::builder()
                            .label("Estación")
                            .halign(Align::Start)
                            .valign(Align::Center)
                            .css_classes(["bento-pill"])
                            .build();
                        apply_genre_color_css(&pill, &format!("label {{ color: {color}; }}"));

                        let icon = Image::builder()
                            .icon_name(icons[idx % icons.len()])
                            .pixel_size(18)
                            .halign(Align::End)
                            .valign(Align::Center)
                            .hexpand(true)
                            .build();
                        apply_genre_color_css(&icon, &format!("image {{ color: {color}; }}"));

                        let top_row = GtkBox::new(Orientation::Horizontal, 8);
                        top_row.append(&pill);
                        top_row.append(&icon);

                        let title = Label::builder()
                            .label(&genre.name)
                            .xalign(0.0)
                            .max_width_chars(18)
                            .ellipsize(gtk4::pango::EllipsizeMode::End)
                            .css_classes(["title-3"])
                            .build();

                        let desc = Label::builder()
                            .label("Playlists y mezclas seleccionadas para este estilo.")
                            .xalign(0.0)
                            .wrap(true)
                            .lines(2)
                            .max_width_chars(24)
                            .ellipsize(gtk4::pango::EllipsizeMode::End)
                            .css_classes(["dim-label", "caption"])
                            .build();

                        let mid_box = GtkBox::new(Orientation::Vertical, 4);
                        mid_box.set_vexpand(true);
                        mid_box.set_valign(Align::End);
                        mid_box.append(&title);
                        mid_box.append(&desc);

                        let cta = Label::builder()
                            .label("Explorar estación →")
                            .xalign(0.0)
                            .hexpand(true)
                            .css_classes(["bento-cta"])
                            .build();
                        apply_genre_color_css(&cta, &format!("label {{ color: {color}; }}"));

                        let quick_play_btn = Button::builder()
                            .icon_name("play-filled")
                            .tooltip_text("Reproducir estación")
                            .valign(Align::Center)
                            .css_classes(["flat", "station-play-btn"])
                            .build();
                        {
                            let state_play = Rc::clone(&state);
                            let gid = genre.id.clone();
                            let gname = genre.name.clone();
                            quick_play_btn.connect_clicked(move |_| {
                                quick_play_genre(
                                    gid.clone(),
                                    gname.clone(),
                                    Rc::clone(&state_play),
                                );
                            });
                        }

                        let bottom_row = GtkBox::new(Orientation::Horizontal, 8);
                        bottom_row.append(&cta);
                        bottom_row.append(&quick_play_btn);

                        let content_box = GtkBox::new(Orientation::Vertical, 12);
                        content_box.add_css_class("bento-card-scrim");
                        apply_genre_color_css(
                            &content_box,
                            &format!(
                                "box {{ background-image: linear-gradient(180deg, alpha({color}, 0.20) 0%, alpha(@window_bg_color, 0.88) 78%); }}"
                            ),
                        );
                        content_box.append(&top_row);
                        content_box.append(&mid_box);
                        content_box.append(&bottom_row);

                        let bg_spacer = GtkBox::new(Orientation::Vertical, 0);
                        let overlay = gtk4::Overlay::new();
                        overlay.add_css_class("bento-overlay");
                        overlay.set_overflow(Overflow::Hidden);
                        overlay.set_child(Some(&bg_spacer));
                        overlay.add_overlay(&bg_picture);
                        overlay.add_overlay(&content_box);
                        overlay.set_measure_overlay(&content_box, true);

                        let btn = Button::builder()
                            .child(&overlay)
                            .width_request(200)
                            .hexpand(true)
                            .css_classes(["flat", "bento-card"])
                            .build();
                        btn.set_overflow(Overflow::Hidden);
                        btn.set_cursor_from_name(Some("pointer"));

                        {
                            let backend = state.borrow().backend.clone();
                            let gid_bg = genre.id.clone();
                            let (tx_bg, rx_bg) = async_channel::bounded(1);
                            runtime().spawn(async move {
                                let res = backend.fetch_genre_playlists(&gid_bg).await;
                                let _ = tx_bg.send(res).await;
                            });
                            let state_bg = Rc::clone(&state);
                            let bg_picture = bg_picture.clone();
                            glib::spawn_future_local(async move {
                                if let Ok(Ok((_, playlists))) = rx_bg.recv().await
                                    && let Some(url) =
                                        playlists.into_iter().find_map(|p| p.cover_url)
                                {
                                    load_cover_into_picture(&bg_picture, &url, state_bg);
                                }
                            });
                        }

                        let state_click = Rc::clone(&state);
                        let gid = genre.id.clone();
                        let gname = genre.name.clone();
                        btn.connect_clicked(move |b| {
                            open_genre_page(
                                gid.clone(),
                                gname.clone(),
                                b.clone().upcast(),
                                Rc::clone(&state_click),
                            );
                        });
                        stations_grid.insert(&btn, -1);
                        count += 1;
                    }
                    stations_section.set_visible(count > 0);
                }
            });
        }
    };

    load_home(false);
    refresh_btn.connect_clicked(move |_| {
        load_home(true);
    });

    (scrolled, quick_grid)
}
