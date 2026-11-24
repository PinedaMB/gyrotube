use super::*;

pub(super) fn runs_text(val: &Value) -> String {
    val["runs"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|r| r["text"].as_str())
                .collect::<String>()
        })
        .or_else(|| val["simpleText"].as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

pub(super) fn extract_thumbs(val: &Value) -> Vec<(String, u32)> {
    let arr = val["thumbnails"]
        .as_array()
        .or_else(|| val["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array())
        .or_else(|| val["thumbnail"]["thumbnails"].as_array())
        .or_else(|| val["thumbnail"]["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array());

    arr.map(|list| {
        list.iter()
            .filter_map(|t| {
                let url = t["url"].as_str()?.to_string();
                let width = t["width"].as_u64().unwrap_or(0) as u32;
                Some((url, width))
            })
            .collect()
    })
    .unwrap_or_default()
}

pub(super) fn parse_duration_str(s: &str) -> Option<u32> {
    let mut parts = s.trim().split(':');
    let a = parts.next()?.parse::<u32>().ok()?;
    let b = parts.next()?.parse::<u32>().ok()?;
    if let Some(c_str) = parts.next() {
        let c = c_str.parse::<u32>().ok()?;
        Some(a * 3600 + b * 60 + c)
    } else {
        Some(a * 60 + b)
    }
}

pub fn parse_compact_count(s: &str) -> Option<u64> {
    let cleaned = s
        .to_lowercase()
        .replace("subscribers", "")
        .replace("suscriptores", "")
        .replace("plays", "")
        .replace("reproducciones", "")
        .replace("views", "")
        .replace("vistas", "")
        .replace("monthly audience", "")
        .replace(',', "")
        .trim()
        .to_string();
    let token = cleaned.split_whitespace().next()?;
    let (num_str, mult) = if let Some(n) = token.strip_suffix('b') {
        (n, 1_000_000_000.0)
    } else if let Some(n) = token.strip_suffix('m') {
        (n, 1_000_000.0)
    } else if let Some(n) = token.strip_suffix('k') {
        (n, 1_000.0)
    } else {
        (token, 1.0)
    };
    let val: f64 = num_str.parse().ok()?;
    if val >= 0.0 {
        Some((val * mult).round() as u64)
    } else {
        None
    }
}

pub(super) fn classify_video_type(mvt: Option<&str>) -> TrackKind {
    match mvt {
        Some("MUSIC_VIDEO_TYPE_ATV") => TrackKind::Track,
        Some("MUSIC_VIDEO_TYPE_OMV" | "MUSIC_VIDEO_TYPE_UGC" | "MUSIC_VIDEO_TYPE_OFFICIAL_SOURCE_MUSIC") => {
            TrackKind::Video
        }
        Some("MUSIC_VIDEO_TYPE_PODCAST_EPISODE") => TrackKind::Episode,
        _ => TrackKind::Unknown,
    }
}

pub(super) fn video_type_from_watch_endpoint(ep: &Value) -> Option<&str> {
    ep["watchEndpointMusicSupportedConfigs"]["watchEndpointMusicConfig"]["musicVideoType"].as_str()
}

pub(super) fn parse_panel_video_renderer(r: &Value) -> Option<(CatalogItem, Vec<(String, u32)>)> {
    let id = r["videoId"].as_str()?.to_string();
    let title = clean_track_title(&runs_text(&r["title"]));
    if title.is_empty() {
        return None;
    }

    let mut artists = Vec::new();
    let mut artist_id = None;
    let mut album = None;
    let mut album_id = None;

    if let Some(runs) = r["longBylineText"]["runs"].as_array() {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "&" || text == "," {
                continue;
            }
            if let Some(bid) = run["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
                if bid.starts_with("UC") {
                    if artist_id.is_none() {
                        artist_id = Some(bid.to_string());
                    }
                    artists.push(text.to_string());
                } else if bid.starts_with("MPREb_") || bid.starts_with("OLAK") {
                    album = Some(text.to_string());
                    album_id = Some(bid.to_string());
                }
            }
        }
    }
    if artists.is_empty() {
        let short = runs_text(&r["shortBylineText"]);
        if !short.is_empty() {
            artists.push(short);
        }
    }
    let artist = if artists.is_empty() {
        String::from("Artista desconocido")
    } else {
        artists.join(", ")
    };

    let duration_secs = parse_duration_str(&runs_text(&r["lengthText"]));
    let covers = extract_thumbs(&r["thumbnail"]);
    let mvt = video_type_from_watch_endpoint(&r["navigationEndpoint"]["watchEndpoint"]);
    let kind = match classify_video_type(mvt) {
        TrackKind::Unknown => TrackKind::Track,
        other => other,
    };
    let unavailable = r["unplayableText"].is_object();

    let track = Track {
        is_music: kind == TrackKind::Track,
        is_video: kind == TrackKind::Video,
        id,
        title,
        artist,
        artist_id,
        album,
        album_id,
        duration_secs,
        view_count: None,
        cover_url: pick_cover_thumbs(&covers, 240),
        cover_large_url: pick_cover_thumbs(&covers, 544),
    };

    Some((
        CatalogItem {
            track,
            kind,
            unavailable,
        },
        covers,
    ))
}

pub(super) fn parse_responsive_list_item(item: &Value) -> Option<CatalogItem> {
    let video_id = item["playlistItemData"]["videoId"]
        .as_str()
        .or_else(|| {
            item["overlay"]["musicItemThumbnailOverlayRenderer"]["content"]
                ["musicPlayButtonRenderer"]["playNavigationEndpoint"]["watchEndpoint"]["videoId"]
                .as_str()
        })
        .or_else(|| {
            item["flexColumns"][0]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"][0]
                ["navigationEndpoint"]["watchEndpoint"]["videoId"]
                .as_str()
        })?;

    let mvt = item["overlay"]["musicItemThumbnailOverlayRenderer"]["content"]
        ["musicPlayButtonRenderer"]["playNavigationEndpoint"]["watchEndpoint"]
        .as_object()
        .and_then(|_| {
            video_type_from_watch_endpoint(
                &item["overlay"]["musicItemThumbnailOverlayRenderer"]["content"]
                    ["musicPlayButtonRenderer"]["playNavigationEndpoint"]["watchEndpoint"],
            )
        })
        .or_else(|| {
            video_type_from_watch_endpoint(
                &item["flexColumns"][0]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"]
                    [0]["navigationEndpoint"]["watchEndpoint"],
            )
        });

    let kind = match classify_video_type(mvt) {
        TrackKind::Unknown => TrackKind::Track,
        other => other,
    };

    let unavailable = item["musicItemRendererDisplayPolicy"].as_str()
        == Some("MUSIC_ITEM_RENDERER_DISPLAY_POLICY_GREY_OUT");

    let flex = item["flexColumns"].as_array()?;
    let title = flex
        .first()
        .map(|c| runs_text(&c["musicResponsiveListItemFlexColumnRenderer"]["text"]))
        .map(|t| clean_track_title(&t))
        .unwrap_or_default();
    if title.is_empty() {
        return None;
    }

    let mut artists = Vec::new();
    let mut artist_id = None;
    let mut album = None;
    let mut album_id = None;
    let mut duration_secs = None;
    let mut view_count = None;

    for col in flex.iter().skip(1) {
        let Some(runs) = col["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"].as_array()
        else {
            continue;
        };
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "&" || text == "," {
                continue;
            }
            if let Some(bid) = run["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
                if bid.starts_with("UC") {
                    if artist_id.is_none() {
                        artist_id = Some(bid.to_string());
                    }
                    artists.push(text.to_string());
                    continue;
                } else if bid.starts_with("MPREb_") || bid.starts_with("OLAK") {
                    album = Some(text.to_string());
                    album_id = Some(bid.to_string());
                    continue;
                }
            }
            let lower = text.to_lowercase();
            let is_media_badge = matches!(
                lower.as_str(),
                "song"
                    | "canción"
                    | "cancion"
                    | "video"
                    | "vídeo"
                    | "album"
                    | "álbum"
                    | "single"
                    | "sencillo"
                    | "ep"
                    | "episode"
                    | "episodio"
                    | "podcast"
                    | "playlist"
                    | "lista"
            );
            if lower.contains("play") || lower.contains("view") || lower.contains("reproduccion") {
                if view_count.is_none() {
                    view_count = parse_compact_count(text);
                }
            } else if duration_secs.is_none()
                && text.contains(':')
                && let Some(secs) = parse_duration_str(text)
            {
                duration_secs = Some(secs);
            } else if artists.is_empty()
                && !is_media_badge
                && !text.chars().all(|c| c.is_ascii_digit())
            {
                artists.push(text.to_string());
            }
        }
    }

    if duration_secs.is_none()
        && let Some(fixed) = item["fixedColumns"].as_array()
    {
        for col in fixed {
            let text = runs_text(&col["musicResponsiveListItemFixedColumnRenderer"]["text"]);
            if let Some(secs) = parse_duration_str(&text) {
                duration_secs = Some(secs);
                break;
            }
        }
    }

    let covers = extract_thumbs(&item["thumbnail"]);
    let artist = if artists.is_empty() {
        String::from("Artista desconocido")
    } else {
        artists.join(", ")
    };

    Some(CatalogItem {
        track: Track {
            is_music: kind == TrackKind::Track,
            is_video: kind == TrackKind::Video,
            id: video_id.to_string(),
            title,
            artist,
            artist_id,
            album,
            album_id,
            duration_secs,
            view_count,
            cover_url: pick_cover_thumbs(&covers, 240),
            cover_large_url: pick_cover_thumbs(&covers, 544),
        },
        kind,
        unavailable,
    })
}

pub(super) fn parse_responsive_artist_item(item: &Value) -> Option<Artist> {
    let bid = item["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("UC") {
        return None;
    }
    let flex = item["flexColumns"].as_array()?;
    let name = runs_text(&flex.first()?["musicResponsiveListItemFlexColumnRenderer"]["text"]);
    if name.is_empty() {
        return None;
    }
    let subscriber_count = flex
        .get(1)
        .map(|c| runs_text(&c["musicResponsiveListItemFlexColumnRenderer"]["text"]))
        .and_then(|s| parse_compact_count(&s));
    let covers = extract_thumbs(&item["thumbnail"]);
    Some(Artist {
        id: bid.to_string(),
        name,
        avatar_url: pick_cover_thumbs(&covers, 360),
        subscriber_count,
    })
}

pub(super) fn parse_two_row_album(r: &Value) -> Option<(Album, bool)> {
    let bid = r["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("MPREb_") && !bid.starts_with("OLAK") {
        return None;
    }
    let title = runs_text(&r["title"]);
    if title.is_empty() {
        return None;
    }
    let sub_runs = r["subtitle"]["runs"].as_array();
    let mut is_single = false;
    let mut year = None;
    let mut artists = Vec::new();
    if let Some(runs) = sub_runs {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "," || text == "&" {
                continue;
            }
            let lower = text.to_lowercase();
            if lower == "single" || lower == "ep" || lower == "sencillo" {
                is_single = true;
                continue;
            }
            if lower == "album" || lower == "álbum" {
                continue;
            }
            if text.len() == 4
                && let Ok(y) = text.parse::<u16>()
                && (1900..=2100).contains(&y)
            {
                year = Some(y);
                continue;
            }
            artists.push(text.to_string());
        }
    }
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    let artist = if artists.is_empty() {
        String::new()
    } else {
        artists.join(", ")
    };
    Some((
        Album {
            id: bid.to_string(),
            title,
            artist,
            year,
            cover_url: pick_cover_thumbs(&covers, 544),
        },
        is_single,
    ))
}

pub(super) fn parse_two_row_playlist(r: &Value) -> Option<Playlist> {
    let bid = r["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("VL") && !bid.starts_with("PL") && !bid.starts_with("RDCLAK") {
        return None;
    }
    let name = runs_text(&r["title"]);
    if name.is_empty() {
        return None;
    }
    let subtitle = runs_text(&r["subtitle"]);
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    Some(Playlist {
        id: bid.strip_prefix("VL").unwrap_or(bid).to_string(),
        name,
        subtitle: if subtitle.is_empty() {
            String::from("YouTube Music")
        } else {
            subtitle
        },
        cover_url: pick_cover_thumbs(&covers, 544),
    })
}

pub(super) fn parse_two_row_artist(r: &Value) -> Option<Artist> {
    let bid = r["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("UC") {
        return None;
    }
    let name = runs_text(&r["title"]);
    if name.is_empty() {
        return None;
    }
    let sub = runs_text(&r["subtitle"]);
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    Some(Artist {
        id: bid.to_string(),
        name,
        avatar_url: pick_cover_thumbs(&covers, 360),
        subscriber_count: parse_compact_count(&sub),
    })
}

pub(super) fn parse_two_row_track(r: &Value) -> Option<CatalogItem> {
    let video_id = r["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str()?;
    let title = clean_track_title(&runs_text(&r["title"]));
    if title.is_empty() {
        return None;
    }
    let mvt = video_type_from_watch_endpoint(&r["navigationEndpoint"]["watchEndpoint"]);
    let kind = match classify_video_type(mvt) {
        TrackKind::Unknown => TrackKind::Video,
        other => other,
    };
    let mut artists = Vec::new();
    let mut artist_id = None;
    let mut view_count = None;
    if let Some(runs) = r["subtitle"]["runs"].as_array() {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "," || text == "&" {
                continue;
            }
            if let Some(bid) = run["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()
                && bid.starts_with("UC")
            {
                if artist_id.is_none() {
                    artist_id = Some(bid.to_string());
                }
                artists.push(text.to_string());
                continue;
            }
            let lower = text.to_lowercase();
            if lower.contains("view") || lower.contains("play") {
                view_count = parse_compact_count(text);
            } else if artists.is_empty() {
                artists.push(text.to_string());
            }
        }
    }
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    Some(CatalogItem {
        track: Track {
            is_music: kind == TrackKind::Track,
            is_video: kind == TrackKind::Video,
            id: video_id.to_string(),
            title,
            artist: if artists.is_empty() {
                String::from("Artista desconocido")
            } else {
                artists.join(", ")
            },
            artist_id,
            album: None,
            album_id: None,
            duration_secs: None,
            view_count,
            cover_url: pick_cover_thumbs(&covers, 240),
            cover_large_url: pick_cover_thumbs(&covers, 544),
        },
        kind,
        unavailable: false,
    })
}

pub(super) fn collect_shelves<'a>(val: &'a Value, out: &mut Vec<(String, &'a [Value])>) {
    match val {
        Value::Object(map) => {
            if let Some(shelf) = map.get("musicCarouselShelfRenderer") {
                let title = runs_text(
                    &shelf["header"]["musicCarouselShelfBasicHeaderRenderer"]["title"],
                );
                if let Some(contents) = shelf["contents"].as_array() {
                    out.push((title, contents.as_slice()));
                }
            }
            if let Some(shelf) = map.get("musicShelfRenderer") {
                let title = runs_text(&shelf["title"]);
                if let Some(contents) = shelf["contents"].as_array() {
                    out.push((title, contents.as_slice()));
                }
            }
            if let Some(grid) = map.get("gridRenderer")
                && let Some(items) = grid["items"].as_array()
            {
                let title = runs_text(&grid["header"]["gridHeaderRenderer"]["title"]);
                out.push((title, items.as_slice()));
            }
            for v in map.values() {
                collect_shelves(v, out);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_shelves(v, out);
            }
        }
        _ => {}
    }
}

pub(super) fn collect_country_codes(val: &Value, out: &mut Vec<String>, seen: &mut HashSet<String>) {
    match val {
        Value::Object(map) => {
            if let Some(sel) = map.get("selectedValues").and_then(|v| v.as_array()) {
                for v in sel {
                    if let Some(code) = v.as_str()
                        && code.len() == 2
                        && code.bytes().all(|b| b.is_ascii_uppercase())
                        && code != "ZZ"
                        && seen.insert(code.to_string())
                    {
                        out.push(code.to_string());
                    }
                }
            }
            for v in map.values() {
                collect_country_codes(v, out, seen);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_country_codes(v, out, seen);
            }
        }
        _ => {}
    }
}

pub fn parse_charts_json(resp: &Value) -> ChartsResult {
    let mut res = ChartsResult::default();
    let mut seen_countries = HashSet::new();
    collect_country_codes(resp, &mut res.available_countries, &mut seen_countries);

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    for (title, items) in shelves {
        let lower = title.to_lowercase();
        let mut shelf_tracks = Vec::new();
        let mut shelf_artists = Vec::new();

        for entry in items {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer") {
                if let Some(item) = parse_responsive_list_item(r) {
                    if !item.unavailable {
                        shelf_tracks.push(item.track);
                    }
                } else if let Some(artist) = parse_responsive_artist_item(r) {
                    shelf_artists.push(artist);
                }
            } else if let Some(r) = entry.get("musicTwoRowItemRenderer") {
                if let Some(item) = parse_two_row_track(r) {
                    if !item.unavailable {
                        shelf_tracks.push(item.track);
                    }
                } else if let Some(artist) = parse_two_row_artist(r) {
                    shelf_artists.push(artist);
                }
            }
        }

        if !shelf_artists.is_empty() && res.artists.is_empty() {
            res.artists = shelf_artists.into_iter().take(12).collect();
            continue;
        }
        if !shelf_tracks.is_empty() {
            if lower.contains("trending") || lower.contains("tendencia") {
                if res.trending.is_empty() {
                    res.trending = shelf_tracks.into_iter().take(12).collect();
                }
            } else if lower.contains("top") || lower.contains("song") || lower.contains("cancion") {
                if res.top.is_empty() {
                    res.top = shelf_tracks.into_iter().take(12).collect();
                } else if res.trending.is_empty() {
                    res.trending = shelf_tracks.into_iter().take(12).collect();
                }
            } else if res.trending.is_empty() {
                res.trending = shelf_tracks.into_iter().take(12).collect();
            } else if res.top.is_empty() {
                res.top = shelf_tracks.into_iter().take(12).collect();
            }
        }
    }

    if res.trending.is_empty() && !res.top.is_empty() {
        res.trending = res.top.clone();
    }
    res
}

pub fn parse_new_albums_json(resp: &Value) -> Vec<Album> {
    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);
    let mut albums = Vec::new();
    let mut seen = HashSet::new();
    for (_title, items) in shelves {
        for entry in items {
            if let Some(r) = entry.get("musicTwoRowItemRenderer")
                && let Some((album, _)) = parse_two_row_album(r)
                && seen.insert(album.id.clone())
            {
                albums.push(album);
            }
        }
    }
    albums
}

pub fn parse_radio_next_json(resp: &Value) -> Vec<CatalogItem> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    collect_panel_videos(resp, &mut out, &mut seen);
    out
}

