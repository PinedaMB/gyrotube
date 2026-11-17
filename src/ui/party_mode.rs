use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Image, Justification, Label, Orientation, Picture, PolicyType,
    ScrolledWindow, ToggleButton,
};
use libadwaita as adw;

use crate::i18n::tr;
use crate::ui::player::{load_current_lyrics, scroll_party_lyric_centered, update_party_lyrics_fade};
use crate::ui::sidebar_player::build_bound_progress_bar;
use crate::ui::state::{AppState, next_queue_index};
use crate::ui::widgets::{clamped_cover, load_cover_into_picture, strip_link_markup};

pub struct PartyWidgets {
    pub root: GtkBox,
    pub lyrics_box: GtkBox,
    pub lyrics_scroll: ScrolledWindow,
    pub lyrics_body_label: Label,
    pub next_box: GtkBox,
    pub next_cover: Picture,
    pub next_title: Label,
    pub next_artist: Label,
    pub exit_btn: Button,
    pub fullscreen_btn: Button,
}

pub fn build_party_mode(
    state: &Rc<RefCell<AppState>>,
    sidebar_shuffle_btn: &ToggleButton,
    sidebar_repeat_btn: &ToggleButton,
) -> PartyWidgets {
    let s = state.borrow();

    // Top bar
    let badge_content = GtkBox::new(Orientation::Horizontal, 6);
    badge_content.append(&Image::from_icon_name("sparkle-regular"));
    badge_content.append(&Label::new(Some(tr("Modo Fiesta", "Party Mode"))));
    let badge = GtkBox::new(Orientation::Horizontal, 0);
    badge.add_css_class("party-badge");
    badge.append(&badge_content);

    let source_label = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["dim-label", "caption"])
        .build();
    s.player_source_label
        .bind_property("label", &source_label, "label")
        .transform_to(|_, val: String| Some(strip_link_markup(&val).to_value()))
        .sync_create()
        .build();

    let top_left = GtkBox::new(Orientation::Horizontal, 12);
    top_left.set_hexpand(true);
    top_left.set_valign(Align::Center);
    top_left.append(&badge);
    top_left.append(&source_label);

    let fullscreen_btn = Button::builder()
        .icon_name("full-screen-minimize-regular")
        .tooltip_text(tr(
            "Alternar pantalla completa (F11)",
            "Toggle fullscreen (F11)",
        ))
        .css_classes(["flat", "circular"])
        .build();
    fullscreen_btn.set_cursor_from_name(Some("pointer"));

    let exit_btn = Button::builder()
        .label(tr("Salir", "Exit"))
        .tooltip_text(tr("Salir del Modo Fiesta (Esc)", "Exit Party Mode (Esc)"))
        .css_classes(["pill"])
        .build();
    exit_btn.set_cursor_from_name(Some("pointer"));

    let top_bar = GtkBox::new(Orientation::Horizontal, 8);
    top_bar.set_margin_top(16);
    top_bar.set_margin_start(24);
    top_bar.set_margin_end(24);
    top_bar.append(&top_left);
    top_bar.append(&fullscreen_btn);
    top_bar.append(&exit_btn);

    // Left column: Big cover + track info + progress + controls + next up
    let (cover_frame, cover_pic) = clamped_cover(300, 300, "cover-art-large");
    cover_frame.add_css_class("party-cover");
    cover_frame.set_halign(Align::Center);
    s.player_cover
        .bind_property("paintable", &cover_pic, "paintable")
        .sync_create()
        .build();

    let title_label = Label::builder()
        .justify(Justification::Center)
        .wrap(true)
        .max_width_chars(32)
        .css_classes(["party-title"])
        .build();
    s.player_title
        .bind_property("label", &title_label, "label")
        .sync_create()
        .build();

    let artist_label = Label::builder()
        .justify(Justification::Center)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(36)
        .css_classes(["party-artist"])
        .build();
    s.player_artist
        .bind_property("label", &artist_label, "label")
        .transform_to(|_, val: String| Some(strip_link_markup(&val).to_value()))
        .sync_create()
        .build();

    let progress_bar = build_bound_progress_bar(&s.seek_scale.adjustment());
    let time_cur = Label::builder()
        .css_classes(["caption", "numeric", "dim-label"])
        .hexpand(true)
        .xalign(0.0)
        .build();
    s.time_label
        .bind_property("label", &time_cur, "label")
        .sync_create()
        .build();
    let time_dur = Label::builder()
        .css_classes(["caption", "numeric", "dim-label"])
        .xalign(1.0)
        .build();
    s.duration_label
        .bind_property("label", &time_dur, "label")
        .sync_create()
        .build();

    let times_row = GtkBox::new(Orientation::Horizontal, 8);
    times_row.append(&time_cur);
    times_row.append(&time_dur);

    let progress_box = GtkBox::new(Orientation::Vertical, 4);
    progress_box.set_size_request(320, -1);
    progress_box.set_halign(Align::Center);
    progress_box.append(&progress_bar);
    progress_box.append(&times_row);

    // Transport controls
    let shuffle_btn = ToggleButton::builder()
        .icon_name("arrow-shuffle-regular")
        .tooltip_text(tr("Aleatorio", "Shuffle"))
        .css_classes(["flat", "circular"])
        .build();
    sidebar_shuffle_btn
        .bind_property("active", &shuffle_btn, "active")
        .flags(gtk4::glib::BindingFlags::BIDIRECTIONAL | gtk4::glib::BindingFlags::SYNC_CREATE)
        .build();

    let prev_btn = Button::builder()
        .icon_name("previous-filled")
        .tooltip_text(tr("Anterior", "Previous"))
        .css_classes(["flat", "circular"])
        .build();
    s.prev_btn
        .bind_property("sensitive", &prev_btn, "sensitive")
        .sync_create()
        .build();
    {
        let p = s.prev_btn.clone();
        prev_btn.connect_clicked(move |_| p.emit_clicked());
    }

    let play_btn = Button::builder()
        .icon_name("play-filled")
        .tooltip_text(tr("Reproducir / Pausar", "Play / Pause"))
        .css_classes(["suggested-action", "play-pill"])
        .build();
    s.play_pause_btn
        .bind_property("icon-name", &play_btn, "icon-name")
        .sync_create()
        .build();
    s.play_pause_btn
        .bind_property("sensitive", &play_btn, "sensitive")
        .sync_create()
        .build();
    {
        let pp = s.play_pause_btn.clone();
        play_btn.connect_clicked(move |_| pp.emit_clicked());
    }

    let next_btn = Button::builder()
        .icon_name("next-filled")
        .tooltip_text(tr("Siguiente", "Next"))
        .css_classes(["flat", "circular"])
        .build();
    s.next_btn
        .bind_property("sensitive", &next_btn, "sensitive")
        .sync_create()
        .build();
    {
        let n = s.next_btn.clone();
        next_btn.connect_clicked(move |_| n.emit_clicked());
    }

    let repeat_btn = ToggleButton::builder()
        .icon_name("arrow-repeat-all-regular")
        .tooltip_text(tr("Repetir", "Repeat"))
        .css_classes(["flat", "circular"])
        .build();
    sidebar_repeat_btn
        .bind_property("active", &repeat_btn, "active")
        .flags(gtk4::glib::BindingFlags::BIDIRECTIONAL | gtk4::glib::BindingFlags::SYNC_CREATE)
        .build();

    let controls_row = GtkBox::new(Orientation::Horizontal, 14);
    controls_row.set_halign(Align::Center);
    controls_row.append(&shuffle_btn);
    controls_row.append(&prev_btn);
    controls_row.append(&play_btn);
    controls_row.append(&next_btn);
    controls_row.append(&repeat_btn);

    // Next up card
    let next_header = Label::builder()
        .label(tr("A CONTINUACIÓN", "UP NEXT"))
        .xalign(0.0)
        .css_classes(["album-type-label"])
        .build();
    let (next_cover_frame, next_cover) = clamped_cover(42, 42, "cover-art");
    let next_title = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["heading"])
        .build();
    let next_artist = Label::builder()
        .xalign(0.0)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["caption", "dim-label"])
        .build();
    let next_text = GtkBox::new(Orientation::Vertical, 2);
    next_text.set_hexpand(true);
    next_text.set_valign(Align::Center);
    next_text.append(&next_title);
    next_text.append(&next_artist);

    let next_row = GtkBox::new(Orientation::Horizontal, 10);
    next_row.append(&next_cover_frame);
    next_row.append(&next_text);

    let next_box = GtkBox::new(Orientation::Vertical, 6);
    next_box.add_css_class("party-next-card");
    next_box.set_size_request(320, -1);
    next_box.set_halign(Align::Center);
    next_box.set_visible(false);
    next_box.append(&next_header);
    next_box.append(&next_row);

    let left_col = GtkBox::new(Orientation::Vertical, 14);
    left_col.set_valign(Align::Center);
    left_col.set_halign(Align::Center);
    left_col.set_hexpand(false);
    left_col.set_size_request(360, -1);
    left_col.set_margin_start(12);
    left_col.set_margin_end(12);
    left_col.append(&cover_frame);
    left_col.append(&title_label);
    left_col.append(&artist_label);
    left_col.append(&progress_box);
    left_col.append(&controls_row);
    left_col.append(&next_box);

    // Right column: Karaoke lyrics
    let lyrics_heading = Label::builder()
        .label(tr("Letra", "Lyrics"))
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["title-3", "dim-label"])
        .build();
    let lyrics_sync_badge = Label::builder()
        .visible(false)
        .halign(Align::End)
        .build();
    s.lyrics_sync_badge
        .bind_property("label", &lyrics_sync_badge, "label")
        .sync_create()
        .build();
    s.lyrics_sync_badge
        .bind_property("visible", &lyrics_sync_badge, "visible")
        .sync_create()
        .build();

    let lyrics_header = GtkBox::new(Orientation::Horizontal, 8);
    lyrics_header.append(&lyrics_heading);
    lyrics_header.append(&lyrics_sync_badge);

    let lyrics_body_label = Label::builder()
        .label(tr(
            "Reproduce una canción para ver su letra.",
            "Play a song to view its lyrics.",
        ))
        .xalign(0.0)
        .yalign(0.0)
        .wrap(true)
        .selectable(true)
        .css_classes(["party-lyrics-plain"])
        .build();

    let lyrics_box = GtkBox::new(Orientation::Vertical, 8);
    lyrics_box.add_css_class("party-lyrics-card");
    lyrics_box.append(&lyrics_body_label);

    let lyrics_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .hexpand(true)
        .child(&lyrics_box)
        .build();
    {
        let state_scroll = Rc::clone(state);
        lyrics_scroll.vadjustment().connect_value_changed(move |_| {
            update_party_lyrics_fade(&state_scroll.borrow());
        });
    }

    let right_col = GtkBox::new(Orientation::Vertical, 8);
    right_col.set_hexpand(true);
    right_col.set_vexpand(true);
    right_col.append(&lyrics_header);
    right_col.append(&lyrics_scroll);

    let content_split = GtkBox::new(Orientation::Horizontal, 36);
    content_split.set_homogeneous(false);
    content_split.set_hexpand(true);
    content_split.set_vexpand(true);
    content_split.set_margin_top(12);
    content_split.set_margin_bottom(24);
    content_split.set_margin_start(32);
    content_split.set_margin_end(32);
    content_split.append(&left_col);
    content_split.append(&right_col);

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.add_css_class("party-mode-bg");
    root.add_css_class("ambient-player");
    root.set_vexpand(true);
    root.append(&top_bar);
    root.append(&content_split);

    PartyWidgets {
        root,
        lyrics_box,
        lyrics_scroll,
        lyrics_body_label,
        next_box,
        next_cover,
        next_title,
        next_artist,
        exit_btn,
        fullscreen_btn,
    }
}

