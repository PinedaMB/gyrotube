use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, CssProvider, Image, Label, ListBox, ListBoxRow,
    MenuButton, Orientation, Overflow, Picture, PolicyType, Popover, ScrolledWindow,
    SelectionMode, ToggleButton, Widget,
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

pub fn spawn_bg<F, Fut, T, C>(work: F, on_done: C)
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
    C: FnOnce(T) + 'static,
{
    let (tx, rx) = async_channel::bounded(1);
    runtime().spawn(async move {
        let res = work().await;
        let _ = tx.send(res).await;
    });
    glib::spawn_future_local(async move {
        if let Ok(val) = rx.recv().await {
            on_done(val);
        }
    });
}

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

pub fn empty_state_card(icon_name: &str, title: &str, subtitle: &str) -> GtkBox {
    let card = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(Align::Fill)
        .css_classes(["empty-state-card"])
        .build();

    let icon = Image::builder()
        .icon_name(icon_name)
        .pixel_size(28)
        .halign(Align::Center)
        .margin_bottom(2)
        .css_classes(["empty-state-icon"])
        .build();
    let title_lbl = Label::builder()
        .label(title)
        .halign(Align::Center)
        .css_classes(["heading"])
        .build();
    let sub_lbl = Label::builder()
        .label(subtitle)
        .halign(Align::Center)
        .wrap(true)
        .justify(gtk4::Justification::Center)
        .css_classes(["dim-label", "caption"])
        .build();

    card.append(&icon);
    card.append(&title_lbl);
    card.append(&sub_lbl);
    card
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
    let callbacks = {
        let s = state.borrow();
        s.save_session();
        s.on_track_changed.clone()
    };
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

#[derive(Clone)]
pub struct PagedCarousel {
    pub carousel: adw::Carousel,
    cards: Rc<RefCell<Vec<Widget>>>,
    last_per_page: Rc<Cell<usize>>,
}

fn calc_carousel_per_page(width: i32) -> usize {
    if width <= 0 {
        return 4;
    }
    ((width + 12) / 188).clamp(1, 6) as usize
}

fn relayout_paged_carousel(pc: &PagedCarousel, per_page: usize) {
    while pc.carousel.n_pages() > 0 {
        let page = pc.carousel.nth_page(0);
        if let Ok(page_box) = page.clone().downcast::<GtkBox>() {
            clear_box(&page_box);
        }
        pc.carousel.remove(&page);
    }

    let cards = pc.cards.borrow();
    for chunk in cards.chunks(per_page) {
        let page_row = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .homogeneous(true)
            .hexpand(true)
            .valign(Align::Start)
            .build();
        for card in chunk {
            card.set_hexpand(true);
            page_row.append(card);
        }
        for _ in chunk.len()..per_page {
            let spacer = GtkBox::new(Orientation::Vertical, 0);
            spacer.set_hexpand(true);
            page_row.append(&spacer);
        }
        pc.carousel.append(&page_row);
    }
    pc.last_per_page.set(per_page);
}

pub fn build_paged_carousel_section(title: &str) -> (GtkBox, PagedCarousel) {
    let header_box = GtkBox::new(Orientation::Horizontal, 8);
    let title_label = Label::builder()
        .label(title)
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-3"])
        .build();

    let prev_btn = Button::builder()
        .icon_name("go-previous-symbolic")
        .css_classes(["circular", "flat"])
        .build();
    let next_btn = Button::builder()
        .icon_name("go-next-symbolic")
        .css_classes(["circular", "flat"])
        .build();
    prev_btn.set_cursor_from_name(Some("pointer"));
    next_btn.set_cursor_from_name(Some("pointer"));

    header_box.append(&title_label);
    header_box.append(&prev_btn);
    header_box.append(&next_btn);

    let carousel = adw::Carousel::builder()
        .spacing(12)
        .hexpand(true)
        .vexpand(false)
        .valign(Align::Start)
        .allow_mouse_drag(true)
        .allow_scroll_wheel(false)
        .build();
    let dots = adw::CarouselIndicatorDots::builder()
        .carousel(&carousel)
        .build();

    {
        let carousel = carousel.clone();
        prev_btn.connect_clicked(move |_| {
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
        let carousel = carousel.clone();
        next_btn.connect_clicked(move |_| {
            let n = carousel.n_pages();
            if n == 0 {
                return;
            }
            let pos = ((carousel.position().round() as u32) + 1).min(n - 1);
            let page = carousel.nth_page(pos);
            carousel.scroll_to(&page, true);
        });
    }

    let pc = PagedCarousel {
        carousel: carousel.clone(),
        cards: Rc::new(RefCell::new(Vec::new())),
        last_per_page: Rc::new(Cell::new(0)),
    };

    {
        let pc_tick = pc.clone();
        let last_width = Cell::new(0i32);
        carousel.add_tick_callback(move |c, _| {
            let w = c.width();
            if w > 0 && w != last_width.get() {
                last_width.set(w);
                let per_page = calc_carousel_per_page(w);
                if per_page != pc_tick.last_per_page.get() && !pc_tick.cards.borrow().is_empty() {
                    relayout_paged_carousel(&pc_tick, per_page);
                }
            }
            glib::ControlFlow::Continue
        });
    }

    let section = GtkBox::new(Orientation::Vertical, 10);
    section.set_visible(false);
    section.append(&header_box);
    section.append(&carousel);
    section.append(&dots);

    (section, pc)
}

pub fn populate_paged_carousel(pc: &PagedCarousel, section: &GtkBox, cards: Vec<Widget>) {
    while pc.carousel.n_pages() > 0 {
        let page = pc.carousel.nth_page(0);
        if let Ok(page_box) = page.clone().downcast::<GtkBox>() {
            clear_box(&page_box);
        }
        pc.carousel.remove(&page);
    }
    let has_items = !cards.is_empty();
    *pc.cards.borrow_mut() = cards;
    if has_items {
        let per_page = calc_carousel_per_page(pc.carousel.width());
        relayout_paged_carousel(pc, per_page);
    } else {
        pc.last_per_page.set(0);
    }
    section.set_visible(has_items);
}

fn build_media_card(
    cover_url: Option<&str>,
    badge: Option<(&str, &[&str])>,
    title: &str,
    subtitle: &str,
    meta: Option<&str>,
    state: &Rc<RefCell<AppState>>,
) -> Button {
    let (cover_frame, cover) = clamped_cover(156, 156, "cover-art");
    cover_frame.set_halign(Align::Center);

    if let Some(url) = cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(state));
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
    if let Some((badge_text, classes)) = badge {
        let badge_label = Label::builder()
            .label(badge_text)
            .halign(Align::Start)
            .valign(Align::Start)
            .margin_start(8)
            .margin_top(8)
            .css_classes(classes.to_vec())
            .build();
        cover_overlay.add_overlay(&badge_label);
    }
    cover_overlay.add_overlay(&play_overlay_badge);

    let title_label = Label::builder()
        .label(title)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(18)
        .css_classes(["heading", "album-title"])
        .build();

    let sub_label = Label::builder()
        .label(subtitle)
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(18)
        .css_classes(["dim-label", "caption"])
        .build();

    let text_box = GtkBox::new(Orientation::Vertical, 2);
    text_box.append(&title_label);
    text_box.append(&sub_label);
    if let Some(meta_text) = meta {
        let meta_label = Label::builder()
            .label(meta_text)
            .xalign(0.0)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["dim-label", "caption"])
            .build();
        meta_label.set_opacity(0.75);
        text_box.append(&meta_label);
    }

    let card_box = GtkBox::new(Orientation::Vertical, 10);
    card_box.append(&cover_overlay);
    card_box.append(&text_box);

    let btn = Button::builder()
        .child(&card_box)
        .css_classes(["flat", "album-featured-card"])
        .build();
    btn.set_cursor_from_name(Some("pointer"));
    btn
}

fn show_collection_preview_dialog(
    title: String,
    cover_url: Option<String>,
    tracks: Vec<Track>,
    state: Rc<RefCell<AppState>>,
) {
    let dialog = adw::Dialog::builder()
        .title(&title)
        .content_width(480)
        .content_height(560)
        .build();

    let (cover_frame, cover_pic) = clamped_cover(88, 88, "cover-art");
    if let Some(url) = cover_url.as_deref() {
        load_cover_into_picture(&cover_pic, url, Rc::clone(&state));
    }

    let title_lbl = Label::builder()
        .label(&title)
        .xalign(0.0)
        .wrap(true)
        .css_classes(["title-3"])
        .build();
    let count_lbl = Label::builder()
        .label(format!("{} canciones", tracks.len()))
        .xalign(0.0)
        .css_classes(["dim-label", "caption"])
        .build();

    let play_all_btn = Button::builder()
        .label("Reproducir todo")
        .css_classes(["suggested-action", "pill"])
        .build();
    let add_queue_btn = Button::builder()
        .label("Añadir a la cola")
        .css_classes(["pill"])
        .build();

    let btns_row = GtkBox::new(Orientation::Horizontal, 8);
    btns_row.append(&play_all_btn);
    btns_row.append(&add_queue_btn);

    let meta_col = GtkBox::new(Orientation::Vertical, 6);
    meta_col.set_hexpand(true);
    meta_col.set_valign(Align::Center);
    meta_col.append(&title_lbl);
    meta_col.append(&count_lbl);
    meta_col.append(&btns_row);

    let hero = GtkBox::new(Orientation::Horizontal, 14);
    hero.set_margin_top(12);
    hero.set_margin_bottom(12);
    hero.set_margin_start(16);
    hero.set_margin_end(16);
    hero.append(&cover_frame);
    hero.append(&meta_col);

    let track_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .margin_start(16)
        .margin_end(16)
        .margin_bottom(16)
        .build();
    populate_track_list(&track_list, &tracks, &title, Rc::clone(&state));

    let body = GtkBox::new(Orientation::Vertical, 4);
    body.append(&hero);
    body.append(&track_list);

    let scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&body)
        .build();

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    toolbar.set_content(Some(&scroller));
    dialog.set_child(Some(&toolbar));

    {
        let state = Rc::clone(&state);
        let tracks = tracks.clone();
        let source = title.clone();
        let dlg = dialog.clone();
        play_all_btn.connect_clicked(move |_| {
            {
                let mut s = state.borrow_mut();
                s.queue = tracks.clone();
                s.source_name = source.clone();
            }
            play_queue_index(Rc::clone(&state), 0);
            dlg.close();
        });
    }
    {
        let state = Rc::clone(&state);
        let tracks = tracks.clone();
        let dlg = dialog.clone();
        add_queue_btn.connect_clicked(move |_| {
            let count = tracks.len();
            {
                let mut s = state.borrow_mut();
                s.queue.extend(tracks.clone());
                s.update_nav_buttons();
                s.save_session();
                s.show_toast(&format!("{count} canciones añadidas a la cola"));
            }
            refresh_queue_ui(Rc::clone(&state));
            dlg.close();
        });
    }

    let parent: Widget = state.borrow().toast_overlay.clone().upcast();
    dialog.present(Some(&parent));
}

