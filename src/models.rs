use std::path::PathBuf;

use rustypipe::model::{AlbumItem, ArtistItem, MusicPlaylistItem, TrackItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    // shortcut: catalog audio includes live albums; require recording metadata to certify studio versions.
    // Older proof flags could be supplied by guests and must be checked again.
    #[serde(default, rename = "verified_catalog_audio")]
    pub is_music: bool,
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration_secs: Option<u32>,
    pub cover_url: Option<String>,
    pub cover_large_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub user_agent: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
    pub local_audio: Option<std::sync::Arc<crate::audio_cache::LocalAudio>>,
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
            is_music: item.track_type == rustypipe::model::TrackType::Track,
            id: item.id.clone(),
            title: clean_track_title(&item.name),
            artist,
            album: item.album.as_ref().map(|a| a.name.clone()),
            duration_secs: item.duration,
            cover_url: pick_cover(&item.cover, 240),
            cover_large_url: pick_cover(&item.cover, 544),
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

pub fn color_to_hex(color: u32) -> String {
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    format!("#{r:02x}{g:02x}{b:02x}")
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

static LOAD_FAILURES: std::sync::Mutex<Vec<(PathBuf, String)>> = std::sync::Mutex::new(Vec::new());

pub fn data_load_warnings() -> Vec<String> {
    LOAD_FAILURES
        .lock()
        .map(|failures| {
            failures
                .iter()
                .map(|(_, message)| message.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn load_json_path<T: serde::de::DeserializeOwned + Default>(path: &std::path::Path) -> T {
    let result = std::fs::read(path)
        .and_then(|raw| serde_json::from_slice(&raw).map_err(std::io::Error::other));
    match result {
        Ok(value) => value,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => T::default(),
        Err(err) => {
            let message = format!(
                "No se pudo cargar {}: {err}. Se conservará el archivo original.",
                path.display()
            );
            eprintln!("{message}");
            if let Ok(mut failures) = LOAD_FAILURES.lock()
                && !failures.iter().any(|(failed, _)| failed == path)
            {
                failures.push((path.to_path_buf(), message));
            }
            T::default()
        }
    }
}

fn load_json_file<T: serde::de::DeserializeOwned + Default>(filename: &str) -> T {
    data_dir()
        .map(|dir| load_json_path(&dir.join(filename)))
        .unwrap_or_default()
}

fn save_json_path<T: serde::Serialize + ?Sized>(
    path: &std::path::Path,
    value: &T,
) -> std::io::Result<()> {
    use std::io::Write;
    let dir = path
        .parent()
        .ok_or_else(|| std::io::Error::other("Ruta de datos sin directorio"))?;
    std::fs::create_dir_all(dir)?;
    let mut staged = tempfile::NamedTempFile::new_in(dir)?;
    serde_json::to_writer_pretty(&mut staged, value).map_err(std::io::Error::other)?;
    staged.flush()?;
    staged.as_file().sync_all()?;

    let preserve = LOAD_FAILURES
        .lock()
        .map_err(|_| std::io::Error::other("Error de sincronización de datos"))?
        .iter()
        .any(|(failed, _)| failed == path);
    if preserve {
        let raw = std::fs::read(path)?;
        let prefix = format!(
            "{}.damaged-",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        let mut backup = tempfile::Builder::new().prefix(&prefix).tempfile_in(dir)?;
        backup.write_all(&raw)?;
        backup.as_file().sync_all()?;
        let (_, backup_path) = backup.keep().map_err(|err| err.error)?;
        eprintln!("Archivo dañado conservado en {}", backup_path.display());
    }
    staged.persist(path).map_err(|err| err.error)?;
    std::fs::File::open(dir)?.sync_all()?;
    if let Ok(mut failures) = LOAD_FAILURES.lock() {
        failures.retain(|(failed, _)| failed != path);
    }
    Ok(())
}

fn save_json_file<T: serde::Serialize + ?Sized>(filename: &str, value: &T) -> std::io::Result<()> {
    let dir =
        data_dir().ok_or_else(|| std::io::Error::other("No se encontró el directorio de datos"))?;
    save_json_path(&dir.join(filename), value)
}

pub fn load_favorites() -> Vec<Track> {
    let mut tracks: Vec<Track> = load_json_file("favorites.json");
    upgrade_tracks_covers(&mut tracks);
    tracks
}

pub fn save_favorites(favorites: &[Track]) -> std::io::Result<()> {
    save_json_file("favorites.json", favorites)
}

pub fn load_recent_tracks() -> Vec<Track> {
    let mut tracks: Vec<Track> = load_json_file("recent.json");
    upgrade_tracks_covers(&mut tracks);
    tracks
}

pub fn save_recent_tracks(recent: &[Track]) -> std::io::Result<()> {
    save_json_file("recent.json", recent)
}

pub fn load_user_playlists() -> Vec<UserPlaylist> {
    let mut playlists: Vec<UserPlaylist> = load_json_file("playlists.json");
    for pl in &mut playlists {
        upgrade_tracks_covers(&mut pl.tracks);
    }
    playlists
}

pub fn save_user_playlists(playlists: &[UserPlaylist]) -> std::io::Result<()> {
    save_json_file("playlists.json", playlists)
}

pub fn load_search_history() -> Vec<String> {
    load_json_file("search_history.json")
}

pub fn save_search_history(history: &[String]) -> std::io::Result<()> {
    save_json_file("search_history.json", history)
}

fn default_volume() -> f64 {
    1.0
}

fn default_crossfade_secs() -> u32 {
    4
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionState {
    pub sidebar_collapsed: bool,
    pub queue: Vec<Track>,
    #[serde(default)]
    pub unshuffled_queue: Vec<Track>,
    pub current_index: Option<usize>,
    pub source_name: String,
    #[serde(default = "default_volume")]
    pub user_volume: f64,
    #[serde(default)]
    pub shuffle: bool,
    #[serde(default)]
    pub repeat: bool,
    #[serde(default)]
    pub position_secs: f64,
    #[serde(default = "default_crossfade_secs")]
    pub crossfade_secs: u32,
    #[serde(default)]
    pub language: crate::i18n::AppLanguage,
}

pub fn load_session_state() -> SessionState {
    let mut state: SessionState = load_json_file("session.json");
    upgrade_tracks_covers(&mut state.queue);
    upgrade_tracks_covers(&mut state.unshuffled_queue);
    state
}

pub fn save_session_state(state: &SessionState) -> std::io::Result<()> {
    save_json_file("session.json", state)
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
                && let (Ok(mins), Ok(secs)) =
                    (min_s.trim().parse::<f64>(), sec_s.trim().parse::<f64>())
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
    fn interrupted_serialization_keeps_previous_json_and_cleans_staging() {
        struct Fails;
        impl Serialize for Fails {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("simulated serialization failure"))
            }
        }
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("favorites.json");
        let original = br#"["saved favorite"]"#;
        std::fs::write(&path, original).unwrap();
        assert!(save_json_path(&path, &Fails).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), original);
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
        save_json_path(&path, &["new favorite"]).unwrap();
        assert_eq!(load_json_path::<Vec<String>>(&path), vec!["new favorite"]);
    }

    #[test]
    fn malformed_or_wrong_schema_data_is_preserved_before_replacement() {
        for raw in [
            br#"[{"#.as_slice(),
            br#"{"valid_json":"wrong schema"}"#.as_slice(),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("playlists.json");
            std::fs::write(&path, raw).unwrap();
            assert!(load_json_path::<Vec<UserPlaylist>>(&path).is_empty());
            assert_eq!(std::fs::read(&path).unwrap(), raw);
            assert!(
                data_load_warnings()
                    .iter()
                    .any(|message| message.contains(path.to_str().unwrap()))
            );
            save_json_path(&path, &Vec::<UserPlaylist>::new()).unwrap();
            let backup = std::fs::read_dir(dir.path())
                .unwrap()
                .flatten()
                .find(|entry| {
                    entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with("playlists.json.damaged-")
                })
                .unwrap();
            assert_eq!(std::fs::read(backup.path()).unwrap(), raw);
            assert!(
                serde_json::from_slice::<Vec<UserPlaylist>>(&std::fs::read(&path).unwrap()).is_ok()
            );
        }
    }

    #[test]
    fn failed_directory_creation_does_not_replace_existing_data() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("blocked");
        std::fs::write(&file, b"original").unwrap();
        assert!(save_json_path(&file.join("session.json"), &SessionState::default()).is_err());
        assert_eq!(std::fs::read(&file).unwrap(), b"original");
    }

    #[test]
    fn shuffled_order_survives_session_roundtrip_and_older_sessions_still_load() {
        let state = SessionState {
            queue: vec![serde_json::from_value(serde_json::json!({
                "id":"audio123456", "title":"Song", "artist":"Artist", "verified_catalog_audio":true
            })).unwrap()],
            shuffle: true,
            ..SessionState::default()
        };
        let mut state = state;
        state.unshuffled_queue = state.queue.clone();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.json");
        save_json_path(&path, &state).unwrap();
        let loaded: SessionState = load_json_path(&path);
        assert_eq!(loaded.unshuffled_queue, state.unshuffled_queue);
        let mut old = serde_json::to_value(&state).unwrap();
        old.as_object_mut().unwrap().remove("unshuffled_queue");
        assert!(
            serde_json::from_value::<SessionState>(old)
                .unwrap()
                .unshuffled_queue
                .is_empty()
        );
    }

    #[test]
    fn music_provenance_comes_from_catalog_type() {
        for (kind, expected) in [("track", true), ("video", false), ("episode", false)] {
            let item: TrackItem = serde_json::from_value(serde_json::json!({
                "id":"Abc_123-xyz", "name":"Song", "cover":[], "artists":[],
                "track_type":kind, "by_va":false, "unavailable":false
            }))
            .unwrap();
            assert_eq!(Track::from(&item).is_music, expected);
        }
    }

    #[test]
    fn old_saved_tracks_require_music_verification() {
        let track: Track = serde_json::from_value(serde_json::json!({
            "id":"Abc_123-xyz", "title":"Song", "artist":"Artist", "album":"Album",
            "is_music":true, "verified_music_audio":true,
            "duration_secs":180,"cover_url":"https://lh3.googleusercontent.com/cover",
            "cover_large_url":null
        }))
        .unwrap();
        assert!(
            !track.is_music,
            "Album and image host do not prove audio identity"
        );
        let mut verified = track;
        verified.is_music = true;
        let roundtrip: Track =
            serde_json::from_slice(&serde_json::to_vec(&verified).unwrap()).unwrap();
        assert_eq!(verified, roundtrip);
    }

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
            upgrade_cover_url("https://yt3.ggpht.com/xyz=s120-c-k-c0x00ffffff-no-rj", 544),
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
