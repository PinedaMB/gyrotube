use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Entry, Label, ListBox, Orientation, PolicyType, ScrolledWindow,
    SelectionMode,
};
use libadwaita as adw;

use crate::models::{UserPlaylist, save_user_playlists};
use crate::ui::player::play_queue_index;
use crate::ui::state::AppState;
use crate::ui::widgets::{clear_box, populate_track_list};

pub fn build_collection_page(state: Rc<RefCell<AppState>>) -> ScrolledWindow {
    let container = GtkBox::new(Orientation::Vertical, 20);
    container.set_margin_top(18);
    container.set_margin_bottom(28);
    container.set_margin_start(20);
    container.set_margin_end(20);

    // 1. Custom local playlists
    let pl_header = GtkBox::new(Orientation::Horizontal, 8);
    pl_header.append(
        &Label::builder()
            .label("Playlists locales")
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["title-2"])
            .build(),
    );

    let new_pl_entry = Entry::builder()
        .placeholder_text("Nueva playlist...")
        .width_chars(18)
        .build();
    let new_pl_btn = Button::builder()
        .icon_name("list-add-symbolic")
        .label("Crear")
        .css_classes(["suggested-action"])
        .build();
    pl_header.append(&new_pl_entry);
    pl_header.append(&new_pl_btn);

    {
        let state = Rc::clone(&state);
        let entry = new_pl_entry.clone();
        let create_pl = move || {
            let name = entry.text().trim().to_string();
            if name.is_empty() {
                return;
            }
            {
                let mut s = state.borrow_mut();
                let id = format!(
                    "local-{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis())
                        .unwrap_or(0)
                );
                s.user_playlists.push(UserPlaylist {
                    id,
                    name: name.clone(),
                    tracks: Vec::new(),
                });
                save_user_playlists(&s.user_playlists);
                s.show_toast(&format!("Playlist creada: {name}"));
            }
            entry.set_text("");
            refresh_playlists_ui(Rc::clone(&state));
        };
        let create_pl_clone = create_pl.clone();
        new_pl_btn.connect_clicked(move |_| create_pl_clone());
        new_pl_entry.connect_activate(move |_| create_pl());
    }

    let (playlists_box, playlists_empty_label) = {
        let s = state.borrow();
        (s.playlists_box.clone(), s.playlists_empty_label.clone())
    };
    container.append(&pl_header);
    container.append(&playlists_empty_label);
    container.append(&playlists_box);

    // 2. Recently played
    container.append(
        &Label::builder()
            .label("Escuchado recientemente")
            .xalign(0.0)
            .css_classes(["title-2"])
            .build(),
    );
    let (recent_list, recent_empty_label) = {
        let s = state.borrow();
        (s.recent_list.clone(), s.recent_empty_label.clone())
    };
    container.append(&recent_empty_label);
    container.append(&recent_list);

    // 3. Favorites
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
    populate_track_list(&favorites_list, &favorites, "Favoritos", state);
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
    populate_track_list(&recent_list, &recent, "Recientes", state);
}

pub fn refresh_playlists_ui(state: Rc<RefCell<AppState>>) {
    let (playlists_box, playlists_empty_label, playlists) = {
        let s = state.borrow();
        (
            s.playlists_box.clone(),
            s.playlists_empty_label.clone(),
            s.user_playlists.clone(),
        )
    };

    clear_box(&playlists_box);
    playlists_empty_label.set_visible(playlists.is_empty());
    playlists_box.set_visible(!playlists.is_empty());

    let list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    for pl in playlists {
        let row_box = GtkBox::new(Orientation::Horizontal, 12);
        row_box.add_css_class("track-row");

        let title = Label::builder()
            .label(&pl.name)
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["heading"])
            .build();
        let count_label = Label::builder()
            .label(&format!("{} pistas", pl.tracks.len()))
            .valign(Align::Center)
            .css_classes(["dim-label", "caption", "numeric"])
            .build();

        let play_btn = Button::builder()
            .icon_name("media-playback-start-symbolic")
            .tooltip_text("Reproducir playlist")
            .valign(Align::Center)
            .sensitive(!pl.tracks.is_empty())
            .css_classes(["flat", "circular"])
            .build();
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
                    s.queue = tracks.clone();
                    s.source_name = name.clone();
                }
                play_queue_index(Rc::clone(&state), 0);
            });
        }

        let del_btn = Button::builder()
            .icon_name("user-trash-symbolic")
            .tooltip_text("Eliminar playlist")
            .valign(Align::Center)
            .css_classes(["flat", "circular"])
            .build();
        {
            let state = Rc::clone(&state);
            let pl_id = pl.id.clone();
            del_btn.connect_clicked(move |_| {
                {
                    let mut s = state.borrow_mut();
                    s.user_playlists.retain(|p| p.id != pl_id);
                    save_user_playlists(&s.user_playlists);
                }
                refresh_playlists_ui(Rc::clone(&state));
            });
        }

        row_box.append(&title);
        row_box.append(&count_label);
        row_box.append(&play_btn);
        row_box.append(&del_btn);
        list.append(&row_box);
    }

    playlists_box.append(&list);
}
