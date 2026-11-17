use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, ContentFit, CssProvider, Image, Label, ListBox, ListBoxRow,
    MenuButton, Orientation, Overflow, Picture, PolicyType, Popover, ScrolledWindow, ToggleButton,
    Widget,
};
use libadwaita as adw;

use crate::backend::runtime;
use crate::i18n::tr;
use crate::models::{
    Album, Playlist, Track, format_duration, save_favorites, save_recent_tracks,
    save_user_playlists,
};
use crate::ui::collection::{refresh_favorites_ui, refresh_playlists_ui, refresh_recent_ui};
use crate::ui::player::{play_queue_index, refresh_queue_ui, show_player_loading};
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct PagedCarousel {
    pub carousel: adw::Carousel,
    cards: Rc<RefCell<Vec<Widget>>>,
    last_per_page: Rc<Cell<usize>>,
}

#[allow(dead_code)]
fn calc_carousel_per_page(width: i32) -> usize {
    if width <= 0 {
        return 1;
    }
    ((width + 12) / 188).clamp(1, 6) as usize
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
) -> (GtkBox, Button) {
    let (cover_frame, cover) = clamped_cover(156, 156, "cover-art");
    cover_frame.set_halign(Align::Center);

    if let Some(url) = cover_url {
        load_cover_into_picture(&cover, url, Rc::clone(state));
    }

    let play_icon = Image::builder()
        .icon_name("play-filled")
        .pixel_size(18)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    let play_btn = Button::builder()
        .child(&play_icon)
        .halign(Align::End)
        .valign(Align::End)
        .margin_end(8)
        .margin_bottom(8)
        .width_request(40)
        .height_request(40)
        .tooltip_text(tr("Reproducir", "Play"))
        .css_classes(["album-play-overlay"])
        .build();
    play_btn.set_cursor_from_name(Some("pointer"));

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
    cover_overlay.add_overlay(&play_btn);

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

    let card_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .css_classes(["album-featured-card"])
        .build();
    card_box.append(&cover_overlay);
    card_box.append(&text_box);
    card_box.set_cursor_from_name(Some("pointer"));

    (card_box, play_btn)
}

pub fn set_artist_links(label: &Label, artists: &str) {
    let trimmed = artists.trim();
    if trimmed.is_empty() {
        label.set_markup("Artista desconocido");
        return;
    }
    let mut markup = String::new();
    for (i, comma_part) in trimmed.split(", ").enumerate() {
        if i > 0 {
            markup.push_str(", ");
        }
        for (j, amp_part) in comma_part.split(" & ").enumerate() {
            if j > 0 {
                markup.push_str(" &amp; ");
            }
            let name = amp_part.trim();
            if !name.is_empty() {
                let esc = glib::markup_escape_text(name);
                markup.push_str(&format!("<a href=\"{esc}\">{esc}</a>"));
            }
        }
    }
    label.set_markup(&markup);
}

pub fn set_album_link(label: &Label, album: Option<&str>) {
    if let Some(name) = album.map(str::trim).filter(|s| !s.is_empty()) {
        let esc = glib::markup_escape_text(name);
        label.set_markup(&format!("Álbum: <a href=\"{esc}\">{esc}</a>"));
        label.set_visible(true);
    } else {
        label.set_text("");
        label.set_visible(false);
    }
}

pub fn set_source_link(label: &Label, album: Option<&str>, fallback: &str) {
    if let Some(name) = album.map(str::trim).filter(|s| !s.is_empty()) {
        let esc = glib::markup_escape_text(name);
        label.set_markup(&format!("<a href=\"{esc}\">{esc}</a>"));
    } else {
        label.set_text(fallback);
    }
}

