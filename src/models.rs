use std::path::PathBuf;

use rustypipe::model::{
    AlbumItem, ArtistItem, MusicArtist, MusicGenreItem, MusicPlaylistItem, TrackItem,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration_secs: Option<u32>,
    pub cover_url: Option<String>,
    pub cover_large_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub year: Option<u16>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub color_hex: String,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub subtitle: String,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub subscriber_count: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ArtistDetails {
    pub name: String,
    pub header_url: Option<String>,
    pub description: Option<String>,
    pub subscriber_count: Option<u64>,
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub playlists: Vec<Playlist>,
    pub similar_artists: Vec<Artist>,
}

#[derive(Debug, Clone, Default)]
pub struct SearchResults {
    pub artists: Vec<Artist>,
    pub albums: Vec<Album>,
    pub playlists: Vec<Playlist>,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub url: String,
    pub quality_label: String,
}

pub fn upgrade_cover_url(url: &str, target_size: i32) -> String {
    if target_size <= 0 {
        return url.to_string();
    }

    if url.contains("i.ytimg.com/vi/") && !url.contains("?sqp=") {
        if let Some(base) = url.strip_suffix("/default.jpg") {
            return format!("{base}/hqdefault.jpg");
        }
        if let Some(base) = url.strip_suffix("/mqdefault.jpg") {
            return format!("{base}/hqdefault.jpg");
        }
    }

    if !url.contains("googleusercontent.com/") && !url.contains("ggpht.com/") {
        return url.to_string();
    }

    let Some(slash_idx) = url.rfind('/') else {
        return url.to_string();
    };
    let Some(eq_rel) = url[slash_idx..].rfind('=') else {
        return url.to_string();
    };

    let eq_idx = slash_idx + eq_rel;
    let prefix = &url[..=eq_idx];
    let params = &url[eq_idx + 1..];
    let mut parts = params.split('-');

    let Some(first) = parts.next() else {
        return url.to_string();
    };

    if let Some(w_str) = first.strip_prefix('w')
        && let Ok(w) = w_str.parse::<i32>()
        && let Some(second) = parts.next()
        && let Some(h_str) = second.strip_prefix('h')
        && h_str.parse::<i32>().is_ok()
    {
        let new_size = w.max(target_size);
        let rest: Vec<&str> = parts.collect();
        return if rest.is_empty() {
            format!("{prefix}w{new_size}-h{new_size}")
        } else {
            format!("{prefix}w{new_size}-h{new_size}-{}", rest.join("-"))
        };
    }

    if let Some(s_str) = first.strip_prefix('s')
        && let Ok(s) = s_str.parse::<i32>()
    {
        let new_size = s.max(target_size);
        let rest: Vec<&str> = parts.collect();
        return if rest.is_empty() {
            format!("{prefix}s{new_size}")
        } else {
            format!("{prefix}s{new_size}-{}", rest.join("-"))
        };
    }

    url.to_string()
}

pub fn pick_cover(covers: &[rustypipe::model::Thumbnail], target_width: i32) -> Option<String> {
    covers
        .iter()
        .min_by_key(|t| (t.width as i32 - target_width).abs())
        .or_else(|| covers.last())
        .map(|t| upgrade_cover_url(&t.url, target_width))
}

impl From<&TrackItem> for Track {
    fn from(item: &TrackItem) -> Self {
        let artist = if item.artists.is_empty() {
            String::from("Artista desconocido")
        } else {
            item.artists
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        };

        Self {
            id: item.id.clone(),
            title: item.name.clone(),
            artist,
            album: item.album.as_ref().map(|a| a.name.clone()),
            duration_secs: item.duration,
            cover_url: pick_cover(&item.cover, 240),
            cover_large_url: pick_cover(&item.cover, 800),
        }
    }
}

impl From<&AlbumItem> for Album {
    fn from(item: &AlbumItem) -> Self {
        let artist = if item.artists.is_empty() {
            String::from("Artista desconocido")
        } else {
            item.artists
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        };

        Self {
            id: item.id.clone(),
            title: item.name.clone(),
            artist,
            year: item.year,
            cover_url: pick_cover(&item.cover, 544),
        }
    }
}

pub fn color_to_hex(color: u32) -> String {
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    format!("#{r:02x}{g:02x}{b:02x}")
}

impl From<&MusicGenreItem> for Genre {
    fn from(item: &MusicGenreItem) -> Self {
        Self {
            id: item.id.clone(),
            name: item.name.clone(),
            color_hex: color_to_hex(item.color),
        }
    }
}

impl From<&MusicPlaylistItem> for Playlist {
    fn from(item: &MusicPlaylistItem) -> Self {
        let subtitle = item
            .channel
            .as_ref()
            .map(|c| c.name.clone())
            .unwrap_or_else(|| String::from("YouTube Music"));
        Self {
            id: item.id.clone(),
            name: item.name.clone(),
            subtitle,
            cover_url: pick_cover(&item.thumbnail, 544),
        }
    }
}

impl From<&ArtistItem> for Artist {
    fn from(item: &ArtistItem) -> Self {
        Self {
            id: item.id.clone(),
            name: item.name.clone(),
            avatar_url: pick_cover(&item.avatar, 360),
            subscriber_count: item.subscriber_count,
        }
    }
}

impl From<&MusicArtist> for ArtistDetails {
    fn from(artist: &MusicArtist) -> Self {
        Self {
            name: artist.name.clone(),
            header_url: pick_cover(&artist.header_image, 1000),
            description: artist.description.clone(),
            subscriber_count: artist.subscriber_count,
            tracks: artist.tracks.iter().map(Track::from).collect(),
            albums: artist.albums.iter().map(Album::from).collect(),
            playlists: artist.playlists.iter().map(Playlist::from).collect(),
            similar_artists: artist.similar_artists.iter().map(Artist::from).collect(),
        }
    }
}

pub fn format_subscribers(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1} M suscriptores", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1} K suscriptores", count as f64 / 1_000.0)
    } else {
        format!("{count} suscriptores")
    }
}

