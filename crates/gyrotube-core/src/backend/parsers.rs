use super::*;

pub(super) fn load_home_cache(path: &std::path::Path) -> Option<(Instant, HomeFeed)> {
    let age = path.metadata().ok()?.modified().ok()?.elapsed().ok()?;
    if age >= HOME_CACHE_TTL {
        return None;
    }
    let feed = serde_json::from_slice(&std::fs::read(path).ok()?).ok()?;
    Some((Instant::now().checked_sub(age)?, feed))
}

pub(super) fn stream_url_is_fresh(created: Instant, stream: &StreamInfo) -> bool {
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

pub(super) fn verified_music_track(
    video_id: &str,
    item: &CatalogItem,
) -> Result<Track, String> {
    if item.track.id != video_id || item.kind != TrackKind::Track || item.unavailable {
        return Err(String::from(
            "Solo se reproducen canciones disponibles del catálogo de YouTube Music.",
        ));
    }
    Ok(item.track.clone())
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
pub(super) fn filesize_from_ytdlp_json(raw: &[u8]) -> Option<u64> {
    let val: serde_json::Value = serde_json::from_slice(raw).ok()?;
    val.get("filesize")
        .and_then(|v| v.as_u64())
        .or_else(|| val.get("filesize_approx").and_then(|v| v.as_u64()))
        .filter(|&sz| sz > 0)
}

#[cfg(not(target_os = "android"))]
pub(super) fn stream_from_ytdlp_json(raw: &[u8]) -> Result<StreamInfo, String> {
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

pub(super) fn best_ytm_thumb(thumbs: &[ytmapi_rs::common::Thumbnail]) -> Option<String> {
    thumbs
        .iter()
        .max_by_key(|t| t.width.saturating_mul(t.height))
        .map(|t| crate::models::upgrade_cover_url(&t.url, 544))
}

pub(super) fn best_ytm_banner(thumbs: &[ytmapi_rs::common::Thumbnail]) -> Option<String> {
    thumbs
        .iter()
        .max_by_key(|t| t.width.saturating_mul(t.height))
        .map(|t| crate::models::upgrade_cover_url(&t.url, 1280))
}

pub(super) fn runs_text(val: &serde_json::Value) -> String {
    val.get("runs")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|run| run.get("text").and_then(|t| t.as_str()))
                .collect::<String>()
        })
        .unwrap_or_default()
}

pub(super) fn best_json_thumb(thumbs: &serde_json::Value) -> Option<String> {
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

pub(super) fn collect_mood_playlists(node: &serde_json::Value, out: &mut Vec<Playlist>) {
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

pub(super) fn parse_mood_playlists_json(json: &serde_json::Value) -> (String, Vec<Playlist>) {
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

pub(super) fn panel_video_id(renderer: &serde_json::Value) -> Option<&str> {
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

pub(super) fn panel_video_music_type(renderer: &serde_json::Value) -> Option<&str> {
    renderer
        .pointer("/navigationEndpoint/watchEndpoint/watchEndpointMusicSupportedConfigs/watchEndpointMusicConfig/musicVideoType")
        .and_then(|v| v.as_str())
}

pub(super) fn track_from_panel_video_renderer(
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

pub(super) fn select_official_counterpart(
    details: &crate::innertube::MusicTrackDetails,
    source: &Track,
    want_video: bool,
) -> Option<Track> {
    // Respect the real catalog musicVideoType of the playing ID:
    // - If we want a video, only pick an official Video counterpart (OMV/UGC), never an ATV track.
    // - If we want studio audio, only pick an official ATV Track counterpart.
    if want_video && details.item.kind == TrackKind::Video {
        return None;
    }
    if !want_video && details.item.kind == TrackKind::Track {
        return None;
    }

    let candidate = details.counterparts.iter().find(|cp| {
        if cp.track.id == source.id || cp.unavailable {
            return false;
        }
        if want_video {
            cp.kind == TrackKind::Video || (cp.kind == TrackKind::Unknown && details.item.kind == TrackKind::Track)
        } else {
            cp.kind == TrackKind::Track
        }
    })?;

    let mut out = candidate.track.clone();
    out.is_music = true;
    out.is_video = want_video;
    if out.artist.is_empty() || out.artist == "Artista desconocido" {
        out.artist = source.artist.clone();
    }
    if out.artist_id.is_none() {
        out.artist_id = source.artist_id.clone();
    }
    if out.album.is_none() {
        out.album = source.album.clone();
    }
    if out.album_id.is_none() {
        out.album_id = source.album_id.clone();
    }
    if out.cover_url.is_none() {
        out.cover_url = source.cover_url.clone();
    }
    if out.cover_large_url.is_none() {
        out.cover_large_url = source.cover_large_url.clone().or_else(|| out.cover_url.clone());
    }
    Some(out)
}

#[allow(dead_code)]
pub(super) fn parse_watch_counterpart_json(
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
pub(super) fn is_playlist_audio(item: &CatalogItem) -> bool {
    item.kind == TrackKind::Track && !item.unavailable
}

pub(super) fn is_playlist_playable(item: &CatalogItem) -> bool {
    matches!(item.kind, TrackKind::Track | TrackKind::Video) && !item.unavailable
}

pub(super) fn playlist_audio_result(
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

pub(super) fn playlist_browse_id(id: &str) -> PlaylistID<'static> {
    PlaylistID::from_raw(format!("VL{}", id.strip_prefix("VL").unwrap_or(id)))
}

pub(super) fn tracks_from_ytm_playlist(items: Vec<PlaylistItem>) -> Vec<Track> {
    tracks_from_ytm_playlist_inner(items, false)
}

pub(super) fn tracks_from_ytm_playlist_with_videos(items: Vec<PlaylistItem>) -> Vec<Track> {
    tracks_from_ytm_playlist_inner(items, true)
}

pub(super) fn tracks_from_ytm_playlist_inner(items: Vec<PlaylistItem>, include_videos: bool) -> Vec<Track> {
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

pub(super) fn track_from_ytm_song(song: SearchResultSong) -> Track {
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

pub(super) fn album_from_ytm_album(alb: SearchResultAlbum) -> Album {
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

pub(super) fn artist_from_ytm_artist(art: SearchResultArtist) -> Artist {
    Artist {
        id: art.browse_id.get_raw().to_string(),
        name: art.artist,
        avatar_url: best_ytm_thumb(&art.thumbnails),
        subscriber_count: art.subscribers.as_deref().and_then(parse_sub_count),
    }
}

pub(super) fn playlist_from_ytm_playlist(pl: SearchResultPlaylist) -> Option<Playlist> {
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

pub(super) fn parse_mm_ss(raw: &str) -> Option<u32> {
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

pub(super) fn parse_sub_count(raw: &str) -> Option<u64> {
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

