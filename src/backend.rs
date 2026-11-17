use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

use rs_ytmusic_api::MusicClient;
use rs_ytmusic_api::types::{
    AlbumDetailed as RsAlbum, ArtistDetailed as RsArtist, HomeContent, PlaylistDetailed as RsPlaylist,
    SongDetailed as RsSong,
};
use ytmapi_rs::YtMusic;
use ytmapi_rs::auth::noauth::NoAuthToken;
use ytmapi_rs::common::{
    AlbumID, ArtistChannelID, MoodCategoryParams, PlaylistID, VideoID, YoutubeID,
};
use ytmapi_rs::parse::{
    GetArtistTopReleases, MoodCategory, MoodPlaylistCategory, PlaylistItem, SearchResultAlbum,
    SearchResultArtist, SearchResultPlaylist, SearchResultSong,
};

use crate::models::{
    Album, Artist, ArtistDetails, Genre, LyricsData, Playlist, SearchResults, StreamInfo, Track,
    cache_dir, clean_track_title, color_to_hex, is_ytm_square_cover, parse_lrc,
};

const HOME_CACHE_TTL: Duration = Duration::from_secs(15 * 60);

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

#[derive(Clone)]
pub struct HomeFeed {
    pub trending: Vec<Track>,
    pub top: Vec<Track>,
    pub new_albums: Vec<Album>,
}

#[derive(Clone)]
pub struct Backend {
    ytm: Arc<tokio::sync::OnceCell<YtMusic<NoAuthToken>>>,
    rs_ytm: Arc<tokio::sync::OnceCell<MusicClient>>,
    http: reqwest::Client,
    home_cache: Arc<RwLock<Option<(Instant, HomeFeed)>>>,
    genres_cache: Arc<RwLock<Option<(Instant, Vec<Genre>)>>>,
    stream_cache: Arc<RwLock<HashMap<String, (Instant, StreamInfo)>>>,
    in_flight_resolves: Arc<Mutex<HashMap<String, Arc<tokio::sync::Notify>>>>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            ytm: Arc::new(tokio::sync::OnceCell::new()),
            rs_ytm: Arc::new(tokio::sync::OnceCell::new()),
            http: reqwest::Client::new(),
            home_cache: Arc::new(RwLock::new(None)),
            genres_cache: Arc::new(RwLock::new(None)),
            stream_cache: Arc::new(RwLock::new(HashMap::new())),
            in_flight_resolves: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn primary(&self) -> Result<&YtMusic<NoAuthToken>, String> {
        self.ytm
            .get_or_try_init(|| async {
                YtMusic::new_unauthenticated()
                    .await
                    .map_err(|e| format!("Error inicializando ytmapi-rs: {e:?}"))
            })
            .await
    }

