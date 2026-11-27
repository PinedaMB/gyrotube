mod browse;
pub(crate) mod js_solver;
mod parsers;
mod player;
#[cfg(test)]
mod tests;

pub use parsers::*;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::models::{
    Album, Artist, AudioQuality, LyricsData, Playlist, RelatedFeed, SearchSuggestions, StreamInfo,
    SyncedLyricLine, Track, clean_track_title, pick_cover_thumbs,
};

const YTM_CLIENT_NAME: &str = "WEB_REMIX";
const YTM_CLIENT_NAME_HDR: &str = "67";
const YTM_CLIENT_VERSION: &str = "1.20250310.01.00";
const YTM_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36";
const VISITOR_DATA_TTL: Duration = Duration::from_secs(6 * 60 * 60);
const DETAILS_CACHE_TTL: Duration = Duration::from_secs(5 * 60);
const DETAILS_CACHE_MAX: usize = 64;

#[cfg(feature = "js-solver")]
pub const DEFAULT_PLAYER_PROFILES: &[&str] = &[
    "ANDROID_VR",
    "VISIONOS",
    "ANDROID_VR_LEGACY",
    "TV_EMBEDDED",
    "IOS",
    "WEB",
];

#[cfg(not(feature = "js-solver"))]
pub const DEFAULT_PLAYER_PROFILES: &[&str] = &[
    "ANDROID_VR",
    "VISIONOS",
    "ANDROID_VR_LEGACY",
    "TV_EMBEDDED",
    "IOS",
];

fn active_hl() -> &'static str {
    match crate::i18n::current_lang() {
        crate::i18n::Lang::Es => "es",
        crate::i18n::Lang::En => "en",
    }
}

#[derive(Clone, Debug)]
pub struct DirectVideoCandidate {
    pub url: String,
    pub user_agent: String,
    pub mime_type: &'static str,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackKind {
    Track,
    Video,
    Episode,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct CatalogItem {
    pub track: Track,
    pub kind: TrackKind,
    pub unavailable: bool,
}

#[derive(Clone, Debug)]
pub struct MusicTrackDetails {
    pub item: CatalogItem,
    pub covers: Vec<(String, u32)>,
    pub lyrics_browse_id: Option<String>,
    pub related_browse_id: Option<String>,
    pub counterparts: Vec<CatalogItem>,
}

#[derive(Clone, Debug, Default)]
pub struct ChartsResult {
    pub trending: Vec<Track>,
    pub top: Vec<Track>,
    pub artists: Vec<Artist>,
    pub available_countries: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ArtistEnrichment {
    pub name: Option<String>,
    pub header_url: Option<String>,
    pub description: Option<String>,
    pub subscriber_count: Option<u64>,
    pub view_counts: HashMap<String, u64>,
    pub tracks: Vec<Track>,
    pub videos: Vec<Track>,
    pub albums: Vec<Album>,
    pub singles: Vec<Album>,
    pub playlists: Vec<Playlist>,
    pub similar_artists: Vec<Artist>,
}

#[derive(Clone, Debug)]
pub struct PlaylistBrowsePage {
    pub title: String,
    pub items: Vec<CatalogItem>,
    pub had_entries: bool,
    pub exhausted: bool,
}

#[derive(Clone)]
pub struct InnerTubeClient {
    http: reqwest::Client,
    visitor_cache: Arc<RwLock<Option<(Instant, String)>>>,
    preferred_profile: Arc<RwLock<Option<&'static str>>>,
    details_cache: Arc<RwLock<HashMap<String, (Instant, MusicTrackDetails)>>>,
    audio_quality: Arc<RwLock<AudioQuality>>,
}

impl InnerTubeClient {
    pub fn new(http: reqwest::Client) -> Self {
        Self {
            http,
            visitor_cache: Arc::new(RwLock::new(None)),
            preferred_profile: Arc::new(RwLock::new(None)),
            details_cache: Arc::new(RwLock::new(HashMap::new())),
            audio_quality: Arc::new(RwLock::new(AudioQuality::High)),
        }
    }

