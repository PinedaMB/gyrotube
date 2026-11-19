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

#[derive(Clone, Default)]
pub struct ViewRequests(Rc<Cell<u64>>);

impl ViewRequests {
    pub fn begin(&self) -> u64 {
        let next = self.0.get().wrapping_add(1);
        self.0.set(next);
        next
    }

    pub fn is_current(&self, request: u64) -> bool {
        self.0.get() == request
    }
}

pub struct AppState {
    pub backend: Backend,
    pub view_requests: ViewRequests,
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
    pub background_playback: bool,
    pub autoplay_related: bool,
    pub related_tracks: Vec<Track>,
    pub related_for_id: Option<String>,
    pub related_generation: u64,
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
    pub playback_status: Label,
    pub playback_load: Cell<Option<crate::ui::player::PlaybackLoad>>,
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
    pub queue_suggestions_box: GtkBox,
    pub queue_suggestions_list: ListBox,
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
    pub jam: crate::ui::jam_dialog::JamRuntime,
    pub spin_bar: GtkBox,
    pub spin_bar_label: Label,
    pub spin_bar_count: Label,
    pub spin_participants_box: GtkBox,
    pub remote_host_name: Option<String>,
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
        self.report_save_result(save_session_state(&SessionState {
            sidebar_collapsed: self.sidebar_collapsed,
            queue: self.queue.clone(),
            unshuffled_queue: self.unshuffled_queue.clone(),
            current_index: self.current_index,
            source_name: self.source_name.clone(),
            user_volume: self.user_volume,
            shuffle: self.shuffle,
            repeat: self.repeat,
            position_secs: pos_secs,
            crossfade_secs: self.crossfade_secs,
            language: self.language,
            background_playback: self.background_playback,
            autoplay_related: self.autoplay_related,
        }));
    }
    pub fn report_save_result(&self, result: std::io::Result<()>) {
        if let Err(err) = result {
            eprintln!("Error al guardar datos: {err}");
            self.show_toast(&format!("No se pudieron guardar los cambios: {err}"));
        }
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
        let has_next = self.current_index.is_some_and(|idx| {
            idx + 1 < self.queue.len() || (self.repeat && !self.queue.is_empty())
        });
        self.prev_btn.set_sensitive(has_prev);
        self.next_btn.set_sensitive(has_next);
    }

    pub fn replace_queue(&mut self, tracks: Vec<Track>, source_name: String) {
        self.queue = tracks;
        self.unshuffled_queue.clear();
        self.current_index = None;
        self.source_name = source_name;
        self.shuffle = false;
        self.preloading_id = None;
    }

    pub fn remove_queue_item(&mut self, index: usize) -> bool {
        let removed_current = remove_queue_track(&mut self.queue, &mut self.current_index, index);
        self.update_nav_buttons();
        removed_current
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
            self.current_index =
                restore_queue_order(&mut self.queue, &self.unshuffled_queue, self.current_index);
            self.unshuffled_queue.clear();
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

fn remove_queue_track(queue: &mut Vec<Track>, current: &mut Option<usize>, index: usize) -> bool {
    if index >= queue.len() {
        return false;
    }
    let removed_current = *current == Some(index);
    queue.remove(index);
    *current = current.and_then(|cur| {
        if queue.is_empty() {
            None
        } else if index < cur {
            Some(cur - 1)
        } else {
            Some(cur.min(queue.len() - 1))
        }
    });
    removed_current
}

fn restore_queue_order(
    queue: &mut Vec<Track>,
    original: &[Track],
    current: Option<usize>,
) -> Option<usize> {
    let mut positions: HashMap<&str, std::collections::VecDeque<usize>> = HashMap::new();
    for (index, track) in original.iter().enumerate() {
        positions.entry(&track.id).or_default().push_back(index);
    }
    let mut indexed: Vec<_> = std::mem::take(queue).into_iter().enumerate().collect();
    indexed.sort_by_cached_key(|(_, track)| {
        positions
            .get_mut(track.id.as_str())
            .and_then(|indices| indices.pop_front())
            .unwrap_or(usize::MAX)
    });
    let current = current.and_then(|index| indexed.iter().position(|(old, _)| *old == index));
    *queue = indexed.into_iter().map(|(_, track)| track).collect();
    current
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_latest_view_request_can_update_the_page() {
        let requests = ViewRequests::default();
        let first_album = requests.begin();
        let second_album = requests.begin();
        let first_album_again = requests.begin();
        assert!(!requests.is_current(first_album));
        assert!(!requests.is_current(second_album));
        assert!(requests.is_current(first_album_again));
        requests.begin(); // Navigation away invalidates a request even without a replacement.
        assert!(!requests.is_current(first_album_again));
    }

    fn track(id: &str) -> Track {
        serde_json::from_value(serde_json::json!({"id":id,"title":id,"artist":"Artist"})).unwrap()
    }

    #[test]
    fn unshuffle_keeps_additions_removals_and_current_metadata() {
        let original = vec![track("a"), track("b"), track("c")];
        let mut queue = vec![track("c"), track("a"), track("d"), track("e")];
        queue[0].is_music = true;
        let current = restore_queue_order(&mut queue, &original, Some(0));
        assert_eq!(
            queue.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
            ["a", "c", "d", "e"]
        );
        assert_eq!(current, Some(1));
        assert!(queue[1].is_music);
    }

    #[test]
    fn unshuffle_preserves_duplicate_counts_and_selected_occurrence() {
        let original = vec![track("a"), track("b"), track("a")];
        let mut queue = vec![track("b"), track("a"), track("a"), track("a")];
        queue[2].title = "Selected occurrence".into();
        let selected = queue[2].clone();
        let current = restore_queue_order(&mut queue, &original, Some(2)).unwrap();
        assert_eq!(queue.len(), 4);
        assert_eq!(queue.iter().filter(|t| t.id == "a").count(), 3);
        assert_eq!(queue[current], selected);
    }

    #[test]
    fn removing_current_selects_replacement_or_stops_when_empty() {
        let mut queue = vec![track("a"), track("b"), track("c")];
        let mut current = Some(1);
        assert!(remove_queue_track(&mut queue, &mut current, 1));
        assert_eq!(queue[current.unwrap()].id, "c");
        assert!(remove_queue_track(&mut queue, &mut current, 1));
        assert_eq!(queue[current.unwrap()].id, "a");
        assert!(remove_queue_track(&mut queue, &mut current, 0));
        assert!(queue.is_empty());
        assert_eq!(current, None);
    }

    #[test]
    fn removing_another_track_preserves_selection_and_invalid_indices_do_nothing() {
        let mut queue = vec![track("a"), track("b"), track("c")];
        let mut current = Some(1);
        assert!(!remove_queue_track(&mut queue, &mut current, 0));
        assert_eq!(queue[current.unwrap()].id, "b");
        assert!(!remove_queue_track(&mut queue, &mut current, 10));
        assert_eq!(queue.len(), 2);
        assert_eq!(current, Some(0));
    }
}