pub(super) fn collect_panel_videos(val: &Value, out: &mut Vec<CatalogItem>, seen: &mut HashSet<String>) {
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("playlistPanelVideoRenderer")
                && let Some((item, _)) = parse_panel_video_renderer(r)
                && seen.insert(item.track.id.clone())
            {
                out.push(item);
                return;
            }
            if let Some(wrapper) = map.get("playlistPanelVideoWrapperRenderer") {
                if let Some(primary) = wrapper
                    .get("primaryRenderer")
                    .and_then(|p| p.get("playlistPanelVideoRenderer"))
                    && let Some((item, _)) = parse_panel_video_renderer(primary)
                    && seen.insert(item.track.id.clone())
                {
                    out.push(item);
                }
                return;
            }
            for v in map.values() {
                collect_panel_videos(v, out, seen);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_panel_videos(v, out, seen);
            }
        }
        _ => {}
    }
}

pub fn parse_music_details_json(video_id: &str, resp: &Value) -> Result<MusicTrackDetails, String> {
    let mut lyrics_browse_id = None;
    let mut related_browse_id = None;
    collect_watch_tab_ids(resp, &mut lyrics_browse_id, &mut related_browse_id);

    // Prefer matching inside playlistPanelVideoWrapperRenderer so we capture both the
    // exact requested track (ATV or OMV) and its official Song <-> Video counterpart(s).
    let mut wrapped: Option<WrappedPanelMatch> = None;
    find_wrapped_panel_video_by_id(resp, video_id, &mut wrapped);
    if let Some((item, covers, counterparts)) = wrapped {
        return Ok(MusicTrackDetails {
            item,
            covers,
            lyrics_browse_id,
            related_browse_id,
            counterparts,
        });
    }

    let mut matched: Option<(CatalogItem, Vec<(String, u32)>)> = None;
    find_panel_video_by_id(resp, video_id, &mut matched);

    let (item, covers) = matched.ok_or_else(|| {
        String::from("No se encontraron detalles de la pista en YouTube Music.")
    })?;

    Ok(MusicTrackDetails {
        item,
        covers,
        lyrics_browse_id,
        related_browse_id,
        counterparts: Vec::new(),
    })
}

