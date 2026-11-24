use super::*;
use super::parsers::*;

impl Backend {
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
            && let Some((ts, feed)) = guard.get(&normalized_country)
            && ts.elapsed() < HOME_CACHE_TTL
        {
            return Ok(feed.clone());
        }

        let mut trending = Vec::new();
        let mut top = Vec::new();
        let mut new_albums = Vec::new();
        let mut top_artists = Vec::new();
        let mut available_countries = Vec::new();

        let (charts_res, releases_res) = tokio::join!(
            self.innertube.fetch_charts(normalized_country.as_deref()),
            self.innertube.fetch_new_albums()
        );
        if let Ok(charts) = charts_res {
            trending = charts.trending;
            top = charts.top;
            top_artists = charts.artists;
            available_countries = charts.available_countries;
        }
        if let Ok(releases) = releases_res {
            new_albums = releases.into_iter().take(12).collect();
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
                guard.retain(|_, (ts, _)| ts.elapsed() < HOME_CACHE_TTL);
                if guard.len() >= HOME_CACHE_MAX_COUNTRIES
                    && !guard.contains_key(&feed.country)
                    && let Some(oldest) = guard
                        .iter()
                        .min_by_key(|(_, (ts, _))| *ts)
                        .map(|(k, _)| k.clone())
                {
                    guard.remove(&oldest);
                }
                guard.insert(feed.country.clone(), (Instant::now(), feed.clone()));
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

        self.innertube
            .fetch_album_tracks(album_id)
            .await
            .map_err(|e| format!("Error cargando álbum: {e}"))
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

        let page = self
            .innertube
            .fetch_playlist_tracks(playlist_id)
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

        had_entries |= page.had_entries;
        let complete = page.exhausted;
        let tracks = page
            .items
            .into_iter()
            .filter(is_playlist_playable)
            .map(|item| {
                let mut tr = item.track;
                if item.kind == TrackKind::Video {
                    tr.is_music = true;
                    tr.is_video = true;
                }
                tr
            })
            .collect();
        playlist_audio_result(page.title, tracks, had_entries, complete)
    }