/// Strips `<a ...>` / `</a>` markup tags and unescapes XML entities for plain-text labels.
pub fn strip_link_markup(markup: &str) -> String {
    let mut out = String::with_capacity(markup.len());
    let mut in_tag = false;
    for ch in markup.chars() {
        match ch {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

pub fn open_collection_page(
    title: String,
    cover_url: Option<String>,
    tracks: Option<Vec<Track>>,
    state: Rc<RefCell<AppState>>,
) {
    let cb = state.borrow().on_open_collection.clone();
    if let Some(open_fn) = cb {
        open_fn(title, cover_url, tracks);
    }
}

pub fn cancel_collection_loading(state: &Rc<RefCell<AppState>>) {
    let cb = state.borrow().on_cancel_collection_loading.clone();
    if let Some(cancel_fn) = cb {
        cancel_fn();
    }
}

fn handle_fetched_queue(
    result: Result<(String, Vec<Track>), String>,
    cover_url: Option<String>,
    empty_msg: &str,
    state: Rc<RefCell<AppState>>,
) {
    match result {
        Ok((source, tracks)) if !tracks.is_empty() => {
            open_collection_page(source, cover_url, Some(tracks), state);
        }
        Ok(_) => {
            cancel_collection_loading(&state);
            state.borrow().show_toast(empty_msg);
        }
        Err(err) => {
            cancel_collection_loading(&state);
            state.borrow().show_toast(&err);
        }
    }
}

fn play_fetched_queue(
    result: Result<(String, Vec<Track>), String>,
    empty_msg: &str,
    state: Rc<RefCell<AppState>>,
) {
    match result {
        Ok((source, tracks)) if !tracks.is_empty() => {
            {
                let mut s = state.borrow_mut();
                s.shuffle = false;
                s.unshuffled_queue.clear();
                s.queue = tracks;
                s.source_name = source;
            }
            play_queue_index(state, 0);
        }
        Ok(_) => {
            let s = state.borrow();
            s.play_spinner.set_spinning(false);
            s.play_pause_btn
                .set_icon_name("play-filled");
            s.play_pause_btn.set_sensitive(true);
            s.show_toast(empty_msg);
        }
        Err(err) => {
            let s = state.borrow();
            s.play_spinner.set_spinning(false);
            s.play_pause_btn
                .set_icon_name("play-filled");
            s.play_pause_btn.set_sensitive(true);
            s.show_toast(&err);
        }
    }
}

fn connect_card_open(card: &GtkBox, on_open: impl Fn() + 'static) {
    let click = gtk4::GestureClick::new();
    click.connect_released(move |gesture, n_press, x, y| {
        if n_press != 1 {
            return;
        }
        if let Some(widget) = gesture.widget() {
            if !widget.contains(x, y) {
                return;
            }
        }
        on_open();
    });
    card.add_controller(click);
}

pub fn open_album_by_id(
    album_id: String,
    album_title: String,
    album_cover: Option<String>,
    state: Rc<RefCell<AppState>>,
) {
    let backend = state.borrow().backend.clone();
    open_collection_page(album_title, album_cover.clone(), None, Rc::clone(&state));
    spawn_bg(
        move || async move { backend.fetch_album_tracks(&album_id).await },
        move |result| {
            handle_fetched_queue(
                result,
                album_cover,
                tr(
                    "El álbum no tiene pistas disponibles.",
                    "This album has no available tracks.",
                ),
                state,
            );
        },
    );
}

pub fn play_album_by_id(album_id: String, album_title: String, state: Rc<RefCell<AppState>>) {
    show_player_loading(&state, &album_title);
    let backend = state.borrow().backend.clone();
    spawn_bg(
        move || async move { backend.fetch_album_tracks(&album_id).await },
        move |result| {
            play_fetched_queue(
                result,
                tr(
                    "El álbum no tiene pistas disponibles.",
                    "This album has no available tracks.",
                ),
                state,
            );
        },
    );
}

pub fn build_album_card(album: &Album, state: Rc<RefCell<AppState>>) -> GtkBox {
    let year_str = album.year.map(|y| y.to_string());
    let badge = year_str
        .as_deref()
        .map(|y| (y, ["album-year-badge", "numeric"].as_slice()));
    let meta_text = match album.year {
        Some(y) => format!("{} • {y}", tr("Álbum", "Album")),
        None => String::from(tr("Álbum", "Album")),
    };
    let (card, play_btn) = build_media_card(
        album.cover_url.as_deref(),
        badge,
        &album.title,
        &album.artist,
        Some(&meta_text),
        &state,
    );

    let album_id = album.id.clone();
    let album_title = album.title.clone();
    let album_title_play = album.title.clone();
    let album_cover = album.cover_url.clone();
    {
        let album_id = album_id.clone();
        let state = Rc::clone(&state);
        connect_card_open(&card, move || {
            open_album_by_id(
                album_id.clone(),
                album_title.clone(),
                album_cover.clone(),
                Rc::clone(&state),
            );
        });
    }

    play_btn.connect_clicked(move |_| {
        play_album_by_id(album_id.clone(), album_title_play.clone(), Rc::clone(&state));
    });

    card
}

pub fn build_playlist_card(playlist: &Playlist, state: Rc<RefCell<AppState>>) -> GtkBox {
    let (card, play_btn) = build_media_card(
        playlist.cover_url.as_deref(),
        Some(("Playlist", ["album-year-badge"].as_slice())),
        &playlist.name,
        &playlist.subtitle,
        None,
        &state,
    );

    let playlist_id = playlist.id.clone();
    let playlist_name = playlist.name.clone();
    let playlist_name_play = playlist.name.clone();
    let playlist_cover = playlist.cover_url.clone();
    {
        let playlist_id = playlist_id.clone();
        let state = Rc::clone(&state);
        connect_card_open(&card, move || {
            let backend = state.borrow().backend.clone();
            open_collection_page(
                playlist_name.clone(),
                playlist_cover.clone(),
                None,
                Rc::clone(&state),
            );
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
    }

    play_btn.connect_clicked(move |_| {
        show_player_loading(&state, &playlist_name_play);
        let backend = state.borrow().backend.clone();
        let id = playlist_id.clone();
        let state = Rc::clone(&state);
        spawn_bg(
            move || async move { backend.fetch_playlist_tracks(&id).await },
            move |result| {
                play_fetched_queue(result, "La playlist está vacía.", state);
            },
        );
    });

    card
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

    let duration_str = track.duration_secs.map(format_duration).unwrap_or_default();
    let duration_label = Label::builder()
        .label(&duration_str)
        .valign(Align::Center)
        .visible(track.duration_secs.is_some())
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

    let add_pl_popover = Popover::new();
    let add_pl_btn = MenuButton::builder()
        .icon_name("music-note-2-regular")
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
        .icon_name("sound-wave-circle-regular")
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

    let row = ListBoxRow::builder().child(&row_box).build();
    row.set_widget_name(&track.id);
    if state
        .borrow()
        .current_track()
        .is_some_and(|t| t.id == track.id)
    {
        row.add_css_class("chart-active-row");
    }
    row
}

pub fn highlight_active_track_in_list(list: &ListBox, active_id: Option<&str>) {
    let mut child = list.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        if let Ok(row) = w.downcast::<ListBoxRow>() {
            let is_active = active_id.is_some_and(|id| row.widget_name() == id);
            if is_active {
                row.add_css_class("chart-active-row");
            } else {
                row.remove_css_class("chart-active-row");
            }
        }
    }
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
                "heart-filled"
            } else {
                "heart-regular"
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
                Ok(_) => state
                    .borrow()
                    .show_toast("No se encontraron canciones para la radio."),
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
                state
                    .borrow_mut()
                    .cover_cache
                    .insert(url_key, texture.clone());
                picture.set_paintable(Some(&texture));
            }
        }
    });
}

