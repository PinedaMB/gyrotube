use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

use rustypipe::client::RustyPipe;
use rustypipe::model::TrackType;
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
use crate::models::{
    Album, Artist, ArtistDetails, Genre, LyricsData, Playlist, SearchResults, StreamInfo, Track,
    cache_dir, clean_track_title, color_to_hex, parse_lrc,
};

const MUSIC_PROOF_TTL: Duration = Duration::from_secs(15 * 60);
const STREAM_CACHE_TTL: Duration = Duration::from_secs(15 * 60);
const HOME_CACHE_TTL: Duration = Duration::from_secs(15 * 60);

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
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AudioWork {
    Verify,
    Resolve,
    Download,
}

type AudioLocks = Mutex<HashMap<(String, AudioWork), Arc<tokio::sync::Mutex<()>>>>;

#[derive(Clone)]
pub struct Backend {
    ytm: Arc<tokio::sync::OnceCell<YtMusic<NoAuthToken>>>,
    rp: RustyPipe,
    http: reqwest::Client,
    home_cache: Arc<RwLock<Option<(Instant, HomeFeed)>>>,
    genres_cache: Arc<RwLock<Option<(Instant, Vec<Genre>)>>>,
    stream_cache: Arc<RwLock<HashMap<String, (Instant, StreamInfo)>>>,
    music_proofs: Arc<RwLock<HashMap<String, (Instant, Track)>>>,
    audio_locks: Arc<AudioLocks>,
}

