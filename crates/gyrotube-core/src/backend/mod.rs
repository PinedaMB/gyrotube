mod catalog;
mod lyrics;
mod parsers;
mod sponsorblock;
mod spotify;
mod streaming;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

use ytmapi_rs::YtMusic;
use ytmapi_rs::auth::noauth::NoAuthToken;
use ytmapi_rs::common::{
    AlbumID, ArtistChannelID, MoodCategoryParams, PlaylistID, VideoID, YoutubeID,
};
use ytmapi_rs::parse::{
    GetArtistTopReleases, MoodCategory, PlaylistItem, SearchResultAlbum, SearchResultArtist,
    SearchResultPlaylist, SearchResultSong,
};

use crate::audio_cache;
use crate::i18n::tr;
use crate::innertube::{CatalogItem, InnerTubeClient, TrackKind};
use crate::models::{
    Album, Artist, ArtistDetails, Genre, LyricsData, Playlist, SearchResults, SponsorSegment,
    StreamInfo, Track, VideoStreamInfo, cache_dir, clean_track_title, color_to_hex, parse_lrc,
    pick_cover_thumbs,
};

const MUSIC_PROOF_TTL: Duration = Duration::from_secs(15 * 60);
const STREAM_CACHE_TTL: Duration = Duration::from_secs(15 * 60);
const HOME_CACHE_TTL: Duration = Duration::from_secs(15 * 60);
const HOME_CACHE_MAX_COUNTRIES: usize = 8;

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn audio_timings_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("GYROTUBE_AUDIO_TIMINGS").as_deref() == Ok("1"))
}

pub fn log_audio_timing(id: &str, phase: &str, since: Instant) {
    if audio_timings_enabled() {
        eprintln!("[audio] id={id} etapa={phase} tiempo_ms={}", since.elapsed().as_millis());
    }
}

pub fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct HomeFeed {
    pub trending: Vec<Track>,
    pub top: Vec<Track>,
    pub new_albums: Vec<Album>,
    #[serde(default)]
    pub top_artists: Vec<Artist>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub available_countries: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AudioWork {
    Verify,
    Resolve,
    Download,
}

type AudioLocks = Mutex<HashMap<(String, AudioWork), Arc<tokio::sync::Mutex<()>>>>;
type HomeCacheMap = HashMap<Option<String>, (Instant, HomeFeed)>;
type GenresCacheSlot = Option<(Instant, Vec<Genre>)>;
type SponsorBlockCacheMap = HashMap<String, (Instant, Vec<SponsorSegment>)>;

#[derive(Clone)]
pub struct Backend {
    ytm: Arc<tokio::sync::OnceCell<YtMusic<NoAuthToken>>>,
    innertube: InnerTubeClient,
    http: reqwest::Client,
    home_cache: Arc<RwLock<HomeCacheMap>>,
    genres_cache: Arc<RwLock<GenresCacheSlot>>,
    stream_cache: Arc<RwLock<HashMap<String, (Instant, StreamInfo)>>>,
    sponsorblock_cache: Arc<RwLock<SponsorBlockCacheMap>>,
    music_proofs: Arc<RwLock<HashMap<String, (Instant, Track)>>>,
    audio_locks: Arc<AudioLocks>,
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend {
    pub fn new() -> Self {
        crate::tls::ensure_crypto_provider();
        let http = crate::tls::http_client_builder()
            .http2_max_header_list_size(64 * 1024)
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .expect("HTTP client");
        let mut initial_home = HashMap::new();
        if let Some((ts, feed)) = cache_dir().and_then(|dir| parsers::load_home_cache(&dir.join("home.json"))) {
            initial_home.insert(feed.country.clone(), (ts, feed));
        }
        Self {
            ytm: Arc::new(tokio::sync::OnceCell::new()),
            innertube: InnerTubeClient::new(http.clone()),
            http,
            home_cache: Arc::new(RwLock::new(initial_home)),
            genres_cache: Arc::new(RwLock::new(None)),
            stream_cache: Arc::new(RwLock::new(HashMap::new())),
            sponsorblock_cache: Arc::new(RwLock::new(HashMap::new())),
            music_proofs: Arc::default(),
            audio_locks: Arc::default(),
        }
    }

    pub fn set_audio_quality(&self, quality: crate::models::AudioQuality) {
        if self.innertube.audio_quality() != quality {
            self.innertube.set_audio_quality(quality);
            if let Ok(mut cache) = self.stream_cache.write() {
                cache.clear();
            }
        }
    }

    async fn primary(&self) -> Result<&YtMusic<NoAuthToken>, String> {
        self.ytm
            .get_or_try_init(|| async {
                let client = ytmapi_rs::Client::new_from_reqwest_client(self.http.clone());
                ytmapi_rs::YtMusicBuilder::new_with_client(client)
                    .build()
                    .await
                    .map_err(|e| format!("Error inicializando ytmapi-rs: {e:?}"))
            })
            .await
    }

}

pub(crate) use parsers::validate_video_id;