thread_local! {
    static AMBIENT_PROVIDER: std::cell::OnceCell<CssProvider> = const { std::cell::OnceCell::new() };
    static ALBUM_AMBIENT_PROVIDER: std::cell::OnceCell<CssProvider> = const { std::cell::OnceCell::new() };
}

fn ensure_display_provider(cell: &std::cell::OnceCell<CssProvider>) -> CssProvider {
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
}

pub fn extract_dominant_rgb(tex: &gdk::Texture) -> Option<(u8, u8, u8)> {
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
            let (b, g, r, a) = (
                buf[idx] as u64,
                buf[idx + 1] as u64,
                buf[idx + 2] as u64,
                buf[idx + 3] as u64,
            );
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
        return None;
    }
    Some((
        (sum_r / total_w).min(255) as u8,
        (sum_g / total_w).min(255) as u8,
        (sum_b / total_w).min(255) as u8,
    ))
}

pub fn update_ambient_color(texture: Option<&gdk::Texture>) {
    let provider = AMBIENT_PROVIDER.with(ensure_display_provider);
    if let Some((r, g, b)) = texture.and_then(extract_dominant_rgb) {
        provider.load_from_string(&format!(
            ".ambient-player {{ background-image: radial-gradient(circle at 50% 28%, rgba({r}, {g}, {b}, 0.24), transparent 72%); }}"
        ));
    } else {
        provider.load_from_string("");
    }
}

