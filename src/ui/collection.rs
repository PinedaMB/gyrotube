use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk4::gdk;
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, CssProvider, FlowBox, Image, Label, ListBox, ListBoxRow,
    MenuButton, Orientation, PolicyType, Popover, ScrolledWindow, SearchEntry, SelectionMode,
    Spinner,
};
use libadwaita as adw;

use crate::backend::runtime;
use crate::i18n::tr;
use crate::models::{
    Track, UserPlaylist, format_duration, save_favorites, save_recent_tracks, save_user_playlists,
};
use crate::ui::player::{play_queue_index, refresh_queue_ui};
use crate::ui::state::AppState;
use crate::ui::widgets::{
    clamped_cover, clear_box, clear_list_box, empty_state_card, load_cover_into_picture,
    populate_track_list,
};

const FAVORITES_PLAYLIST_ID: &str = "__favorites__";

fn favorites_as_playlist(s: &AppState) -> UserPlaylist {
    UserPlaylist {
        id: FAVORITES_PLAYLIST_ID.to_string(),
        name: tr("Favoritos", "Favorites").to_string(),
        description: tr("Tus canciones con me gusta", "Your liked songs").to_string(),
        color_hex: Some("#e01b24".to_string()),
        icon_name: Some("heart-filled".to_string()),
        cover_path: None,
        tracks: s.favorites.clone(),
    }
}

const PLAYLIST_COLORS: &[&str] = &[
    "#e01b24", "#3584e4", "#c061cb", "#2ec27e", "#e66100", "#e5a50a", "#0097a7", "#6366f1",
];

const PLAYLIST_ICONS: &[&str] = &[
    "music-note-2-regular",
    "heart-filled",
    "sparkle-regular",
    "sound-wave-circle-regular",
    "library-regular",
    "people-community-regular",
];

fn format_playlist_stats(pl: &UserPlaylist) -> String {
    let count = pl.tracks.len();
    let count_str = if count == 1 {
        format!("1 {}", tr("canción", "song"))
    } else {
        format!("{count} {}", tr("canciones", "songs"))
    };
    let total_secs = pl.total_duration_secs();
    if total_secs == 0 {
        return count_str;
    }
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    let dur_str = if hours > 0 {
        format!("{hours}h {mins}m")
    } else if mins > 0 {
        format!("{mins}m {secs:02}s")
    } else {
        format!("{secs}s")
    };
    format!("{count_str} • {dur_str}")
}

fn apply_inline_css(widget: &impl IsA<gtk4::Widget>, css: &str) {
    let provider = CssProvider::new();
    provider.load_from_string(css);
    #[allow(deprecated)]
    widget
        .style_context()
        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn build_playlist_art(
    pl: &UserPlaylist,
    size: i32,
    fallback_idx: usize,
    state: Rc<RefCell<AppState>>,
) -> gtk4::Widget {
    if let Some(path) = pl.cover_path.as_deref()
        && let Ok(tex) = gdk::Texture::from_filename(path)
    {
        let (frame, pic) = clamped_cover(size, size, "cover-art");
        pic.set_paintable(Some(&tex));
        return frame.upcast();
    }

    if pl.color_hex.is_none()
        && pl.icon_name.is_none()
        && let Some(url) = pl.effective_cover_url()
    {
        let (frame, pic) = clamped_cover(size, size, "cover-art");
        load_cover_into_picture(&pic, url, state);
        return frame.upcast();
    }

    let color = pl
        .color_hex
        .as_deref()
        .unwrap_or(PLAYLIST_COLORS[fallback_idx % PLAYLIST_COLORS.len()]);
    let icon_name = pl.icon_name.as_deref().unwrap_or("music-note-2-regular");

    let icon = Image::builder()
        .icon_name(icon_name)
        .pixel_size((size / 2).clamp(18, 48))
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    let badge = GtkBox::builder()
        .width_request(size)
        .height_request(size)
        .hexpand(false)
        .vexpand(false)
        .valign(Align::Center)
        .halign(Align::Start)
        .css_classes(["playlist-art-badge"])
        .build();
    apply_inline_css(
        &badge,
        &format!(
            "box {{ background-image: linear-gradient(135deg, {color} 0%, shade({color}, 0.68) 100%); }}"
        ),
    );
    badge.append(&icon);
    badge.upcast()
}

pub fn build_collection_page(state: Rc<RefCell<AppState>>) -> ScrolledWindow {
    let container = GtkBox::new(Orientation::Vertical, 22);
    container.set_margin_top(18);
    container.set_margin_bottom(112);
    container.set_margin_start(20);
    container.set_margin_end(20);

    let page_header = GtkBox::new(Orientation::Vertical, 2);
    page_header.append(
        &Label::builder()
            .label(tr("Colección", "Collection"))
            .xalign(0.0)
            .css_classes(["title-1"])
            .build(),
    );
    page_header.append(
        &Label::builder()
            .label(tr(
                "Tus playlists locales, historial reciente y canciones favoritas",
                "Your local playlists, recent history, and favorite songs",
            ))
            .xalign(0.0)
            .css_classes(["dim-label", "caption"])
            .build(),
    );
    container.append(&page_header);

    // 1. Custom local playlists
    let pl_header = GtkBox::new(Orientation::Horizontal, 8);
    pl_header.append(
        &Label::builder()
            .label(tr("Playlists locales", "Local playlists"))
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["title-2"])
            .build(),
    );

    let new_pl_content = GtkBox::new(Orientation::Horizontal, 6);
    new_pl_content.append(&Image::from_icon_name("add-regular"));
    new_pl_content.append(&Label::new(Some(tr("Nueva playlist", "New playlist"))));
    let new_pl_btn = Button::builder()
        .child(&new_pl_content)
        .css_classes(["suggested-action", "pill"])
        .build();
    new_pl_btn.set_cursor_from_name(Some("pointer"));
    pl_header.append(&new_pl_btn);

    {
        let state = Rc::clone(&state);
        new_pl_btn.connect_clicked(move |btn| {
            open_create_playlist_dialog(btn, Rc::clone(&state), None, Vec::new());
        });
    }

    let (playlists_box, playlists_empty_label) = {
        let s = state.borrow();
        (s.playlists_box.clone(), s.playlists_empty_label.clone())
    };
    container.append(&pl_header);
    container.append(&playlists_empty_label);
    container.append(&playlists_box);

    // 2. Recently played
    let recent_header = GtkBox::new(Orientation::Horizontal, 8);
    recent_header.append(
        &Label::builder()
            .label(tr("Escuchado recientemente", "Recently played"))
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["title-2"])
            .build(),
    );
    let play_recent_btn = Button::builder()
        .icon_name("play-filled")
        .label(tr("Reproducir todo", "Play all"))
        .css_classes(["flat", "pill-switch"])
        .build();
    play_recent_btn.set_cursor_from_name(Some("pointer"));
    {
        let state = Rc::clone(&state);
        play_recent_btn.connect_clicked(move |_| {
            let has_tracks = {
                let mut s = state.borrow_mut();
                if s.recent_tracks.is_empty() {
                    false
                } else {
                    let tracks = s.recent_tracks.clone();
                    s.replace_queue(tracks, String::from(tr("Recientes", "Recent")));
                    true
                }
            };
            if has_tracks {
                play_queue_index(Rc::clone(&state), 0);
            }
        });
    }
    let clear_recent_btn = Button::builder()
        .icon_name("delete-regular")
        .tooltip_text(tr("Limpiar historial", "Clear history"))
        .css_classes(["flat", "circular"])
        .build();
    {
        let state = Rc::clone(&state);
        clear_recent_btn.connect_clicked(move |_| {
            {
                let mut s = state.borrow_mut();
                s.recent_tracks.clear();
                s.report_save_result(save_recent_tracks(&s.recent_tracks));
            }
            refresh_recent_ui(Rc::clone(&state));
        });
    }
    recent_header.append(&play_recent_btn);
    recent_header.append(&clear_recent_btn);
    container.append(&recent_header);

    let (recent_list, recent_empty_label) = {
        let s = state.borrow();
        (s.recent_list.clone(), s.recent_empty_label.clone())
    };
    recent_list
        .bind_property("visible", &play_recent_btn, "visible")
        .sync_create()
        .build();
    recent_list
        .bind_property("visible", &clear_recent_btn, "visible")
        .sync_create()
        .build();
    container.append(&recent_empty_label);
    container.append(&recent_list);

    let clamp = adw::Clamp::builder()
        .maximum_size(1200)
        .tightening_threshold(900)
        .child(&container)
        .build();

    ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&clamp)
        .build()
}