type CoverList = Vec<(String, u32)>;
type WrappedPanelMatch = (CatalogItem, CoverList, Vec<CatalogItem>);

pub(super) fn find_wrapped_panel_video_by_id(
    val: &Value,
    video_id: &str,
    out: &mut Option<WrappedPanelMatch>,
) {
    if out.is_some() {
        return;
    }
    match val {
        Value::Object(map) => {
            if let Some(wrapper) = map.get("playlistPanelVideoWrapperRenderer") {
                let mut parsed_all: Vec<(CatalogItem, Vec<(String, u32)>)> = Vec::new();
                if let Some(primary) = wrapper.pointer("/primaryRenderer/playlistPanelVideoRenderer")
                    && let Some(p) = parse_panel_video_renderer(primary)
                {
                    parsed_all.push(p);
                }
                if let Some(cps) = wrapper.get("counterpart").and_then(Value::as_array) {
                    for cp in cps {
                        if let Some(r) = cp.pointer("/counterpartRenderer/playlistPanelVideoRenderer")
                            && let Some(p) = parse_panel_video_renderer(r)
                        {
                            parsed_all.push(p);
                        }
                    }
                }
                if let Some(pos) = parsed_all.iter().position(|(item, _)| item.track.id == video_id) {
                    let (matched_item, covers) = parsed_all.remove(pos);
                    let counterparts = parsed_all
                        .into_iter()
                        .map(|(item, _)| item)
                        .filter(|item| item.track.id != video_id && !item.unavailable)
                        .collect();
                    *out = Some((matched_item, covers, counterparts));
                    return;
                }
            }
            for v in map.values() {
                find_wrapped_panel_video_by_id(v, video_id, out);
                if out.is_some() {
                    return;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                find_wrapped_panel_video_by_id(v, video_id, out);
                if out.is_some() {
                    return;
                }
            }
        }
        _ => {}
    }
}

pub(super) fn collect_watch_tab_ids(
    val: &Value,
    lyrics_id: &mut Option<String>,
    related_id: &mut Option<String>,
) {
    match val {
        Value::Object(map) => {
            if let Some(bid) = map
                .get("browseEndpoint")
                .and_then(|e| e.get("browseId"))
                .and_then(Value::as_str)
            {
                if bid.starts_with("MPLYt_") && lyrics_id.is_none() {
                    *lyrics_id = Some(bid.to_string());
                } else if bid.starts_with("MPTRt_") && related_id.is_none() {
                    *related_id = Some(bid.to_string());
                }
            }
            for v in map.values() {
                collect_watch_tab_ids(v, lyrics_id, related_id);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_watch_tab_ids(v, lyrics_id, related_id);
            }
        }
        _ => {}
    }
}

pub(super) fn find_panel_video_by_id(
    val: &Value,
    video_id: &str,
    out: &mut Option<(CatalogItem, Vec<(String, u32)>)>,
) {
    if out.is_some() {
        return;
    }
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("playlistPanelVideoRenderer")
                && r["videoId"].as_str() == Some(video_id)
                && let Some(parsed) = parse_panel_video_renderer(r)
            {
                *out = Some(parsed);
                return;
            }
            for v in map.values() {
                find_panel_video_by_id(v, video_id, out);
                if out.is_some() {
                    return;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                find_panel_video_by_id(v, video_id, out);
                if out.is_some() {
                    return;
                }
            }
        }
        _ => {}
    }
}