impl Backend {
    pub fn new() -> Self {
        let mut rp_builder = RustyPipe::builder()
            .timeout(Duration::from_secs(8))
            .n_http_retries(1);
        if let Some(dir) = cache_dir()
            && std::fs::create_dir_all(&dir).is_ok()
        {
            rp_builder = rp_builder.storage_dir(dir);
        }
        Self {
            ytm: Arc::new(tokio::sync::OnceCell::new()),
            rp: rp_builder.build().unwrap_or_else(|_| RustyPipe::new()),
            http: reqwest::Client::builder()
                .http2_max_header_list_size(64 * 1024)
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(10))
                .build()
                .expect("HTTP client"),
            home_cache: Arc::new(RwLock::new(
                cache_dir().and_then(|dir| load_home_cache(&dir.join("home.json"))),
            )),
            genres_cache: Arc::new(RwLock::new(None)),
            stream_cache: Arc::new(RwLock::new(HashMap::new())),
            music_proofs: Arc::default(),
            audio_locks: Arc::default(),
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

    #[allow(dead_code)]
    pub async fn fetch_home_feed(&self) -> Result<HomeFeed, String> {
        self.fetch_home_feed_cached(false).await
    }

    pub async fn fetch_home_feed_cached(&self, force_refresh: bool) -> Result<HomeFeed, String> {
        if !force_refresh
            && let Ok(guard) = self.home_cache.read()
            && let Some((ts, feed)) = guard.as_ref()
            && ts.elapsed() < HOME_CACHE_TTL
        {
            return Ok(feed.clone());
        }

        let mut trending = Vec::new();
        let mut top = Vec::new();
        let mut new_albums = Vec::new();

        let rp_q = self.rp.query();
        let (charts_res, releases_res) =
            tokio::join!(rp_q.music_charts(None), rp_q.music_new_albums());
        if let Ok(charts) = charts_res {
            trending = charts
                .trending_tracks
                .iter()
                .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
                .take(12)
                .map(Track::from)
                .collect();
            top = charts
                .top_tracks
                .iter()
                .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
                .take(12)
                .map(Track::from)
                .collect();
        }
        if let Ok(releases) = releases_res {
            new_albums = releases.iter().take(12).map(Album::from).collect();
        }

        if trending.is_empty()
            && let Ok(ytm) = self.primary().await
        {
            if let Ok(songs) = ytm.search_songs("top hits").await {
                trending = songs
                    .into_iter()
                    .take(12)
                    .map(track_from_ytm_song)
                    .collect();
            }
            if new_albums.is_empty()
                && let Ok(albums) = ytm.search_albums("new releases").await
            {
                new_albums = albums
                    .into_iter()
                    .take(12)
                    .map(album_from_ytm_album)
                    .collect();
            }
        }

        if top.is_empty() && trending.len() > 6 {
            top = trending.split_off(6);
        }

        let feed = HomeFeed {
            trending,
            top,
            new_albums,
        };
        if !feed.trending.is_empty() || !feed.top.is_empty() || !feed.new_albums.is_empty() {
            if let Ok(mut guard) = self.home_cache.write() {
                *guard = Some((Instant::now(), feed.clone()));
            }
            if let Some(path) = cache_dir().map(|dir| dir.join("home.json"))
                && let Ok(json) = serde_json::to_vec(&feed)
            {
                let _ = std::fs::write(path, json);
            }
        }
        Ok(feed)
    }

    pub async fn fetch_album_tracks(&self, album_id: &str) -> Result<(String, Vec<Track>), String> {
        if let Ok(ytm) = self.primary().await
            && let Ok(album) = ytm.get_album(AlbumID::from_raw(album_id)).await
        {
            let album_name = album.title.clone();
            let artist_names = album
                .artists
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let cover = best_ytm_thumb(&album.thumbnails);
            // Album browse rows can point to clips; its audio playlist carries ATV IDs.
            if let Some(playlist_id) = album.audio_playlist_id
                && let Ok(items) = ytm
                    .get_playlist_tracks(playlist_browse_id(playlist_id.get_raw()))
                    .await
            {
                let mut tracks = tracks_from_ytm_playlist(items);
                for track in &mut tracks {
                    track.album = Some(album_name.clone());
                    if track.artist.is_empty() {
                        track.artist = artist_names.clone();
                    }
                    if track.cover_url.is_none() {
                        track.cover_url = cover.clone();
                        track.cover_large_url = cover.clone();
                    }
                }
                if !tracks.is_empty() {
                    return Ok((album_name, tracks));
                }
            }
        }

        let album = self
            .rp
            .query()
            .music_album(album_id)
            .await
            .map_err(|e| format!("Error cargando álbum: {e}"))?;
        let tracks = album
            .tracks
            .iter()
            .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
            .map(|item| {
                let mut track = Track::from(item);
                if let Some(large) = crate::models::pick_cover(&album.cover, 544) {
                    track.cover_large_url = Some(large);
                }
                if track.cover_url.is_none() {
                    track.cover_url = crate::models::pick_cover(&album.cover, 240);
                }
                track
            })
            .collect();
        Ok((album.name, tracks))
    }

    #[allow(dead_code)]
    pub async fn fetch_genres(&self) -> Result<Vec<Genre>, String> {
        self.fetch_genres_cached(false).await
    }

    pub async fn fetch_genres_cached(&self, force_refresh: bool) -> Result<Vec<Genre>, String> {
        if !force_refresh
            && let Ok(guard) = self.genres_cache.read()
            && let Some((ts, cached)) = guard.as_ref()
            && ts.elapsed() < HOME_CACHE_TTL
        {
            return Ok(cached.clone());
        }

        let ytm = self.primary().await?;
        let sections = ytm
            .get_mood_categories()
            .await
            .map_err(|e| format!("Error obteniendo géneros: {e:?}"))?;

        const PALETTE: &[u32] = &[
            0xFFE01B24, 0xFF1C71D8, 0xFF2EC27E, 0xFFE66100, 0xFF9141AC, 0xFF00B4D8, 0xFFC061CB,
            0xFFF5C211,
        ];
        let mut genres = Vec::new();
        for section in sections {
            for (idx, cat) in section.mood_categories.into_iter().enumerate() {
                let MoodCategory { title, params } = cat;
                genres.push(Genre {
                    id: params.get_raw().to_string(),
                    name: title,
                    color_hex: color_to_hex(PALETTE[(genres.len() + idx) % PALETTE.len()]),
                });
            }
        }

        if let Ok(mut guard) = self.genres_cache.write() {
            *guard = Some((Instant::now(), genres.clone()));
        }
        Ok(genres)
    }

    pub async fn fetch_genre_playlists(
        &self,
        genre_id: &str,
    ) -> Result<(String, Vec<Playlist>), String> {
        let ytm = self.primary().await?;
        match ytm
            .get_mood_playlists(MoodCategoryParams::from_raw(genre_id))
            .await
        {
            Ok(categories) => {
                let name = categories
                    .first()
                    .map(|c| c.category_name.clone())
                    .unwrap_or_default();
                let playlists = categories
                    .into_iter()
                    .flat_map(|c| c.playlists)
                    .map(|pl| Playlist {
                        id: pl.playlist_id.get_raw().to_string(),
                        name: pl.title,
                        subtitle: pl.author,
                        cover_url: best_ytm_thumb(&pl.thumbnails),
                    })
                    .collect();
                Ok((name, playlists))
            }
            Err(primary_err) => {
                let raw = ytm
                    .raw_json_query(ytmapi_rs::query::GetMoodPlaylistsQuery::new(
                        MoodCategoryParams::from_raw(genre_id),
                    ))
                    .await
                    .map_err(|_| {
                        format!("Error obteniendo playlists del género: {primary_err:?}")
                    })?;
                let json: serde_json::Value = serde_json::from_str(&raw).map_err(|_| {
                    format!("Error obteniendo playlists del género: {primary_err:?}")
                })?;
                let parsed = parse_mood_playlists_json(&json);
                if parsed.1.is_empty() {
                    Err(format!("Error obteniendo playlists del género: {primary_err:?}"))
                } else {
                    Ok(parsed)
                }
            }
        }
    }

    pub async fn fetch_playlist_tracks(
        &self,
        playlist_id: &str,
    ) -> Result<(String, Vec<Track>), String> {
        let mut had_entries = false;
        if let Ok(ytm) = self.primary().await {
            let pid = playlist_browse_id(playlist_id);
            let (details, items) = tokio::join!(
                ytm.get_playlist_details(pid.clone()),
                ytm.get_playlist_tracks(pid)
            );
            if let Ok(items) = items {
                had_entries = !items.is_empty();
                let tracks = tracks_from_ytm_playlist(items);
                if !tracks.is_empty() {
                    let title = details.map_or_else(
                        |_| String::from("Playlist"),
                        |details| details.title,
                    );
                    return Ok((title, tracks));
                }
            }
        }

        let query = self.rp.query();
        let mut playlist = query
            .music_playlist(playlist_id.strip_prefix("VL").unwrap_or(playlist_id))
            .await
            .map_err(|e| {
                if had_entries {
                    tr(
                        "La playlist tiene entradas, pero no se pudo confirmar si contiene canciones reproducibles de YouTube Music.",
                        "The playlist has entries, but playable YouTube Music songs could not be confirmed.",
                    )
                    .to_owned()
                } else {
                    format!("Error cargando playlist: {e}")
                }
            })?;

        // shortcut: check up to three continuation pages for audio, extend further on demand if needed.
        for _ in 0..3 {
            if playlist.tracks.is_exhausted() || playlist.tracks.items.iter().any(is_playlist_audio) {
                break;
            }
            if let Err(err) = playlist.tracks.extend(&query).await {
                eprintln!("GyroTube: no se pudo consultar otra página de la playlist: {err}");
                break;
            }
        }
        had_entries |= !playlist.tracks.items.is_empty()
            || playlist.track_count.is_some_and(|count| count > 0);
        let complete = playlist.tracks.is_exhausted();
        let tracks = playlist
            .tracks
            .items
            .iter()
            .filter(|t| is_playlist_audio(t))
            .map(Track::from)
            .collect();
        playlist_audio_result(playlist.name, tracks, had_entries, complete)
    }

    pub async fn fetch_radio_tracks(&self, video_id: &str) -> Result<Vec<Track>, String> {
        if let Ok(radio) = self.rp.query().music_radio_track(video_id).await {
            let mut tracks: Vec<Track> = radio
                .items
                .iter()
                .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
                .map(Track::from)
                .collect();
            if tracks.is_empty() {
                tracks = radio
                    .items
                    .iter()
                    .filter(|t| !t.unavailable)
                    .map(Track::from)
                    .collect();
            }
            if !tracks.is_empty() {
                return Ok(tracks);
            }
        }

        Err(String::from(
            "No se encontraron canciones de YouTube Music para la radio.",
        ))
    }

    pub async fn search_suggestions(&self, query_str: &str) -> Result<Vec<String>, String> {
        if let Ok(ytm) = self.primary().await
            && let Ok(suggestions) = ytm.get_search_suggestions(query_str).await
        {
            let list: Vec<String> = suggestions
                .into_iter()
                .map(|s| {
                    s.runs
                        .into_iter()
                        .map(|r| r.take_text())
                        .collect::<String>()
                })
                .filter(|s| !s.is_empty())
                .collect();
            if !list.is_empty() {
                return Ok(list);
            }
        }
        self.rp
            .query()
            .music_search_suggestion(query_str)
            .await
            .map(|suggestions| suggestions.terms)
            .map_err(|e| format!("Error obteniendo sugerencias: {e}"))
    }

    pub async fn search_all(&self, query_str: &str) -> Result<SearchResults, String> {
        let ytm = self.primary().await.ok();
        let rp = self.rp.query();
        let (tracks, albums, artists, playlists) = tokio::join!(
            async {
                if let Some(ytm) = ytm
                    && let Ok(songs) = ytm.search_songs(query_str).await
                {
                    return Ok(songs
                        .into_iter()
                        .map(track_from_ytm_song)
                        .collect::<Vec<_>>());
                }
                rp.music_search_tracks(query_str).await.map(|results| {
                    results
                        .items
                        .items
                        .iter()
                        .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
                        .map(Track::from)
                        .collect()
                })
            },
            async {
                if let Some(ytm) = ytm
                    && let Ok(albums) = ytm.search_albums(query_str).await
                {
                    return Ok(albums
                        .into_iter()
                        .map(album_from_ytm_album)
                        .collect::<Vec<_>>());
                }
                rp.music_search_albums(query_str)
                    .await
                    .map(|results| results.items.items.iter().map(Album::from).collect())
            },
            async {
                if let Some(ytm) = ytm
                    && let Ok(artists) = ytm.search_artists(query_str).await
                {
                    return Ok(artists
                        .into_iter()
                        .map(artist_from_ytm_artist)
                        .collect::<Vec<_>>());
                }
                rp.music_search_artists(query_str)
                    .await
                    .map(|results| results.items.items.iter().map(Artist::from).collect())
            },
            async {
                if let Some(ytm) = ytm
                    && let Ok(playlists) = ytm.search_playlists(query_str).await
                {
                    return Ok(playlists
                        .into_iter()
                        .filter_map(playlist_from_ytm_playlist)
                        .collect::<Vec<_>>());
                }
                rp.music_search_playlists(query_str, false)
                    .await
                    .map(|results| results.items.items.iter().map(Playlist::from).collect())
            },
        );
        if tracks.is_err() && albums.is_err() && artists.is_err() && playlists.is_err() {
            return Err(String::from(
                "No se pudo completar la búsqueda en YouTube Music.",
            ));
        }
        Ok(SearchResults {
            tracks: tracks.unwrap_or_default(),
            albums: albums.unwrap_or_default(),
            artists: artists.unwrap_or_default(),
            playlists: playlists.unwrap_or_default(),
        })
    }

    pub async fn fetch_artist(&self, artist_id: &str) -> Result<ArtistDetails, String> {
        if let Ok(ytm) = self.primary().await
            && let Ok(artist) = ytm.get_artist(ArtistChannelID::from_raw(artist_id)).await
        {
            let name = artist.name.clone();
            let GetArtistTopReleases {
                songs,
                albums,
                singles,
                ..
            } = artist.top_releases;

            let tracks: Vec<Track> = if let Some(s) = songs {
                let full_tracks = if let Ok(ytm) = self.primary().await
                    && let Ok(items) = ytm
                        .get_playlist_tracks(playlist_browse_id(s.browse_id.get_raw()))
                        .await
                {
                    items
                        .into_iter()
                        .filter_map(|item| match item {
                            PlaylistItem::Song(ps) if ps.is_available => {
                                let artist_str = ps
                                    .artists
                                    .into_iter()
                                    .map(|a| a.name)
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                Some(Track {
                                    is_music: true,
                                    id: ps.video_id.get_raw().to_string(),
                                    title: clean_track_title(&ps.title),
                                    artist: if artist_str.is_empty() {
                                        name.clone()
                                    } else {
                                        artist_str
                                    },
                                    album: Some(ps.album.name),
                                    duration_secs: parse_mm_ss(&ps.duration),
                                    cover_url: best_ytm_thumb(&ps.thumbnails),
                                    cover_large_url: best_ytm_thumb(&ps.thumbnails),
                                })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };

                if !full_tracks.is_empty() {
                    full_tracks
                } else {
                    s.results
                        .into_iter()
                        .map(|t| {
                            let id = t.video_id.get_raw().to_string();
                            let cover = Some(format!("https://i.ytimg.com/vi/{id}/hqdefault.jpg"));
                            Track {
                                is_music: false,
                                id,
                                title: clean_track_title(&t.title),
                                artist: if t.artists.is_empty() {
                                    name.clone()
                                } else {
                                    t.artists
                                        .into_iter()
                                        .map(|a| a.name)
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                },
                                album: Some(t.album.name),
                                duration_secs: None,
                                cover_url: cover.clone(),
                                cover_large_url: cover,
                            }
                        })
                        .collect()
                }
            } else {
                Vec::new()
            };

            let mut all_albums = Vec::new();
            if let Some(a) = albums {
                for item in a.results {
                    all_albums.push(Album {
                        id: item.album_id.get_raw().to_string(),
                        title: item.title,
                        artist: name.clone(),
                        year: item.year.parse::<u16>().ok(),
                        cover_url: best_ytm_thumb(&item.thumbnails),
                    });
                }
            }
            if let Some(s) = singles {
                for item in s.results {
                    all_albums.push(Album {
                        id: item.album_id.get_raw().to_string(),
                        title: item.title,
                        artist: name.clone(),
                        year: item.year.parse::<u16>().ok(),
                        cover_url: best_ytm_thumb(&item.thumbnails),
                    });
                }
            }

            return Ok(ArtistDetails {
                name,
                header_url: best_ytm_banner(&artist.thumbnails),
                subscriber_count: artist.subscribers.as_deref().and_then(parse_sub_count),
                description: artist.description,
                tracks,
                albums: all_albums,
                playlists: Vec::new(),
                similar_artists: Vec::new(),
            });
        }

        if let Ok(rp_artist) = self.rp.query().music_artist(artist_id, true).await {
            let tracks: Vec<Track> = rp_artist
                .tracks
                .iter()
                .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
                .map(Track::from)
                .collect();
            if !tracks.is_empty() || !rp_artist.albums.is_empty() {
                return Ok(ArtistDetails {
                    name: rp_artist.name,
                    header_url: crate::models::pick_cover(&rp_artist.header_image, 1000),
                    description: rp_artist.description,
                    subscriber_count: rp_artist.subscriber_count,
                    tracks,
                    albums: rp_artist.albums.iter().map(Album::from).collect(),
                    playlists: rp_artist.playlists.iter().map(Playlist::from).collect(),
                    similar_artists: rp_artist.similar_artists.iter().map(Artist::from).collect(),
                });
            }
        }

        Err(String::from("Error cargando artista"))
    }

    fn streams_dir() -> Result<std::path::PathBuf, String> {
        let base = cache_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("gyrotube"))
            .join("streams");
        std::fs::create_dir_all(&base)
            .map_err(|e| format!("No se pudo crear caché de audio: {e}"))?;
        Ok(base)
    }

    fn cached_stream(&self, video_id: &str) -> Option<StreamInfo> {
        validate_video_id(video_id).ok()?;
        let dir = Self::streams_dir().ok()?;
        audio_cache::find(video_id, &dir)
    }

    pub fn cached_track_stream(&self, track: &Track) -> Option<StreamInfo> {
        let video_id = &track.id;
        validate_video_id(video_id).ok()?;
        if !track.is_music && self.cached_music_proof(video_id).is_none() {
            return None;
        }
        self.cached_url(video_id).or_else(|| self.cached_stream(video_id))
    }

    pub fn cancel_stale_streams(&self, keep_ids: &[&str]) {
        if let Some(streamer) = crate::audio_streamer::AudioStreamer::try_global() {
            let aborted = streamer.abort_downloads_except(keep_ids);
            if !aborted.is_empty()
                && let Ok(mut cache) = self.stream_cache.write()
            {
                for vid in aborted {
                    cache.remove(&vid);
                }
            }
        }
    }

    fn cached_url(&self, video_id: &str) -> Option<StreamInfo> {
        let cache = self.stream_cache.read().ok()?;
        let (created, stream) = cache.get(video_id)?;
        stream_url_is_fresh(*created, stream).then(|| stream.clone())
    }

    fn audio_lock(
        &self,
        video_id: &str,
        work: AudioWork,
    ) -> Result<Arc<tokio::sync::Mutex<()>>, String> {
        let mut locks = self
            .audio_locks
            .lock()
            .map_err(|_| String::from("Error interno de sincronización"))?;
        locks.retain(|_, lock| Arc::strong_count(lock) > 1);
        Ok(locks.entry((video_id.to_owned(), work)).or_default().clone())
    }

    pub fn trim_audio_cache_on_startup() {
        runtime().spawn_blocking(|| {
            if let Ok(dir) = Self::streams_dir() {
                audio_cache::trim_existing(&dir);
            }
        });
    }

    fn cached_music_proof(&self, video_id: &str) -> Option<Track> {
        let proofs = self.music_proofs.read().ok()?;
        let (checked, track) = proofs.get(video_id)?;
        (checked.elapsed() < MUSIC_PROOF_TTL).then(|| track.clone())
    }

    pub async fn verify_music_track(&self, video_id: &str) -> Result<Track, String> {
        validate_video_id(video_id)?;
        if let Some(track) = self.cached_music_proof(video_id) {
            return Ok(track);
        }
        let lock = self.audio_lock(video_id, AudioWork::Verify)?;
        let _guard = lock.lock().await;
        if let Some(track) = self.cached_music_proof(video_id) {
            return Ok(track);
        }

        let started = Instant::now();
        let result = self
            .rp
            .query()
            .music_details(video_id)
            .await;
        log_audio_timing(video_id, "catalogo", started);
        let details = result.map_err(|e| format!("No se pudo verificar la canción de YouTube Music: {e}"))?;
        let track = verified_music_track(video_id, &details.track)?;
        if let Ok(mut proofs) = self.music_proofs.write() {
            proofs.retain(|_, (checked, _)| checked.elapsed() < MUSIC_PROOF_TTL);
            proofs.insert(video_id.to_owned(), (Instant::now(), track.clone()));
        }
        Ok(track)
    }

    pub async fn resolve_track_stream(&self, track: &Track) -> Result<StreamInfo, String> {
        validate_video_id(&track.id)?;
        if !track.is_music && self.cached_music_proof(&track.id).is_none() {
            let has_cached =
                self.cached_stream(&track.id).is_some() || self.cached_url(&track.id).is_some();
            if has_cached {
                self.verify_music_track(&track.id).await?;
            } else {
                let (verify_res, stream_res) = tokio::join!(
                    self.verify_music_track(&track.id),
                    self.resolve_audio_stream(&track.id)
                );
                if let Err(err) = verify_res {
                    if let Ok(mut cache) = self.stream_cache.write() {
                        cache.remove(&track.id);
                    }
                    return Err(err);
                }
                return stream_res;
            }
        }
        self.resolve_audio_stream(&track.id).await
    }

    async fn extract_fast_ios_stream(
        &self,
        video_id: &str,
    ) -> Option<(StreamInfo, &'static str, Option<u64>)> {
        use rustypipe::client::ClientType;
        use rustypipe::model::AudioFormat;
        let started = Instant::now();
        let player = tokio::time::timeout(
            Duration::from_millis(2500),
            self.rp.query().player_from_client(video_id, ClientType::Ios),
        )
        .await
        .ok()?
        .ok()?;
        // Require itag 251 (Opus WebM ~160kbps) for the fast-start iOS path so byte offsets
        // match the background visionos handover identically to the single byte.
        let stream = player.audio_streams.iter().find(|s| s.itag == 251)?;
        let ext = match stream.format {
            AudioFormat::M4a => "m4a",
            _ => "webm",
        };
        let kbps = (stream.bitrate / 1000).max(128);
        let quality_label = format!("{kbps}kbps ({})", ext.to_uppercase());
        let ua = self.rp.query().user_agent(ClientType::Ios).into_owned();
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), ua.clone());
        let size = (stream.size > 0).then_some(stream.size);
        log_audio_timing(video_id, "extraccion-ios-rapida", started);
        Some((
            StreamInfo {
                url: stream.url.clone(),
                quality_label,
                user_agent: Some(ua),
                headers,
                local_audio: None,
            },
            ext,
            size,
        ))
    }

    async fn resolve_audio_stream(&self, video_id: &str) -> Result<StreamInfo, String> {
        validate_video_id(video_id)?;
        let cached = || self.cached_stream(video_id).or_else(|| self.cached_url(video_id));
        if let Some(stream) = cached() {
            return Ok(stream);
        }
        let lock = self.audio_lock(video_id, AudioWork::Resolve)?;
        let _guard = lock.lock().await;
        if let Some(stream) = cached() {
            return Ok(stream);
        }

        let streams_dir = Self::streams_dir()?;
        let streamer = crate::audio_streamer::AudioStreamer::global().await?;

        // Fast-path: extract stream via pure-Rust iOS client (~140-250ms, no Python/Node startup)
        // to serve the initial audio blocks immediately while AudioStreamer transparently refreshes
        // the URL via visionos in the background before the 1 MB iOS limit.
        if let Some((fast_remote, ext, size_hint)) = self.extract_fast_ios_stream(video_id).await
            && let Ok(local_stream) = streamer
                .start_chunked_http_stream(video_id, fast_remote, ext, size_hint, &streams_dir)
                .await
        {
            if let Ok(mut cache) = self.stream_cache.write() {
                cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
                cache.insert(video_id.to_string(), (Instant::now(), local_stream.clone()));
            }
            return Ok(local_stream);
        }

        #[cfg(target_os = "android")]
        {
            return Err("No se pudo resolver el flujo de audio (Android).".to_string());
        }

        // Fallback: extract stream URL via yt-dlp excluding tv_samsung (which triggers 6s ad / 403)
        // and skipping HLS/DASH manifest downloads.
        #[cfg(not(target_os = "android"))]
        {
        let mut cmd = tokio::process::Command::new("yt-dlp");
        cmd.kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-warnings",
                "--socket-timeout",
                "5",
                "--retries",
                "1",
                "--dump-single-json",
                "--skip-download",
                "--extractor-args",
                "youtube:player_client=default,-tv_samsung;skip=hls,dash",
                "--js-runtimes",
                "node",
                "--remote-components",
                "ejs:github",
                "-f",
                "251/140/bestaudio",
                "--",
            ])
            .arg(format!("https://music.youtube.com/watch?v={video_id}"));
        let started = Instant::now();
        let result = tokio::time::timeout(Duration::from_secs(12), cmd.output()).await;
        log_audio_timing(video_id, "extraccion-yt-dlp", started);
        let mut expected_bytes_hint: Option<u64> = None;
        if let Ok(Ok(output)) = result
            && output.status.success()
        {
            expected_bytes_hint = filesize_from_ytdlp_json(&output.stdout);
            if let Ok(remote) = stream_from_ytdlp_json(&output.stdout) {
                let ext = remote
                    .url
                    .contains("mime=audio%2Fmp4")
                    .then_some("m4a")
                    .unwrap_or("webm");
                if let Ok(local_stream) = streamer
                    .start_chunked_http_stream(
                        video_id,
                        remote,
                        ext,
                        expected_bytes_hint,
                        &streams_dir,
                    )
                    .await
                {
                    if let Ok(mut cache) = self.stream_cache.write() {
                        cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
                        cache.insert(video_id.to_string(), (Instant::now(), local_stream.clone()));
                    }
                    return Ok(local_stream);
                }
            }
        }

        let pipe_started = Instant::now();
        let resolved = streamer
            .start_ytdlp_pipe_stream(video_id, expected_bytes_hint, &streams_dir)
            .await?;
        log_audio_timing(video_id, "stream-tuberia-yt-dlp", pipe_started);

        if let Ok(mut cache) = self.stream_cache.write() {
            cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
            cache.insert(video_id.to_string(), (Instant::now(), resolved.clone()));
        }
        Ok(resolved)
        }
    }

    pub async fn download_with_ytdlp_fallback(&self, video_id: &str) -> Result<StreamInfo, String> {
        validate_video_id(video_id)?;
        let streams_dir = Self::streams_dir()?;
        if let Some(cached) = audio_cache::find(video_id, &streams_dir) {
            return Ok(cached);
        }

        let download_lock = self.audio_lock(video_id, AudioWork::Download)?;
        let _guard = download_lock.lock().await;
        if let Some(cached) = audio_cache::find(video_id, &streams_dir) {
            return Ok(cached);
        }
        self.run_ytdlp_download(video_id, &streams_dir).await
    }

    #[cfg(target_os = "android")]
    async fn run_ytdlp_download(
        &self,
        _video_id: &str,
        _streams_dir: &std::path::Path,
    ) -> Result<StreamInfo, String> {
        Err("La descarga de respaldo con yt-dlp no está disponible en Android.".to_string())
    }

    #[cfg(not(target_os = "android"))]
    async fn run_ytdlp_download(
        &self,
        video_id: &str,
        streams_dir: &std::path::Path,
    ) -> Result<StreamInfo, String> {
        if let Some(cached) = audio_cache::find(video_id, streams_dir) {
            return Ok(cached);
        }

        let staging = tempfile::tempdir_in(streams_dir)
            .map_err(|e| format!("Error al preparar descarga de audio: {e}"))?;

        let mut command = tokio::process::Command::new("yt-dlp");
        command
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-progress",
                "--socket-timeout",
                "5",
                "--retries",
                "1",
                "--concurrent-fragments",
                "4",
                "--extractor-args",
                "youtube:player_client=default,-tv_samsung;skip=hls,dash",
                "--js-runtimes",
                "node",
                "--remote-components",
                "ejs:github",
            ])
            .args(["-f", "251/140/bestaudio", "-o"])
            .arg(staging.path().join("audio.%(ext)s"))
            .arg("--")
            .arg(format!("https://music.youtube.com/watch?v={video_id}"));
        let started = Instant::now();
        let result = tokio::time::timeout(Duration::from_secs(90), command.output()).await;
        log_audio_timing(video_id, "descarga-de-respaldo", started);
        let output = result
            .map_err(|_| String::from("Tiempo de espera agotado en yt-dlp"))?
            .map_err(|e| format!("Error al ejecutar yt-dlp: {e}"))?;

        if !output.status.success() {
            eprintln!(
                "yt-dlp ({video_id}): {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(String::from("yt-dlp no pudo descargar el audio."));
        }

        let (downloaded, extension) = std::fs::read_dir(staging.path())
            .ok()
            .and_then(|entries| {
                entries.filter_map(Result::ok).find_map(|entry| {
                    let name = entry.file_name();
                    let extension = name.to_str()?.strip_prefix("audio.")?;
                    (audio_cache::audio_extension(extension)
                        && entry.metadata().is_ok_and(|m| m.is_file() && m.len() > 0))
                        .then(|| (entry.path(), extension.to_owned()))
                })
            })
            .ok_or_else(|| String::from("yt-dlp no generó archivo de audio."))?;
        let stream = audio_cache::finish(streams_dir, video_id, downloaded, &extension, staging);
        if let Ok(mut guard) = self.stream_cache.write() {
            guard.remove(video_id);
        }
        Ok(stream)
    }

    pub async fn prefetch_audio(&self, track: &Track) {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let _ = self.resolve_track_stream(track).await;
    }

    pub async fn fetch_synced_lyrics(&self, track: &Track) -> Result<LyricsData, String> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LrcLibResp {
            synced_lyrics: Option<String>,
            plain_lyrics: Option<String>,
        }

        let artist_q = track
            .artist
            .split(',')
            .next()
            .unwrap_or(&track.artist)
            .trim();
        let mut params = vec![
            ("artist_name", artist_q.to_string()),
            ("track_name", track.title.clone()),
        ];
        if let Some(ref album) = track.album {
            params.push(("album_name", album.clone()));
        }
        if let Some(dur) = track.duration_secs {
            params.push(("duration", dur.to_string()));
        }

        if let Ok(resp) = self
            .http
            .get("https://lrclib.net/api/get")
            .query(&params)
            .header(reqwest::header::USER_AGENT, "GyroTube/0.1 (Linux GTK4)")
            .send()
            .await
            && resp.status().is_success()
            && let Ok(body) = resp.json::<LrcLibResp>().await
        {
            if let Some(synced) = body.synced_lyrics.filter(|s| !s.trim().is_empty()) {
                let lines = parse_lrc(&synced);
                if !lines.is_empty() {
                    return Ok(LyricsData::Synced(lines));
                }
            }
            if let Some(plain) = body.plain_lyrics.filter(|s| !s.trim().is_empty()) {
                return Ok(LyricsData::Plain(plain));
            }
        }

        if let Ok(resp) = self
            .http
            .get("https://lrclib.net/api/search")
            .query(&[("q", format!("{} {}", artist_q, track.title))])
            .header(reqwest::header::USER_AGENT, "GyroTube/0.1 (Linux GTK4)")
            .send()
            .await
            && resp.status().is_success()
            && let Ok(list) = resp.json::<Vec<LrcLibResp>>().await
        {
            for item in list {
                if let Some(synced) = item.synced_lyrics.filter(|s| !s.trim().is_empty()) {
                    let lines = parse_lrc(&synced);
                    if !lines.is_empty() {
                        return Ok(LyricsData::Synced(lines));
                    }
                }
                if let Some(plain) = item.plain_lyrics.filter(|s| !s.trim().is_empty()) {
                    return Ok(LyricsData::Plain(plain));
                }
            }
        }

        let plain = self.fetch_lyrics(&track.id).await?;
        Ok(LyricsData::Plain(plain))
    }

    pub async fn fetch_lyrics(&self, video_id: &str) -> Result<String, String> {
        if let Ok(ytm) = self.primary().await
            && let Ok(lyrics_id) = ytm.get_lyrics_id(VideoID::from_raw(video_id)).await
            && let Ok(lyrics) = ytm.get_lyrics(lyrics_id).await
        {
            let text = lyrics.lyrics.trim().to_string();
            if !text.is_empty() {
                return Ok(text);
            }
        }

        if let Ok(details) = self.rp.query().music_details(video_id).await
            && let Some(id) = details.lyrics_id
            && let Ok(lyrics) = self.rp.query().music_lyrics(id).await
            && !lyrics.body.trim().is_empty()
        {
            return Ok(lyrics.body);
        }

        Err(String::from("No hay letras disponibles para esta pista."))
    }

    pub async fn fetch_image_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        let resp = self
            .http
            .get(url)
            .header(reqwest::header::USER_AGENT, "GyroTube/0.1 (Linux GTK4)")
            .send()
            .await
            .map_err(|e| format!("Error de imagen: {e}"))?;
        if !resp.status().is_success() {
            return Err(format!("Imagen HTTP {}", resp.status()));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| format!("Error leyendo imagen: {e}"))?;
        Ok(bytes.to_vec())
    }

    pub async fn resolve_ytm_cover(&self, track: &Track) -> Option<String> {
        // Exact ID only: another search result can be a different recording.
        let details = self.rp.query().music_details(&track.id).await.ok()?;
        if details.track.id != track.id {
            return None;
        }
        if let Some(cover) = crate::models::pick_cover(&details.track.cover, 800) {
            return Some(cover);
        }
        let album = details.track.album?;
        let album = self.rp.query().music_album(album.id).await.ok()?;
        crate::models::pick_cover(&album.cover, 800)
    }
}