pub fn format_duration(secs: u32) -> String {
    let mins = secs / 60;
    let rem = secs % 60;
    format!("{mins}:{rem:02}")
}

pub fn is_ytm_square_cover(url: &str) -> bool {
    url.contains("googleusercontent.com/") || url.contains("ggpht.com/")
}

pub fn clean_track_title(title: &str) -> String {
    let mut out = title.trim().to_string();
    for (open, close) in [('(', ')'), ('[', ']')] {
        loop {
            let Some(start) = out.rfind(open) else {
                break;
            };
            let Some(end_rel) = out[start..].find(close) else {
                break;
            };
            let end = start + end_rel;
            if end + 1 != out.len() {
                break;
            }
            let inner = out[start + 1..end].to_ascii_lowercase();
            let is_video_tag = inner.contains("official")
                || inner.contains("audio")
                || inner.contains("video")
                || inner.contains("lyric")
                || inner.contains("visualizer");
            if !is_video_tag {
                break;
            }
            out = out[..start].trim_end().to_string();
        }
    }
    out
}

fn data_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME")
        && !xdg.is_empty()
    {
        return Some(PathBuf::from(xdg).join("gyrotube"));
    }
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".local/share/gyrotube"))
}

pub fn cache_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CACHE_HOME")
        && !xdg.is_empty()
    {
        return Some(PathBuf::from(xdg).join("gyrotube"));
    }
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".cache/gyrotube"))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserPlaylist {
    pub id: String,
    pub name: String,
    pub tracks: Vec<Track>,
}

fn upgrade_tracks_covers(tracks: &mut [Track]) {
    for track in tracks {
        if let Some(url) = &track.cover_url {
            track.cover_url = Some(upgrade_cover_url(url, 240));
        }
        if let Some(url) = &track.cover_large_url {
            track.cover_large_url = Some(upgrade_cover_url(url, 800));
        } else if let Some(url) = &track.cover_url {
            track.cover_large_url = Some(upgrade_cover_url(url, 800));
        }
    }
}

