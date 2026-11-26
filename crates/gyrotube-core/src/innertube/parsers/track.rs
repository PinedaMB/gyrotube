use super::*;

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

