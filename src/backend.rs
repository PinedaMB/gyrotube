use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use rustypipe::client::RustyPipe;
use rustypipe::model::MusicItem;
use rustypipe::param::StreamFilter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{OnceCell, RwLock};

use crate::models::{
    Album, Artist, ArtistDetails, Genre, Playlist, SearchResults, StreamInfo, Track, cache_dir,
    clean_track_title, is_ytm_square_cover, pick_cover,
};

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
struct ProxyEntry {
    upstream_url: String,
    user_agent: String,
    content_type: String,
}

struct StreamProxy {
    port: u16,
    next_id: AtomicU64,
    entries: Arc<RwLock<HashMap<u64, ProxyEntry>>>,
}

static STREAM_PROXY: OnceCell<StreamProxy> = OnceCell::const_new();

impl StreamProxy {
    async fn get_or_init(http: reqwest::Client) -> Result<&'static StreamProxy, String> {
        STREAM_PROXY
            .get_or_try_init(|| async move {
                let listener = TcpListener::bind("127.0.0.1:0")
                    .await
                    .map_err(|e| format!("No se pudo iniciar proxy local: {e}"))?;
                let port = listener
                    .local_addr()
                    .map_err(|e| format!("Error en puerto de proxy: {e}"))?
                    .port();
                let entries = Arc::new(RwLock::new(HashMap::new()));
                let entries_bg = Arc::clone(&entries);

                tokio::spawn(async move {
                    loop {
                        let Ok((socket, _)) = listener.accept().await else {
                            continue;
                        };
                        let entries = Arc::clone(&entries_bg);
                        let http = http.clone();
                        tokio::spawn(async move {
                            let _ = handle_proxy_connection(socket, entries, http).await;
                        });
                    }
                });

                Ok(StreamProxy {
                    port,
                    next_id: AtomicU64::new(1),
                    entries,
                })
            })
            .await
    }

    async fn register(&self, upstream_url: String, user_agent: String, content_type: String) -> String {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut map = self.entries.write().await;
        if map.len() >= 64
            && let Some(oldest) = map.keys().min().copied()
        {
            map.remove(&oldest);
        }
        map.insert(
            id,
            ProxyEntry {
                upstream_url,
                user_agent,
                content_type,
            },
        );
        format!("http://127.0.0.1:{}/stream/{id}", self.port)
    }
}

async fn handle_proxy_connection(
    mut socket: TcpStream,
    entries: Arc<RwLock<HashMap<u64, ProxyEntry>>>,
    http: reqwest::Client,
) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(2048);
    let mut tmp = [0u8; 1024];
    loop {
        let n = socket.read(&mut tmp).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") || buf.len() > 16 * 1024 {
            break;
        }
    }

    let req_str = String::from_utf8_lossy(&buf);
    let mut lines = req_str.lines();
    let Some(first_line) = lines.next() else {
        return Ok(());
    };
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("");
    let id = path
        .strip_prefix("/stream/")
        .and_then(|s| s.split('?').next())
        .and_then(|s| s.parse::<u64>().ok());

    let Some(entry) = (match id {
        Some(id) => entries.read().await.get(&id).cloned(),
        None => None,
    }) else {
        socket
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await?;
        return Ok(());
    };

    let range_header = lines.find_map(|line| {
        let (k, v) = line.split_once(':')?;
        if k.trim().eq_ignore_ascii_case("range") {
            Some(v.trim().to_string())
        } else {
            None
        }
    });

    let mut req = http
        .request(
            if method.eq_ignore_ascii_case("HEAD") {
                reqwest::Method::HEAD
            } else {
                reqwest::Method::GET
            },
            &entry.upstream_url,
        )
        .header(reqwest::header::USER_AGENT, &entry.user_agent);

    if let Some(range) = range_header {
        req = req.header(reqwest::header::RANGE, range);
    }

    let Ok(mut resp) = req.send().await else {
        socket
            .write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await?;
        return Ok(());
    };

    let status = resp.status();
    let reason = status.canonical_reason().unwrap_or("OK");
    let mut head = format!(
        "HTTP/1.1 {} {reason}\r\nAccept-Ranges: bytes\r\nContent-Type: {}\r\nConnection: close\r\n",
        status.as_u16(),
        resp.headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or(&entry.content_type)
    );
    if let Some(len) = resp
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
    {
        head.push_str(&format!("Content-Length: {len}\r\n"));
    }
    if let Some(cr) = resp
        .headers()
        .get(reqwest::header::CONTENT_RANGE)
        .and_then(|v| v.to_str().ok())
    {
        head.push_str(&format!("Content-Range: {cr}\r\n"));
    }
    head.push_str("\r\n");
    socket.write_all(head.as_bytes()).await?;

    if !method.eq_ignore_ascii_case("HEAD") {
        while let Ok(Some(chunk)) = resp.chunk().await {
            if socket.write_all(&chunk).await.is_err() {
                break;
            }
        }
    }
    let _ = socket.shutdown().await;
    Ok(())
}

