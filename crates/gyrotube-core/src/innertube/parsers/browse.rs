use super::*;

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

pub(in crate::innertube) fn extract_continuation(val: &Value) -> Option<String> {
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

pub(in crate::innertube) fn parse_playlist_continuation_items(resp: &Value) -> Vec<CatalogItem> {
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