fn handle_fetched_queue(
    result: Result<(String, Vec<Track>), String>,
    cover_url: Option<String>,
    empty_msg: &str,
    state: Rc<RefCell<AppState>>,
) {
    match result {
        Ok((source, tracks)) if !tracks.is_empty() => {
            show_collection_preview_dialog(source, cover_url, tracks, state);
        }
        Ok(_) => state.borrow().show_toast(empty_msg),
        Err(err) => state.borrow().show_toast(&err),
    }
}

pub fn build_album_card(album: &Album, state: Rc<RefCell<AppState>>) -> Button {
    let year_str = album.year.map(|y| y.to_string());
    let badge = year_str
        .as_deref()
        .map(|y| (y, ["album-year-badge", "numeric"].as_slice()));
    let meta_text = match album.year {
        Some(y) => format!("Álbum • {y}"),
        None => String::from("Álbum"),
    };
    let btn = build_media_card(
        album.cover_url.as_deref(),
        badge,
        &album.title,
        &album.artist,
        Some(&meta_text),
        &state,
    );

    let album_id = album.id.clone();
    let album_title = album.title.clone();
    let album_cover = album.cover_url.clone();
    btn.connect_clicked(move |_| {
        let backend = state.borrow().backend.clone();
        state
            .borrow()
            .show_toast(&format!("Cargando álbum: {album_title}"));
        let id = album_id.clone();
        let cover = album_cover.clone();
        let state = Rc::clone(&state);
        spawn_bg(
            move || async move { backend.fetch_album_tracks(&id).await },
            move |result| {
                handle_fetched_queue(
                    result,
                    cover,
                    "El álbum no tiene pistas disponibles.",
                    state,
                );
            },
        );
    });

    btn
}