    pub async fn fetch_radio_tracks(&self, video_id: &str) -> Result<Vec<Track>, String> {
        if let Ok(items) = self.innertube.fetch_radio_track(video_id).await {
            let mut tracks: Vec<Track> = items
                .iter()
                .filter(|t| t.kind == TrackKind::Track && !t.unavailable)
                .map(|t| t.track.clone())
                .collect();
            if tracks.is_empty() {
                tracks = items
                    .into_iter()
                    .filter(|t| !t.unavailable)
                    .map(|t| t.track)
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
        Ok(self.search_suggestions_rich(query_str).await?.terms)
    }

    pub async fn search_suggestions_rich(
        &self,
        query_str: &str,
    ) -> Result<crate::models::SearchSuggestions, String> {
        if let Ok(mut rich) = self.innertube.search_suggestions_rich(query_str).await {
            if !rich.terms.is_empty()
                || !rich.tracks.is_empty()
                || !rich.artists.is_empty()
                || !rich.albums.is_empty()
            {
                return Ok(rich);
            }
            if let Ok(ytm) = self.primary().await
                && let Ok(suggestions) = ytm.get_search_suggestions(query_str).await
            {
                rich.terms = suggestions
                    .into_iter()
                    .map(|s| {
                        s.runs
                            .into_iter()
                            .map(|r| r.take_text())
                            .collect::<String>()
                    })
                    .filter(|s| !s.is_empty())
                    .collect();
                return Ok(rich);
            }
        }
        if let Ok(ytm) = self.primary().await
            && let Ok(suggestions) = ytm.get_search_suggestions(query_str).await
        {
            let terms = suggestions
                .into_iter()
                .map(|s| {
                    s.runs
                        .into_iter()
                        .map(|r| r.take_text())
                        .collect::<String>()
                })
                .filter(|s| !s.is_empty())
                .collect();
            return Ok(crate::models::SearchSuggestions {
                terms,
                ..Default::default()
            });
        }
        Err(String::from("Error obteniendo sugerencias"))
    }

    pub async fn search_all(&self, query_str: &str) -> Result<SearchResults, String> {
        let ytm = self.primary().await?;
        let (tracks, albums, artists, playlists) = tokio::join!(
            async {
                ytm.search_songs(query_str)
                    .await
                    .map(|songs| songs.into_iter().map(track_from_ytm_song).collect::<Vec<_>>())
            },
            async {
                ytm.search_albums(query_str)
                    .await
                    .map(|albums| albums.into_iter().map(album_from_ytm_album).collect::<Vec<_>>())
            },
            async {
                ytm.search_artists(query_str)
                    .await
                    .map(|artists| artists.into_iter().map(artist_from_ytm_artist).collect::<Vec<_>>())
            },
            async {
                ytm.search_playlists(query_str).await.map(|playlists| {
                    playlists
                        .into_iter()
                        .filter_map(playlist_from_ytm_playlist)
                        .collect::<Vec<_>>()
                })
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
        let it_fut = async { self.innertube.fetch_artist_enrichment(artist_id).await.ok() };
        let (ytm_artist, it_artist) = tokio::join!(ytm_fut, it_fut);

        let rp_view_counts: HashMap<String, u64> = it_artist
            .as_ref()
            .map(|it| it.view_counts.clone())
            .unwrap_or_default();

        let rp_videos: Vec<Track> = it_artist
            .as_ref()
            .map(|it| it.videos.clone())
            .unwrap_or_default();

        let rp_playlists: Vec<Playlist> = it_artist
            .as_ref()
            .map(|it| it.playlists.clone())
            .unwrap_or_default();

        let rp_similar: Vec<Artist> = it_artist
            .as_ref()
            .map(|it| it.similar_artists.clone())
            .unwrap_or_default();

        let (rp_studio_albums, rp_single_releases): (Vec<Album>, Vec<Album>) = it_artist
            .as_ref()
            .map(|it| (it.albums.clone(), it.singles.clone()))
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
                    .or_else(|| it_artist.as_ref().and_then(|it| it.header_url.clone())),
                subscriber_count: artist
                    .subscribers
                    .as_deref()
                    .and_then(parse_sub_count)
                    .or_else(|| it_artist.as_ref().and_then(|it| it.subscriber_count)),
                description: artist
                    .description
                    .or_else(|| it_artist.as_ref().and_then(|it| it.description.clone())),
                tracks,
                albums: studio_albums,
                singles: single_releases,
                videos: rp_videos,
                playlists: rp_playlists,
                similar_artists: rp_similar,
            });
        }

        if let Some(it_artist) = it_artist
            && (!it_artist.tracks.is_empty()
                || !rp_studio_albums.is_empty()
                || !rp_single_releases.is_empty())
        {
            return Ok(ArtistDetails {
                name: it_artist.name.unwrap_or_else(|| String::from("Artista")),
                header_url: it_artist.header_url,
                description: it_artist.description,
                subscriber_count: it_artist.subscriber_count,
                tracks: it_artist.tracks,
                albums: rp_studio_albums,
                singles: rp_single_releases,
                videos: rp_videos,
                playlists: rp_playlists,
                similar_artists: rp_similar,
            });
        }

        Err(String::from("Error cargando artista"))
    }

    pub async fn fetch_related_for_track(
        &self,
        track_id: &str,
    ) -> Result<crate::models::RelatedFeed, String> {
        validate_video_id(track_id)?;
        let details = self
            .innertube
            .fetch_music_details(track_id)
            .await
            .map_err(|e| format!("Error obteniendo detalles: {e}"))?;
        let Some(related_id) = details.related_browse_id else {
            return Ok(crate::models::RelatedFeed::default());
        };
        self.innertube
            .fetch_related(&related_id)
            .await
            .map_err(|e| format!("Error obteniendo relacionados: {e}"))
    }

    pub async fn fetch_playlist_radio(&self, playlist_id: &str) -> Result<Vec<Track>, String> {
        let items = self
            .innertube
            .fetch_radio_playlist(playlist_id)
            .await
            .map_err(|e| format!("Error iniciando radio de playlist: {e}"))?;
        Ok(items
            .into_iter()
            .filter(|t| t.kind == TrackKind::Track && !t.unavailable)
            .map(|t| t.track)
            .collect())
    }

    pub async fn fetch_image_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        let resp = self
            .http
            .get(url)
            .header(reqwest::header::USER_AGENT, "GyroTube/1.0")
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
        let details = self.innertube.fetch_music_details(&track.id).await.ok()?;
        if details.item.track.id != track.id {
            return None;
        }
        if let Some(cover) = pick_cover_thumbs(&details.covers, 800) {
            return Some(cover);
        }
        details.item.track.cover_large_url
    }
}