pub fn parse_related_browse_json(resp: &Value) -> RelatedFeed {
    let mut feed = RelatedFeed::default();
    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    let mut seen_tracks = HashSet::new();
    let mut seen_albums = HashSet::new();
    let mut seen_playlists = HashSet::new();
    let mut seen_artists = HashSet::new();

    for (_title, items) in shelves {
        for entry in items {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer")
                && let Some(item) = parse_responsive_list_item(r)
                && !item.unavailable
                && seen_tracks.insert(item.track.id.clone())
                && feed.tracks.len() < 16
            {
                feed.tracks.push(item.track);
            } else if let Some(r) = entry.get("musicTwoRowItemRenderer") {
                if let Some((album, _)) = parse_two_row_album(r)
                    && seen_albums.insert(album.id.clone())
                    && feed.albums.len() < 12
                {
                    feed.albums.push(album);
                } else if let Some(pl) = parse_two_row_playlist(r)
                    && seen_playlists.insert(pl.id.clone())
                    && feed.playlists.len() < 12
                {
                    feed.playlists.push(pl);
                } else if let Some(artist) = parse_two_row_artist(r)
                    && seen_artists.insert(artist.id.clone())
                    && feed.artists.len() < 12
                {
                    feed.artists.push(artist);
                }
            }
        }
    }
    feed
}