pub struct HomeFeed {
    pub trending: Vec<Track>,
    pub top: Vec<Track>,
    pub new_albums: Vec<Album>,
}

#[derive(Clone)]
pub struct Backend {
    rp: RustyPipe,
    http: reqwest::Client,
}

impl Backend {
    pub fn new() -> Self {
        let mut builder = RustyPipe::builder();
        if let Some(dir) = cache_dir()
            && std::fs::create_dir_all(&dir).is_ok()
        {
            builder = builder.storage_dir(dir);
        }
        if let Ok(home) = std::env::var("HOME") {
            let cargo_bg = std::path::PathBuf::from(home).join(".cargo/bin/rustypipe-botguard");
            if cargo_bg.exists() {
                builder = builder.botguard_bin(cargo_bg);
            }
        }

        Self {
            rp: builder.build().unwrap_or_else(|_| RustyPipe::new()),
            http: reqwest::Client::new(),
        }
    }

    pub async fn fetch_home_feed(&self) -> Result<HomeFeed, String> {
        let query = self.rp.query();
        let (charts_res, albums_res) =
            tokio::join!(query.music_charts(None), query.music_new_albums());

        let charts = charts_res.map_err(|e| format!("Error al cargar listas: {e}"))?;
        let new_albums = albums_res
            .unwrap_or_default()
            .iter()
            .map(Album::from)
            .collect();

        Ok(HomeFeed {
            trending: charts.trending_tracks.iter().map(Track::from).collect(),
            top: charts.top_tracks.iter().map(Track::from).collect(),
            new_albums,
        })
    }

    pub async fn fetch_album_tracks(&self, album_id: &str) -> Result<(String, Vec<Track>), String> {
        let album = self
            .rp
            .query()
            .music_album(album_id)
            .await
            .map_err(|e| format!("Error al cargar álbum: {e}"))?;

        let fallback_small = pick_cover(&album.cover, 240);
        let fallback_large = pick_cover(&album.cover, 800);
        let tracks = album
            .tracks
            .iter()
            .map(|item| {
                let mut t = Track::from(item);
                if t.cover_url.is_none() {
                    t.cover_url = fallback_small.clone();
                }
                if t.cover_large_url.is_none() {
                    t.cover_large_url = fallback_large.clone();
                }
                if t.album.is_none() {
                    t.album = Some(album.name.clone());
                }
                t
            })
            .collect();
        Ok((album.name, tracks))
    }

    pub async fn fetch_genres(&self) -> Result<Vec<Genre>, String> {
        let genres = self
            .rp
            .query()
            .music_genres()
            .await
            .map_err(|e| format!("Error al cargar géneros: {e}"))?;

        Ok(genres.iter().map(Genre::from).collect())
    }

    pub async fn fetch_genre_playlists(
        &self,
        genre_id: &str,
    ) -> Result<(String, Vec<Playlist>), String> {
        let genre = self
            .rp
            .query()
            .music_genre(genre_id)
            .await
            .map_err(|e| format!("Error al cargar playlists del género: {e}"))?;

        let playlists = genre
            .sections
            .iter()
            .flat_map(|s| s.playlists.iter())
            .map(Playlist::from)
            .collect();

        Ok((genre.name, playlists))
    }

    pub async fn fetch_playlist_tracks(
        &self,
        playlist_id: &str,
    ) -> Result<(String, Vec<Track>), String> {
        let playlist = self
            .rp
            .query()
            .music_playlist(playlist_id)
            .await
            .map_err(|e| format!("Error al cargar playlist: {e}"))?;

        let tracks = playlist.tracks.items.iter().map(Track::from).collect();
        Ok((playlist.name, tracks))
    }

    pub async fn fetch_radio_tracks(&self, video_id: &str) -> Result<Vec<Track>, String> {
        let radio = self
            .rp
            .query()
            .music_radio_track(video_id)
            .await
            .map_err(|e| format!("Error al iniciar radio: {e}"))?;

        Ok(radio.items.iter().map(Track::from).collect())
    }

