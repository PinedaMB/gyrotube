use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use adw::prelude::*;
use gtk4::gdk;
use gtk4::{
    Box as GtkBox, Button, Label, ListBox, Picture, Scale, ScrolledWindow, Spinner, ToggleButton,
};
use libadwaita as adw;

use crate::backend::Backend;
use crate::gst_deck::GStreamerDeck;
use crate::models::{SessionState, Track, UserPlaylist, save_session_state};
use crate::mpris::MprisController;

pub struct AppState {
    pub backend: Backend,
    pub decks: [GStreamerDeck; 2],
    pub active_deck: usize,
    pub user_volume: f64,
    pub crossfade_enabled: bool,
    pub crossfade_secs: u32,
    pub crossfade_active: bool,
    pub crossfade_generation: u64,
    pub crossfade_triggered_for: Option<u64>,
    pub queue: Vec<Track>,
    pub unshuffled_queue: Vec<Track>,
    pub favorites: Vec<Track>,
    pub recent_tracks: Vec<Track>,
    pub user_playlists: Vec<UserPlaylist>,
    pub search_history: Vec<String>,
    pub current_index: Option<usize>,
    pub source_name: String,
    pub shuffle: bool,
    pub repeat: bool,
    pub language: crate::i18n::AppLanguage,
    pub cover_cache: HashMap<String, gdk::Texture>,
    pub toast_overlay: adw::ToastOverlay,
    // Sidebar player widgets
    pub player_source_label: Label,
    pub player_cover: Picture,
    pub player_cover_spinner: Spinner,
    pub player_title: Label,
    pub player_artist: Label,
    pub player_album_label: Label,
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
    pub lyrics_artist_label: Label,
    pub lyrics_sync_badge: Label,
    pub lyrics_body_label: Label,
    pub lyrics_box: GtkBox,
    pub lyrics_scroll: ScrolledWindow,
    pub lyrics_resync_btn: Button,
    pub synced_lyrics: Vec<(f64, Button)>,
    pub active_lyric_idx: Option<usize>,
    pub lyrics_auto_scroll: bool,
    pub lyrics_programmatic_scroll: Rc<Cell<bool>>,
    pub lyrics_loaded_for: Option<String>,
    // Party Mode widgets
    pub party_active: bool,
    pub party_lyrics_box: GtkBox,
    pub party_lyrics_scroll: ScrolledWindow,
    pub party_lyrics_body_label: Label,
    pub party_synced_lyrics: Vec<(f64, Button)>,
    pub party_next_box: GtkBox,
    pub party_next_cover: Picture,
    pub party_next_title: Label,
    pub party_next_artist: Label,
    pub queue_list: ListBox,
    pub queue_empty_label: Label,
    pub queue_title_label: Label,
    // Collection (favorites, recent, custom playlists) widgets
    pub favorites_list: ListBox,
    pub favorites_empty_label: GtkBox,
    pub recent_list: ListBox,
    pub recent_empty_label: GtkBox,
    pub playlists_box: GtkBox,
    pub playlists_empty_label: GtkBox,
    pub mpris: Option<MprisController>,
    pub pending_autoplay: bool,
    pub sidebar_collapsed: bool,
    pub loading_generation: u64,
    pub preloading_id: Option<String>,
    pub pending_restore_seek: Option<f64>,
    pub on_track_changed: Vec<Rc<dyn Fn()>>,
    pub on_open_collection: Option<Rc<dyn Fn(String, Option<String>, Option<Vec<Track>>)>>,
    pub on_cancel_collection_loading: Option<Rc<dyn Fn()>>,
}

impl AppState {
    pub fn save_session(&self) {
        let ts_us = self.active_media().timestamp();
        let pos_secs = if ts_us > 0 {
            ts_us as f64 / 1_000_000.0
        } else if let Some(pending) = self.pending_restore_seek {
            pending
        } else {
            self.seek_scale.value().max(0.0)
        };
        save_session_state(&SessionState {
            sidebar_collapsed: self.sidebar_collapsed,
            queue: self.queue.clone(),
            current_index: self.current_index,
            source_name: self.source_name.clone(),
            user_volume: self.user_volume,
            shuffle: self.shuffle,
            repeat: self.repeat,
            position_secs: pos_secs,
            crossfade_secs: self.crossfade_secs,
            language: self.language,
        });
    }
    pub fn active_media(&self) -> &GStreamerDeck {
        &self.decks[self.active_deck]
    }

    pub fn standby_media(&self) -> &GStreamerDeck {
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

    pub fn set_shuffled_queue(&mut self, tracks: Vec<Track>, source_name: String) {
        self.unshuffled_queue = tracks.clone();
        let mut shuffled = tracks;
        shuffle_slice(&mut shuffled);
        self.queue = shuffled;
        self.source_name = source_name;
        self.shuffle = true;
        self.update_nav_buttons();
    }

    pub fn set_shuffle_mode(&mut self, enabled: bool) {
        self.shuffle = enabled;
        if enabled {
            if self.unshuffled_queue.is_empty() && !self.queue.is_empty() {
                self.unshuffled_queue = self.queue.clone();
            }
            if self.queue.len() > 1 {
                if let Some(cur) = self.current_index.filter(|&i| i < self.queue.len()) {
                    let current = self.queue.remove(cur);
                    shuffle_slice(&mut self.queue);
                    self.queue.insert(0, current);
                    self.current_index = Some(0);
                } else {
                    shuffle_slice(&mut self.queue);
                }
            }
        } else if !self.unshuffled_queue.is_empty() {
            let current_id = self.current_track().map(|t| t.id.clone());
            self.queue = std::mem::take(&mut self.unshuffled_queue);
            if let Some(id) = current_id {
                self.current_index = self
                    .queue
                    .iter()
                    .position(|t| t.id == id)
                    .or_else(|| self.current_index.map(|i| i.min(self.queue.len().saturating_sub(1))));
            }
        }
        self.update_nav_buttons();
        self.save_session();
    }

    pub fn move_queue_item(&mut self, from: usize, to: usize) {
        if from >= self.queue.len() || to >= self.queue.len() || from == to {
            return;
        }
        let item = self.queue.remove(from);
        self.queue.insert(to, item);
        if let Some(cur) = self.current_index {
            self.current_index = Some(if cur == from {
                to
            } else if from < cur && cur <= to {
                cur - 1
            } else if to <= cur && cur < from {
                cur + 1
            } else {
                cur
            });
        }
        self.update_nav_buttons();
        self.save_session();
    }
}

pub fn shuffle_slice(tracks: &mut [Track]) {
    let len = tracks.len();
    if len <= 1 {
        return;
    }
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e37_79b9_7f4a_7c15);
    if seed == 0 {
        seed = 0x9e37_79b9_7f4a_7c15;
    }
    for i in (1..len).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let j = (seed as usize) % (i + 1);
        tracks.swap(i, j);
    }
}

pub fn next_queue_index(s: &AppState) -> Option<usize> {
    let len = s.queue.len();
    if len == 0 {
        return None;
    }
    let current = s.current_index?;
    if current + 1 < len {
        Some(current + 1)
    } else if s.repeat {
        Some(0)
    } else {
        None
    }
}