pub fn refresh_party_next_up(state: &Rc<RefCell<AppState>>) {
    let (next_track, next_box, next_title, next_artist, next_cover, cached_tex) = {
        let s = state.borrow();
        let next = next_queue_index(&s).and_then(|i| s.queue.get(i).cloned());
        let tex = next
            .as_ref()
            .and_then(|t| t.cover_url.as_ref())
            .and_then(|u| s.cover_cache.get(u).cloned());
        (
            next,
            s.party_next_box.clone(),
            s.party_next_title.clone(),
            s.party_next_artist.clone(),
            s.party_next_cover.clone(),
            tex,
        )
    };

    let Some(track) = next_track else {
        next_box.set_visible(false);
        return;
    };

    next_title.set_label(&track.title);
    next_artist.set_label(&track.artist);
    next_box.set_visible(true);

    if let Some(tex) = cached_tex {
        next_cover.set_paintable(Some(&tex));
    } else {
        next_cover.set_paintable(None::<&gtk4::gdk::Texture>);
        if let Some(url) = track.cover_url {
            load_cover_into_picture(&next_cover, &url, Rc::clone(state));
        }
    }
}

pub fn enter_party_mode(
    state: &Rc<RefCell<AppState>>,
    root_stack: &gtk4::Stack,
    window: &adw::ApplicationWindow,
    fullscreen_btn: &Button,
) {
    state.borrow_mut().party_active = true;
    root_stack.set_visible_child_name("party");
    window.fullscreen();
    fullscreen_btn.set_icon_name("full-screen-minimize-regular");
    load_current_lyrics(Rc::clone(state));
    refresh_party_next_up(state);
    let state_fade = Rc::clone(state);
    gtk4::glib::idle_add_local_once(move || {
        let s = state_fade.borrow();
        scroll_party_lyric_centered(&s, false);
        update_party_lyrics_fade(&s);
    });
}

pub fn exit_party_mode(
    state: &Rc<RefCell<AppState>>,
    root_stack: &gtk4::Stack,
    window: &adw::ApplicationWindow,
) {
    state.borrow_mut().party_active = false;
    root_stack.set_visible_child_name("main");
    window.unfullscreen();
}