fn load_home_cache(path: &std::path::Path) -> Option<(Instant, HomeFeed)> {
    let age = path.metadata().ok()?.modified().ok()?.elapsed().ok()?;
    if age >= HOME_CACHE_TTL {
        return None;
    }
    let feed = serde_json::from_slice(&std::fs::read(path).ok()?).ok()?;
    Some((Instant::now().checked_sub(age)?, feed))
}

fn stream_url_is_fresh(created: Instant, stream: &StreamInfo) -> bool {
    if created.elapsed() >= STREAM_CACHE_TTL {
        return false;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    reqwest::Url::parse(&stream.url).ok().is_some_and(|url| {
        url.query_pairs()
            .find(|(key, _)| key == "expire")
            .map(|(_, value)| value.parse::<u64>().is_ok_and(|expires| expires > now + 30))
            .unwrap_or(true)
    })
}

fn verified_music_track(
    video_id: &str,
    item: &rustypipe::model::TrackItem,
) -> Result<Track, String> {
    if item.id != video_id || item.track_type != TrackType::Track || item.unavailable {
        return Err(String::from(
            "Solo se reproducen canciones disponibles del catálogo de YouTube Music.",
        ));
    }
    Ok(Track::from(item))
}

pub(crate) fn validate_video_id(video_id: &str) -> Result<(), String> {
    if video_id.len() == 11
        && video_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
    {
        Ok(())
    } else {
        Err(String::from("ID de video inválido"))
    }
}

#[cfg(not(target_os = "android"))]
fn filesize_from_ytdlp_json(raw: &[u8]) -> Option<u64> {
    let val: serde_json::Value = serde_json::from_slice(raw).ok()?;
    val.get("filesize")
        .and_then(|v| v.as_u64())
        .or_else(|| val.get("filesize_approx").and_then(|v| v.as_u64()))
        .filter(|&sz| sz > 0)
}

#[cfg(not(target_os = "android"))]
fn stream_from_ytdlp_json(raw: &[u8]) -> Result<StreamInfo, String> {
    #[derive(serde::Deserialize)]
    struct Audio {
        url: String,
        ext: String,
        vcodec: String,
        acodec: String,
        #[serde(default)]
        abr: Option<f64>,
        #[serde(default)]
        tbr: Option<f64>,
        #[serde(default)]
        http_headers: HashMap<String, String>,
    }
    let audio: Audio =
        serde_json::from_slice(raw).map_err(|e| format!("Respuesta de yt-dlp inválida: {e}"))?;
    let url = reqwest::Url::parse(&audio.url).map_err(|e| format!("URL de audio inválida: {e}"))?;
    if !matches!(url.scheme(), "https" | "http")
        || url.host_str().is_none()
        || audio.vcodec != "none"
        || audio.acodec == "none"
        || audio.acodec.is_empty()
    {
        return Err(String::from(
            "yt-dlp no devolvió una fuente de solo audio HTTP(S).",
        ));
    }
    let ext_upper = audio.ext.to_uppercase();
    let kbps = audio
        .abr
        .or(audio.tbr)
        .map(|b| b.round() as u32)
        .filter(|&b| b > 0);
    let quality_label = match kbps {
        Some(k) => format!("{k}kbps ({ext_upper})"),
        None => match ext_upper.as_str() {
            "WEBM" | "OPUS" => format!("160kbps ({ext_upper})"),
            "M4A" | "MP4" => format!("128kbps ({ext_upper})"),
            _ => ext_upper,
        },
    };
    Ok(StreamInfo {
        url: audio.url,
        quality_label,
        user_agent: audio
            .http_headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("user-agent"))
            .map(|(_, v)| v.clone()),
        headers: audio.http_headers,
        local_audio: None,
    })
}