pub fn build_playlist_card(playlist: &Playlist, state: Rc<RefCell<AppState>>) -> Button {
    let btn = build_media_card(
        playlist.cover_url.as_deref(),
        Some(("Playlist", ["album-year-badge"].as_slice())),
        &playlist.name,
        &playlist.subtitle,
        None,
        &state,
    );

    let playlist_id = playlist.id.clone();
    let playlist_name = playlist.name.clone();
    let playlist_cover = playlist.cover_url.clone();
    btn.connect_clicked(move |_| {
        let backend = state.borrow().backend.clone();
        state
            .borrow()
            .show_toast(&format!("Cargando playlist: {playlist_name}"));
        let id = playlist_id.clone();
        let cover = playlist_cover.clone();
        let state = Rc::clone(&state);
        spawn_bg(
            move || async move { backend.fetch_playlist_tracks(&id).await },
            move |result| {
                handle_fetched_queue(result, cover, "La playlist está vacía.", state);
            },
        );
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
        .icon_name("folder-music-symbolic")
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

thread_local! {
    static AMBIENT_PROVIDER: std::cell::OnceCell<CssProvider> = const { std::cell::OnceCell::new() };
}

pub fn update_ambient_color(texture: Option<&gdk::Texture>) {
    let provider = AMBIENT_PROVIDER.with(|cell| {
        cell.get_or_init(|| {
            let p = CssProvider::new();
            if let Some(display) = gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(
                    &display,
                    &p,
                    gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
                );
            }
            p
        })
        .clone()
    });

    let Some(tex) = texture else {
        provider.load_from_string("");
        return;
    };

    let w = tex.width().max(1) as usize;
    let h = tex.height().max(1) as usize;
    let stride = w * 4;
    let mut buf = vec![0u8; stride * h];
    tex.download(&mut buf, stride);

    let step_x = (w / 16).max(1);
    let step_y = (h / 16).max(1);
    let (mut sum_r, mut sum_g, mut sum_b, mut total_w) = (0u64, 0u64, 0u64, 0u64);
    for y in (0..h).step_by(step_y) {
        for x in (0..w).step_by(step_x) {
            let idx = y * stride + x * 4;
            let (b, g, r, a) = (buf[idx] as u64, buf[idx + 1] as u64, buf[idx + 2] as u64, buf[idx + 3] as u64);
            if a < 128 {
                continue;
            }
            let luma = r + g + b;
            if luma < 50 || luma > 700 {
                continue;
            }
            let max_c = r.max(g).max(b);
            let min_c = r.min(g).min(b);
            let weight = (max_c - min_c) + 16;
            sum_r += r * weight;
            sum_g += g * weight;
            sum_b += b * weight;
            total_w += weight;
        }
    }

    if total_w == 0 {
        provider.load_from_string("");
        return;
    }
    let r = (sum_r / total_w).min(255);
    let g = (sum_g / total_w).min(255);
    let b = (sum_b / total_w).min(255);
    provider.load_from_string(&format!(
        ".ambient-player {{ background-image: radial-gradient(circle at 50% 28%, rgba({r}, {g}, {b}, 0.24), transparent 72%); }}"
    ));
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
            update_ambient_color(Some(&texture));
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
                update_ambient_color(Some(&texture));
            } else {
                update_ambient_color(None);
            }
            s.player_cover_spinner.set_spinning(false);
            s.player_cover_spinner.set_visible(false);
        }
    });
}