    pub async fn search_all(&self, query_str: &str) -> Result<SearchResults, String> {
        let query = self.rp.query();
        let (main_res, tracks_res) = tokio::join!(
            query.music_search_main(query_str),
            query.music_search_tracks(query_str)
        );

        let mut results = SearchResults::default();
        let mut fallback_tracks = Vec::new();

        if let Ok(main) = main_res {
            for item in &main.items.items {
                match item {
                    MusicItem::Artist(a) => results.artists.push(Artist::from(a)),
                    MusicItem::Album(a) => results.albums.push(Album::from(a)),
                    MusicItem::Playlist(p) => results.playlists.push(Playlist::from(p)),
                    MusicItem::Track(t) => fallback_tracks.push(Track::from(t)),
                    MusicItem::User(_) => {}
                }
            }
        }

        match tracks_res {
            Ok(tracks) => {
                results.tracks = tracks.items.items.iter().map(Track::from).collect();
                if results.tracks.is_empty() {
                    results.tracks = fallback_tracks;
                }
            }
            Err(e) if results.artists.is_empty() && results.albums.is_empty() && fallback_tracks.is_empty() => {
                return Err(format!("Error en la búsqueda: {e}"));
            }
            Err(_) => {
                results.tracks = fallback_tracks;
            }
        }

        Ok(results)
    }

    pub async fn fetch_artist(&self, artist_id: &str) -> Result<ArtistDetails, String> {
        let artist = self
            .rp
            .query()
            .music_artist(artist_id, true)
            .await
            .map_err(|e| format!("Error al cargar artista: {e}"))?;

        Ok(ArtistDetails::from(&artist))
    }