fn best_ytm_thumb(thumbs: &[ytmapi_rs::common::Thumbnail]) -> Option<String> {
    thumbs
        .iter()
        .max_by_key(|t| t.width.saturating_mul(t.height))
        .map(|t| crate::models::upgrade_cover_url(&t.url, 544))
}

fn best_ytm_banner(thumbs: &[ytmapi_rs::common::Thumbnail]) -> Option<String> {
    thumbs
        .iter()
        .max_by_key(|t| t.width.saturating_mul(t.height))
        .map(|t| crate::models::upgrade_cover_url(&t.url, 1280))
}

fn runs_text(val: &serde_json::Value) -> String {
    val.get("runs")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|run| run.get("text").and_then(|t| t.as_str()))
                .collect::<String>()
        })
        .unwrap_or_default()
}

fn best_json_thumb(thumbs: &serde_json::Value) -> Option<String> {
    thumbs
        .as_array()?
        .iter()
        .filter_map(|t| {
            let url = t.get("url")?.as_str()?;
            let w = t.get("width").and_then(|v| v.as_u64()).unwrap_or(0);
            let h = t.get("height").and_then(|v| v.as_u64()).unwrap_or(0);
            Some((w.saturating_mul(h), url))
        })
        .max_by_key(|(area, _)| *area)
        .map(|(_, url)| crate::models::upgrade_cover_url(url, 544))
}

