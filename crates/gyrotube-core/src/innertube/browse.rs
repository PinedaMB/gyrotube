use super::*;
use super::parsers::*;

impl InnerTubeClient {
    pub async fn fetch_charts(&self, country: Option<&str>) -> Result<ChartsResult, String> {
        let selected = country.unwrap_or("ZZ");
        let body = json!({
            "browseId": "FEmusic_charts",
            "formData": {
                "selectedValues": [selected]
            }
        });
        let resp = self.ytm_post("browse", body, country).await?;
        Ok(parse_charts_json(&resp))
    }

    pub async fn fetch_new_albums(&self) -> Result<Vec<Album>, String> {
        let body = json!({
            "browseId": "FEmusic_new_releases_albums"
        });
        let resp = self.ytm_post("browse", body, None).await?;
        Ok(parse_new_albums_json(&resp))
    }

    pub async fn fetch_album_tracks(&self, album_id: &str) -> Result<(String, Vec<Track>), String> {
        let body = json!({ "browseId": album_id });
        let resp = self.ytm_post("browse", body, None).await?;
        parse_album_browse_json(&resp)
    }

    pub async fn fetch_playlist_tracks(&self, playlist_id: &str) -> Result<PlaylistBrowsePage, String> {
        let clean = playlist_id.strip_prefix("VL").unwrap_or(playlist_id);
        let browse_id = format!("VL{clean}");
        let resp = self
            .ytm_post("browse", json!({ "browseId": browse_id }), None)
            .await?;
        let mut page = parse_playlist_browse_json(&resp);

        let mut continuation = extract_continuation(&resp);
        for _ in 0..3 {
            let Some(token) = continuation.take() else {
                page.exhausted = true;
                break;
            };
            if page
                .items
                .iter()
                .any(|i| matches!(i.kind, TrackKind::Track | TrackKind::Video) && !i.unavailable)
            {
                break;
            }
            let Ok(next_resp) = self
                .ytm_post("browse", json!({ "continuation": token }), None)
                .await
            else {
                break;
            };
            let next_items = parse_playlist_continuation_items(&next_resp);
            page.had_entries |= !next_items.is_empty();
            page.items.extend(next_items);
            continuation = extract_continuation(&next_resp);
            if continuation.is_none() {
                page.exhausted = true;
            }
        }
        if continuation.is_none() {
            page.exhausted = true;
        }
        Ok(page)
    }

    pub async fn fetch_radio_track(&self, video_id: &str) -> Result<Vec<CatalogItem>, String> {
        let body = json!({
            "videoId": video_id,
            "playlistId": format!("RDAMVM{video_id}"),
            "isAudioOnly": true,
            "enablePersistentPlaylistPanel": true
        });
        let resp = self.ytm_post("next", body, None).await?;
        Ok(parse_radio_next_json(&resp))
    }

    pub async fn fetch_radio_playlist(&self, playlist_id: &str) -> Result<Vec<CatalogItem>, String> {
        let clean = playlist_id.strip_prefix("VL").unwrap_or(playlist_id);
        let body = json!({
            "playlistId": format!("RDAMPL{clean}"),
            "isAudioOnly": true,
            "enablePersistentPlaylistPanel": true
        });
        let resp = self.ytm_post("next", body, None).await?;
        Ok(parse_radio_next_json(&resp))
    }

    pub async fn search_suggestions(&self, query: &str) -> Result<Vec<String>, String> {
        Ok(self.search_suggestions_rich(query).await?.terms)
    }

    pub async fn search_suggestions_rich(&self, query: &str) -> Result<SearchSuggestions, String> {
        let resp = self
            .ytm_post("music/get_search_suggestions", json!({ "input": query }), None)
            .await?;
        Ok(parse_rich_search_suggestions_json(&resp))
    }

    pub async fn fetch_music_details(&self, video_id: &str) -> Result<MusicTrackDetails, String> {
        if let Ok(guard) = self.details_cache.read()
            && let Some((ts, cached)) = guard.get(video_id)
            && ts.elapsed() < DETAILS_CACHE_TTL
        {
            return Ok(cached.clone());
        }
        let body = json!({
            "videoId": video_id,
            "playlistId": format!("RDAMVM{video_id}"),
            "isAudioOnly": true,
            "enablePersistentPlaylistPanel": true
        });
        let resp = self.ytm_post("next", body, None).await?;
        let details = parse_music_details_json(video_id, &resp)?;
        if let Ok(mut guard) = self.details_cache.write() {
            guard.retain(|_, (ts, _)| ts.elapsed() < DETAILS_CACHE_TTL);
            if guard.len() >= DETAILS_CACHE_MAX
                && let Some(oldest_key) = guard
                    .iter()
                    .min_by_key(|(_, (ts, _))| *ts)
                    .map(|(k, _)| k.clone())
            {
                guard.remove(&oldest_key);
            }
            guard.insert(video_id.to_string(), (Instant::now(), details.clone()));
        }
        Ok(details)
    }

    pub async fn fetch_related(&self, related_browse_id: &str) -> Result<RelatedFeed, String> {
        let resp = self
            .ytm_post("browse", json!({ "browseId": related_browse_id }), None)
            .await?;
        Ok(parse_related_browse_json(&resp))
    }

    pub async fn fetch_lyrics(&self, lyrics_browse_id: &str) -> Result<String, String> {
        let resp = self
            .ytm_post("browse", json!({ "browseId": lyrics_browse_id }), None)
            .await?;
        parse_lyrics_browse_json(&resp)
            .ok_or_else(|| String::from("No hay letras disponibles para esta pista."))
    }

    pub async fn fetch_lyrics_data(&self, lyrics_browse_id: &str) -> Result<LyricsData, String> {
        // 1. Try ANDROID_MUSIC mobile client first: returns official synced timedLyricsData when available.
        let visitor = self.visitor_data(false).await;
        let mut client_ctx = json!({
            "clientName": "ANDROID_MUSIC",
            "clientVersion": "7.27.52",
            "androidSdkVersion": 34,
            "osName": "Android",
            "osVersion": "14",
            "hl": active_hl(),
            "gl": "US"
        });
        if let Some(ref vd) = visitor
            && let Some(obj) = client_ctx.as_object_mut()
        {
            obj.insert("visitorData".into(), Value::String(vd.clone()));
        }
        let body = json!({
            "context": { "client": client_ctx },
            "browseId": lyrics_browse_id
        });
        let mut req = self
            .http
            .post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .timeout(Duration::from_secs(6))
            .header(
                "User-Agent",
                "com.google.android.apps.youtube.music/7.27.52 (Linux; U; Android 14) gzip",
            )
            .header("X-YouTube-Client-Name", "21")
            .header("X-YouTube-Client-Version", "7.27.52");
        if let Some(ref vd) = visitor {
            req = req.header("X-Goog-Visitor-Id", vd.as_str());
        }

        if let Ok(resp) = req.json(&body).send().await
            && let Ok(resp) = resp.error_for_status()
            && let Ok(val) = resp.json::<Value>().await
            && let Some(parsed) = parse_timed_lyrics_browse_json(&val)
        {
            return Ok(parsed);
        }

        // 2. Fallback to WEB_REMIX static lyrics.
        let plain = self.fetch_lyrics(lyrics_browse_id).await?;
        Ok(LyricsData::Plain(plain))
    }

    pub async fn fetch_artist_enrichment(&self, artist_id: &str) -> Result<ArtistEnrichment, String> {
        let resp = self
            .ytm_post("browse", json!({ "browseId": artist_id }), None)
            .await?;
        Ok(parse_artist_enrichment_json(artist_id, &resp))
    }
}
