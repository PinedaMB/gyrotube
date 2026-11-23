use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

use rustypipe::client::RustyPipe;
use rustypipe::model::{AlbumType, TrackType};
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
    VideoStreamInfo, cache_dir, clean_track_title, color_to_hex, parse_lrc,
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
        crate::tls::ensure_crypto_provider();
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
            http: crate::tls::http_client_builder()
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
        self.fetch_home_feed_for_country(None, force_refresh).await
    }

    pub async fn fetch_home_feed_for_country(
        &self,
        country_code: Option<&str>,
        force_refresh: bool,
    ) -> Result<HomeFeed, String> {
        let normalized_country = country_code
            .map(str::trim)
            .filter(|c| !c.is_empty() && !c.eq_ignore_ascii_case("global") && !c.eq_ignore_ascii_case("zz"))
            .map(|c| c.to_ascii_uppercase());

        if !force_refresh
            && let Ok(guard) = self.home_cache.read()
            && let Some((ts, feed)) = guard.as_ref()
            && ts.elapsed() < HOME_CACHE_TTL
            && feed.country == normalized_country
        {
            return Ok(feed.clone());
        }

        let parsed_country = normalized_country
            .as_deref()
            .and_then(|code| code.parse::<rustypipe::param::Country>().ok());

        let mut trending = Vec::new();
        let mut top = Vec::new();
        let mut new_albums = Vec::new();
        let mut top_artists = Vec::new();
        let mut available_countries = Vec::new();

        let rp_q = self.rp.query();
        let (charts_res, releases_res) =
            tokio::join!(rp_q.music_charts(parsed_country), rp_q.music_new_albums());
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
            top_artists = charts
                .artists
                .iter()
                .take(12)
                .map(Artist::from)
                .collect();
            available_countries = charts
                .available_countries
                .iter()
                .map(|c| c.to_string())
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
            top_artists,
            country: normalized_country,
            available_countries,
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
                let tracks = tracks_from_ytm_playlist_with_videos(items);
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

        // shortcut: check up to three continuation pages for playable items, extend further on demand if needed.
        for _ in 0..3 {
            if playlist.tracks.is_exhausted()
                || playlist.tracks.items.iter().any(is_playlist_playable)
            {
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
            .filter(|t| is_playlist_playable(t))
            .map(|t| {
                let mut tr = Track::from(t);
                if t.track_type == TrackType::Video {
                    tr.is_music = true;
                    tr.is_video = true;
                }
                tr
            })
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
        let ytm_fut = async {
            let ytm = self.primary().await.ok()?;
            ytm.get_artist(ArtistChannelID::from_raw(artist_id)).await.ok()
        };
        let rp_fut = async { self.rp.query().music_artist(artist_id, true).await.ok() };
        let (ytm_artist, rp_artist) = tokio::join!(ytm_fut, rp_fut);

        let rp_view_counts: HashMap<String, u64> = rp_artist
            .as_ref()
            .map(|rp| {
                rp.tracks
                    .iter()
                    .filter_map(|t| t.view_count.map(|vc| (t.id.clone(), vc)))
                    .collect()
            })
            .unwrap_or_default();

        let rp_videos: Vec<Track> = rp_artist
            .as_ref()
            .map(|rp| {
                rp.tracks
                    .iter()
                    .filter(|t| t.track_type == TrackType::Video && !t.unavailable)
                    .map(|t| {
                        let mut tr = Track::from(t);
                        tr.is_music = true;
                        tr.is_video = true;
                        if tr.artist_id.is_none() {
                            tr.artist_id = Some(artist_id.to_string());
                        }
                        tr
                    })
                    .collect()
            })
            .unwrap_or_default();

        let rp_playlists: Vec<Playlist> = rp_artist
            .as_ref()
            .map(|rp| rp.playlists.iter().map(Playlist::from).collect())
            .unwrap_or_default();

        let rp_similar: Vec<Artist> = rp_artist
            .as_ref()
            .map(|rp| rp.similar_artists.iter().map(Artist::from).collect())
            .unwrap_or_default();

        let (rp_studio_albums, rp_single_releases): (Vec<Album>, Vec<Album>) = rp_artist
            .as_ref()
            .map(|rp| {
                let mut albums = Vec::new();
                let mut singles = Vec::new();
                for alb in &rp.albums {
                    match alb.album_type {
                        AlbumType::Single | AlbumType::Ep => singles.push(Album::from(alb)),
                        _ => albums.push(Album::from(alb)),
                    }
                }
                (albums, singles)
            })
            .unwrap_or_default();

        if let Some(artist) = ytm_artist {
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
                                let first_artist_id = ps
                                    .artists
                                    .iter()
                                    .find_map(|a| a.id.as_ref().map(|id| id.get_raw().to_string()))
                                    .or_else(|| Some(artist_id.to_string()));
                                let artist_str = ps
                                    .artists
                                    .into_iter()
                                    .map(|a| a.name)
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                let vid = ps.video_id.get_raw().to_string();
                                let vc = rp_view_counts.get(&vid).copied();
                                Some(Track {
                                    is_music: true,
                                    is_video: false,
                                    id: vid,
                                    title: clean_track_title(&ps.title),
                                    artist: if artist_str.is_empty() {
                                        name.clone()
                                    } else {
                                        artist_str
                                    },
                                    artist_id: first_artist_id,
                                    album: Some(ps.album.name),
                                    album_id: Some(ps.album.id.get_raw().to_string()),
                                    duration_secs: parse_mm_ss(&ps.duration),
                                    view_count: vc,
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
                            let first_artist_id = t
                                .artists
                                .iter()
                                .find_map(|a| a.id.as_ref().map(|aid| aid.get_raw().to_string()))
                                .or_else(|| Some(artist_id.to_string()));
                            let vc = rp_view_counts.get(&id).copied();
                            Track {
                                is_music: false,
                                is_video: false,
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
                                artist_id: first_artist_id,
                                album: Some(t.album.name),
                                album_id: Some(t.album.id.get_raw().to_string()),
                                duration_secs: None,
                                view_count: vc,
                                cover_url: cover.clone(),
                                cover_large_url: cover,
                            }
                        })
                        .collect()
                }
            } else {
                Vec::new()
            };

            let mut studio_albums = Vec::new();
            let mut seen_album_ids = HashSet::new();
            if let Some(a) = albums {
                for item in a.results {
                    let id = item.album_id.get_raw().to_string();
                    if seen_album_ids.insert(id.clone()) {
                        studio_albums.push(Album {
                            id,
                            title: item.title,
                            artist: name.clone(),
                            year: item.year.parse::<u16>().ok(),
                            cover_url: best_ytm_thumb(&item.thumbnails),
                        });
                    }
                }
            }
            for alb in rp_studio_albums {
                if seen_album_ids.insert(alb.id.clone()) {
                    studio_albums.push(alb);
                }
            }

            let mut single_releases = Vec::new();
            let mut seen_single_ids = HashSet::new();
            if let Some(s) = singles {
                for item in s.results {
                    let id = item.album_id.get_raw().to_string();
                    if !seen_album_ids.contains(&id) && seen_single_ids.insert(id.clone()) {
                        single_releases.push(Album {
                            id,
                            title: item.title,
                            artist: name.clone(),
                            year: item.year.parse::<u16>().ok(),
                            cover_url: best_ytm_thumb(&item.thumbnails),
                        });
                    }
                }
            }
            // ytmapi-rs 0.3.x misses YouTube Music's renamed "Singles & EPs" shelf;
            // supplement/populate singles & EPs from RustyPipe's full discography.
            for alb in rp_single_releases {
                if !seen_album_ids.contains(&alb.id) && seen_single_ids.insert(alb.id.clone()) {
                    single_releases.push(alb);
                }
            }

            return Ok(ArtistDetails {
                name,
                header_url: best_ytm_banner(&artist.thumbnails)
                    .or_else(|| rp_artist.as_ref().and_then(|rp| crate::models::pick_cover(&rp.header_image, 1000))),
                subscriber_count: artist
                    .subscribers
                    .as_deref()
                    .and_then(parse_sub_count)
                    .or_else(|| rp_artist.as_ref().and_then(|rp| rp.subscriber_count)),
                description: artist
                    .description
                    .or_else(|| rp_artist.as_ref().and_then(|rp| rp.description.clone())),
                tracks,
                albums: studio_albums,
                singles: single_releases,
                videos: rp_videos,
                playlists: rp_playlists,
                similar_artists: rp_similar,
            });
        }

        if let Some(rp_artist) = rp_artist {
            let tracks: Vec<Track> = rp_artist
                .tracks
                .iter()
                .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
                .map(Track::from)
                .collect();
            if !tracks.is_empty() || !rp_studio_albums.is_empty() || !rp_single_releases.is_empty() {
                return Ok(ArtistDetails {
                    name: rp_artist.name,
                    header_url: crate::models::pick_cover(&rp_artist.header_image, 1000),
                    description: rp_artist.description,
                    subscriber_count: rp_artist.subscriber_count,
                    tracks,
                    albums: rp_studio_albums,
                    singles: rp_single_releases,
                    videos: rp_videos,
                    playlists: rp_playlists,
                    similar_artists: rp_similar,
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

    pub fn downloads_dir_ready() -> Result<std::path::PathBuf, String> {
        let dir = crate::models::downloads_dir()
            .ok_or_else(|| "Downloads directory unavailable".to_string())?;
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create downloads directory: {e}"))?;
        Ok(dir)
    }

    pub fn list_downloaded_track_ids() -> Vec<String> {
        let Some(dir) = crate::models::downloads_dir() else {
            return Vec::new();
        };
        let Ok(entries) = std::fs::read_dir(dir) else {
            return Vec::new();
        };
        let mut ids = Vec::new();
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else { continue };
            if !meta.is_file() || meta.len() == 0 {
                continue;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some((id, ext)) = name.split_once('.')
                && validate_video_id(id).is_ok()
                && (matches!(ext, "m4a" | "webm")
                    || ext.strip_prefix("ytdlp.").is_some_and(audio_cache::audio_extension))
                && !ids.iter().any(|existing| existing == id)
            {
                ids.push(id.to_string());
            }
        }
        ids
    }

    pub fn remove_downloaded_tracks(track_ids: &[String]) -> Result<(), String> {
        let Some(dir) = crate::models::downloads_dir() else {
            return Ok(());
        };
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return Ok(());
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some((id, _)) = name.split_once('.')
                && track_ids.iter().any(|target| target == id)
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
        Ok(())
    }

    pub fn clear_all_downloads() -> Result<(), String> {
        let Some(dir) = crate::models::downloads_dir() else {
            return Ok(());
        };
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if entry.metadata().is_ok_and(|m| m.is_file()) {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
        Ok(())
    }

    pub fn downloads_bytes() -> u64 {
        let Some(dir) = crate::models::downloads_dir() else {
            return 0;
        };
        std::fs::read_dir(dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .map(|m| m.len())
            .sum()
    }

    async fn download_cover_offline(track: &Track, dl_dir: &std::path::Path) {
        let cover_path = dl_dir.join(format!("{}.cover", track.id));
        if std::fs::metadata(&cover_path).is_ok_and(|m| m.is_file() && m.len() > 0) {
            return;
        }
        let Some(url) = track
            .cover_large_url
            .as_deref()
            .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
            .or_else(|| {
                track
                    .cover_url
                    .as_deref()
                    .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
            })
        else {
            return;
        };
        let Ok(client) = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(6))
            .read_timeout(Duration::from_secs(10))
            .build()
        else {
            return;
        };
        if let Ok(resp) = client.get(url).send().await
            && resp.status().is_success()
            && let Ok(bytes) = resp.bytes().await
            && !bytes.is_empty()
            && bytes.len() <= 5 * 1024 * 1024
        {
            let tmp = dl_dir.join(format!("{}.cover.part", track.id));
            if tokio::fs::write(&tmp, &bytes).await.is_ok() {
                let _ = tokio::fs::rename(&tmp, &cover_path).await;
            }
        }
    }

    async fn download_lyrics_offline(&self, track: &Track, dl_dir: &std::path::Path) {
        let lyrics_path = dl_dir.join(format!("{}.lyrics.json", track.id));
        if std::fs::metadata(&lyrics_path).is_ok_and(|m| m.is_file() && m.len() > 0) {
            return;
        }
        if let Ok(data) = self.fetch_synced_lyrics(track).await
            && let Ok(json) = serde_json::to_vec(&data)
        {
            let tmp = dl_dir.join(format!("{}.lyrics.json.part", track.id));
            if tokio::fs::write(&tmp, &json).await.is_ok() {
                let _ = tokio::fs::rename(&tmp, &lyrics_path).await;
            }
        }
    }

    pub async fn download_track_offline(&self, track: &Track) -> Result<String, String> {
        validate_video_id(&track.id)?;
        let dl_dir = Self::downloads_dir_ready()?;
        let ((), ()) = tokio::join!(
            Self::download_cover_offline(track, &dl_dir),
            self.download_lyrics_offline(track, &dl_dir)
        );
        if audio_cache::find(&track.id, &dl_dir).is_some() {
            return Ok(track.id.clone());
        }

        // Fast-path: copy from warm audio cache if already downloaded during playback
        if let Ok(streams_dir) = Self::streams_dir()
            && let Some(cached) = audio_cache::find(&track.id, &streams_dir)
        {
            let src = std::path::PathBuf::from(&cached.url);
            if src.is_file() {
                let ext = src
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("webm");
                let tmp = dl_dir.join(format!("{}.part.{ext}", track.id));
                let dest = dl_dir.join(format!("{}.ytdlp.{ext}", track.id));
                tokio::fs::copy(&src, &tmp)
                    .await
                    .map_err(|e| format!("Failed copying cached track: {e}"))?;
                tokio::fs::rename(&tmp, &dest)
                    .await
                    .map_err(|e| format!("Failed finalizing downloaded track: {e}"))?;
                return Ok(track.id.clone());
            }
        }

        // Resolve via the standard stream pipeline (works on both desktop and Android)
        let info = self.resolve_track_stream(track).await?;
        if !info.url.starts_with("http://") && !info.url.starts_with("https://") {
            let src = std::path::PathBuf::from(&info.url);
            let ext = src.extension().and_then(|s| s.to_str()).unwrap_or("webm");
            let tmp = dl_dir.join(format!("{}.part.{ext}", track.id));
            let dest = dl_dir.join(format!("{}.ytdlp.{ext}", track.id));
            tokio::fs::copy(&src, &tmp)
                .await
                .map_err(|e| format!("Failed saving track: {e}"))?;
            tokio::fs::rename(&tmp, &dest)
                .await
                .map_err(|e| format!("Failed finalizing track: {e}"))?;
            return Ok(track.id.clone());
        }

        let ext = if info.url.contains(".m4a") || info.url.contains(".mp4") {
            "m4a"
        } else {
            "webm"
        };
        let tmp = dl_dir.join(format!("{}.part.{ext}", track.id));
        let dest = dl_dir.join(format!("{}.ytdlp.{ext}", track.id));

        let client = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| format!("Failed to build download client: {e}"))?;
        let mut resp = client
            .get(&info.url)
            .send()
            .await
            .map_err(|e| format!("Offline download request failed: {}", e.without_url()))?;
        if !resp.status().is_success() {
            return Err(format!("Offline download returned HTTP {}", resp.status()));
        }

        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(&tmp)
            .await
            .map_err(|e| format!("Failed to create download file: {e}"))?;
        let mut written = 0u64;
        while let Some(chunk) = resp
            .chunk()
            .await
            .map_err(|e| format!("Download interrupted: {}", e.without_url()))?
        {
            if !chunk.is_empty() {
                file.write_all(&chunk)
                    .await
                    .map_err(|e| format!("Failed writing download: {e}"))?;
                written += chunk.len() as u64;
            }
        }
        file.flush()
            .await
            .map_err(|e| format!("Failed flushing download: {e}"))?;
        drop(file);

        if written == 0 {
            let _ = tokio::fs::remove_file(&tmp).await;
            return Err("Downloaded audio was empty".to_string());
        }

        tokio::fs::rename(&tmp, &dest)
            .await
            .map_err(|e| format!("Failed finalizing download: {e}"))?;
        Ok(track.id.clone())
    }

    pub async fn resolve_track_stream(&self, track: &Track) -> Result<StreamInfo, String> {
        validate_video_id(&track.id)?;
        // Always prefer permanent offline downloads first (zero network required)
        if let Some(dl_dir) = crate::models::downloads_dir()
            && let Some(downloaded) = audio_cache::find(&track.id, &dl_dir)
        {
            return Ok(downloaded);
        }
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

    pub async fn resolve_video_stream(
        &self,
        video_id: &str,
        max_height: u32,
    ) -> Result<VideoStreamInfo, String> {
        validate_video_id(video_id)?;

        // Desktop: prefer yt-dlp (solves YouTube n-parameter / PO-token reliably without 403)
        #[cfg(not(target_os = "android"))]
        {
            #[cfg(target_os = "linux")]
            let fmt = format!(
                "bestvideo[vcodec^=vp9][height<={max_height}]/bestvideo[ext=webm][height<={max_height}]/bestvideo[vcodec^=avc1][height<={max_height}]/18"
            );
            #[cfg(not(target_os = "linux"))]
            let fmt = format!(
                "bestvideo[vcodec^=avc1][height<={max_height}]/bestvideo[ext=mp4][height<={max_height}]/bestvideo[ext=webm][height<={max_height}]/18"
            );
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
                    &fmt,
                    "--",
                ])
                .arg(format!("https://www.youtube.com/watch?v={video_id}"));
            if let Ok(Ok(output)) = tokio::time::timeout(Duration::from_secs(12), cmd.output()).await
                && output.status.success()
                && let Ok(val) = serde_json::from_slice::<serde_json::Value>(&output.stdout)
                && let Some(raw_url) = val["url"].as_str()
                && let Ok(parsed) = reqwest::Url::parse(raw_url)
                && matches!(parsed.scheme(), "http" | "https")
            {
                let height = val["height"].as_u64().unwrap_or(720) as u32;
                let width = val["width"].as_u64().unwrap_or(1280) as u32;
                let ext = val["ext"].as_str().unwrap_or("webm");
                let mime_type = if ext == "webm" { "video/webm" } else { "video/mp4" };
                let ua = val["http_headers"]["User-Agent"]
                    .as_str()
                    .unwrap_or("Mozilla/5.0");
                let streamer = crate::audio_streamer::AudioStreamer::global().await?;
                let proxied = streamer.serve_remote_proxy(video_id, parsed.as_str(), ua, mime_type);
                return Ok(VideoStreamInfo {
                    url: proxied,
                    quality_label: format!("{height}p"),
                    width,
                    height,
                });
            }
        }

        let visitor_data = self.rp.query().get_visitor_data(true).await.ok();
        let profiles = [
            (
                "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip",
                "28",
                "1.65.10",
                serde_json::json!({
                    "context": { "client": {
                        "clientName": "ANDROID_VR",
                        "clientVersion": "1.65.10",
                        "deviceMake": "Oculus",
                        "deviceModel": "Quest 3",
                        "androidSdkVersion": 32,
                        "osName": "Android",
                        "osVersion": "12L",
                        "hl": "en",
                        "gl": "US"
                    } },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            (
                "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version",
                "85",
                "2.0",
                serde_json::json!({
                    "context": {
                        "client": {
                            "clientName": "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
                            "clientVersion": "2.0",
                            "hl": "en",
                            "gl": "US"
                        },
                        "thirdParty": { "embedUrl": "https://www.youtube.com" }
                    },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            (
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15",
                "101",
                "1.02",
                serde_json::json!({
                    "context": { "client": {
                        "clientName": "VISIONOS",
                        "clientVersion": "1.02",
                        "deviceMake": "Apple",
                        "deviceModel": "RealityDevice17,1",
                        "osName": "visionOS",
                        "osVersion": "26.5.23O471"
                    } },
                    "videoId": video_id,
                }),
            ),
        ];

        for (user_agent, client_name_hdr, client_version_hdr, mut payload) in profiles {
            if let Some(ref vd) = visitor_data
                && let Some(client_obj) = payload["context"]["client"].as_object_mut()
            {
                client_obj.insert("visitorData".to_string(), serde_json::Value::String(vd.clone()));
            }
            let mut req = self
                .http
                .post("https://www.youtube.com/youtubei/v1/player?prettyPrint=false")
                .timeout(Duration::from_secs(6))
                .header("User-Agent", user_agent)
                .header("X-YouTube-Client-Name", client_name_hdr)
                .header("X-YouTube-Client-Version", client_version_hdr);
            if let Some(ref vd) = visitor_data {
                req = req.header("X-Goog-Visitor-Id", vd);
            }
            let Ok(resp) = req.json(&payload).send().await else {
                continue;
            };
            let Ok(resp) = resp.error_for_status() else {
                continue;
            };
            let Ok(json) = resp.json::<serde_json::Value>().await else {
                continue;
            };
            let adaptive = json["streamingData"]["adaptiveFormats"].as_array();
            let muxed = json["streamingData"]["formats"].as_array();
            let all_formats: Vec<&serde_json::Value> = adaptive
                .into_iter()
                .flatten()
                .chain(muxed.into_iter().flatten())
                .collect();

            // On Linux (Fedora/WebKitGTK), prefer WebM VP9 (open-source, always decoded in GStreamer)
            // with H.264 MP4 fallback; on Android/macOS/Windows prefer H.264 MP4 with WebM VP9 fallback.
            let mut best_webm: Option<(u32, u64, String, u32, u32)> = None;
            let mut best_mp4: Option<(u32, u64, String, u32, u32)> = None;
            for f in all_formats {
                let Some(mime) = f["mimeType"].as_str() else { continue };
                let is_webm_vp9 = mime.starts_with("video/webm") && (mime.contains("vp9") || mime.contains("vp09") || mime.contains("vp8"));
                let is_mp4_h264 = mime.starts_with("video/mp4") && mime.contains("avc1");
                if !is_webm_vp9 && !is_mp4_h264 {
                    continue;
                }
                let Some(url_str) = f["url"].as_str() else { continue };
                let Some(parsed) = reqwest::Url::parse(url_str).ok() else { continue };
                if parsed.scheme() != "https"
                    || !parsed.host_str().is_some_and(|h| h.ends_with(".googlevideo.com"))
                {
                    continue;
                }
                let height = f["height"].as_u64().unwrap_or(0) as u32;
                let width = f["width"].as_u64().unwrap_or(0) as u32;
                if height == 0 || height > max_height {
                    continue;
                }
                let bitrate = f["bitrate"].as_u64().unwrap_or(0);
                let slot = if is_webm_vp9 { &mut best_webm } else { &mut best_mp4 };
                let replace = match slot {
                    None => true,
                    Some((bh, bb, _, _, _)) => height > *bh || (height == *bh && bitrate > *bb),
                };
                if replace {
                    *slot = Some((height, bitrate, parsed.into(), width, height));
                }
            }
            #[cfg(target_os = "linux")]
            let chosen = best_webm
                .map(|v| (v, "video/webm"))
                .or_else(|| best_mp4.map(|v| (v, "video/mp4")));
            #[cfg(not(target_os = "linux"))]
            let chosen = best_mp4
                .map(|v| (v, "video/mp4"))
                .or_else(|| best_webm.map(|v| (v, "video/webm")));

            if let Some(((height, _, raw_url, width, h), mime_type)) = chosen {
                // Probe 1 byte before returning so 403 profiles fall through to the next client
                let probe_ok = self
                    .http
                    .get(&raw_url)
                    .timeout(Duration::from_secs(4))
                    .header("User-Agent", user_agent)
                    .header("Range", "bytes=0-0")
                    .send()
                    .await
                    .is_ok_and(|r| r.status().is_success() || r.status().as_u16() == 206);
                if !probe_ok {
                    continue;
                }
                let streamer = crate::audio_streamer::AudioStreamer::global().await?;
                let proxied = streamer.serve_remote_proxy(video_id, &raw_url, user_agent, mime_type);
                return Ok(VideoStreamInfo {
                    url: proxied,
                    quality_label: format!("{height}p"),
                    width,
                    height: h,
                });
            }
        }
        Err(String::from("No hay stream de video compatible disponible para esta pista"))
    }

    pub async fn find_counterpart_track(
        &self,
        track: &Track,
        want_video: bool,
    ) -> Result<Option<Track>, String> {
        // Ask YouTube Music's /next watch playlist for the official Song <-> Video
        // counterpart linked inside playlistPanelVideoWrapperRenderer.
        // If no official counterpart exists, return None so the UI disables switching.
        if validate_video_id(&track.id).is_err() {
            return Ok(None);
        }
        let Ok(ytm) = self.primary().await else {
            return Ok(None);
        };
        let Ok(raw) = ytm
            .raw_json_query(
                ytmapi_rs::query::GetWatchPlaylistQuery::new_from_video_id(VideoID::from_raw(
                    track.id.as_str(),
                )),
            )
            .await
        else {
            return Ok(None);
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw) else {
            return Ok(None);
        };
        Ok(parse_watch_counterpart_json(&json, track, want_video))
    }

    #[cfg(not(target_os = "android"))]
    async fn extract_fast_ios_stream(
        &self,
        video_id: &str,
    ) -> Result<(StreamInfo, &'static str, Option<u64>), String> {
        use rustypipe::client::ClientType;
        use rustypipe::model::AudioFormat;
        let started = Instant::now();
        let player = tokio::time::timeout(
            Duration::from_millis(2500),
            self.rp.query().player_from_client(video_id, ClientType::Ios),
        )
        .await
        .map_err(|_| "Tiempo de espera agotado al consultar el reproductor".to_string())?
        .map_err(|_| "No se pudo consultar el reproductor".to_string())?;
        // Require itag 251 (Opus WebM ~160kbps) for the fast-start iOS path so byte offsets
        // match the background visionos handover identically to the single byte.
        let stream = player.audio_streams.iter().find(|s| s.itag == 251)
            .ok_or_else(|| "El reproductor no ofreció audio WebM".to_string())?;
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
        Ok((
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

    #[cfg(target_os = "android")]
    async fn extract_innertube_direct_stream(
        &self,
        video_id: &str,
        profile: &str,
        visitor_data: Option<&str>,
    ) -> Result<(StreamInfo, &'static str, Option<u64>), String> {
        let (user_agent, client_name_hdr, client_version_hdr, mut payload) = match profile {
            "ANDROID_VR" => (
                "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip",
                "28",
                "1.65.10",
                serde_json::json!({
                    "context": { "client": {
                        "clientName": "ANDROID_VR",
                        "clientVersion": "1.65.10",
                        "deviceMake": "Oculus",
                        "deviceModel": "Quest 3",
                        "androidSdkVersion": 32,
                        "osName": "Android",
                        "osVersion": "12L",
                        "hl": "en",
                        "gl": "US"
                    } },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            "ANDROID_VR_LEGACY" => (
                "com.google.android.apps.youtube.vr.oculus/1.43.32 (Linux; U; Android 12; Quest 2) gzip",
                "28",
                "1.43.32",
                serde_json::json!({
                    "context": { "client": {
                        "clientName": "ANDROID_VR",
                        "clientVersion": "1.43.32",
                        "deviceMake": "Oculus",
                        "deviceModel": "Quest 2",
                        "androidSdkVersion": 32,
                        "osName": "Android",
                        "osVersion": "12",
                        "hl": "en",
                        "gl": "US"
                    } },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            "TV_EMBEDDED" => (
                "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version",
                "85",
                "2.0",
                serde_json::json!({
                    "context": {
                        "client": {
                            "clientName": "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
                            "clientVersion": "2.0",
                            "hl": "en",
                            "gl": "US"
                        },
                        "thirdParty": {
                            "embedUrl": "https://www.youtube.com"
                        }
                    },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            _ => (
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15",
                "101",
                "1.02",
                serde_json::json!({
                    "context": { "client": {
                        "clientName": "VISIONOS", "clientVersion": "1.02",
                        "deviceMake": "Apple", "deviceModel": "RealityDevice17,1",
                        "osName": "visionOS", "osVersion": "26.5.23O471"
                    } },
                    "videoId": video_id,
                }),
            ),
        };

        if let Some(vd) = visitor_data
            && let Some(client_obj) = payload["context"]["client"].as_object_mut()
        {
            client_obj.insert("visitorData".to_string(), serde_json::Value::String(vd.to_string()));
        }

        let mut req = self.http
            .post("https://www.youtube.com/youtubei/v1/player?prettyPrint=false")
            .timeout(Duration::from_secs(6))
            .header("User-Agent", user_agent)
            .header("X-YouTube-Client-Name", client_name_hdr)
            .header("X-YouTube-Client-Version", client_version_hdr);
        if let Some(vd) = visitor_data {
            req = req.header("X-Goog-Visitor-Id", vd);
        }

        let response: serde_json::Value = req
            .json(&payload)
            .send().await.and_then(reqwest::Response::error_for_status)
            .map_err(|err| err.without_url().to_string())?
            .json().await.map_err(|err| err.without_url().to_string())?;

        let adaptive = response["streamingData"]["adaptiveFormats"].as_array();
        let muxed = response["streamingData"]["formats"].as_array();
        if adaptive.is_none() && muxed.is_none() {
            let status = response["playabilityStatus"]["status"].as_str()
                .filter(|status| status.len() <= 40 && status.bytes().all(|b| b.is_ascii_uppercase() || b == b'_'));
            return Err(format!("El reproductor no ofreció audio ({})", status.unwrap_or("SIN_FORMATOS")));
        }

        let all_formats: Vec<&serde_json::Value> = adaptive
            .into_iter()
            .flatten()
            .chain(muxed.into_iter().flatten())
            .collect();

        for (itag, extension) in [
            (251u64, "webm"),
            (140u64, "m4a"),
            (250u64, "webm"),
            (249u64, "webm"),
            (139u64, "m4a"),
            (18u64, "mp4"),
        ] {
            let Some(format) = all_formats
                .iter()
                .find(|f| f["itag"].as_u64() == Some(itag) && f["url"].is_string())
            else {
                continue;
            };
            let Some(url) = format["url"].as_str().and_then(|url| reqwest::Url::parse(url).ok()) else {
                continue;
            };
            if url.scheme() != "https" || !url.host_str().is_some_and(|host| host.ends_with(".googlevideo.com")) {
                continue;
            }
            let size = format["contentLength"].as_str().and_then(|v| v.parse().ok());
            let kbps = format["bitrate"].as_u64().unwrap_or(128_000) / 1000;
            return Ok((StreamInfo {
                url: url.into(),
                quality_label: format!("{kbps}kbps ({})", extension.to_uppercase()),
                user_agent: Some(user_agent.into()),
                headers: HashMap::new(),
                local_audio: None,
            }, extension, size));
        }
        Err("El reproductor no ofreció una URL directa de audio compatible".into())
    }

    #[cfg(target_os = "android")]
    async fn extract_rustypipe_client_stream(
        &self,
        video_id: &str,
        client: rustypipe::client::ClientType,
    ) -> Result<(StreamInfo, &'static str, Option<u64>), String> {
        use rustypipe::model::AudioFormat;
        let player = tokio::time::timeout(
            Duration::from_secs(6),
            self.rp.query().player_from_client(video_id, client),
        )
        .await
        .map_err(|_| format!("Timeout consultando cliente {client:?}"))?
        .map_err(|e| format!("Error en cliente {client:?}: {e}"))?;

        let stream = player
            .audio_streams
            .iter()
            .find(|s| s.itag == 251)
            .or_else(|| player.audio_streams.iter().find(|s| s.itag == 140))
            .or_else(|| player.audio_streams.iter().max_by_key(|s| s.bitrate))
            .ok_or_else(|| format!("Sin streams de audio en {client:?}"))?;

        let ext = match stream.format {
            AudioFormat::M4a => "m4a",
            _ => "webm",
        };
        let kbps = (stream.bitrate / 1000).max(96);
        let quality_label = format!("{kbps}kbps ({})", ext.to_uppercase());
        let ua = self.rp.query().user_agent(client).into_owned();
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), ua.clone());
        let size = (stream.size > 0).then_some(stream.size);
        Ok((
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

        #[cfg(target_os = "android")]
        {
            use rustypipe::client::ClientType;
            let visitor_data = self.rp.query().get_visitor_data(false).await.ok();
            let attempts = ["ANDROID_VR", "ANDROID_VR_LEGACY", "TV_EMBEDDED", "VISIONOS", "iOS"];
            for source in attempts {
                let extracted = match source {
                    "iOS" => self.extract_rustypipe_client_stream(video_id, ClientType::Ios).await,
                    profile => {
                        self.extract_innertube_direct_stream(
                            video_id,
                            profile,
                            visitor_data.as_deref(),
                        )
                        .await
                    }
                };
                let (remote, ext, size_hint) = match extracted {
                    Ok(stream) => stream,
                    Err(err) => {
                        eprintln!("[audio-android] id={video_id} fuente={source} extracción: {err}");
                        continue;
                    }
                };
                match streamer.start_chunked_http_stream(video_id, remote, ext, size_hint, &streams_dir).await {
                    Ok(local_stream) => {
                        eprintln!("[audio-android] id={video_id} fuente={source} OK -> {}", local_stream.url);
                        if let Ok(mut cache) = self.stream_cache.write() {
                            cache.retain(|_, (ts, s)| stream_url_is_fresh(*ts, s));
                            cache.insert(video_id.to_owned(), (Instant::now(), local_stream.clone()));
                        }
                        return Ok(local_stream);
                    }
                    Err(err) => eprintln!("[audio-android] id={video_id} fuente={source} descarga: {err}"),
                }
            }
            Err("No se pudo resolver el flujo de audio (Android).".to_string())
        }

        #[cfg(not(target_os = "android"))]
        {
        // Fast-path: extract stream via pure-Rust iOS client (~140-250ms, no Python/Node startup)
        // to serve the initial audio blocks immediately while AudioStreamer transparently refreshes
        // the URL via visionos in the background before the 1 MB iOS limit.
        if let Ok((fast_remote, ext, size_hint)) = self.extract_fast_ios_stream(video_id).await
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

        // Fallback: extract stream URL via yt-dlp excluding tv_samsung (which triggers 6s ad / 403)
        // and skipping HLS/DASH manifest downloads.
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

    pub async fn fetch_related_for_track(
        &self,
        track_id: &str,
    ) -> Result<crate::models::RelatedFeed, String> {
        validate_video_id(track_id)?;
        let details = self
            .rp
            .query()
            .music_details(track_id)
            .await
            .map_err(|e| format!("Error obteniendo detalles: {e}"))?;
        let Some(related_id) = details.related_id else {
            return Ok(crate::models::RelatedFeed::default());
        };
        let related = self
            .rp
            .query()
            .music_related(&related_id)
            .await
            .map_err(|e| format!("Error obteniendo relacionados: {e}"))?;
        Ok(crate::models::RelatedFeed {
            tracks: related
                .tracks
                .iter()
                .filter(|t| !t.unavailable)
                .take(16)
                .map(Track::from)
                .collect(),
            albums: related.albums.iter().take(12).map(Album::from).collect(),
            playlists: related.playlists.iter().take(12).map(Playlist::from).collect(),
            artists: related.artists.iter().take(12).map(Artist::from).collect(),
        })
    }

    pub async fn fetch_playlist_radio(&self, playlist_id: &str) -> Result<Vec<Track>, String> {
        let radio = self
            .rp
            .query()
            .music_radio_playlist(playlist_id)
            .await
            .map_err(|e| format!("Error iniciando radio de playlist: {e}"))?;
        Ok(radio
            .items
            .iter()
            .filter(|t| t.track_type == TrackType::Track && !t.unavailable)
            .map(Track::from)
            .collect())
    }

    pub async fn fetch_synced_lyrics(&self, track: &Track) -> Result<LyricsData, String> {
        if !track.id.is_empty()
            && validate_video_id(&track.id).is_ok()
            && let Some(dl_dir) = crate::models::downloads_dir()
        {
            let lyrics_path = dl_dir.join(format!("{}.lyrics.json", track.id));
            if let Ok(bytes) = tokio::fs::read(&lyrics_path).await
                && let Ok(saved) = serde_json::from_slice::<LyricsData>(&bytes)
            {
                return Ok(saved);
            }
        }

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
        if url.host_str() == Some("127.0.0.1") && url.path().starts_with("/stream/") {
            return crate::audio_streamer::AudioStreamer::try_global()
                .is_some_and(|streamer| streamer.can_reuse_url(&url));
        }
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

fn panel_video_id(renderer: &serde_json::Value) -> Option<&str> {
    renderer
        .get("videoId")
        .and_then(|v| v.as_str())
        .or_else(|| {
            renderer
                .pointer("/navigationEndpoint/watchEndpoint/videoId")
                .and_then(|v| v.as_str())
        })
        .filter(|id| validate_video_id(id).is_ok())
}

fn panel_video_music_type(renderer: &serde_json::Value) -> Option<&str> {
    renderer
        .pointer("/navigationEndpoint/watchEndpoint/watchEndpointMusicSupportedConfigs/watchEndpointMusicConfig/musicVideoType")
        .and_then(|v| v.as_str())
}

fn track_from_panel_video_renderer(
    renderer: &serde_json::Value,
    source: &Track,
    want_video: bool,
) -> Option<Track> {
    let id = panel_video_id(renderer)?.to_string();
    let raw_title = renderer.get("title").map(runs_text).unwrap_or_default();
    let title = if raw_title.trim().is_empty() {
        source.title.clone()
    } else {
        clean_track_title(&raw_title)
    };

    let mut artists: Vec<String> = Vec::new();
    let mut artist_id: Option<String> = None;
    let mut album: Option<String> = None;
    let mut album_id: Option<String> = None;

    for byline_key in ["longBylineText", "shortBylineText"] {
        if let Some(runs) = renderer
            .get(byline_key)
            .and_then(|v| v.get("runs"))
            .and_then(|r| r.as_array())
        {
            for run in runs {
                let text = run
                    .get("text")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .trim();
                if text.is_empty() {
                    continue;
                }
                let browse_id = run
                    .pointer("/navigationEndpoint/browseEndpoint/browseId")
                    .and_then(|v| v.as_str());
                let page_type = run
                    .pointer("/navigationEndpoint/browseEndpoint/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
                    .and_then(|v| v.as_str());
                match page_type {
                    Some("MUSIC_PAGE_TYPE_ARTIST") => {
                        if !artists.iter().any(|a| a == text) {
                            artists.push(text.to_string());
                        }
                        if artist_id.is_none() {
                            artist_id = browse_id.map(ToString::to_string);
                        }
                    }
                    Some("MUSIC_PAGE_TYPE_ALBUM") => {
                        if album.is_none() {
                            album = Some(text.to_string());
                            album_id = browse_id.map(ToString::to_string);
                        }
                    }
                    _ => {
                        if let Some(bid) = browse_id {
                            if bid.starts_with("UC") && !artists.iter().any(|a| a == text) {
                                artists.push(text.to_string());
                                if artist_id.is_none() {
                                    artist_id = Some(bid.to_string());
                                }
                            } else if bid.starts_with("MPRE") && album.is_none() {
                                album = Some(text.to_string());
                                album_id = Some(bid.to_string());
                            }
                        }
                    }
                }
            }
            if !artists.is_empty() {
                break;
            }
        }
    }

    let artist = if !artists.is_empty() {
        artists.join(", ")
    } else {
        let fallback = renderer
            .get("shortBylineText")
            .or_else(|| renderer.get("longBylineText"))
            .map(runs_text)
            .unwrap_or_default();
        let first_segment = fallback
            .split(['•', '·'])
            .next()
            .unwrap_or("")
            .trim();
        if first_segment.is_empty() {
            source.artist.clone()
        } else {
            first_segment.to_string()
        }
    };

    let duration_secs = renderer
        .get("lengthText")
        .map(runs_text)
        .as_deref()
        .and_then(parse_mm_ss)
        .or(source.duration_secs);

    let cover = renderer
        .pointer("/thumbnail/thumbnails")
        .and_then(best_json_thumb);
    let cover_url = cover.clone().or_else(|| source.cover_url.clone());
    let cover_large_url = cover.or_else(|| source.cover_large_url.clone());

    let is_atv = panel_video_music_type(renderer) == Some("MUSIC_VIDEO_TYPE_ATV");

    Some(Track {
        is_music: true,
        is_video: if want_video { !is_atv } else { false },
        id,
        title,
        artist,
        artist_id: artist_id.or_else(|| source.artist_id.clone()),
        album: album.or_else(|| (!want_video).then(|| source.album.clone()).flatten()),
        album_id: album_id.or_else(|| (!want_video).then(|| source.album_id.clone()).flatten()),
        duration_secs,
        view_count: None,
        cover_url,
        cover_large_url,
    })
}

fn parse_watch_counterpart_json(
    json: &serde_json::Value,
    source: &Track,
    want_video: bool,
) -> Option<Track> {
    let tabs = json
        .pointer("/contents/singleColumnMusicWatchNextResultsRenderer/tabbedRenderer/watchNextTabbedResultsRenderer/tabs")
        .and_then(|v| v.as_array())?;

    let contents = tabs.iter().find_map(|tab| {
        tab.pointer("/tabRenderer/content/musicQueueRenderer/content/playlistPanelRenderer/contents")
            .and_then(|v| v.as_array())
    })?;

    for item in contents {
        let Some(wrapper) = item.get("playlistPanelVideoWrapperRenderer") else {
            continue;
        };

        let mut renderers: Vec<&serde_json::Value> = Vec::new();
        if let Some(primary) = wrapper.pointer("/primaryRenderer/playlistPanelVideoRenderer") {
            renderers.push(primary);
        }
        if let Some(counterparts) = wrapper.get("counterpart").and_then(|v| v.as_array()) {
            for cp in counterparts {
                if let Some(r) = cp.pointer("/counterpartRenderer/playlistPanelVideoRenderer") {
                    renderers.push(r);
                }
            }
        }

        // Only match the wrapper that corresponds to the requested track.
        if !renderers
            .iter()
            .any(|r| panel_video_id(r) == Some(source.id.as_str()))
        {
            continue;
        }

        let candidate = renderers.into_iter().find(|r| {
            let Some(vid) = panel_video_id(r) else {
                return false;
            };
            if vid == source.id {
                return false;
            }
            match panel_video_music_type(r) {
                Some("MUSIC_VIDEO_TYPE_ATV") => !want_video,
                Some(_) => want_video,
                None => true,
            }
        });

        if let Some(renderer) = candidate {
            return track_from_panel_video_renderer(renderer, source, want_video);
        }
    }

    None
}

#[allow(dead_code)]
fn is_playlist_audio(track: &rustypipe::model::TrackItem) -> bool {
    track.track_type == TrackType::Track && !track.unavailable
}

fn is_playlist_playable(track: &rustypipe::model::TrackItem) -> bool {
    matches!(track.track_type, TrackType::Track | TrackType::Video) && !track.unavailable
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
    tracks_from_ytm_playlist_inner(items, false)
}

fn tracks_from_ytm_playlist_with_videos(items: Vec<PlaylistItem>) -> Vec<Track> {
    tracks_from_ytm_playlist_inner(items, true)
}

fn tracks_from_ytm_playlist_inner(items: Vec<PlaylistItem>, include_videos: bool) -> Vec<Track> {
    items
        .into_iter()
        .filter_map(|item| match item {
            PlaylistItem::Song(song) if song.is_available => {
                let cover = best_ytm_thumb(&song.thumbnails);
                let first_artist_id = song
                    .artists
                    .iter()
                    .find_map(|a| a.id.as_ref().map(|id| id.get_raw().to_string()));
                Some(Track {
                    is_music: true,
                    is_video: false,
                    id: song.video_id.get_raw().to_string(),
                    title: clean_track_title(&song.title),
                    artist: song
                        .artists
                        .into_iter()
                        .map(|a| a.name)
                        .collect::<Vec<_>>()
                        .join(", "),
                    artist_id: first_artist_id,
                    album: Some(song.album.name),
                    album_id: Some(song.album.id.get_raw().to_string()),
                    duration_secs: parse_mm_ss(&song.duration),
                    view_count: None,
                    cover_url: cover.clone(),
                    cover_large_url: cover,
                })
            }
            PlaylistItem::Video(video) if include_videos && video.is_available => {
                let cover = best_ytm_thumb(&video.thumbnails);
                let artist_id = {
                    let raw = video.channel_id.get_raw();
                    (!raw.is_empty()).then(|| raw.to_string())
                };
                Some(Track {
                    is_music: true,
                    is_video: true,
                    id: video.video_id.get_raw().to_string(),
                    title: clean_track_title(&video.title),
                    artist: if video.channel_name.is_empty() {
                        String::from("Artista desconocido")
                    } else {
                        video.channel_name
                    },
                    artist_id,
                    album: None,
                    album_id: None,
                    duration_secs: parse_mm_ss(&video.duration),
                    view_count: None,
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
    let album_id = song.album.as_ref().map(|a| a.id.get_raw().to_string());
    Track {
        is_music: false,
        is_video: false,
        id: song.video_id.get_raw().to_string(),
        title: clean_track_title(&song.title),
        artist: if song.artist.is_empty() {
            String::from("Artista desconocido")
        } else {
            song.artist
        },
        artist_id: None,
        album: song.album.map(|a| a.name),
        album_id,
        duration_secs: parse_mm_ss(&song.duration),
        view_count: None,
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
        let items: Vec<PlaylistItem> =
            serde_json::from_value(serde_json::json!([video, song, unavailable])).unwrap();
        let tracks = tracks_from_ytm_playlist(items.clone());
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].id, "audio123456");
        assert_eq!(tracks[0].duration_secs, Some(180));
        assert!(tracks[0].is_music);
        assert!(!tracks[0].is_video);

        let with_videos = tracks_from_ytm_playlist_with_videos(items);
        assert_eq!(with_videos.len(), 2);
        assert_eq!(with_videos[0].id, "video123456");
        assert!(with_videos[0].is_music);
        assert!(with_videos[0].is_video);
        assert_eq!(with_videos[1].id, "audio123456");
        assert!(!with_videos[1].is_video);
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
            top_artists: vec![],
            country: None,
            available_countries: vec![],
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

    #[test]
    fn parses_official_watch_counterpart_song_to_video_and_back() {
        let make_renderer = |id: &str, title: &str, mtype: &str, with_album: bool| {
            let mut byline_runs = vec![serde_json::json!({
                "text": "Daft Punk",
                "navigationEndpoint": {
                    "browseEndpoint": {
                        "browseId": "UC_daftpunk123",
                        "browseEndpointContextSupportedConfigs": {
                            "browseEndpointContextMusicConfig": {
                                "pageType": "MUSIC_PAGE_TYPE_ARTIST"
                            }
                        }
                    }
                }
            })];
            if with_album {
                byline_runs.push(serde_json::json!({ "text": " • " }));
                byline_runs.push(serde_json::json!({
                    "text": "Random Access Memories",
                    "navigationEndpoint": {
                        "browseEndpoint": {
                            "browseId": "MPREb_ram123",
                            "browseEndpointContextSupportedConfigs": {
                                "browseEndpointContextMusicConfig": {
                                    "pageType": "MUSIC_PAGE_TYPE_ALBUM"
                                }
                            }
                        }
                    }
                }));
            }
            serde_json::json!({
                "videoId": id,
                "title": { "runs": [{ "text": title }] },
                "lengthText": { "runs": [{ "text": "4:08" }] },
                "longBylineText": { "runs": byline_runs },
                "shortBylineText": { "runs": [{ "text": "Daft Punk" }] },
                "navigationEndpoint": {
                    "watchEndpoint": {
                        "videoId": id,
                        "watchEndpointMusicSupportedConfigs": {
                            "watchEndpointMusicConfig": {
                                "musicVideoType": mtype
                            }
                        }
                    }
                },
                "thumbnail": {
                    "thumbnails": [
                        { "url": "https://lh3.googleusercontent.com/thumb=w226-h226", "width": 226, "height": 226 }
                    ]
                }
            })
        };

        let watch_json = serde_json::json!({
            "contents": {
                "singleColumnMusicWatchNextResultsRenderer": {
                    "tabbedRenderer": {
                        "watchNextTabbedResultsRenderer": {
                            "tabs": [{
                                "tabRenderer": {
                                    "content": {
                                        "musicQueueRenderer": {
                                            "content": {
                                                "playlistPanelRenderer": {
                                                    "contents": [{
                                                        "playlistPanelVideoWrapperRenderer": {
                                                            "primaryRenderer": {
                                                                "playlistPanelVideoRenderer": make_renderer(
                                                                    "atvSong1234",
                                                                    "Get Lucky",
                                                                    "MUSIC_VIDEO_TYPE_ATV",
                                                                    true
                                                                )
                                                            },
                                                            "counterpart": [{
                                                                "counterpartRenderer": {
                                                                    "playlistPanelVideoRenderer": make_renderer(
                                                                        "omvVid56789",
                                                                        "Get Lucky (Official Video)",
                                                                        "MUSIC_VIDEO_TYPE_OMV",
                                                                        false
                                                                    )
                                                                }
                                                            }]
                                                        }
                                                    }]
                                                }
                                            }
                                        }
                                    }
                                }
                            }]
                        }
                    }
                }
            }
        });

        let song = Track {
            is_music: true,
            is_video: false,
            id: "atvSong1234".into(),
            title: "Get Lucky".into(),
            artist: "Daft Punk".into(),
            artist_id: Some("UC_daftpunk123".into()),
            album: Some("Random Access Memories".into()),
            album_id: Some("MPREb_ram123".into()),
            duration_secs: Some(248),
            view_count: None,
            cover_url: None,
            cover_large_url: None,
        };

        // Song -> Video
        let mv = parse_watch_counterpart_json(&watch_json, &song, true)
            .expect("should resolve official OMV counterpart");
        assert_eq!(mv.id, "omvVid56789");
        assert!(mv.is_video);
        assert!(mv.is_music);
        assert_eq!(mv.title, "Get Lucky");
        assert_eq!(mv.artist, "Daft Punk");
        assert_eq!(mv.artist_id.as_deref(), Some("UC_daftpunk123"));
        assert_eq!(mv.duration_secs, Some(248));

        // Video -> Studio Audio (even when ATV is in primaryRenderer and OMV is in counterpart)
        let back_to_song = parse_watch_counterpart_json(&watch_json, &mv, false)
            .expect("should resolve official ATV counterpart");
        assert_eq!(back_to_song.id, "atvSong1234");
        assert!(!back_to_song.is_video);
        assert!(back_to_song.is_music);
        assert_eq!(back_to_song.album.as_deref(), Some("Random Access Memories"));
        assert_eq!(back_to_song.album_id.as_deref(), Some("MPREb_ram123"));

        // Non-matching track ID returns None so caller falls back to search
        let other = Track {
            id: "otherId9999".into(),
            ..song
        };
        assert!(parse_watch_counterpart_json(&watch_json, &other, true).is_none());
    }
}