fn collect_mood_playlists(node: &serde_json::Value, out: &mut Vec<Playlist>) {
    match node {
        serde_json::Value::Object(map) => {
            if let Some(item) = map.get("musicTwoRowItemRenderer") {
                let browse_id = item
                    .pointer("/navigationEndpoint/browseEndpoint/browseId")
                    .and_then(|v| v.as_str())
                    .map(|s| s.strip_prefix("VL").unwrap_or(s).to_string())
                    .or_else(|| {
                        item.pointer("/thumbnailOverlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchPlaylistEndpoint/playlistId")
                            .and_then(|v| v.as_str())
                            .map(ToString::to_string)
                    });
                let title = item.get("title").map(runs_text).unwrap_or_default();
                if let Some(id) = browse_id
                    && !id.is_empty()
                    && !title.is_empty()
                {
                    let subtitle = item.get("subtitle").map(runs_text).unwrap_or_default();
                    let cover_url = item
                        .pointer("/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails")
                        .and_then(best_json_thumb);
                    out.push(Playlist {
                        id,
                        name: title,
                        subtitle,
                        cover_url,
                    });
                }
            } else {
                for v in map.values() {
                    collect_mood_playlists(v, out);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                collect_mood_playlists(v, out);
            }
        }
        _ => {}
    }
}

fn parse_mood_playlists_json(json: &serde_json::Value) -> (String, Vec<Playlist>) {
    let name = json
        .pointer("/header/musicHeaderRenderer/title")
        .or_else(|| json.pointer("/header/musicVisualHeaderRenderer/title"))
        .map(runs_text)
        .unwrap_or_default();
    let mut playlists = Vec::new();
    if let Some(contents) = json.get("contents") {
        collect_mood_playlists(contents, &mut playlists);
    }
    (name, playlists)
}

fn is_playlist_audio(track: &rustypipe::model::TrackItem) -> bool {
    track.track_type == TrackType::Track && !track.unavailable
}

fn playlist_audio_result(
    title: String,
    tracks: Vec<Track>,
    had_entries: bool,
    complete: bool,
) -> Result<(String, Vec<Track>), String> {
    if !tracks.is_empty() || (!had_entries && complete) {
        return Ok((title, tracks));
    }
    let message = if complete {
        tr(
            "La playlist no contiene canciones reproducibles de YouTube Music. Se omiten videos y pistas no disponibles.",
            "This playlist has no playable YouTube Music songs. Videos and unavailable tracks are skipped.",
        )
    } else {
        tr(
            "No se encontraron canciones reproducibles de YouTube Music en las páginas consultadas.",
            "No playable YouTube Music songs were found in the pages checked.",
        )
    };
    Err(message.into())
}

fn playlist_browse_id(id: &str) -> PlaylistID<'static> {
    PlaylistID::from_raw(format!("VL{}", id.strip_prefix("VL").unwrap_or(id)))
}

