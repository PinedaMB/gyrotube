use std::collections::HashMap;
use std::rc::Rc;

use adw::prelude::*;
use gtk4::gdk;
use gtk4::{
    Box as GtkBox, Button, Label, ListBox, MediaFile, Picture, Scale, ScrolledWindow, Spinner,
    ToggleButton,
};
use libadwaita as adw;

use crate::backend::Backend;
use crate::models::{Track, UserPlaylist};
use crate::mpris::MprisController;

pub struct AppState {
    pub backend: Backend,
    pub decks: [MediaFile; 2],
    pub active_deck: usize,
    pub user_volume: f64,
    pub crossfade_enabled: bool,
    pub crossfade_active: bool,
    pub crossfade_generation: u64,
    pub crossfade_triggered_for: Option<u64>,
    pub queue: Vec<Track>,
    pub favorites: Vec<Track>,
    pub recent_tracks: Vec<Track>,
    pub user_playlists: Vec<UserPlaylist>,
    pub search_history: Vec<String>,
    pub current_index: Option<usize>,
    pub source_name: String,
    pub shuffle: bool,
    pub repeat: bool,
    pub cover_cache: HashMap<String, gdk::Texture>,
    pub toast_overlay: adw::ToastOverlay,
    // Sidebar player widgets
    pub player_source_label: Label,
    pub player_cover: Picture,
    pub player_cover_spinner: Spinner,
    pub player_title: Label,
    pub player_artist: Label,
    pub player_fav_btn: ToggleButton,
    pub player_radio_btn: Button,
    pub quality_label: Label,
    pub play_pause_btn: Button,
    pub play_spinner: Spinner,
    pub prev_btn: Button,
    pub next_btn: Button,
    pub seek_scale: Scale,
    pub time_label: Label,
    pub duration_label: Label,
    // Sidebar lyrics & queue widgets
    pub lyrics_title_label: Label,
    pub lyrics_body_label: Label,
    pub lyrics_box: GtkBox,
    pub lyrics_scroll: ScrolledWindow,
    pub synced_lyrics: Vec<(f64, Button)>,
    pub active_lyric_idx: Option<usize>,
    pub lyrics_loaded_for: Option<String>,
    pub queue_list: ListBox,
    pub queue_empty_label: Label,
    pub queue_title_label: Label,
    // Collection (favorites, recent, custom playlists) widgets
    pub favorites_list: ListBox,
    pub favorites_empty_label: Label,
    pub recent_list: ListBox,
    pub recent_empty_label: Label,
    pub playlists_box: GtkBox,
    pub playlists_empty_label: Label,
    pub mpris: Option<MprisController>,
    pub pending_autoplay: bool,
    pub loading_generation: u64,
    pub on_track_changed: Vec<Rc<dyn Fn()>>,
}

impl AppState {
    pub fn active_media(&self) -> &MediaFile {
        &self.decks[self.active_deck]
    }

    pub fn standby_media(&self) -> &MediaFile {
        &self.decks[1 - self.active_deck]
    }

    pub fn show_toast(&self, msg: &str) {
        let toast = adw::Toast::new(msg);
        toast.set_timeout(3);
        self.toast_overlay.add_toast(toast);
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.current_index.and_then(|i| self.queue.get(i))
    }

    pub fn is_favorite(&self, id: &str) -> bool {
        self.favorites.iter().any(|t| t.id == id)
    }

    pub fn update_nav_buttons(&self) {
        let has_prev = self.current_index.is_some_and(|idx| idx > 0);
        let has_next = self
            .current_index
            .is_some_and(|idx| idx + 1 < self.queue.len() || (self.repeat && !self.queue.is_empty()));
        self.prev_btn.set_sensitive(has_prev);
        self.next_btn.set_sensitive(has_next);
    }
}

pub fn next_queue_index(s: &AppState) -> Option<usize> {
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