pub fn parse_timed_lyrics_browse_json(resp: &Value) -> Option<LyricsData> {
    let mut synced = Vec::new();
    collect_timed_lyrics(resp, &mut synced);
    if !synced.is_empty() {
        return Some(LyricsData::Synced(synced));
    }
    parse_lyrics_browse_json(resp).map(LyricsData::Plain)
}

pub(super) fn collect_timed_lyrics(val: &Value, out: &mut Vec<SyncedLyricLine>) {
    if !out.is_empty() {
        return;
    }
    match val {
        Value::Object(map) => {
            if let Some(arr) = map.get("timedLyricsData").and_then(Value::as_array) {
                for entry in arr {
                    let text = entry["lyricLine"].as_str().unwrap_or("").trim();
                    let ms = entry["cueRange"]["startTimeMilliseconds"]
                        .as_str()
                        .and_then(|s| s.parse::<u64>().ok())
                        .or_else(|| entry["cueRange"]["startTimeMilliseconds"].as_u64());
                    if !text.is_empty()
                        && let Some(ms) = ms
                    {
                        out.push(SyncedLyricLine {
                            time_secs: ms as f64 / 1000.0,
                            text: text.to_string(),
                        });
                    }
                }
                if !out.is_empty() {
                    return;
                }
            }
            for v in map.values() {
                collect_timed_lyrics(v, out);
                if !out.is_empty() {
                    return;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_timed_lyrics(v, out);
                if !out.is_empty() {
                    return;
                }
            }
        }
        _ => {}
    }
}

pub fn parse_lyrics_browse_json(resp: &Value) -> Option<String> {
    let mut found = None;
    find_description_shelf(resp, &mut found);
    found.filter(|s| !s.trim().is_empty())
}

pub(super) fn find_description_shelf(val: &Value, out: &mut Option<String>) {
    if out.is_some() {
        return;
    }
    match val {
        Value::Object(map) => {
            if let Some(shelf) = map.get("musicDescriptionShelfRenderer") {
                let text = runs_text(&shelf["description"]);
                if !text.trim().is_empty() {
                    *out = Some(text);
                    return;
                }
            }
            for v in map.values() {
                find_description_shelf(v, out);
                if out.is_some() {
                    return;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                find_description_shelf(v, out);
                if out.is_some() {
                    return;
                }
            }
        }
        _ => {}
    }
}

pub(super) fn parse_responsive_album_item(item: &Value) -> Option<Album> {
    let bid = item["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("MPREb_") && !bid.starts_with("OLAK") {
        return None;
    }
    let flex = item["flexColumns"].as_array()?;
    let title = runs_text(&flex.first()?["musicResponsiveListItemFlexColumnRenderer"]["text"]);
    if title.is_empty() {
        return None;
    }
    let mut artist_parts = Vec::new();
    let mut year = None;
    if let Some(runs) = flex
        .get(1)
        .and_then(|c| c["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"].as_array())
    {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "," || text == "&" {
                continue;
            }
            let lower = text.to_lowercase();
            if matches!(lower.as_str(), "album" | "álbum" | "single" | "sencillo" | "ep") {
                continue;
            }
            if text.len() == 4
                && let Ok(y) = text.parse::<u16>()
                && (1900..=2100).contains(&y)
            {
                year = Some(y);
                continue;
            }
            artist_parts.push(text.to_string());
        }
    }
    let covers = extract_thumbs(&item["thumbnail"]);
    Some(Album {
        id: bid.to_string(),
        title,
        artist: artist_parts.join(", "),
        year,
        cover_url: pick_cover_thumbs(&covers, 240),
    })
}

pub fn parse_search_suggestions_json(resp: &Value) -> Vec<String> {
    parse_rich_search_suggestions_json(resp).terms
}

pub fn parse_rich_search_suggestions_json(resp: &Value) -> SearchSuggestions {
    let mut out = SearchSuggestions::default();
    let mut seen_terms = HashSet::new();
    let mut seen_tracks = HashSet::new();
    let mut seen_artists = HashSet::new();
    let mut seen_albums = HashSet::new();
    collect_rich_suggestions(
        resp,
        &mut out,
        &mut seen_terms,
        &mut seen_tracks,
        &mut seen_artists,
        &mut seen_albums,
    );
    out
}

pub(super) fn collect_rich_suggestions(
    val: &Value,
    out: &mut SearchSuggestions,
    seen_terms: &mut HashSet<String>,
    seen_tracks: &mut HashSet<String>,
    seen_artists: &mut HashSet<String>,
    seen_albums: &mut HashSet<String>,
) {
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("searchSuggestionRenderer") {
                let text = runs_text(&r["suggestion"]);
                if !text.is_empty()
                    && out.terms.len() < 6
                    && seen_terms.insert(text.to_lowercase())
                {
                    out.terms.push(text);
                }
                return;
            }
            if let Some(r) = map.get("musicResponsiveListItemRenderer") {
                if let Some(item) = parse_responsive_list_item(r) {
                    if !item.unavailable
                        && out.tracks.len() < 4
                        && seen_tracks.insert(item.track.id.clone())
                    {
                        out.tracks.push(item.track);
                    }
                } else if let Some(artist) = parse_responsive_artist_item(r) {
                    if out.artists.len() < 3 && seen_artists.insert(artist.id.clone()) {
                        out.artists.push(artist);
                    }
                } else if let Some(album) = parse_responsive_album_item(r)
                    && out.albums.len() < 3
                    && seen_albums.insert(album.id.clone())
                {
                    out.albums.push(album);
                }
                return;
            }
            for v in map.values() {
                collect_rich_suggestions(
                    v,
                    out,
                    seen_terms,
                    seen_tracks,
                    seen_artists,
                    seen_albums,
                );
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_rich_suggestions(
                    v,
                    out,
                    seen_terms,
                    seen_tracks,
                    seen_artists,
                    seen_albums,
                );
            }
        }
        _ => {}
    }
}