pub fn refresh_favorites_ui(state: Rc<RefCell<AppState>>) {
    let (favorites_list, favorites_empty_label, favorites) = {
        let s = state.borrow();
        (
            s.favorites_list.clone(),
            s.favorites_empty_label.clone(),
            s.favorites.clone(),
        )
    };

    favorites_empty_label.set_visible(favorites.is_empty());
    favorites_list.set_visible(!favorites.is_empty());
    populate_track_list(
        &favorites_list,
        &favorites,
        tr("Favoritos", "Favorites"),
        Rc::clone(&state),
    );
    refresh_playlists_ui(state);
}

pub fn refresh_recent_ui(state: Rc<RefCell<AppState>>) {
    let (recent_list, recent_empty_label, recent) = {
        let s = state.borrow();
        (
            s.recent_list.clone(),
            s.recent_empty_label.clone(),
            s.recent_tracks.clone(),
        )
    };

    recent_empty_label.set_visible(recent.is_empty());
    recent_list.set_visible(!recent.is_empty());
    populate_track_list(&recent_list, &recent, tr("Recientes", "Recent"), state);
}

pub fn refresh_playlists_ui(state: Rc<RefCell<AppState>>) {
    let (playlists_box, playlists_empty_label, mut playlists) = {
        let s = state.borrow();
        let mut all = Vec::with_capacity(s.user_playlists.len() + 1);
        all.push(favorites_as_playlist(&s));
        all.extend(s.user_playlists.clone());
        (
            s.playlists_box.clone(),
            s.playlists_empty_label.clone(),
            all,
        )
    };

    clear_box(&playlists_box);
    playlists_empty_label.set_visible(false);
    playlists_box.set_visible(true);

    let flow = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .homogeneous(true)
        .min_children_per_line(1)
        .max_children_per_line(3)
        .row_spacing(12)
        .column_spacing(12)
        .build();

    for (idx, pl) in playlists.drain(..).enumerate() {
        let is_favorites = pl.id == FAVORITES_PLAYLIST_ID;
        let art = build_playlist_art(&pl, 56, idx, Rc::clone(&state));

        let title = Label::builder()
            .label(&pl.name)
            .xalign(0.0)
            .max_width_chars(20)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["heading"])
            .build();
        let text_box = GtkBox::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(Align::Center);
        text_box.append(&title);

        if !pl.description.trim().is_empty() {
            let desc = Label::builder()
                .label(pl.description.trim())
                .xalign(0.0)
                .max_width_chars(22)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .css_classes(["dim-label", "caption"])
                .build();
            text_box.append(&desc);
        }

        let stats_label = Label::builder()
            .label(&format_playlist_stats(&pl))
            .xalign(0.0)
            .max_width_chars(22)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(["dim-label", "caption", "numeric"])
            .build();
        text_box.append(&stats_label);

        let info_row = GtkBox::new(Orientation::Horizontal, 12);
        info_row.set_hexpand(true);
        info_row.set_halign(Align::Fill);
        info_row.append(&art);
        info_row.append(&text_box);

        let open_btn = Button::builder()
            .child(&info_row)
            .hexpand(true)
            .tooltip_text(tr("Abrir y editar playlist", "Open and manage playlist"))
            .css_classes(["flat", "playlist-open-btn"])
            .build();
        open_btn.set_cursor_from_name(Some("pointer"));
        {
            let state = Rc::clone(&state);
            let pl_id = pl.id.clone();
            open_btn.connect_clicked(move |btn| {
                open_playlist_detail_dialog(btn, Rc::clone(&state), pl_id.clone(), idx);
            });
        }

        let play_btn = Button::builder()
            .icon_name("play-filled")
            .tooltip_text(tr("Reproducir playlist", "Play playlist"))
            .valign(Align::Center)
            .sensitive(!pl.tracks.is_empty())
            .css_classes(["flat", "circular"])
            .build();
        play_btn.set_cursor_from_name(Some("pointer"));
        {
            let state = Rc::clone(&state);
            let tracks = pl.tracks.clone();
            let name = pl.name.clone();
            play_btn.connect_clicked(move |_| {
                if tracks.is_empty() {
                    return;
                }
                {
                    let mut s = state.borrow_mut();
                    s.replace_queue(tracks.clone(), name.clone());
                }
                play_queue_index(Rc::clone(&state), 0);
            });
        }

        let edit_btn = Button::builder()
            .icon_name("document-edit-symbolic")
            .tooltip_text(tr("Personalizar playlist", "Customize playlist"))
            .valign(Align::Center)
            .css_classes(["flat", "circular"])
            .build();
        edit_btn.set_cursor_from_name(Some("pointer"));
        {
            let state = Rc::clone(&state);
            let pl_id = pl.id.clone();
            edit_btn.connect_clicked(move |btn| {
                open_edit_playlist_dialog(btn, Rc::clone(&state), pl_id.clone(), None);
            });
        }

        let del_btn = Button::builder()
            .icon_name("delete-regular")
            .tooltip_text(tr("Eliminar playlist", "Delete playlist"))
            .valign(Align::Center)
            .css_classes(["flat", "circular"])
            .build();
        del_btn.set_cursor_from_name(Some("pointer"));
        {
            let state = Rc::clone(&state);
            let pl_id = pl.id.clone();
            let pl_name = pl.name.clone();
            del_btn.connect_clicked(move |btn| {
                confirm_delete_playlist(btn, Rc::clone(&state), pl_id.clone(), pl_name.clone(), None);
            });
        }

        let actions = GtkBox::new(Orientation::Horizontal, 2);
        actions.set_valign(Align::Center);
        actions.append(&play_btn);
        if !is_favorites {
            actions.append(&edit_btn);
            actions.append(&del_btn);
        }

        let card_row = GtkBox::new(Orientation::Horizontal, 6);
        card_row.add_css_class("playlist-card");
        card_row.append(&open_btn);
        card_row.append(&actions);
        flow.insert(&card_row, -1);
    }

    playlists_box.append(&flow);
}