    pub fn set_audio_quality(&self, quality: AudioQuality) {
        if let Ok(mut guard) = self.audio_quality.write() {
            *guard = quality;
        }
    }

    pub fn audio_quality(&self) -> AudioQuality {
        self.audio_quality
            .read()
            .ok()
            .map(|g| *g)
            .unwrap_or(AudioQuality::High)
    }

    pub fn ordered_profiles(&self) -> Vec<&'static str> {
        let preferred = self.preferred_profile.read().ok().and_then(|g| *g);
        let mut out = Vec::with_capacity(DEFAULT_PLAYER_PROFILES.len());
        if let Some(p) = preferred
            && DEFAULT_PLAYER_PROFILES.contains(&p)
        {
            out.push(p);
        }
        for &p in DEFAULT_PLAYER_PROFILES {
            if !out.contains(&p) {
                out.push(p);
            }
        }
        out
    }

    pub fn record_working_profile(&self, profile: &'static str) {
        if DEFAULT_PLAYER_PROFILES.contains(&profile)
            && let Ok(mut guard) = self.preferred_profile.write()
        {
            *guard = Some(profile);
        }
    }

    pub async fn visitor_data(&self, force_refresh: bool) -> Option<String> {
        if !force_refresh
            && let Ok(guard) = self.visitor_cache.read()
            && let Some((ts, ref vd)) = *guard
            && ts.elapsed() < VISITOR_DATA_TTL
        {
            return Some(vd.clone());
        }

        let payload = json!({
            "context": {
                "client": {
                    "clientName": YTM_CLIENT_NAME,
                    "clientVersion": YTM_CLIENT_VERSION,
                    "hl": "en",
                    "gl": "US"
                }
            }
        });

        let resp = self
            .http
            .post("https://www.youtube.com/youtubei/v1/visitor_id?prettyPrint=false")
            .timeout(Duration::from_secs(5))
            .header("User-Agent", YTM_USER_AGENT)
            .header("X-YouTube-Client-Name", YTM_CLIENT_NAME_HDR)
            .header("X-YouTube-Client-Version", YTM_CLIENT_VERSION)
            .json(&payload)
            .send()
            .await
            .ok()?
            .json::<Value>()
            .await
            .ok()?;

        let vd = resp["responseContext"]["visitorData"]
            .as_str()
            .filter(|s| !s.is_empty())?
            .to_string();

        if let Ok(mut guard) = self.visitor_cache.write() {
            *guard = Some((Instant::now(), vd.clone()));
        }
        Some(vd)
    }

    async fn ytm_post(&self, endpoint: &str, mut body: Value, country: Option<&str>) -> Result<Value, String> {
        let gl = country.unwrap_or("US");
        let visitor = self.visitor_data(false).await;
        let mut client_ctx = json!({
            "clientName": YTM_CLIENT_NAME,
            "clientVersion": YTM_CLIENT_VERSION,
            "hl": active_hl(),
            "gl": gl
        });
        if let Some(ref vd) = visitor
            && let Some(obj) = client_ctx.as_object_mut()
        {
            obj.insert("visitorData".into(), Value::String(vd.clone()));
        }
        if let Some(obj) = body.as_object_mut() {
            obj.insert("context".into(), json!({ "client": client_ctx }));
        }

        let url = format!("https://music.youtube.com/youtubei/v1/{endpoint}?prettyPrint=false");
        let mut req = self
            .http
            .post(&url)
            .timeout(Duration::from_secs(8))
            .header("User-Agent", YTM_USER_AGENT)
            .header("Origin", "https://music.youtube.com")
            .header("Referer", "https://music.youtube.com/")
            .header("X-YouTube-Client-Name", YTM_CLIENT_NAME_HDR)
            .header("X-YouTube-Client-Version", YTM_CLIENT_VERSION);
        if let Some(ref vd) = visitor {
            req = req.header("X-Goog-Visitor-Id", vd.as_str());
        }

        req.json(&body)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|e| e.without_url().to_string())?
            .json::<Value>()
            .await
            .map_err(|e| e.without_url().to_string())
    }

}