fn tracks_from_ytm_playlist(items: Vec<PlaylistItem>) -> Vec<Track> {
    items
        .into_iter()
        .filter_map(|item| match item {
            PlaylistItem::Song(song) if song.is_available => {
                let cover = best_ytm_thumb(&song.thumbnails);
                Some(Track {
                    is_music: true,
                    id: song.video_id.get_raw().to_string(),
                    title: clean_track_title(&song.title),
                    artist: song
                        .artists
                        .into_iter()
                        .map(|a| a.name)
                        .collect::<Vec<_>>()
                        .join(", "),
                    album: Some(song.album.name),
                    duration_secs: parse_mm_ss(&song.duration),
                    cover_url: cover.clone(),
                    cover_large_url: cover,
                })
            }
            _ => None,
        })
        .collect()
}

fn track_from_ytm_song(song: SearchResultSong) -> Track {
    let cover = best_ytm_thumb(&song.thumbnails);
    Track {
        is_music: false,
        id: song.video_id.get_raw().to_string(),
        title: clean_track_title(&song.title),
        artist: if song.artist.is_empty() {
            String::from("Artista desconocido")
        } else {
            song.artist
        },
        album: song.album.map(|a| a.name),
        duration_secs: parse_mm_ss(&song.duration),
        cover_url: cover.clone(),
        cover_large_url: cover,
    }
}

fn album_from_ytm_album(alb: SearchResultAlbum) -> Album {
    Album {
        id: alb.album_id.get_raw().to_string(),
        title: alb.title,
        artist: if alb.artist.is_empty() {
            String::from("Artista desconocido")
        } else {
            alb.artist
        },
        year: alb.year.parse::<u16>().ok(),
        cover_url: best_ytm_thumb(&alb.thumbnails),
    }
}

fn artist_from_ytm_artist(art: SearchResultArtist) -> Artist {
    Artist {
        id: art.browse_id.get_raw().to_string(),
        name: art.artist,
        avatar_url: best_ytm_thumb(&art.thumbnails),
        subscriber_count: art.subscribers.as_deref().and_then(parse_sub_count),
    }
}

fn playlist_from_ytm_playlist(pl: SearchResultPlaylist) -> Option<Playlist> {
    match pl {
        SearchResultPlaylist::Featured(f) => Some(Playlist {
            id: f.playlist_id.get_raw().to_string(),
            name: f.title,
            subtitle: f.author,
            cover_url: best_ytm_thumb(&f.thumbnails),
        }),
        SearchResultPlaylist::Community(c) => Some(Playlist {
            id: c.playlist_id.get_raw().to_string(),
            name: c.title,
            subtitle: c.author,
            cover_url: best_ytm_thumb(&c.thumbnails),
        }),
        SearchResultPlaylist::Podcast(p) => Some(Playlist {
            id: p.podcast_id.get_raw().to_string(),
            name: p.title,
            subtitle: p.publisher,
            cover_url: best_ytm_thumb(&p.thumbnails),
        }),
        _ => None,
    }
}

fn parse_mm_ss(raw: &str) -> Option<u32> {
    let parts: Vec<u32> = raw
        .trim()
        .split(':')
        .map(|p| p.parse::<u32>().ok())
        .collect::<Option<Vec<_>>>()?;
    match parts.as_slice() {
        [m, s] => Some(m * 60 + s),
        [h, m, s] => Some(h * 3600 + m * 60 + s),
        _ => None,
    }
}