fn confirm_delete_playlist(
    anchor: &impl IsA<gtk4::Widget>,
    state: Rc<RefCell<AppState>>,
    playlist_id: String,
    playlist_name: String,
    on_deleted: Option<Rc<dyn Fn()>>,
) {
    let alert = adw::AlertDialog::builder()
        .heading(tr("¿Eliminar playlist?", "Delete playlist?"))
        .body(&format!(
            "{} «{playlist_name}». {}",
            tr("Se eliminará la playlist local", "The local playlist will be deleted:"),
            tr("Esta acción no se puede deshacer.", "This action cannot be undone.")
        ))
        .default_response("cancel")
        .close_response("cancel")
        .build();
    alert.add_response("cancel", tr("Cancelar", "Cancel"));
    alert.add_response("delete", tr("Eliminar", "Delete"));
    alert.set_response_appearance("delete", adw::ResponseAppearance::Destructive);

    alert.connect_response(None, move |_, resp| {
        if resp == "delete" {
            {
                let mut s = state.borrow_mut();
                s.user_playlists.retain(|p| p.id != playlist_id);
                s.report_save_result(save_user_playlists(&s.user_playlists));
                s.show_toast(&format!(
                    "{}: {playlist_name}",
                    tr("Playlist eliminada", "Playlist deleted")
                ));
            }
            refresh_playlists_ui(Rc::clone(&state));
            if let Some(cb) = &on_deleted {
                cb();
            }
        }
    });
    alert.present(Some(anchor));
}

pub fn open_create_playlist_dialog(
    anchor: &impl IsA<gtk4::Widget>,
    state: Rc<RefCell<AppState>>,
    suggested_name: Option<String>,
    initial_tracks: Vec<Track>,
) {
    open_playlist_editor_dialog(
        anchor,
        state,
        None,
        suggested_name.unwrap_or_default(),
        String::new(),
        None,
        None,
        None,
        initial_tracks,
        None,
    );
}

fn open_edit_playlist_dialog(
    anchor: &impl IsA<gtk4::Widget>,
    state: Rc<RefCell<AppState>>,
    playlist_id: String,
    on_saved: Option<Rc<dyn Fn()>>,
) {
    let existing = {
        let s = state.borrow();
        s.user_playlists
            .iter()
            .find(|p| p.id == playlist_id)
            .cloned()
    };
    let Some(pl) = existing else {
        return;
    };
    open_playlist_editor_dialog(
        anchor,
        state,
        Some(pl.id),
        pl.name,
        pl.description,
        pl.color_hex,
        pl.icon_name,
        pl.cover_path,
        pl.tracks,
        on_saved,
    );
}