pub(super) fn find_header_title_and_thumbs(resp: &Value) -> (String, Vec<(String, u32)>) {
    let header_candidates = [
        &resp["header"]["musicResponsiveHeaderRenderer"],
        &resp["header"]["musicDetailHeaderRenderer"],
        &resp["header"]["musicImmersiveHeaderRenderer"],
        &resp["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]
            ["sectionListRenderer"]["contents"][0]["musicResponsiveHeaderRenderer"],
    ];
    for h in header_candidates {
        if h.is_object() {
            let title = runs_text(&h["title"]);
            let thumbs = extract_thumbs(&h["thumbnail"]);
            if !title.is_empty() {
                return (title, thumbs);
            }
        }
    }
    (String::new(), Vec::new())
}

pub fn parse_album_browse_json(resp: &Value) -> Result<(String, Vec<Track>), String> {
    let (title, covers) = find_header_title_and_thumbs(resp);
    let large_cover = pick_cover_thumbs(&covers, 544);
    let small_cover = pick_cover_thumbs(&covers, 240);

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    let mut tracks = Vec::new();
    for (_shelf_title, items) in shelves {
        for entry in items {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer")
                && let Some(item) = parse_responsive_list_item(r)
                && item.kind == TrackKind::Track
                && !item.unavailable
            {
                let mut tr = item.track;
                if tr.album.is_none() && !title.is_empty() {
                    tr.album = Some(title.clone());
                }
                if large_cover.is_some() {
                    tr.cover_large_url = large_cover.clone();
                }
                if tr.cover_url.is_none() {
                    tr.cover_url = small_cover.clone();
                }
                tracks.push(tr);
            }
        }
    }
    if title.is_empty() && tracks.is_empty() {
        return Err(String::from("Error cargando álbum"));
    }
    Ok((
        if title.is_empty() {
            String::from("Álbum")
        } else {
            title
        },
        tracks,
    ))
}

pub(super) fn extract_continuation(val: &Value) -> Option<String> {
    match val {
        Value::Object(map) => {
            if let Some(tok) = map
                .get("nextContinuationData")
                .and_then(|d| d.get("continuation"))
                .and_then(Value::as_str)
            {
                return Some(tok.to_string());
            }
            if let Some(tok) = map
                .get("continuationEndpoint")
                .and_then(|e| e.get("continuationCommand"))
                .and_then(|c| c.get("token"))
                .and_then(Value::as_str)
            {
                return Some(tok.to_string());
            }
            for v in map.values() {
                if let Some(tok) = extract_continuation(v) {
                    return Some(tok);
                }
            }
            None
        }
        Value::Array(arr) => arr.iter().find_map(extract_continuation),
        _ => None,
    }
}

pub fn parse_playlist_browse_json(resp: &Value) -> PlaylistBrowsePage {
    let (title, _) = find_header_title_and_thumbs(resp);
    let mut items = Vec::new();
    let mut had_entries = false;

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);
    for (_shelf_title, entries) in shelves {
        for entry in entries {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer") {
                had_entries = true;
                if let Some(item) = parse_responsive_list_item(r) {
                    items.push(item);
                }
            }
        }
    }
    let exhausted = extract_continuation(resp).is_none();
    PlaylistBrowsePage {
        title: if title.is_empty() {
            String::from("Playlist")
        } else {
            title
        },
        items,
        had_entries,
        exhausted,
    }
}