fn load_json_file<T: serde::de::DeserializeOwned + Default>(filename: &str) -> T {
    let Some(path) = data_dir().map(|d| d.join(filename)) else {
        return T::default();
    };
    let Ok(raw) = std::fs::read_to_string(path) else {
        return T::default();
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

fn save_json_file<T: serde::Serialize + ?Sized>(filename: &str, value: &T) {
    let Some(dir) = data_dir() else {
        return;
    };
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    if let Ok(json) = serde_json::to_string_pretty(value) {
        let _ = std::fs::write(dir.join(filename), json);
    }
}

pub fn load_favorites() -> Vec<Track> {
    let mut tracks: Vec<Track> = load_json_file("favorites.json");
    upgrade_tracks_covers(&mut tracks);
    tracks
}

pub fn save_favorites(favorites: &[Track]) {
    save_json_file("favorites.json", favorites);
}

pub fn load_recent_tracks() -> Vec<Track> {
    let mut tracks: Vec<Track> = load_json_file("recent.json");
    upgrade_tracks_covers(&mut tracks);
    tracks
}

pub fn save_recent_tracks(recent: &[Track]) {
    save_json_file("recent.json", recent);
}

pub fn load_user_playlists() -> Vec<UserPlaylist> {
    let mut playlists: Vec<UserPlaylist> = load_json_file("playlists.json");
    for pl in &mut playlists {
        upgrade_tracks_covers(&mut pl.tracks);
    }
    playlists
}

pub fn save_user_playlists(playlists: &[UserPlaylist]) {
    save_json_file("playlists.json", playlists);
}

pub fn load_search_history() -> Vec<String> {
    load_json_file("search_history.json")
}

pub fn save_search_history(history: &[String]) {
    save_json_file("search_history.json", history);
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionState {
    pub sidebar_collapsed: bool,
    pub queue: Vec<Track>,
    pub current_index: Option<usize>,
    pub source_name: String,
}

pub fn load_session_state() -> SessionState {
    let mut state: SessionState = load_json_file("session.json");
    upgrade_tracks_covers(&mut state.queue);
    state
}

pub fn save_session_state(state: &SessionState) {
    save_json_file("session.json", state);
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncedLyricLine {
    pub time_secs: f64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum LyricsData {
    Synced(Vec<SyncedLyricLine>),
    Plain(String),
}

pub fn parse_lrc(lrc: &str) -> Vec<SyncedLyricLine> {
    let mut out = Vec::new();
    for raw_line in lrc.lines() {
        let mut rest = raw_line.trim();
        let mut timestamps = Vec::new();
        while let Some(stripped) = rest.strip_prefix('[') {
            let Some(end) = stripped.find(']') else {
                break;
            };
            let tag = &stripped[..end];
            if let Some((min_s, sec_s)) = tag.split_once(':')
                && let (Ok(mins), Ok(secs)) = (min_s.trim().parse::<f64>(), sec_s.trim().parse::<f64>())
            {
                timestamps.push(mins * 60.0 + secs);
            }
            rest = stripped[end + 1..].trim();
        }
        if timestamps.is_empty() || rest.is_empty() {
            continue;
        }
        for t in timestamps {
            out.push(SyncedLyricLine {
                time_secs: t,
                text: rest.to_string(),
            });
        }
    }
    out.sort_by(|a, b| a.time_secs.total_cmp(&b.time_secs));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_duration_mm_ss() {
        assert_eq!(format_duration(0), "0:00");
        assert_eq!(format_duration(5), "0:05");
        assert_eq!(format_duration(65), "1:05");
        assert_eq!(format_duration(225), "3:45");
    }

    #[test]
    fn converts_genre_color_to_hex() {
        assert_eq!(color_to_hex(0xFFE53935), "#e53935");
    }

    #[test]
    fn upgrades_googleusercontent_cover_urls() {
        assert_eq!(
            upgrade_cover_url(
                "https://lh3.googleusercontent.com/abc123=w120-h120-l90-rj",
                800
            ),
            "https://lh3.googleusercontent.com/abc123=w800-h800-l90-rj"
        );
        assert_eq!(
            upgrade_cover_url(
                "https://yt3.ggpht.com/xyz=s120-c-k-c0x00ffffff-no-rj",
                544
            ),
            "https://yt3.ggpht.com/xyz=s544-c-k-c0x00ffffff-no-rj"
        );
        assert_eq!(
            upgrade_cover_url("https://i.ytimg.com/vi/abc/hqdefault.jpg?sqp=-oaymwE", 800),
            "https://i.ytimg.com/vi/abc/hqdefault.jpg?sqp=-oaymwE"
        );
    }

    #[test]
    fn formats_subscriber_counts() {
        assert_eq!(format_subscribers(850), "850 suscriptores");
        assert_eq!(format_subscribers(12_500), "12.5 K suscriptores");
        assert_eq!(format_subscribers(4_200_000), "4.2 M suscriptores");
    }

    #[test]
    fn cleans_youtube_video_tags_from_title() {
        assert_eq!(
            clean_track_title("Don't Stop Believin' (Official Audio)"),
            "Don't Stop Believin'"
        );
        assert_eq!(
            clean_track_title("Feel Good Inc. (Official Video) [HD]"),
            "Feel Good Inc. (Official Video) [HD]"
        );
        assert_eq!(
            clean_track_title("DARE (feat. Shaun Ryder) (Official Audio)"),
            "DARE (feat. Shaun Ryder)"
        );
    }

    #[test]
    fn parses_lrc_timestamps() {
        let lrc = "[00:12.50] First line\n[01:02.00][01:10.25] Chorus\n[ti:Ignored]\n";
        let lines = parse_lrc(lrc);
        assert_eq!(lines.len(), 3);
        assert!((lines[0].time_secs - 12.5).abs() < 0.01);
        assert_eq!(lines[0].text, "First line");
        assert!((lines[1].time_secs - 62.0).abs() < 0.01);
        assert_eq!(lines[1].text, "Chorus");
        assert!((lines[2].time_secs - 70.25).abs() < 0.01);
    }
}