#[allow(clippy::too_many_arguments)]
fn open_playlist_editor_dialog(
    anchor: &impl IsA<gtk4::Widget>,
    state: Rc<RefCell<AppState>>,
    editing_id: Option<String>,
    initial_name: String,
    initial_description: String,
    initial_color: Option<String>,
    initial_icon: Option<String>,
    initial_cover_path: Option<String>,
    initial_tracks: Vec<Track>,
    on_saved: Option<Rc<dyn Fn()>>,
) {
    let is_editing = editing_id.is_some();
    let title = if is_editing {
        tr("Personalizar playlist", "Customize playlist")
    } else {
        tr("Nueva playlist", "New playlist")
    };

    let selected_color: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(initial_color));
    let selected_icon: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(initial_icon));
    let selected_cover: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(initial_cover_path));
    let preview_tracks = Rc::new(initial_tracks);

    let content = GtkBox::new(Orientation::Vertical, 12);
    content.set_margin_top(8);
    content.set_margin_bottom(16);
    content.set_margin_start(18);
    content.set_margin_end(18);

    // Live artwork preview container
    let preview_holder = GtkBox::new(Orientation::Vertical, 0);
    preview_holder.set_halign(Align::Center);

    let clear_img_btn = Button::builder()
        .label(tr("Quitar imagen", "Remove image"))
        .css_classes(["flat", "caption"])
        .visible(selected_cover.borrow().is_some())
        .build();
    clear_img_btn.set_cursor_from_name(Some("pointer"));

    let refresh_preview: Rc<dyn Fn()> = {
        let holder = preview_holder.clone();
        let col = Rc::clone(&selected_color);
        let ico = Rc::clone(&selected_icon);
        let cov = Rc::clone(&selected_cover);
        let trks = Rc::clone(&preview_tracks);
        let st = Rc::clone(&state);
        let clear_btn = clear_img_btn.clone();
        Rc::new(move || {
            clear_box(&holder);
            clear_btn.set_visible(cov.borrow().is_some());
            let dummy = UserPlaylist {
                id: String::new(),
                name: String::new(),
                description: String::new(),
                color_hex: col.borrow().clone(),
                icon_name: ico.borrow().clone(),
                cover_path: cov.borrow().clone(),
                tracks: (*trks).clone(),
            };
            holder.append(&build_playlist_art(&dummy, 96, 0, Rc::clone(&st)));
        })
    };
    refresh_preview();

    let pick_img_content = GtkBox::new(Orientation::Horizontal, 6);
    pick_img_content.append(&Image::from_icon_name("image-x-generic-symbolic"));
    pick_img_content.append(&Label::new(Some(tr("Elegir portada…", "Choose cover…"))));
    let pick_img_btn = Button::builder()
        .child(&pick_img_content)
        .css_classes(["pill"])
        .build();
    pick_img_btn.set_cursor_from_name(Some("pointer"));

    let img_btns = GtkBox::new(Orientation::Horizontal, 8);
    img_btns.set_halign(Align::Center);
    img_btns.append(&pick_img_btn);
    img_btns.append(&clear_img_btn);

    content.append(&preview_holder);
    content.append(&img_btns);

    // Name & description fields
    let details_group = adw::PreferencesGroup::new();
    let name_row = adw::EntryRow::builder()
        .title(tr("Nombre", "Name"))
        .text(&initial_name)
        .build();
    let desc_row = adw::EntryRow::builder()
        .title(tr("Descripción (opcional)", "Description (optional)"))
        .text(&initial_description)
        .build();
    details_group.add(&name_row);
    details_group.add(&desc_row);
    content.append(&details_group);

    let name_error_label = Label::builder()
        .label(tr(
            "Escribe un nombre para la playlist.",
            "Please enter a name for the playlist.",
        ))
        .xalign(0.0)
        .visible(false)
        .css_classes(["error", "caption"])
        .build();
    content.append(&name_error_label);
    {
        let err_lbl = name_error_label.clone();
        name_row.connect_changed(move |row| {
            if !row.text().trim().is_empty() {
                row.remove_css_class("error");
                err_lbl.set_visible(false);
            }
        });
    }

    // Color palette picker
    let color_label = Label::builder()
        .label(tr("Color de portada", "Cover color"))
        .xalign(0.0)
        .css_classes(["heading", "caption"])
        .build();
    let color_row = GtkBox::new(Orientation::Horizontal, 8);
    color_row.set_halign(Align::Start);

    let swatch_buttons: Rc<RefCell<Vec<(Option<String>, Button)>>> =
        Rc::new(RefCell::new(Vec::new()));

    let update_swatch_active: Rc<dyn Fn()> = {
        let buttons = Rc::clone(&swatch_buttons);
        let col = Rc::clone(&selected_color);
        Rc::new(move || {
            let current = col.borrow().clone();
            for (val, btn) in buttons.borrow().iter() {
                if *val == current {
                    btn.add_css_class("active");
                } else {
                    btn.remove_css_class("active");
                }
            }
        })
    };

    let auto_color_btn = Button::builder()
        .label("Auto")
        .tooltip_text(tr("Usar portada automática", "Use automatic cover"))
        .css_classes(["flat", "pill", "caption"])
        .build();
    auto_color_btn.set_cursor_from_name(Some("pointer"));
    {
        let col = Rc::clone(&selected_color);
        let ico = Rc::clone(&selected_icon);
        let prev = Rc::clone(&refresh_preview);
        let upd = Rc::clone(&update_swatch_active);
        auto_color_btn.connect_clicked(move |_| {
            *col.borrow_mut() = None;
            *ico.borrow_mut() = None;
            upd();
            prev();
        });
    }
    swatch_buttons.borrow_mut().push((None, auto_color_btn.clone()));
    color_row.append(&auto_color_btn);

    for &hex in PLAYLIST_COLORS {
        let btn = Button::builder()
            .width_request(28)
            .height_request(28)
            .valign(Align::Center)
            .halign(Align::Center)
            .css_classes(["playlist-color-swatch"])
            .build();
        btn.set_cursor_from_name(Some("pointer"));
        apply_inline_css(&btn, &format!("button {{ background-color: {hex}; }}"));
        let col = Rc::clone(&selected_color);
        let cov = Rc::clone(&selected_cover);
        let prev = Rc::clone(&refresh_preview);
        let upd = Rc::clone(&update_swatch_active);
        let hex_owned = hex.to_string();
        btn.connect_clicked(move |_| {
            *cov.borrow_mut() = None;
            *col.borrow_mut() = Some(hex_owned.clone());
            upd();
            prev();
        });
        swatch_buttons
            .borrow_mut()
            .push((Some(hex.to_string()), btn.clone()));
        color_row.append(&btn);
    }
    update_swatch_active();

    let color_section = GtkBox::new(Orientation::Vertical, 6);
    color_section.append(&color_label);
    color_section.append(&color_row);
    content.append(&color_section);

    // Icon picker
    let icon_label = Label::builder()
        .label(tr("Ícono", "Icon"))
        .xalign(0.0)
        .css_classes(["heading", "caption"])
        .build();
    let icon_row = GtkBox::new(Orientation::Horizontal, 8);
    icon_row.set_halign(Align::Start);

    let icon_buttons: Rc<RefCell<Vec<(String, Button)>>> = Rc::new(RefCell::new(Vec::new()));
    let update_icon_active: Rc<dyn Fn()> = {
        let buttons = Rc::clone(&icon_buttons);
        let ico = Rc::clone(&selected_icon);
        Rc::new(move || {
            let current = ico.borrow().clone();
            for (name, btn) in buttons.borrow().iter() {
                if current.as_deref() == Some(name.as_str()) {
                    btn.add_css_class("suggested-action");
                    btn.remove_css_class("flat");
                } else {
                    btn.remove_css_class("suggested-action");
                    btn.add_css_class("flat");
                }
            }
        })
    };

    for &icon_name in PLAYLIST_ICONS {
        let btn = Button::builder()
            .icon_name(icon_name)
            .css_classes(["flat", "circular"])
            .build();
        btn.set_cursor_from_name(Some("pointer"));
        let ico = Rc::clone(&selected_icon);
        let cov = Rc::clone(&selected_cover);
        let prev = Rc::clone(&refresh_preview);
        let upd = Rc::clone(&update_icon_active);
        let name_owned = icon_name.to_string();
        btn.connect_clicked(move |_| {
            *cov.borrow_mut() = None;
            *ico.borrow_mut() = Some(name_owned.clone());
            upd();
            prev();
        });
        icon_buttons
            .borrow_mut()
            .push((icon_name.to_string(), btn.clone()));
        icon_row.append(&btn);
    }
    update_icon_active();

    let icon_section = GtkBox::new(Orientation::Vertical, 6);
    icon_section.append(&icon_label);
    icon_section.append(&icon_row);
    content.append(&icon_section);

    // Save / Create CTA button
    let save_btn = Button::builder()
        .label(if is_editing {
            tr("Guardar cambios", "Save changes")
        } else {
            tr("Crear playlist", "Create playlist")
        })
        .halign(Align::End)
        .css_classes(["suggested-action", "pill"])
        .build();
    save_btn.set_cursor_from_name(Some("pointer"));
    content.append(&save_btn);

    let scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .propagate_natural_height(true)
        .child(&content)
        .build();

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    toolbar.set_content(Some(&scroller));

    let dialog = adw::Dialog::builder()
        .title(title)
        .content_width(440)
        .follows_content_size(true)
        .child(&toolbar)
        .build();

    // Wire image picker & clear
    {
        let cov = Rc::clone(&selected_cover);
        let prev = Rc::clone(&refresh_preview);
        pick_img_btn.connect_clicked(move |btn| {
            let filter = gtk4::FileFilter::new();
            filter.set_name(Some(tr("Imágenes", "Images")));
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            let filters = gio::ListStore::new::<gtk4::FileFilter>();
            filters.append(&filter);

            let fd = gtk4::FileDialog::builder()
                .title(tr("Seleccionar portada", "Select cover image"))
                .filters(&filters)
                .default_filter(&filter)
                .build();
            let root = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            let cov_cb = Rc::clone(&cov);
            let prev_cb = Rc::clone(&prev);
            fd.open(root.as_ref(), gio::Cancellable::NONE, move |res| {
                if let Ok(file) = res
                    && let Some(path) = file.path()
                {
                    *cov_cb.borrow_mut() = Some(path.to_string_lossy().into_owned());
                    prev_cb();
                }
            });
        });
    }
    {
        let cov = Rc::clone(&selected_cover);
        let prev = Rc::clone(&refresh_preview);
        clear_img_btn.connect_clicked(move |_| {
            *cov.borrow_mut() = None;
            prev();
        });
    }

    // Wire Save / Create action
    let do_save: Rc<dyn Fn()> = {
        let state = Rc::clone(&state);
        let dialog = dialog.clone();
        let name_row = name_row.clone();
        let name_error_label = name_error_label.clone();
        let desc_row = desc_row.clone();
        let col = Rc::clone(&selected_color);
        let ico = Rc::clone(&selected_icon);
        let cov = Rc::clone(&selected_cover);
        let tracks_rc = Rc::clone(&preview_tracks);
        Rc::new(move || {
            let name = name_row.text().trim().to_string();
            if name.is_empty() {
                name_row.add_css_class("error");
                name_error_label.set_visible(true);
                name_row.grab_focus();
                return;
            }
            name_row.remove_css_class("error");
            name_error_label.set_visible(false);
            let description = desc_row.text().trim().to_string();
            let color_hex = col.borrow().clone();
            let icon_name = ico.borrow().clone();
            let cover_path = cov.borrow().clone();

            {
                let mut s = state.borrow_mut();
                if let Some(ref edit_id) = editing_id {
                    if let Some(pl) = s.user_playlists.iter_mut().find(|p| &p.id == edit_id) {
                        pl.name = name.clone();
                        pl.description = description;
                        pl.color_hex = color_hex;
                        pl.icon_name = icon_name;
                        pl.cover_path = cover_path;
                    }
                    s.report_save_result(save_user_playlists(&s.user_playlists));
                    s.show_toast(&format!(
                        "{}: {name}",
                        tr("Playlist actualizada", "Playlist updated")
                    ));
                } else {
                    let mut pl = UserPlaylist::new_local(name.clone(), (*tracks_rc).clone());
                    pl.description = description;
                    pl.color_hex = color_hex;
                    pl.icon_name = icon_name;
                    pl.cover_path = cover_path;
                    s.user_playlists.push(pl);
                    s.report_save_result(save_user_playlists(&s.user_playlists));
                    s.show_toast(&format!(
                        "{}: {name}",
                        tr("Playlist creada", "Playlist created")
                    ));
                }
            }
            refresh_playlists_ui(Rc::clone(&state));
            if let Some(cb) = &on_saved {
                cb();
            }
            dialog.close();
        })
    };

    {
        let save = Rc::clone(&do_save);
        save_btn.connect_clicked(move |_| save());
    }
    {
        let save = Rc::clone(&do_save);
        name_row.connect_entry_activated(move |_| save());
    }

    dialog.present(Some(anchor));
    name_row.grab_focus();
}