pub fn update_album_ambient_color(texture: Option<&gdk::Texture>) {
    let provider = ALBUM_AMBIENT_PROVIDER.with(ensure_display_provider);
    if let Some((r, g, b)) = texture.and_then(extract_dominant_rgb) {
        provider.load_from_string(&format!(
            ".album-page-hero {{ background-image: linear-gradient(180deg, rgba({r}, {g}, {b}, 0.42) 0%, rgba({r}, {g}, {b}, 0.12) 100%); }}"
        ));
    } else {
        provider.load_from_string("");
    }
}

pub fn load_track_player_cover(
    picture: &Picture,
    track: &Track,
    expected_gen: u64,
    state: Rc<RefCell<AppState>>,
) {
    picture.set_paintable(None::<&gdk::Paintable>);
    update_ambient_color(None);
    {
        let s = state.borrow();
        s.player_cover_spinner.set_visible(true);
        s.player_cover_spinner.set_spinning(true);
    }
    if let Some(url) = track
        .cover_large_url
        .as_deref()
        .or(track.cover_url.as_deref())
    {
        load_player_cover_with_fallback(
            picture,
            url,
            track.cover_url.as_deref(),
            expected_gen,
            state,
        );
        return;
    }

    let backend = state.borrow().backend.clone();
    let track = track.clone();
    let picture = picture.clone();
    spawn_bg(
        move || async move { backend.resolve_ytm_cover(&track).await },
        move |url| {
            if state.borrow().loading_generation != expected_gen {
                return;
            }
            if let Some(url) = url {
                load_player_cover_with_fallback(&picture, &url, None, expected_gen, state);
            } else {
                let s = state.borrow();
                s.player_cover_spinner.set_spinning(false);
                s.player_cover_spinner.set_visible(false);
                eprintln!("Sin carátula para la pista actual");
            }
        },
    );
}

pub fn load_player_cover_with_fallback(
    picture: &Picture,
    url: &str,
    fallback_url: Option<&str>,
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
    if let Some(texture) = fallback_url.and_then(|fb| state.borrow().cover_cache.get(fb).cloned()) {
        picture.set_paintable(Some(&texture));
        update_ambient_color(Some(&texture));
    }

    let mut urls = vec![url.to_string()];
    if let Some(fb) = fallback_url.filter(|fb| *fb != url) {
        urls.push(fb.to_string());
    }
    let backend = state.borrow().backend.clone();
    let picture = picture.clone();
    glib::spawn_future_local(async move {
        for url in urls {
            if state.borrow().loading_generation != expected_gen {
                return;
            }
            let backend = backend.clone();
            let request_url = url.clone();
            let (tx, rx) = async_channel::bounded(1);
            runtime().spawn(async move {
                let _ = tx.send(backend.fetch_image_bytes(&request_url).await).await;
            });
            let texture = match rx.recv().await {
                Ok(Ok(bytes)) => gdk::Texture::from_bytes(&glib::Bytes::from_owned(bytes))
                    .map_err(|e| e.to_string()),
                Ok(Err(err)) => Err(err),
                Err(err) => Err(err.to_string()),
            };
            if state.borrow().loading_generation != expected_gen {
                return;
            }
            match texture {
                Ok(texture) => {
                    state.borrow_mut().cover_cache.insert(url, texture.clone());
                    picture.set_paintable(Some(&texture));
                    update_ambient_color(Some(&texture));
                    break;
                }
                Err(err) => eprintln!("Error de carátula ({url}): {err}"),
            }
        }
        let s = state.borrow();
        s.player_cover_spinner.set_spinning(false);
        s.player_cover_spinner.set_visible(false);
    });
}
