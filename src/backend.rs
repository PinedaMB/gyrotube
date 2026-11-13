use std::sync::OnceLock;

use rustypipe::client::RustyPipe;
use rustypipe::model::MusicItem;
use rustypipe::param::StreamFilter;

use crate::models::{
    Album, Artist, ArtistDetails, Genre, Playlist, SearchResults, StreamInfo, Track, cache_dir,
    pick_cover,
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

    pub async fn resolve_audio_stream(&self, video_id: &str) -> Result<StreamInfo, String> {
        if video_id.len() != 11
            || !video_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err("ID de video inválido".into());
        }

        let query = self.rp.query();
        let streams_dir = cache_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("gyrotube"))
            .join("streams");
        std::fs::create_dir_all(&streams_dir)
            .map_err(|e| format!("Error al crear caché de audio: {e}"))?;

        let mut errors = Vec::new();
        for &client in query.player_client_order() {
            let player = match tokio::time::timeout(
                std::time::Duration::from_secs(15),
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

            let ext = if stream.mime.contains("mp4") || stream.mime.contains("m4a") {
                "m4a"
            } else {
                "webm"
            };
            let file_path = streams_dir.join(format!("{video_id}.{ext}"));
            if !file_path.exists() {
                let ua = query.user_agent(player.client_type);
                match self.download_audio(&stream.url, ua.as_ref()).await {
                    Ok(bytes) => std::fs::write(&file_path, bytes)
                        .map_err(|e| format!("Error al guardar audio: {e}"))?,
                    Err(e) if e.status() == Some(reqwest::StatusCode::FORBIDDEN) => {
                        if let Some(visitor_data) = &player.visitor_data {
                            query.remove_visitor_data(visitor_data);
                        }
                        errors.push(format!("{client:?}: HTTP 403 al descargar audio"));
                        continue;
                    }
                    Err(e) => return Err(format!("Error al descargar audio: {}", e.without_url())),
                }
            }

            return Ok(StreamInfo {
                url: file_path.to_string_lossy().into_owned(),
                quality_label,
            });
        }

        Self::download_with_ytdlp(video_id, &streams_dir)
            .await
            .map_err(|error| {
                eprintln!("RustyPipe ({video_id}): {}", errors.join(" | "));
                error
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
    }
}