fn open_playlist_detail_dialog(
    anchor: &impl IsA<gtk4::Widget>,
    state: Rc<RefCell<AppState>>,
    playlist_id: String,
    fallback_idx: usize,
) {
    let body = GtkBox::new(Orientation::Vertical, 16);
    body.set_margin_top(14);
    body.set_margin_bottom(24);
    body.set_margin_start(20);
    body.set_margin_end(20);

    let hero_holder = GtkBox::new(Orientation::Vertical, 0);
    body.append(&hero_holder);

    // Quick add songs search section
    let add_search_entry = SearchEntry::builder()
        .placeholder_text(tr(
            "Buscar en YouTube Music para agregar canciones…",
            "Search YouTube Music to add songs…",
        ))
        .hexpand(true)
        .build();
    let add_search_spinner = Spinner::builder()
        .visible(false)
        .valign(Align::Center)
        .build();
    let add_search_bar = GtkBox::new(Orientation::Horizontal, 8);
    add_search_bar.append(&add_search_entry);
    add_search_bar.append(&add_search_spinner);

    let search_results_box = GtkBox::new(Orientation::Vertical, 6);
    search_results_box.add_css_class("playlist-search-results");
    search_results_box.set_visible(false);

    body.append(&add_search_bar);
    body.append(&search_results_box);

    // Filter & sort bar for existing tracks
    let filter_entry = SearchEntry::builder()
        .placeholder_text(tr("Filtrar pistas en esta playlist…", "Filter tracks in this playlist…"))
        .hexpand(true)
        .build();

    let sort_popover = Popover::new();
    let sort_btn_content = GtkBox::new(Orientation::Horizontal, 6);
    sort_btn_content.append(&Image::from_icon_name("view-sort-ascending-symbolic"));
    sort_btn_content.append(&Label::new(Some(tr("Ordenar", "Sort"))));
    let sort_btn = MenuButton::builder()
        .child(&sort_btn_content)
        .popover(&sort_popover)
        .css_classes(["flat", "pill"])
        .build();
    sort_btn.set_cursor_from_name(Some("pointer"));

    let tools_row = GtkBox::new(Orientation::Horizontal, 8);
    tools_row.append(&filter_entry);
    tools_row.append(&sort_btn);
    body.append(&tools_row);

    let tracks_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    let empty_holder = GtkBox::new(Orientation::Vertical, 0);
    body.append(&empty_holder);
    body.append(&tracks_list);

    let scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&body)
        .build();

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    toolbar.set_content(Some(&scroller));

    let dialog = adw::Dialog::builder()
        .title(tr("Playlist", "Playlist"))
        .content_width(760)
        .content_height(620)
        .child(&toolbar)
        .build();

    let filter_text: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let rebuild_slot: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));

    let rebuild: Rc<dyn Fn()> = {
        let state = Rc::clone(&state);
        let playlist_id = playlist_id.clone();
        let hero_holder = hero_holder.clone();
        let tracks_list = tracks_list.clone();
        let empty_holder = empty_holder.clone();
        let tools_row = tools_row.clone();
        let dialog = dialog.clone();
        let filter_text = Rc::clone(&filter_text);
        let rebuild_slot = Rc::clone(&rebuild_slot);
        Rc::new(move || {
            let is_favorites = playlist_id == FAVORITES_PLAYLIST_ID;
            let pl_opt = {
                let s = state.borrow();
                if is_favorites {
                    Some(favorites_as_playlist(&s))
                } else {
                    s.user_playlists
                        .iter()
                        .find(|p| p.id == playlist_id)
                        .cloned()
                }
            };
            let Some(pl) = pl_opt else {
                dialog.close();
                return;
            };

            dialog.set_title(&pl.name);
            clear_box(&hero_holder);

            let accent = pl
                .color_hex
                .as_deref()
                .unwrap_or(PLAYLIST_COLORS[fallback_idx % PLAYLIST_COLORS.len()]);
            let hero = GtkBox::new(Orientation::Horizontal, 18);
            hero.add_css_class("playlist-detail-hero");
            apply_inline_css(
                &hero,
                &format!(
                    "box {{ background-image: linear-gradient(135deg, alpha({accent}, 0.28) 0%, alpha(@window_fg_color, 0.04) 75%); }}"
                ),
            );

            let art = build_playlist_art(&pl, 118, fallback_idx, Rc::clone(&state));
            let art_widget: gtk4::Widget = if is_favorites {
                art
            } else {
                let art_btn = Button::builder()
                    .child(&art)
                    .tooltip_text(tr("Cambiar portada o estilo", "Change cover or style"))
                    .css_classes(["flat"])
                    .build();
                art_btn.set_cursor_from_name(Some("pointer"));
                {
                    let st = Rc::clone(&state);
                    let pid = pl.id.clone();
                    let slot = Rc::clone(&rebuild_slot);
                    art_btn.connect_clicked(move |btn| {
                        let cb = slot.borrow().clone();
                        open_edit_playlist_dialog(btn, Rc::clone(&st), pid.clone(), cb);
                    });
                }
                art_btn.upcast()
            };

            let meta_col = GtkBox::new(Orientation::Vertical, 4);
            meta_col.set_hexpand(true);
            meta_col.set_valign(Align::Center);
            meta_col.append(
                &Label::builder()
                    .label(if is_favorites {
                        tr("PLAYLIST • FAVORITOS", "PLAYLIST • FAVORITES")
                    } else {
                        tr("PLAYLIST LOCAL", "LOCAL PLAYLIST")
                    })
                    .xalign(0.0)
                    .css_classes(["album-type-label"])
                    .build(),
            );
            meta_col.append(
                &Label::builder()
                    .label(&pl.name)
                    .xalign(0.0)
                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                    .css_classes(["title-1"])
                    .build(),
            );
            if !pl.description.trim().is_empty() {
                meta_col.append(
                    &Label::builder()
                        .label(pl.description.trim())
                        .xalign(0.0)
                        .wrap(true)
                        .css_classes(["dim-label"])
                        .build(),
                );
            }
            meta_col.append(
                &Label::builder()
                    .label(&format_playlist_stats(&pl))
                    .xalign(0.0)
                    .css_classes(["dim-label", "caption", "numeric"])
                    .build(),
            );

            let action_row = GtkBox::new(Orientation::Horizontal, 8);
            action_row.set_margin_top(8);

            let play_content = GtkBox::new(Orientation::Horizontal, 6);
            play_content.append(&Image::from_icon_name("play-filled"));
            play_content.append(&Label::new(Some(tr("Reproducir", "Play"))));
            let play_btn = Button::builder()
                .child(&play_content)
                .sensitive(!pl.tracks.is_empty())
                .css_classes(["suggested-action", "pill"])
                .build();
            play_btn.set_cursor_from_name(Some("pointer"));
            {
                let st = Rc::clone(&state);
                let trks = pl.tracks.clone();
                let name = pl.name.clone();
                play_btn.connect_clicked(move |_| {
                    if trks.is_empty() {
                        return;
                    }
                    {
                        let mut s = st.borrow_mut();
                        s.replace_queue(trks.clone(), name.clone());
                    }
                    play_queue_index(Rc::clone(&st), 0);
                });
            }

            let shuffle_btn = Button::builder()
                .icon_name("arrow-shuffle-regular")
                .tooltip_text(tr("Reproducción aleatoria", "Shuffle play"))
                .sensitive(!pl.tracks.is_empty())
                .css_classes(["circular", "album-action-btn"])
                .build();
            shuffle_btn.set_cursor_from_name(Some("pointer"));
            {
                let st = Rc::clone(&state);
                let trks = pl.tracks.clone();
                let name = pl.name.clone();
                shuffle_btn.connect_clicked(move |_| {
                    if trks.is_empty() {
                        return;
                    }
                    {
                        let mut s = st.borrow_mut();
                        s.set_shuffled_queue(trks.clone(), name.clone());
                    }
                    play_queue_index(Rc::clone(&st), 0);
                });
            }

            let enqueue_btn = Button::builder()
                .icon_name("add-regular")
                .tooltip_text(tr("Agregar todas a la cola", "Add all to queue"))
                .sensitive(!pl.tracks.is_empty())
                .css_classes(["circular", "album-action-btn"])
                .build();
            enqueue_btn.set_cursor_from_name(Some("pointer"));
            {
                let st = Rc::clone(&state);
                let trks = pl.tracks.clone();
                let name = pl.name.clone();
                enqueue_btn.connect_clicked(move |_| {
                    let should_start = {
                        let mut s = st.borrow_mut();
                        let was_empty = s.current_index.is_none() && s.queue.is_empty();
                        s.queue.extend(trks.clone());
                        s.update_nav_buttons();
                        s.show_toast(&format!(
                            "{}: {name}",
                            tr("Agregada a la cola", "Added to queue")
                        ));
                        was_empty
                    };
                    refresh_queue_ui(Rc::clone(&st));
                    if should_start {
                        play_queue_index(Rc::clone(&st), 0);
                    }
                });
            }

            let edit_btn = Button::builder()
                .icon_name("document-edit-symbolic")
                .tooltip_text(tr("Personalizar playlist", "Customize playlist"))
                .css_classes(["circular", "album-action-btn"])
                .build();
            edit_btn.set_cursor_from_name(Some("pointer"));
            {
                let st = Rc::clone(&state);
                let pid = pl.id.clone();
                let slot = Rc::clone(&rebuild_slot);
                edit_btn.connect_clicked(move |btn| {
                    let cb = slot.borrow().clone();
                    open_edit_playlist_dialog(btn, Rc::clone(&st), pid.clone(), cb);
                });
            }

            let dup_btn = Button::builder()
                .icon_name("edit-copy-symbolic")
                .tooltip_text(tr("Duplicar playlist", "Duplicate playlist"))
                .css_classes(["circular", "album-action-btn"])
                .build();
            dup_btn.set_cursor_from_name(Some("pointer"));
            {
                let st = Rc::clone(&state);
                let pl_clone = pl.clone();
                dup_btn.connect_clicked(move |_| {
                    {
                        let mut s = st.borrow_mut();
                        let copy_name = format!("{} ({})", pl_clone.name, tr("copia", "copy"));
                        let mut dup = UserPlaylist::new_local(copy_name.clone(), pl_clone.tracks.clone());
                        dup.description = pl_clone.description.clone();
                        dup.color_hex = pl_clone.color_hex.clone();
                        dup.icon_name = pl_clone.icon_name.clone();
                        dup.cover_path = pl_clone.cover_path.clone();
                        s.user_playlists.push(dup);
                        s.report_save_result(save_user_playlists(&s.user_playlists));
                        s.show_toast(&format!(
                            "{}: {copy_name}",
                            tr("Playlist duplicada", "Playlist duplicated")
                        ));
                    }
                    refresh_playlists_ui(Rc::clone(&st));
                });
            }

            let del_btn = Button::builder()
                .icon_name("delete-regular")
                .tooltip_text(tr("Eliminar playlist", "Delete playlist"))
                .css_classes(["circular", "album-action-btn"])
                .build();
            del_btn.set_cursor_from_name(Some("pointer"));
            {
                let st = Rc::clone(&state);
                let pid = pl.id.clone();
                let pname = pl.name.clone();
                let dlg = dialog.clone();
                del_btn.connect_clicked(move |btn| {
                    let dlg_close = dlg.clone();
                    confirm_delete_playlist(
                        btn,
                        Rc::clone(&st),
                        pid.clone(),
                        pname.clone(),
                        Some(Rc::new(move || {
                            let _ = dlg_close.close();
                        })),
                    );
                });
            }

            action_row.append(&play_btn);
            action_row.append(&shuffle_btn);
            action_row.append(&enqueue_btn);
            if !is_favorites {
                action_row.append(&edit_btn);
            }
            action_row.append(&dup_btn);
            if !is_favorites {
                action_row.append(&del_btn);
            }
            meta_col.append(&action_row);

            hero.append(&art_widget);
            hero.append(&meta_col);
            hero_holder.append(&hero);

            // Track list
            clear_list_box(&tracks_list);
            clear_box(&empty_holder);

            let has_tracks = !pl.tracks.is_empty();
            tools_row.set_visible(has_tracks);
            tracks_list.set_visible(has_tracks);

            if !has_tracks {
                empty_holder.append(&empty_state_card(
                    "music-note-2-regular",
                    tr("Esta playlist está vacía", "This playlist is empty"),
                    tr(
                        "Usa el buscador de arriba o el botón de playlist en cualquier canción para agregar pistas.",
                        "Use the search bar above or the playlist button on any song to add tracks.",
                    ),
                ));
                return;
            }

            let q = filter_text.borrow().trim().to_lowercase();
            let total_tracks = pl.tracks.len();
            let mut shown = 0usize;

            for (idx, track) in pl.tracks.iter().enumerate() {
                if !q.is_empty()
                    && !track.title.to_lowercase().contains(&q)
                    && !track.artist.to_lowercase().contains(&q)
                    && !track
                        .album
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&q)
                {
                    continue;
                }
                shown += 1;

                let row_box = GtkBox::new(Orientation::Horizontal, 10);
                row_box.add_css_class("track-row");

                let num_lbl = Label::builder()
                    .label(&format!("{}", idx + 1))
                    .width_chars(3)
                    .xalign(0.5)
                    .css_classes(["dim-label", "caption", "numeric"])
                    .build();

                let (cover_frame, cover) = clamped_cover(40, 40, "cover-art");
                if let Some(url) = &track.cover_url {
                    load_cover_into_picture(&cover, url, Rc::clone(&state));
                }

                let subtitle = match &track.album {
                    Some(alb) if !alb.is_empty() => format!("{} • {alb}", track.artist),
                    _ => track.artist.clone(),
                };
                let text_col = GtkBox::new(Orientation::Vertical, 1);
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
                        .label(&subtitle)
                        .xalign(0.0)
                        .ellipsize(gtk4::pango::EllipsizeMode::End)
                        .css_classes(["dim-label", "caption"])
                        .build(),
                );

                let dur_lbl = Label::builder()
                    .label(&track.duration_secs.map(format_duration).unwrap_or_default())
                    .valign(Align::Center)
                    .visible(track.duration_secs.is_some())
                    .css_classes(["dim-label", "caption", "numeric"])
                    .build();

                let up_btn = Button::builder()
                    .icon_name("go-up-symbolic")
                    .tooltip_text(tr("Subir", "Move up"))
                    .sensitive(q.is_empty() && idx > 0)
                    .valign(Align::Center)
                    .css_classes(["flat", "circular"])
                    .build();
                {
                    let st = Rc::clone(&state);
                    let pid = pl.id.clone();
                    let slot = Rc::clone(&rebuild_slot);
                    up_btn.connect_clicked(move |_| {
                        {
                            let mut s = st.borrow_mut();
                            if pid == FAVORITES_PLAYLIST_ID {
                                if idx > 0 && idx < s.favorites.len() {
                                    s.favorites.swap(idx - 1, idx);
                                    s.report_save_result(save_favorites(&s.favorites));
                                }
                            } else if let Some(target) =
                                s.user_playlists.iter_mut().find(|p| p.id == pid)
                                && idx > 0
                                && idx < target.tracks.len()
                            {
                                target.tracks.swap(idx - 1, idx);
                                s.report_save_result(save_user_playlists(&s.user_playlists));
                            }
                        }
                        refresh_playlists_ui(Rc::clone(&st));
                        if let Some(cb) = slot.borrow().as_ref() {
                            cb();
                        }
                    });
                }

                let down_btn = Button::builder()
                    .icon_name("go-down-symbolic")
                    .tooltip_text(tr("Bajar", "Move down"))
                    .sensitive(q.is_empty() && idx + 1 < total_tracks)
                    .valign(Align::Center)
                    .css_classes(["flat", "circular"])
                    .build();
                {
                    let st = Rc::clone(&state);
                    let pid = pl.id.clone();
                    let slot = Rc::clone(&rebuild_slot);
                    down_btn.connect_clicked(move |_| {
                        {
                            let mut s = st.borrow_mut();
                            if pid == FAVORITES_PLAYLIST_ID {
                                if idx + 1 < s.favorites.len() {
                                    s.favorites.swap(idx, idx + 1);
                                    s.report_save_result(save_favorites(&s.favorites));
                                }
                            } else if let Some(target) =
                                s.user_playlists.iter_mut().find(|p| p.id == pid)
                                && idx + 1 < target.tracks.len()
                            {
                                target.tracks.swap(idx, idx + 1);
                                s.report_save_result(save_user_playlists(&s.user_playlists));
                            }
                        }
                        refresh_playlists_ui(Rc::clone(&st));
                        if let Some(cb) = slot.borrow().as_ref() {
                            cb();
                        }
                    });
                }

                let play_row_btn = Button::builder()
                    .icon_name("play-filled")
                    .tooltip_text(tr("Reproducir desde aquí", "Play from here"))
                    .valign(Align::Center)
                    .css_classes(["flat", "circular"])
                    .build();
                {
                    let st = Rc::clone(&state);
                    let trks = pl.tracks.clone();
                    let name = pl.name.clone();
                    play_row_btn.connect_clicked(move |_| {
                        {
                            let mut s = st.borrow_mut();
                            s.replace_queue(trks.clone(), name.clone());
                        }
                        play_queue_index(Rc::clone(&st), idx);
                    });
                }

                let remove_btn = Button::builder()
                    .icon_name("window-close-symbolic")
                    .tooltip_text(tr("Quitar de la playlist", "Remove from playlist"))
                    .valign(Align::Center)
                    .css_classes(["flat", "circular"])
                    .build();
                {
                    let st = Rc::clone(&state);
                    let pid = pl.id.clone();
                    let slot = Rc::clone(&rebuild_slot);
                    remove_btn.connect_clicked(move |_| {
                        {
                            let mut s = st.borrow_mut();
                            if pid == FAVORITES_PLAYLIST_ID {
                                if idx < s.favorites.len() {
                                    let removed = s.favorites.remove(idx);
                                    if s.current_track().is_some_and(|cur| cur.id == removed.id) {
                                        s.player_fav_btn.set_icon_name("heart-regular");
                                        if s.player_fav_btn.is_active() {
                                            s.player_fav_btn.set_active(false);
                                        }
                                    }
                                    s.report_save_result(save_favorites(&s.favorites));
                                }
                            } else if let Some(target) =
                                s.user_playlists.iter_mut().find(|p| p.id == pid)
                                && idx < target.tracks.len()
                            {
                                target.tracks.remove(idx);
                                s.report_save_result(save_user_playlists(&s.user_playlists));
                            }
                        }
                        refresh_playlists_ui(Rc::clone(&st));
                        if let Some(cb) = slot.borrow().as_ref() {
                            cb();
                        }
                    });
                }

                row_box.append(&num_lbl);
                row_box.append(&cover_frame);
                row_box.append(&text_col);
                row_box.append(&dur_lbl);
                row_box.append(&up_btn);
                row_box.append(&down_btn);
                row_box.append(&play_row_btn);
                row_box.append(&remove_btn);

                let row = ListBoxRow::builder().child(&row_box).build();
                tracks_list.append(&row);
            }

            if shown == 0 {
                tracks_list.set_visible(false);
                empty_holder.append(&empty_state_card(
                    "search-regular",
                    tr("Sin coincidencias", "No matches"),
                    tr(
                        "Ninguna canción coincide con tu filtro.",
                        "No songs match your filter.",
                    ),
                ));
            }
        })
    };

    *rebuild_slot.borrow_mut() = Some(Rc::clone(&rebuild));

    // Populate sort menu popover
    {
        let sort_box = GtkBox::new(Orientation::Vertical, 4);
        sort_box.set_margin_top(6);
        sort_box.set_margin_bottom(6);
        sort_box.set_margin_start(6);
        sort_box.set_margin_end(6);

        let sort_options: [(&str, fn(&mut Vec<Track>)); 4] = [
            (tr("Título (A–Z)", "Title (A–Z)"), |v| {
                v.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            }),
            (tr("Artista (A–Z)", "Artist (A–Z)"), |v| {
                v.sort_by(|a, b| a.artist.to_lowercase().cmp(&b.artist.to_lowercase()));
            }),
            (tr("Duración (menor a mayor)", "Duration (shortest first)"), |v| {
                v.sort_by_key(|t| t.duration_secs.unwrap_or(u32::MAX));
            }),
            (tr("Invertir orden", "Reverse order"), |v| {
                v.reverse();
            }),
        ];

        for (label, sorter) in sort_options {
            let item = Button::builder()
                .label(label)
                .css_classes(["flat"])
                .build();
            if let Some(lbl) = item.child().and_downcast::<Label>() {
                lbl.set_xalign(0.0);
            }
            let st = Rc::clone(&state);
            let pid = playlist_id.clone();
            let pop = sort_popover.clone();
            let rb = Rc::clone(&rebuild);
            item.connect_clicked(move |_| {
                {
                    let mut s = st.borrow_mut();
                    if pid == FAVORITES_PLAYLIST_ID {
                        sorter(&mut s.favorites);
                        s.report_save_result(save_favorites(&s.favorites));
                    } else if let Some(pl) = s.user_playlists.iter_mut().find(|p| p.id == pid) {
                        sorter(&mut pl.tracks);
                        s.report_save_result(save_user_playlists(&s.user_playlists));
                    }
                }
                pop.popdown();
                refresh_playlists_ui(Rc::clone(&st));
                rb();
            });
            sort_box.append(&item);
        }
        sort_popover.set_child(Some(&sort_box));
    }

    // Wire filter entry
    {
        let ft = Rc::clone(&filter_text);
        let rb = Rc::clone(&rebuild);
        filter_entry.connect_search_changed(move |entry| {
            *ft.borrow_mut() = entry.text().to_string();
            rb();
        });
    }

    // Wire inline YouTube Music search to add tracks directly to this playlist
    {
        let st = Rc::clone(&state);
        let pid = playlist_id.clone();
        let results_box = search_results_box.clone();
        let spinner = add_search_spinner.clone();
        let rb = Rc::clone(&rebuild);
        add_search_entry.connect_activate(move |entry| {
            let query = entry.text().trim().to_string();
            if query.is_empty() {
                results_box.set_visible(false);
                return;
            }
            spinner.set_visible(true);
            spinner.set_spinning(true);
            let backend = st.borrow().backend.clone();
            let (tx, rx) = async_channel::bounded(1);
            runtime().spawn(async move {
                let res = backend.search_all(&query).await;
                let _ = tx.send(res).await;
            });

            let st_cb = Rc::clone(&st);
            let pid_cb = pid.clone();
            let results_cb = results_box.clone();
            let spinner_cb = spinner.clone();
            let rb_cb = Rc::clone(&rb);
            glib::spawn_future_local(async move {
                let recv = rx.recv().await;
                spinner_cb.set_spinning(false);
                spinner_cb.set_visible(false);
                clear_box(&results_cb);

                let header = GtkBox::new(Orientation::Horizontal, 8);
                header.append(
                    &Label::builder()
                        .label(tr("Resultados para agregar", "Results to add"))
                        .xalign(0.0)
                        .hexpand(true)
                        .css_classes(["heading", "caption"])
                        .build(),
                );
                let close_res_btn = Button::builder()
                    .icon_name("window-close-symbolic")
                    .tooltip_text(tr("Cerrar resultados", "Close results"))
                    .css_classes(["flat", "circular"])
                    .build();
                {
                    let res_hide = results_cb.clone();
                    close_res_btn.connect_clicked(move |_| res_hide.set_visible(false));
                }
                header.append(&close_res_btn);
                results_cb.append(&header);

                match recv {
                    Ok(Ok(search)) if !search.tracks.is_empty() => {
                        for track in search.tracks.into_iter().take(6) {
                            let row = GtkBox::new(Orientation::Horizontal, 10);
                            let (cov_frame, cov_pic) = clamped_cover(36, 36, "cover-art");
                            if let Some(url) = &track.cover_url {
                                load_cover_into_picture(&cov_pic, url, Rc::clone(&st_cb));
                            }
                            let info = GtkBox::new(Orientation::Vertical, 1);
                            info.set_hexpand(true);
                            info.set_valign(Align::Center);
                            info.append(
                                &Label::builder()
                                    .label(&track.title)
                                    .xalign(0.0)
                                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                                    .css_classes(["heading"])
                                    .build(),
                            );
                            info.append(
                                &Label::builder()
                                    .label(&track.artist)
                                    .xalign(0.0)
                                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                                    .css_classes(["dim-label", "caption"])
                                    .build(),
                            );

                            let already_in = {
                                let s = st_cb.borrow();
                                if pid_cb == FAVORITES_PLAYLIST_ID {
                                    s.favorites.iter().any(|t| t.id == track.id)
                                } else {
                                    s.user_playlists
                                        .iter()
                                        .find(|p| p.id == pid_cb)
                                        .is_some_and(|p| p.tracks.iter().any(|t| t.id == track.id))
                                }
                            };
                            let add_btn = Button::builder()
                                .label(if already_in {
                                    tr("Agregada", "Added")
                                } else {
                                    tr("Agregar", "Add")
                                })
                                .sensitive(!already_in)
                                .valign(Align::Center)
                                .css_classes(["pill"])
                                .build();
                            add_btn.set_cursor_from_name(Some("pointer"));
                            {
                                let st_add = Rc::clone(&st_cb);
                                let pid_add = pid_cb.clone();
                                let trk_add = track.clone();
                                let rb_add = Rc::clone(&rb_cb);
                                add_btn.connect_clicked(move |btn| {
                                    {
                                        let mut s = st_add.borrow_mut();
                                        if pid_add == FAVORITES_PLAYLIST_ID {
                                            if !s.favorites.iter().any(|t| t.id == trk_add.id) {
                                                s.favorites.push(trk_add.clone());
                                                if s.current_track()
                                                    .is_some_and(|cur| cur.id == trk_add.id)
                                                {
                                                    s.player_fav_btn.set_icon_name("heart-filled");
                                                    if !s.player_fav_btn.is_active() {
                                                        s.player_fav_btn.set_active(true);
                                                    }
                                                }
                                                s.report_save_result(save_favorites(&s.favorites));
                                            }
                                        } else if let Some(pl) =
                                            s.user_playlists.iter_mut().find(|p| p.id == pid_add)
                                            && !pl.tracks.iter().any(|t| t.id == trk_add.id)
                                        {
                                            pl.tracks.push(trk_add.clone());
                                            s.report_save_result(save_user_playlists(
                                                &s.user_playlists,
                                            ));
                                        }
                                    }
                                    btn.set_label(tr("Agregada", "Added"));
                                    btn.set_sensitive(false);
                                    refresh_playlists_ui(Rc::clone(&st_add));
                                    rb_add();
                                });
                            }

                            row.append(&cov_frame);
                            row.append(&info);
                            row.append(&add_btn);
                            results_cb.append(&row);
                        }
                    }
                    _ => {
                        results_cb.append(
                            &Label::builder()
                                .label(tr("No se encontraron canciones.", "No songs found."))
                                .xalign(0.0)
                                .css_classes(["dim-label", "caption"])
                                .build(),
                        );
                    }
                }
                results_cb.set_visible(true);
            });
        });
    }

    rebuild();
    dialog.present(Some(anchor));
}