    async fn fallback(&self) -> Result<&MusicClient, String> {
        self.rs_ytm
            .get_or_try_init(|| async {
                MusicClient::new()
                    .init()
                    .await
                    .map_err(|e| format!("Error inicializando rs-ytmusic-api: {e}"))
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

        if let Ok(rs) = self.fallback().await
            && let Ok(sections) = rs.fetch_home().await
        {
            for section in sections {
                for item in section.contents {
                    match item {
                        HomeContent::Song(song) => {
                            let track = track_from_rs_song(song);
                            if trending.len() < 12 {
                                trending.push(track);
                            } else if top.len() < 12 {
                                top.push(track);
                            }
                        }
                        HomeContent::Album(alb) => {
                            if new_albums.len() < 12 {
                                new_albums.push(album_from_rs_album(alb));
                            }
                        }
                        HomeContent::Playlist(_) => {}
                    }
                }
            }
        }

        if trending.is_empty()
            && let Ok(ytm) = self.primary().await
        {
            if let Ok(songs) = ytm.search_songs("top hits").await {
                trending = songs.into_iter().take(12).map(track_from_ytm_song).collect();
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
        if let Ok(mut guard) = self.home_cache.write() {
            *guard = Some((Instant::now(), feed.clone()));
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
            let tracks = album
                .tracks
                .into_iter()
                .map(|song| Track {
                    id: song.video_id.get_raw().to_string(),
                    title: clean_track_title(&song.title),
                    artist: if artist_names.is_empty() {
                        String::from("Artista desconocido")
                    } else {
                        artist_names.clone()
                    },
                    album: Some(album_name.clone()),
                    duration_secs: parse_mm_ss(&song.duration),
                    cover_url: cover.clone(),
                    cover_large_url: cover.clone(),
                })
                .collect();
            return Ok((album_name, tracks));
        }

        let rs = self.fallback().await?;
        let album = rs
            .fetch_album(album_id)
            .await
            .map_err(|e| format!("Error cargando álbum: {e}"))?;
        let album_name = album.name.clone();
        let cover = best_rs_thumb(&album.thumbnails);
        let tracks = album
            .songs
            .into_iter()
            .map(|s| {
                let mut t = track_from_rs_song(s);
                if t.album.is_none() {
                    t.album = Some(album_name.clone());
                }
                if t.cover_url.is_none() {
                    t.cover_url = cover.clone();
                    t.cover_large_url = cover.clone();
                }
                t
            })
            .collect();
        Ok((album_name, tracks))
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
        let categories: Vec<MoodPlaylistCategory> = ytm
            .get_mood_playlists(MoodCategoryParams::from_raw(genre_id))
            .await
            .map_err(|e| format!("Error obteniendo playlists del género: {e:?}"))?;

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

    pub async fn fetch_playlist_tracks(
        &self,
        playlist_id: &str,
    ) -> Result<(String, Vec<Track>), String> {
        let ytm = self.primary().await?;
        let pid = PlaylistID::from_raw(playlist_id);
        let details = ytm
            .get_playlist_details(pid.clone())
            .await
            .map_err(|e| format!("Error cargando playlist: {e:?}"))?;
        let items = ytm
            .get_playlist_tracks(pid)
            .await
            .map_err(|e| format!("Error cargando pistas de playlist: {e:?}"))?;

        let tracks = items
            .into_iter()
            .filter_map(|item| match item {
                PlaylistItem::Song(s) => {
                    let artist = s
                        .artists
                        .into_iter()
                        .map(|a| a.name)
                        .collect::<Vec<_>>()
                        .join(", ");
                    Some(Track {
                        id: s.video_id.get_raw().to_string(),
                        title: clean_track_title(&s.title),
                        artist: if artist.is_empty() {
                            String::from("Artista desconocido")
                        } else {
                            artist
                        },
                        album: Some(s.album.name),
                        duration_secs: parse_mm_ss(&s.duration),
                        cover_url: best_ytm_thumb(&s.thumbnails),
                        cover_large_url: best_ytm_thumb(&s.thumbnails),
                    })
                }
                PlaylistItem::Video(v) => Some(Track {
                    id: v.video_id.get_raw().to_string(),
                    title: clean_track_title(&v.title),
                    artist: v.channel_name,
                    album: None,
                    duration_secs: parse_mm_ss(&v.duration),
                    cover_url: best_ytm_thumb(&v.thumbnails),
                    cover_large_url: best_ytm_thumb(&v.thumbnails),
                }),
                _ => None,
            })
            .collect();

        Ok((details.title, tracks))
    }

    pub async fn fetch_radio_tracks(&self, video_id: &str) -> Result<Vec<Track>, String> {
        if let Ok(rs) = self.fallback().await
            && let Ok(up_next) = rs.fetch_watch_queue(video_id).await
            && !up_next.is_empty()
        {
            return Ok(up_next
                .into_iter()
                .map(|item| Track {
                    id: item.video_id,
                    title: clean_track_title(&item.title),
                    artist: if item.artists.name.is_empty() {
                        String::from("Artista desconocido")
                    } else {
                        item.artists.name
                    },
                    album: None,
                    duration_secs: item.duration.map(|d| d as u32),
                    cover_url: best_rs_thumb(&item.thumbnails),
                    cover_large_url: best_rs_thumb(&item.thumbnails),
                })
                .collect());
        }

        let ytm = self.primary().await?;
        let watch = ytm
            .get_watch_playlist_from_video_id(VideoID::from_raw(video_id))
            .await
            .map_err(|e| format!("Error iniciando radio: {e:?}"))?;
        let tracks = watch
            .into_iter()
            .map(|t| Track {
                id: t.video_id.get_raw().to_string(),
                title: clean_track_title(&t.title),
                artist: t.author,
                album: None,
                duration_secs: parse_mm_ss(&t.duration),
                cover_url: best_ytm_thumb(&t.thumbnails),
                cover_large_url: best_ytm_thumb(&t.thumbnails),
            })
            .collect();
        Ok(tracks)
    }

    pub async fn search_suggestions(&self, query_str: &str) -> Result<Vec<String>, String> {
        if let Ok(ytm) = self.primary().await
            && let Ok(suggestions) = ytm.get_search_suggestions(query_str).await
        {
            let list: Vec<String> = suggestions
                .into_iter()
                .map(|s| s.runs.into_iter().map(|r| r.take_text()).collect::<String>())
                .filter(|s| !s.is_empty())
                .collect();
            if !list.is_empty() {
                return Ok(list);
            }
        }
        let rs = self.fallback().await?;
        rs.suggest(query_str)
            .await
            .map_err(|e| format!("Error obteniendo sugerencias: {e}"))
    }

    pub async fn search_all(&self, query_str: &str) -> Result<SearchResults, String> {
        if let Ok(ytm) = self.primary().await {
            let (songs_res, albums_res, artists_res, playlists_res) = tokio::join!(
                ytm.search_songs(query_str),
                ytm.search_albums(query_str),
                ytm.search_artists(query_str),
                ytm.search_playlists(query_str),
            );

            if songs_res.is_ok() || albums_res.is_ok() || artists_res.is_ok() {
                let tracks: Vec<Track> = songs_res
                    .unwrap_or_default()
                    .into_iter()
                    .map(track_from_ytm_song)
                    .collect();
                let albums: Vec<Album> = albums_res
                    .unwrap_or_default()
                    .into_iter()
                    .map(album_from_ytm_album)
                    .collect();
                let artists: Vec<Artist> = artists_res
                    .unwrap_or_default()
                    .into_iter()
                    .map(artist_from_ytm_artist)
                    .collect();
                let playlists: Vec<Playlist> = playlists_res
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(playlist_from_ytm_playlist)
                    .collect();
                if !tracks.is_empty() || !albums.is_empty() || !artists.is_empty() {
                    return Ok(SearchResults {
                        tracks,
                        albums,
                        artists,
                        playlists,
                    });
                }
            }
        }

        let rs = self.fallback().await?;
        let (songs_res, albums_res, artists_res, playlists_res) = tokio::join!(
            rs.find_songs(query_str),
            rs.find_albums(query_str),
            rs.find_artists(query_str),
            rs.find_playlists(query_str),
        );

        Ok(SearchResults {
            tracks: songs_res
                .unwrap_or_default()
                .into_iter()
                .map(track_from_rs_song)
                .collect(),
            albums: albums_res
                .unwrap_or_default()
                .into_iter()
                .map(album_from_rs_album)
                .collect(),
            artists: artists_res
                .unwrap_or_default()
                .into_iter()
                .map(artist_from_rs_artist)
                .collect(),
            playlists: playlists_res
                .unwrap_or_default()
                .into_iter()
                .map(playlist_from_rs_playlist)
                .collect(),
        })
    }

    pub async fn fetch_artist(&self, artist_id: &str) -> Result<ArtistDetails, String> {
        let (ytm_res, rs_res) = tokio::join!(
            async {
                match self.primary().await {
                    Ok(ytm) => ytm
                        .get_artist(ArtistChannelID::from_raw(artist_id))
                        .await
                        .ok(),
                    Err(_) => None,
                }
            },
            async {
                match self.fallback().await {
                    Ok(rs) => rs.fetch_artist(artist_id).await.ok(),
                    Err(_) => None,
                }
            }
        );

        let rs_cover_map: HashMap<String, Option<String>> = rs_res
            .as_ref()
            .map(|r| {
                r.top_songs
                    .iter()
                    .map(|s| (s.video_id.clone(), best_rs_thumb(&s.thumbnails)))
                    .collect()
            })
            .unwrap_or_default();

        if let Some(artist) = ytm_res {
            let name = artist.name.clone();
            let GetArtistTopReleases {
                songs,
                albums,
                singles,
                ..
            } = artist.top_releases;

            let tracks: Vec<Track> = if let Some(s) = songs {
                let full_tracks = if let Ok(ytm) = self.primary().await
                    && let Ok(items) = ytm.get_playlist_tracks(s.browse_id).await
                {
                    items
                        .into_iter()
                        .filter_map(|item| match item {
                            PlaylistItem::Song(ps) => {
                                let artist_str = ps
                                    .artists
                                    .into_iter()
                                    .map(|a| a.name)
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                Some(Track {
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
                            let cover = rs_cover_map.get(&id).cloned().flatten().or_else(|| {
                                Some(format!("https://i.ytimg.com/vi/{id}/hqdefault.jpg"))
                            });
                            Track {
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
                header_url: best_ytm_thumb(&artist.thumbnails),
                subscriber_count: artist.subscribers.as_deref().and_then(parse_sub_count),
                description: artist.description,
                tracks,
                albums: all_albums,
                playlists: Vec::new(),
                similar_artists: Vec::new(),
            });
        }

        let artist = rs_res.ok_or_else(|| String::from("Error cargando artista"))?;
        let tracks = artist
            .top_songs
            .into_iter()
            .map(track_from_rs_song)
            .collect();
        let mut albums: Vec<Album> = artist
            .top_albums
            .into_iter()
            .map(album_from_rs_album)
            .collect();
        albums.extend(artist.top_singles.into_iter().map(album_from_rs_album));

        Ok(ArtistDetails {
            name: artist.name,
            header_url: best_rs_thumb(&artist.thumbnails),
            subscriber_count: None,
            description: None,
            tracks,
            albums,
            playlists: Vec::new(),
            similar_artists: Vec::new(),
        })
    }

    fn streams_dir() -> Result<std::path::PathBuf, String> {
        let base = cache_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("gyrotube"))
            .join("streams");
        std::fs::create_dir_all(&base)
            .map_err(|e| format!("No se pudo crear caché de audio: {e}"))?;
        Ok(base)
    }

    pub fn cached_stream(&self, video_id: &str) -> Option<StreamInfo> {
        let dir = Self::streams_dir().ok()?;
        Self::find_cached_audio(video_id, &dir)
    }

    fn find_cached_audio(video_id: &str, streams_dir: &std::path::Path) -> Option<StreamInfo> {
        for (ext, label) in [
            ("m4a", "M4A"),
            ("webm", "WEBM"),
            ("ytdlp.m4a", "M4A"),
            ("ytdlp.webm", "WEBM"),
        ] {
            let path = streams_dir.join(format!("{video_id}.{ext}"));
            if path.is_file()
                && std::fs::metadata(&path)
                    .map(|m| m.len() > 0)
                    .unwrap_or(false)
            {
                return Some(StreamInfo {
                    url: path.to_string_lossy().into_owned(),
                    quality_label: label.to_string(),
                    user_agent: None,
                    headers: HashMap::new(),
                });
            }
        }
        None
    }

    pub async fn resolve_audio_stream(&self, video_id: &str) -> Result<StreamInfo, String> {
        if video_id.len() != 11
            || !video_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err("ID de video inválido".into());
        }

        let streams_dir = Self::streams_dir()?;
        if let Some(cached) = Self::find_cached_audio(video_id, &streams_dir) {
            return Ok(cached);
        }

        self.download_with_ytdlp_fallback(video_id).await
    }

    pub async fn download_with_ytdlp_fallback(&self, video_id: &str) -> Result<StreamInfo, String> {
        if video_id.len() != 11
            || !video_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err("ID de video inválido".into());
        }
        let streams_dir = Self::streams_dir()?;
        if let Some(cached) = Self::find_cached_audio(video_id, &streams_dir) {
            return Ok(cached);
        }

        let existing_notify = {
            let mut in_flight = self
                .in_flight_resolves
                .lock()
                .map_err(|_| String::from("Error interno de sincronización"))?;
            if let Some(notify) = in_flight.get(video_id) {
                Some(Arc::clone(notify))
            } else {
                in_flight.insert(video_id.to_string(), Arc::new(tokio::sync::Notify::new()));
                None
            }
        };

        if let Some(notify) = existing_notify {
            notify.notified().await;
            if let Some(cached) = Self::find_cached_audio(video_id, &streams_dir) {
                return Ok(cached);
            }
            return Err(String::from("La descarga previa no produjo audio."));
        }

        let result = self.run_ytdlp_download(video_id, &streams_dir).await;

        if let Ok(mut in_flight) = self.in_flight_resolves.lock()
            && let Some(notify) = in_flight.remove(video_id)
        {
            notify.notify_waiters();
        }

        result
    }

    async fn run_ytdlp_download(
        &self,
        video_id: &str,
        streams_dir: &std::path::Path,
    ) -> Result<StreamInfo, String> {
        if let Some(cached) = Self::find_cached_audio(video_id, streams_dir) {
            return Ok(cached);
        }

        let staging = tempfile::tempdir_in(streams_dir)
            .map_err(|e| format!("Error al preparar descarga de audio: {e}"))?;

        let mut fast_cmd = tokio::process::Command::new("yt-dlp");
        fast_cmd
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-progress",
                "--concurrent-fragments",
                "4",
                "--extractor-args",
                "youtube:player_client=android_vr",
            ])
            .args(["-f", "251/140/bestaudio", "-o"])
            .arg(staging.path().join("audio.%(ext)s"))
            .arg("--")
            .arg(format!("https://www.youtube.com/watch?v={video_id}"));

        let mut output = tokio::time::timeout(Duration::from_secs(90), fast_cmd.output())
            .await
            .map_err(|_| String::from("Tiempo de espera agotado en yt-dlp"))?
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    String::from("Instala yt-dlp en PATH para reproducir audio.")
                } else {
                    format!("Error al ejecutar yt-dlp: {e}")
                }
            })?;

        if !output.status.success() {
            let mut command = tokio::process::Command::new("yt-dlp");
            command
                .kill_on_drop(true)
                .args([
                    "--ignore-config",
                    "--no-playlist",
                    "--no-progress",
                    "--concurrent-fragments",
                    "4",
                    "--js-runtimes",
                    "node",
                    "--remote-components",
                    "ejs:github",
                ])
                .args(["-f", "251/140/bestaudio", "-o"])
                .arg(staging.path().join("audio.%(ext)s"))
                .arg("--")
                .arg(format!("https://www.youtube.com/watch?v={video_id}"));
            output = tokio::time::timeout(Duration::from_secs(90), command.output())
                .await
                .map_err(|_| String::from("Tiempo de espera agotado en yt-dlp"))?
                .map_err(|e| format!("Error al ejecutar yt-dlp: {e}"))?;
        }

        if !output.status.success() {
            eprintln!(
                "yt-dlp ({video_id}): {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(String::from("yt-dlp no pudo descargar el audio."));
        }

        let downloaded = std::fs::read_dir(staging.path())
            .ok()
            .and_then(|mut it| it.find_map(|e| e.ok()))
            .map(|e| e.path())
            .ok_or_else(|| String::from("yt-dlp no generó archivo de audio."))?;
        let ext = downloaded
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("webm");
        let path = streams_dir.join(format!("{video_id}.ytdlp.{ext}"));
        std::fs::rename(&downloaded, &path)
            .map_err(|e| format!("Error al guardar audio de yt-dlp: {e}"))?;

        if let Ok(mut guard) = self.stream_cache.write() {
            guard.remove(video_id);
        }

        Ok(StreamInfo {
            url: path.to_string_lossy().into_owned(),
            quality_label: ext.to_uppercase(),
            user_agent: None,
            headers: HashMap::new(),
        })
    }

    pub async fn prefetch_audio(&self, video_id: &str) {
        if video_id.len() != 11
            || !video_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return;
        }
        if let Ok(dir) = Self::streams_dir()
            && Self::find_cached_audio(video_id, &dir).is_some()
        {
            return;
        }
        if let Ok(in_flight) = self.in_flight_resolves.lock()
            && in_flight.contains_key(video_id)
        {
            return;
        }

        let _ = self.resolve_audio_stream(video_id).await;
    }

    pub async fn fetch_synced_lyrics(&self, track: &Track) -> Result<LyricsData, String> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LrcLibResp {
            synced_lyrics: Option<String>,
            plain_lyrics: Option<String>,
        }

        let artist_q = track.artist.split(',').next().unwrap_or(&track.artist).trim();
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
        if let Ok(rs) = self.fallback().await
            && let Ok(Some(lines)) = rs.fetch_lyrics(video_id).await
        {
            let text = lines.join("\n").trim().to_string();
            if !text.is_empty() {
                return Ok(text);
            }
        }

        if let Ok(ytm) = self.primary().await
            && let Ok(lyrics_id) = ytm.get_lyrics_id(VideoID::from_raw(video_id)).await
            && let Ok(lyrics) = ytm.get_lyrics(lyrics_id).await
        {
            let text = lyrics.lyrics.trim().to_string();
            if !text.is_empty() {
                return Ok(text);
            }
        }

        Err(String::from("No hay letras disponibles para esta pista."))
    }

    pub async fn fetch_image_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        let resp = self
            .http
            .get(url)
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
        if let Some(url) = track
            .cover_large_url
            .as_deref()
            .or(track.cover_url.as_deref())
            && is_ytm_square_cover(url)
        {
            return Some(url.to_string());
        }

        let query_str = format!("{} {}", track.artist, track.title);
        if let Ok(ytm) = self.primary().await
            && let Ok(songs) = ytm.search_songs(query_str.as_str()).await
            && let Some(found) = songs
                .iter()
                .find(|s| s.video_id.get_raw() == track.id)
                .or_else(|| songs.first())
        {
            return best_ytm_thumb(&found.thumbnails);
        }

        None
    }
}

fn best_ytm_thumb(thumbs: &[ytmapi_rs::common::Thumbnail]) -> Option<String> {
    thumbs.last().map(|t| t.url.clone())
}

fn best_rs_thumb(thumbs: &[rs_ytmusic_api::types::ThumbnailFull]) -> Option<String> {
    thumbs.last().map(|t| t.url.clone())
}

fn track_from_ytm_song(song: SearchResultSong) -> Track {
    let cover = best_ytm_thumb(&song.thumbnails);
    Track {
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

fn track_from_rs_song(song: RsSong) -> Track {
    let cover = best_rs_thumb(&song.thumbnails);
    Track {
        id: song.video_id,
        title: clean_track_title(&song.name),
        artist: if song.artist.name.is_empty() {
            String::from("Artista desconocido")
        } else {
            song.artist.name
        },
        album: song.album.map(|a| a.name),
        duration_secs: song.duration.map(|d| d as u32),
        cover_url: cover.clone(),
        cover_large_url: cover,
    }
}

fn album_from_rs_album(alb: RsAlbum) -> Album {
    Album {
        id: alb.album_id,
        title: alb.name,
        artist: if alb.artist.name.is_empty() {
            String::from("Artista desconocido")
        } else {
            alb.artist.name
        },
        year: alb.year.map(|y| y as u16),
        cover_url: best_rs_thumb(&alb.thumbnails),
    }
}

fn artist_from_rs_artist(art: RsArtist) -> Artist {
    Artist {
        id: art.artist_id,
        name: art.name,
        avatar_url: best_rs_thumb(&art.thumbnails),
        subscriber_count: None,
    }
}

fn playlist_from_rs_playlist(pl: RsPlaylist) -> Playlist {
    Playlist {
        id: pl.playlist_id,
        name: pl.name,
        subtitle: pl.artist.name,
        cover_url: best_rs_thumb(&pl.thumbnails),
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
    let num_part: String = s
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let val = num_part.parse::<f64>().ok()?;
    let mult = if s.contains('M') {
        1_000_000.0
    } else if s.contains('K') {
        1_000.0
    } else {
        1.0
    };
    Some((val * mult) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_cached_audio_reuses_local_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file = dir.path().join("dQw4w9WgXcQ.m4a");
        std::fs::write(&file, b"audio-bytes").expect("write cached audio");

        let found = Backend::find_cached_audio("dQw4w9WgXcQ", dir.path()).expect("cached audio");
        assert_eq!(found.url, file.to_string_lossy());
        assert_eq!(found.quality_label, "M4A");
    }

    #[test]
    fn parses_mm_ss_and_sub_counts() {
        assert_eq!(parse_mm_ss("3:45"), Some(225));
        assert_eq!(parse_mm_ss("1:02:03"), Some(3723));
        assert_eq!(parse_sub_count("9.4M subscribers"), Some(9_400_000));
        assert_eq!(parse_sub_count("120K"), Some(120_000));
    }
}
