mod browse;
mod track;

pub use browse::*;
pub(super) use browse::{extract_continuation, parse_playlist_continuation_items};
pub use track::*;

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