fn parse_sub_count(raw: &str) -> Option<u64> {
    let s = raw.trim().to_uppercase();
    let num_end = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == ','))
        .unwrap_or(s.len());
    if num_end == 0 {
        return None;
    }
    let num_str = s[..num_end].replace(',', ".");
    let val = num_str.parse::<f64>().ok()?;
    let rest = s[num_end..].trim_start();
    let unit = rest.split_whitespace().next().unwrap_or("");
    let mult = if unit == "B" || unit.starts_with("MRD") || unit.starts_with("BILL") {
        1_000_000_000.0
    } else if unit == "M" || unit.starts_with("MILL") || unit == "MLN" {
        1_000_000.0
    } else if unit == "K" || unit == "MIL" {
        1_000.0
    } else {
        1.0
    };
    Some((val * mult) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn playback_reuses_a_url_published_by_an_in_flight_preload() {
        use std::future::{Future, poll_fn};
        use std::task::Poll;

        let backend = Backend::new();
        let id = "timingTest_";
        let preload = backend.audio_lock(id, AudioWork::Resolve).unwrap();
        let guard = preload.lock().await;
        let mut playback = std::pin::pin!(backend.resolve_audio_stream(id));
        assert!(poll_fn(|cx| Poll::Ready(playback.as_mut().poll(cx).is_pending())).await);
        let stream = StreamInfo {
            url: "https://example.com/preloaded-audio".into(),
            quality_label: "OPUS".into(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };
        backend
            .stream_cache
            .write()
            .unwrap()
            .insert(id.into(), (Instant::now(), stream.clone()));
        drop(guard);
        assert_eq!(playback.await.unwrap().url, stream.url);
    }

    #[test]
    fn unverified_tracks_cannot_use_a_cached_url() {
        let backend = Backend::new();
        let mut track: Track = serde_json::from_value(serde_json::json!({
            "id":"video123456", "title":"Song", "artist":"Artist"
        }))
        .unwrap();
        backend.stream_cache.write().unwrap().insert(
            track.id.clone(),
            (
                Instant::now(),
                StreamInfo {
                    url: "https://example.com/audio".into(),
                    quality_label: "OPUS".into(),
                    user_agent: None,
                    headers: HashMap::new(),
                    local_audio: None,
                },
            ),
        );
        assert!(backend.cached_track_stream(&track).is_none());
        track.is_music = true;
        assert!(backend.cached_track_stream(&track).is_some());
    }

    #[test]
    fn cached_url_for_a_search_result_requires_a_fresh_exact_id_proof() {
        let backend = Backend::new();
        let track: Track = serde_json::from_value(serde_json::json!({
            "id": "timingTest_", "title": "Song", "artist": "Artist"
        })).unwrap();
        let url = "https://example.com/audio";
        backend.stream_cache.write().unwrap().insert(
            track.id.clone(),
            (Instant::now(), StreamInfo {
                url: url.into(),
                quality_label: "OPUS".into(),
                user_agent: None,
                headers: HashMap::new(),
                local_audio: None,
            }),
        );
        assert!(backend.cached_track_stream(&track).is_none());

        let mut proof = track.clone();
        proof.is_music = true;
        let mut other = track.clone();
        other.id = "anotherTest".into();
        backend.music_proofs.write().unwrap().insert(
            other.id.clone(),
            (Instant::now(), other),
        );
        assert!(backend.cached_track_stream(&track).is_none());

        backend.music_proofs.write().unwrap().insert(
            track.id.clone(),
            (Instant::now(), proof.clone()),
        );
        assert_eq!(backend.cached_track_stream(&track).unwrap().url, url);
        backend.music_proofs.write().unwrap().insert(
            track.id.clone(),
            (Instant::now() - MUSIC_PROOF_TTL, proof),
        );
        assert!(backend.cached_track_stream(&track).is_none());
    }

    #[tokio::test]
    async fn cached_audio_cannot_bypass_failed_catalog_verification() {
        let mut backend = Backend::new();
        backend.rp = RustyPipe::builder()
            .timeout(Duration::ZERO)
            .n_http_retries(1)
            .no_storage()
            .visitor_data("test")
            .build().unwrap();
        let mut track: Track = serde_json::from_value(serde_json::json!({
            "id":"video123456", "title":"Song", "artist":"Artist"
        }))
        .unwrap();
        backend.stream_cache.write().unwrap().insert(
            track.id.clone(),
            (
                Instant::now(),
                StreamInfo {
                    url: "https://example.com/audio".into(),
                    quality_label: "OPUS".into(),
                    user_agent: None,
                    headers: HashMap::new(),
                    local_audio: None,
                },
            ),
        );
        assert!(backend.resolve_track_stream(&track).await.is_err());
        track.is_music = true;
        assert_eq!(
            backend.resolve_track_stream(&track).await.unwrap().url,
            "https://example.com/audio"
        );
    }

    #[test]
    fn guest_claims_do_not_override_catalog_metadata_or_type() {
        let claimed: Track = serde_json::from_value(serde_json::json!({
            "id":"audio123456", "title":"Forged", "artist":"Forged",
            "verified_catalog_audio":true, "cover_url":"http://127.0.0.1/private"
        }))
        .unwrap();
        let mut item: rustypipe::model::TrackItem = serde_json::from_value(serde_json::json!({
            "id":claimed.id, "name":"Catalog song", "cover":[], "artists":[],
            "track_type":"video", "by_va":false, "unavailable":false
        }))
        .unwrap();
        assert!(verified_music_track(&claimed.id, &item).is_err());
        item.track_type = TrackType::Episode;
        assert!(verified_music_track(&claimed.id, &item).is_err());
        item.track_type = TrackType::Track;
        item.unavailable = true;
        assert!(verified_music_track(&claimed.id, &item).is_err());
        item.unavailable = false;
        assert!(verified_music_track("other123456", &item).is_err());
        let verified = verified_music_track(&claimed.id, &item).unwrap();
        assert_eq!(verified.title, "Catalog song");
        assert!(verified.is_music);
        assert_eq!(verified.cover_url, None);
    }

    #[test]
    fn playlist_browse_requests_require_exactly_one_vl_prefix() {
        assert_eq!(playlist_browse_id("OLAK-test").get_raw(), "VLOLAK-test");
        assert_eq!(playlist_browse_id("VLOLAK-test").get_raw(), "VLOLAK-test");
    }

    #[test]
    fn audio_playlist_excludes_clips_and_unavailable_songs() {
        let song = serde_json::json!({"Song": {
            "video_id":"audio123456", "track_no":1,
            "album":{"name":"Album", "id":"MPRE-test"},
            "duration":"3:00", "library_management":null, "title":"Song",
            "artists":[{"name":"Artist", "id":null}], "like_status":"INDIFFERENT",
            "thumbnails":[], "explicit":"NotExplicit", "is_available":true,
            "playlist_id":"OLAK-test"
        }});
        let mut unavailable = song.clone();
        unavailable["Song"]["is_available"] = serde_json::json!(false);
        let video = serde_json::json!({"Video": {
            "video_id":"video123456", "track_no":1, "duration":"4:00", "title":"Song",
            "channel_name":"Artist", "channel_id":"UC-test", "like_status":"INDIFFERENT",
            "thumbnails":[], "is_available":true, "playlist_id":"OLAK-test"
        }});
        let items = serde_json::from_value(serde_json::json!([video, song, unavailable])).unwrap();
        let tracks = tracks_from_ytm_playlist(items);
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].id, "audio123456");
        assert_eq!(tracks[0].duration_secs, Some(180));
        assert!(tracks[0].is_music);
    }

    #[test]
    fn playlists_with_filtered_or_unchecked_entries_are_not_reported_as_empty() {
        assert!(
            playlist_audio_result("Lista".into(), vec![], false, true)
                .unwrap()
                .1
                .is_empty()
        );
        let filtered = playlist_audio_result("Lista".into(), vec![], true, true).unwrap_err();
        let incomplete = playlist_audio_result("Lista".into(), vec![], true, false).unwrap_err();
        assert_ne!(filtered, incomplete);
        assert!(playlist_audio_result("Lista".into(), vec![], false, false).is_err());

        let mut item: rustypipe::model::TrackItem = serde_json::from_value(serde_json::json!({
            "id":"audio123456", "name":"Song", "cover":[], "artists":[],
            "track_type":"video", "by_va":false, "unavailable":false
        })).unwrap();
        assert!(!is_playlist_audio(&item));
        item.track_type = TrackType::Track;
        let tracks = [&item]
            .into_iter()
            .filter(|t| is_playlist_audio(t))
            .map(Track::from)
            .collect();
        let (_, tracks) = playlist_audio_result("Lista".into(), tracks, true, false).unwrap();
        assert_eq!(tracks[0].id, "audio123456");
        item.unavailable = true;
        assert!(!is_playlist_audio(&item));
    }

    #[tokio::test]
    #[ignore = "Requires YouTube Music access"]
    async fn live_album_uses_audio_playlist() {
        let backend = Backend::new();
        let ytm = backend.primary().await.unwrap();
        let albums = ytm
            .search_albums("La Casa De Mis Sueños Santiago Benavides")
            .await
            .unwrap();
        let album = albums
            .iter()
            .find(|a| a.title.to_lowercase() == "la casa de mis sueños")
            .expect("album from saved session");
        let id = album.album_id.get_raw();
        let browse = ytm.get_album(AlbumID::from_raw(id)).await.unwrap();
        let audio_playlist = browse.audio_playlist_id.expect("album audio playlist");
        let expected = tracks_from_ytm_playlist(
            ytm.get_playlist_tracks(playlist_browse_id(audio_playlist.get_raw()))
                .await
                .unwrap(),
        );
        let (title, tracks) = backend.fetch_album_tracks(id).await.unwrap();
        assert!(!tracks.is_empty());
        for track in &tracks {
            assert!(expected.iter().any(|audio| audio.id == track.id));
            assert!(track.is_music);
        }
        let audio = tracks
            .iter()
            .find(|t| t.title.to_lowercase() == "me habla de amor")
            .unwrap();
        let old = backend
            .rp
            .query()
            .music_details("zuFs_gXr_Wk")
            .await
            .unwrap();
        let current = backend.rp.query().music_details(&audio.id).await.unwrap();
        eprintln!(
            "Álbum: {title}; ID anterior: {} ({:?}); ID de audio: {} ({:?})",
            old.track.id, old.track.track_type, current.track.id, current.track.track_type
        );
        assert_eq!(current.track.track_type, TrackType::Track);
        if old.track.track_type == TrackType::Video {
            assert_ne!(old.track.id, audio.id);
        }
    }

    #[test]
    fn reuses_home_snapshot_only_while_fresh() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("home.json");
        assert!(load_home_cache(&path).is_none());
        let feed = HomeFeed {
            trending: vec![],
            top: vec![],
            new_albums: vec![],
        };
        std::fs::write(&path, serde_json::to_vec(&feed).unwrap()).unwrap();
        assert!(load_home_cache(&path).is_some());
        let file = std::fs::File::options().write(true).open(&path).unwrap();
        file.set_times(
            std::fs::FileTimes::new().set_modified(
                std::time::SystemTime::now() - HOME_CACHE_TTL - Duration::from_secs(1),
            ),
        )
        .unwrap();
        assert!(load_home_cache(&path).is_none());
        std::fs::write(&path, b"interrupted write").unwrap();
        assert!(load_home_cache(&path).is_none());
    }

    #[test]
    fn preloaded_url_survives_a_song_and_expires_safely() {
        let mut stream = StreamInfo {
            url: "https://example.com/audio".into(),
            quality_label: "OPUS".into(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };
        assert!(stream_url_is_fresh(
            Instant::now() - Duration::from_secs(240),
            &stream
        ));
        assert!(!stream_url_is_fresh(
            Instant::now() - STREAM_CACHE_TTL,
            &stream
        ));
        stream.url.push_str("?expire=123");
        assert!(!stream_url_is_fresh(Instant::now(), &stream));
    }

    #[test]
    fn extracts_audio_url_and_required_headers() {
        let stream = stream_from_ytdlp_json(
            br#"{
            "url":"https://rr1.googlevideo.com/videoplayback?expire=123",
            "ext":"webm","vcodec":"none","acodec":"opus",
            "http_headers":{"User-Agent":"test-agent","Referer":"https://music.youtube.com/"}
        }"#,
        )
        .unwrap();
        assert!(stream.url.starts_with("https://rr1.googlevideo.com/"));
        assert_eq!(stream.quality_label, "160kbps (WEBM)");
        assert_eq!(stream.user_agent.as_deref(), Some("test-agent"));
        assert_eq!(stream.headers["Referer"], "https://music.youtube.com/");
    }

    #[test]
    fn rejects_non_audio_or_non_http_extractor_output() {
        for (url, video, audio) in [
            ("file:///etc/passwd", "none", "opus"),
            ("https://example.com/", "avc1", "opus"),
            ("https://example.com/", "none", "none"),
        ] {
            let raw = serde_json::json!({"url":url,"vcodec":video,"acodec":audio,"ext":"webm"});
            assert!(stream_from_ytdlp_json(&serde_json::to_vec(&raw).unwrap()).is_err());
        }
        assert!(stream_from_ytdlp_json(b"{}").is_err());
    }

    #[test]
    fn rejects_invalid_ids_before_cache_or_process_access() {
        for id in ["../secret", "--help", "1234567890/", "", "123456789012"] {
            assert!(validate_video_id(id).is_err());
        }
        assert!(validate_video_id("Abc_123-xyz").is_ok());
    }

    #[test]
    fn selects_largest_cover_without_assuming_thumbnail_order() {
        let thumbs: Vec<ytmapi_rs::common::Thumbnail> = serde_json::from_value(serde_json::json!([
            {"url":"large", "width":800, "height":800},
            {"url":"small", "width":60, "height":60}
        ]))
        .unwrap();
        assert_eq!(best_ytm_thumb(&thumbs).as_deref(), Some("large"));
    }

    #[tokio::test]
    #[ignore = "Requires YouTube access and yt-dlp in PATH"]
    async fn live_search_cover_and_audio() {
        let backend = Backend::new();
        let start = Instant::now();
        let results = backend.search_all("Daft Punk Get Lucky").await.unwrap();
        let track = results.tracks.first().expect("music result");
        eprintln!("Búsqueda: {:?}, pista: {}", start.elapsed(), track.id);
        let cover = track
            .cover_large_url
            .as_deref()
            .or(track.cover_url.as_deref())
            .unwrap();
        let image = backend.fetch_image_bytes(cover).await.unwrap();
        assert!(image.len() > 256);
        let cold_start = Instant::now();
        let stream = backend.resolve_track_stream(track).await.unwrap();
        eprintln!("Resolución inicial de audio: {:?}", cold_start.elapsed());
        assert!(!stream.url.is_empty());
        let start = Instant::now();
        assert_eq!(
            backend.resolve_track_stream(track).await.unwrap().url,
            stream.url
        );
        eprintln!("Resolución con URL precargada: {:?}", start.elapsed());
    }

    #[test]
    fn find_cached_audio_reuses_local_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file = dir.path().join("dQw4w9WgXcQ.m4a");
        std::fs::write(&file, b"audio-bytes").expect("write cached audio");

        let found = audio_cache::find("dQw4w9WgXcQ", dir.path()).expect("cached audio");
        assert_eq!(found.url, file.to_string_lossy());
        assert_eq!(found.quality_label, "128kbps (M4A)");
    }

    #[test]
    fn parses_mm_ss_and_sub_counts() {
        assert_eq!(parse_mm_ss("3:45"), Some(225));
        assert_eq!(parse_mm_ss("1:02:03"), Some(3723));
        assert_eq!(parse_sub_count("9.4M subscribers"), Some(9_400_000));
        assert_eq!(parse_sub_count("120K"), Some(120_000));
        assert_eq!(parse_sub_count("176K monthly audience"), Some(176_000));
        assert_eq!(parse_sub_count("176 mil suscriptores"), Some(176_000));
        assert_eq!(parse_sub_count("1,5 M de suscriptores"), Some(1_500_000));
        assert_eq!(parse_sub_count("721 monthly listeners"), Some(721));
    }

    #[test]
    fn parses_mood_playlists_from_raw_json() {
        let json = serde_json::json!({
            "header": {
                "musicHeaderRenderer": {
                    "title": { "runs": [{ "text": "Cumbia" }] }
                }
            },
            "contents": {
                "singleColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "gridRenderer": {
                                            "items": [{
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Cumbia Sonidera" }] },
                                                    "subtitle": { "runs": [{ "text": "YouTube Music" }] },
                                                    "navigationEndpoint": {
                                                        "browseEndpoint": { "browseId": "VLRDCLAK5uy_test123" }
                                                    },
                                                    "thumbnailRenderer": {
                                                        "musicThumbnailRenderer": {
                                                            "thumbnail": {
                                                                "thumbnails": [
                                                                    { "url": "https://lh3.googleusercontent.com/abc=w226-h226", "width": 226, "height": 226 },
                                                                    { "url": "https://lh3.googleusercontent.com/abc=w544-h544", "width": 544, "height": 544 }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            }
        });

        let (name, playlists) = parse_mood_playlists_json(&json);
        assert_eq!(name, "Cumbia");
        assert_eq!(playlists.len(), 1);
        assert_eq!(playlists[0].id, "RDCLAK5uy_test123");
        assert_eq!(playlists[0].name, "Cumbia Sonidera");
        assert_eq!(playlists[0].subtitle, "YouTube Music");
        assert!(playlists[0].cover_url.is_some());
    }
}