pub(super) fn parse_playlist_continuation_items(resp: &Value) -> Vec<CatalogItem> {
    let mut items = Vec::new();
    collect_responsive_items_deep(resp, &mut items);
    items
}

pub(super) fn collect_responsive_items_deep(val: &Value, out: &mut Vec<CatalogItem>) {
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("musicResponsiveListItemRenderer") {
                if let Some(item) = parse_responsive_list_item(r) {
                    out.push(item);
                }
                return;
            }
            for v in map.values() {
                collect_responsive_items_deep(v, out);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_responsive_items_deep(v, out);
            }
        }
        _ => {}
    }
}

pub fn parse_artist_enrichment_json(artist_id: &str, resp: &Value) -> ArtistEnrichment {
    let mut out = ArtistEnrichment::default();

    let header = if resp["header"]["musicImmersiveHeaderRenderer"].is_object() {
        &resp["header"]["musicImmersiveHeaderRenderer"]
    } else {
        &resp["header"]["musicVisualHeaderRenderer"]
    };

    if header.is_object() {
        let name = runs_text(&header["title"]);
        if !name.is_empty() {
            out.name = Some(name);
        }
        let desc = runs_text(&header["description"]);
        if !desc.is_empty() {
            out.description = Some(desc);
        }
        let sub_text = runs_text(
            &header["subscriptionButton"]["subscribeButtonRenderer"]["subscriberCountText"],
        );
        out.subscriber_count = parse_compact_count(&sub_text);
        let thumbs = extract_thumbs(&header["thumbnail"]);
        out.header_url = pick_cover_thumbs(&thumbs, 1000);
    }

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    let mut seen_tracks = HashSet::new();
    let mut seen_videos = HashSet::new();
    let mut seen_albums = HashSet::new();
    let mut seen_singles = HashSet::new();
    let mut seen_playlists = HashSet::new();
    let mut seen_similar = HashSet::new();

    for (shelf_title, entries) in shelves {
        let lower = shelf_title.to_lowercase();
        let is_singles_shelf =
            lower.contains("single") || lower.contains("sencillo") || lower.contains("ep");

        for entry in entries {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer")
                && let Some(item) = parse_responsive_list_item(r)
            {
                if let Some(vc) = item.track.view_count {
                    out.view_counts.insert(item.track.id.clone(), vc);
                }
                if !item.unavailable {
                    if item.kind == TrackKind::Video {
                        if seen_videos.insert(item.track.id.clone()) {
                            let mut tr = item.track;
                            tr.is_music = true;
                            tr.is_video = true;
                            if tr.artist_id.is_none() {
                                tr.artist_id = Some(artist_id.to_string());
                            }
                            out.videos.push(tr);
                        }
                    } else if seen_tracks.insert(item.track.id.clone()) {
                        out.tracks.push(item.track);
                    }
                }
            } else if let Some(r) = entry.get("musicTwoRowItemRenderer") {
                if let Some((album, is_single_badge)) = parse_two_row_album(r) {
                    if is_singles_shelf || is_single_badge {
                        if seen_singles.insert(album.id.clone()) {
                            out.singles.push(album);
                        }
                    } else if seen_albums.insert(album.id.clone()) {
                        out.albums.push(album);
                    }
                } else if let Some(pl) = parse_two_row_playlist(r) {
                    if seen_playlists.insert(pl.id.clone()) {
                        out.playlists.push(pl);
                    }
                } else if let Some(artist) = parse_two_row_artist(r) {
                    if seen_similar.insert(artist.id.clone()) {
                        out.similar_artists.push(artist);
                    }
                } else if let Some(item) = parse_two_row_track(r)
                    && !item.unavailable
                    && seen_videos.insert(item.track.id.clone())
                {
                    let mut tr = item.track;
                    tr.is_music = true;
                    tr.is_video = true;
                    if tr.artist_id.is_none() {
                        tr.artist_id = Some(artist_id.to_string());
                    }
                    out.videos.push(tr);
                }
            }
        }
    }

    out
}