    fn streams_dir() -> Result<std::path::PathBuf, String> {
        let dir = cache_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("gyrotube"))
            .join("streams");
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Error al crear caché de audio: {e}"))?;
        Ok(dir)
    }

    fn find_cached_audio(video_id: &str, streams_dir: &std::path::Path) -> Option<StreamInfo> {
        for (suffix, label) in [
            ("m4a", "M4A"),
            ("webm", "WEBM"),
            ("ytdlp.m4a", "M4A"),
            ("ytdlp.webm", "WEBM"),
        ] {
            let path = streams_dir.join(format!("{video_id}.{suffix}"));
            if let Ok(meta) = std::fs::metadata(&path)
                && meta.is_file()
                && meta.len() > 0
            {
                return Some(StreamInfo {
                    url: path.to_string_lossy().into_owned(),
                    quality_label: label.to_string(),
                    is_local_file: true,
                });
            }
        }
        None
    }

    async fn probe_stream(&self, url: &str, user_agent: &str) -> Result<bool, reqwest::Error> {
        let resp = self
            .http
            .get(url)
            .header(reqwest::header::USER_AGENT, user_agent)
            .header(reqwest::header::RANGE, "bytes=0-1")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;
        Ok(resp.status().is_success())
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

        let query = self.rp.query();
        let mut errors = Vec::new();
        for &client in query.player_client_order().iter().take(2) {
            let player = match tokio::time::timeout(
                std::time::Duration::from_secs(8),
                query.player_from_clients(video_id, &[client]),
            )
            .await
            {
                Ok(Ok(player)) => player,
                Ok(Err(e)) => {
                    errors.push(format!("{client:?}: {e}"));
                    continue;
                }
                Err(_) => {
                    errors.push(format!("{client:?}: tiempo de espera agotado"));
                    continue;
                }
            };

            let Some(stream) = player.select_audio_stream(&StreamFilter::default()) else {
                errors.push(format!("{client:?}: no hay un flujo de audio disponible"));
                continue;
            };

            let kbps = stream.bitrate / 1000;
            let codec = format!("{:?}", stream.codec).to_uppercase();
            let quality_label = if kbps > 0 {
                format!("{codec} {kbps}k")
            } else {
                codec
            };

            let ua = query.user_agent(player.client_type);
            match self.probe_stream(&stream.url, ua.as_ref()).await {
                Ok(true) => {
                    let proxy = StreamProxy::get_or_init(self.http.clone()).await?;
                    let content_type = if stream.mime.contains("mp4") || stream.mime.contains("m4a") {
                        "audio/mp4"
                    } else {
                        "audio/webm"
                    };
                    let proxy_url = proxy
                        .register(
                            stream.url.clone(),
                            ua.to_string(),
                            content_type.to_string(),
                        )
                        .await;
                    return Ok(StreamInfo {
                        url: proxy_url,
                        quality_label,
                        is_local_file: false,
                    });
                }
                Ok(false) => {
                    if let Some(visitor_data) = &player.visitor_data {
                        query.remove_visitor_data(visitor_data);
                    }
                    errors.push(format!("{client:?}: HTTP 403 en stream"));
                    continue;
                }
                Err(e) => {
                    errors.push(format!("{client:?}: {}", e.without_url()));
                    continue;
                }
            }
        }

        if let Ok(stream) = self.resolve_with_ytdlp_stream(video_id).await {
            return Ok(stream);
        }

        Self::download_with_ytdlp(video_id, &streams_dir)
            .await
            .map_err(|error| {
                eprintln!("RustyPipe ({video_id}): {}", errors.join(" | "));
                error
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
        let Ok(streams_dir) = Self::streams_dir() else {
            return;
        };
        if Self::find_cached_audio(video_id, &streams_dir).is_some() {
            return;
        }

        let query = self.rp.query();
        for &client in query.player_client_order().iter().take(2) {
            let Ok(Ok(player)) = tokio::time::timeout(
                std::time::Duration::from_secs(8),
                query.player_from_clients(video_id, &[client]),
            )
            .await
            else {
                continue;
            };
            let Some(stream) = player.select_audio_stream(&StreamFilter::default()) else {
                continue;
            };
            let ext = if stream.mime.contains("mp4") || stream.mime.contains("m4a") {
                "m4a"
            } else {
                "webm"
            };
            let file_path = streams_dir.join(format!("{video_id}.{ext}"));
            let ua = query.user_agent(player.client_type);
            if let Ok(bytes) = self.download_audio(&stream.url, ua.as_ref()).await {
                let _ = std::fs::write(&file_path, bytes);
                return;
            }
        }

        let _ = Self::download_with_ytdlp(video_id, &streams_dir).await;
    }

    async fn resolve_with_ytdlp_stream(&self, video_id: &str) -> Result<StreamInfo, String> {
        let mut command = tokio::process::Command::new("yt-dlp");
        command
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--js-runtimes",
                "node",
                "-f",
                "bestaudio[ext=m4a]/bestaudio[ext=webm]",
                "--print",
                "urls",
                "--print",
                "ext",
                "--",
            ])
            .arg(format!("https://www.youtube.com/watch?v={video_id}"));

        let output = tokio::time::timeout(std::time::Duration::from_secs(20), command.output())
            .await
            .map_err(|_| "Tiempo de espera agotado en yt-dlp".to_string())?
            .map_err(|e| format!("Error al ejecutar yt-dlp: {e}"))?;

        if !output.status.success() {
            return Err("yt-dlp no devolvió URL".into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut lines = stdout.lines().map(str::trim).filter(|l| !l.is_empty());
        let url = lines.next().ok_or("Sin URL de yt-dlp")?.to_string();
        let ext = lines.next().unwrap_or("m4a").to_ascii_lowercase();

        let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
        if !self.probe_stream(&url, ua).await.unwrap_or(false) {
            return Err("Stream de yt-dlp rechazado".into());
        }

        let proxy = StreamProxy::get_or_init(self.http.clone()).await?;
        let content_type = if ext == "webm" {
            "audio/webm"
        } else {
            "audio/mp4"
        };
        let proxy_url = proxy
            .register(url, ua.to_string(), content_type.to_string())
            .await;

        Ok(StreamInfo {
            url: proxy_url,
            quality_label: ext.to_uppercase(),
            is_local_file: false,
        })
    }

    async fn download_with_ytdlp(
        video_id: &str,
        streams_dir: &std::path::Path,
    ) -> Result<StreamInfo, String> {
        let mut path = streams_dir.join(format!("{video_id}.ytdlp.m4a"));
        let webm = path.with_extension("webm");
        if !path.is_file() && webm.is_file() {
            path = webm;
        }
        if !path.is_file() {
            let staging = tempfile::tempdir_in(streams_dir)
                .map_err(|e| format!("Error al preparar descarga de audio: {e}"))?;
            let mut command = tokio::process::Command::new("yt-dlp");
            command
                .kill_on_drop(true)
                .args([
                    "--ignore-config",
                    "--no-playlist",
                    "--no-progress",
                    "--js-runtimes",
                    "node",
                ])
                .args(["-f", "bestaudio[ext=m4a]/bestaudio[ext=webm]", "-o"])
                .arg(staging.path().join("audio.%(ext)s"))
                .arg("--")
                .arg(format!("https://www.youtube.com/watch?v={video_id}"));
            let output = tokio::time::timeout(std::time::Duration::from_secs(90), command.output())
                .await
                .map_err(|_| "Tiempo de espera agotado en yt-dlp".to_string())?
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        "RustyPipe no pudo obtener el audio. Instala yt-dlp en PATH para activar el respaldo.".into()
                    } else {
                        format!("Error al ejecutar yt-dlp: {e}")
                    }
                })?;
            if !output.status.success() {
                eprintln!(
                    "yt-dlp ({video_id}): {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return Err("yt-dlp no pudo descargar el audio. Consulta la terminal.".into());
            }
            let downloaded = staging.path().join("audio.m4a");
            let downloaded = if downloaded.is_file() {
                downloaded
            } else {
                path.set_extension("webm");
                downloaded.with_extension("webm")
            };
            std::fs::rename(downloaded, &path)
                .map_err(|e| format!("Error al guardar audio de yt-dlp: {e}"))?;
        }

        Ok(StreamInfo {
            url: path.to_string_lossy().into_owned(),
            quality_label: path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_uppercase(),
            is_local_file: true,
        })
    }

    async fn download_audio(&self, url: &str, user_agent: &str) -> Result<Vec<u8>, reqwest::Error> {
        let bytes = self
            .http
            .get(url)
            .header(reqwest::header::USER_AGENT, user_agent)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_lyrics(&self, video_id: &str) -> Result<String, String> {
        let details = self
            .rp
            .query()
            .music_details(video_id)
            .await
            .map_err(|e| format!("Error al cargar detalles: {e}"))?;

        let Some(lyrics_id) = details.lyrics_id else {
            return Ok(String::from("No hay letra disponible para esta canción."));
        };

        let lyrics = match self.rp.query().music_lyrics(lyrics_id).await {
            Ok(lyrics) => lyrics,
            Err(rustypipe::error::Error::Extraction(
                rustypipe::error::ExtractionError::NotFound { .. },
            )) => return Ok(String::from("No hay letra disponible para esta canción.")),
            Err(_) => {
                return Err(String::from(
                    "No se pudo cargar la letra en este momento. Intenta de nuevo más tarde.",
                ));
            }
        };

        if lyrics.footer.is_empty() {
            Ok(lyrics.body)
        } else {
            Ok(format!("{}\n\n— {}", lyrics.body, lyrics.footer))
        }
    }

    pub async fn fetch_image_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Error al descargar portada: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("Error HTTP {} al descargar portada", resp.status()));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| format!("Error al leer portada: {e}"))?;

        Ok(bytes.to_vec())
    }

    pub async fn resolve_ytm_cover(&self, track: &Track) -> Option<String> {
        let clean_title = clean_track_title(&track.title);
        let query = format!("{clean_title} {}", track.artist);
        let res = self.rp.query().music_search_tracks(&query).await.ok()?;
        res.items
            .items
            .iter()
            .map(Track::from)
            .find_map(|t| t.cover_large_url.filter(|u| is_ytm_square_cover(u)))
    }
}

#[cfg(test)]
mod tests {
    use super::Backend;

    #[tokio::test]
    async fn ytdlp_reuses_cached_webm() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("abcdefghijk.ytdlp.webm");
        std::fs::write(&path, b"cached audio").unwrap();

        let stream = Backend::download_with_ytdlp("abcdefghijk", dir.path())
            .await
            .unwrap();
        assert_eq!(std::path::Path::new(&stream.url), path);
        assert_eq!(stream.quality_label, "WEBM");
        assert!(stream.is_local_file);
    }

    #[test]
    fn find_cached_audio_reuses_local_files() {
        let dir = tempfile::tempdir().unwrap();
        let empty = dir.path().join("12345678901.m4a");
        std::fs::write(&empty, b"").unwrap();
        assert!(Backend::find_cached_audio("12345678901", dir.path()).is_none());

        let valid = dir.path().join("12345678901.ytdlp.m4a");
        std::fs::write(&valid, b"audio bytes").unwrap();
        let found = Backend::find_cached_audio("12345678901", dir.path()).unwrap();
        assert_eq!(found.quality_label, "M4A");
        assert!(found.is_local_file);
    }
}
