use super::*;
use super::parsers::*;

impl Backend {
    pub async fn fetch_synced_lyrics(&self, track: &Track) -> Result<LyricsData, String> {
        // 1. Offline local cache
        if !track.id.is_empty()
            && validate_video_id(&track.id).is_ok()
            && let Some(dl_dir) = crate::models::downloads_dir()
        {
            let lyrics_path = dl_dir.join(format!("{}.lyrics.json", track.id));
            if let Ok(bytes) = tokio::fs::read(&lyrics_path).await
                && let Ok(saved) = serde_json::from_slice::<LyricsData>(&bytes)
            {
                return Ok(saved);
            }
        }

        // 2. YouTube Music official lyrics via innertube.rs (ANDROID_MUSIC synced -> WEB_REMIX plain).
        //    Reuses cached /next details so lyrics_browse_id costs 0 extra network calls.
        let mut ytm_plain: Option<String> = None;
        if !track.id.is_empty()
            && validate_video_id(&track.id).is_ok()
            && let Ok(details) = self.innertube.fetch_music_details(&track.id).await
            && let Some(lyrics_id) = details.lyrics_browse_id
            && let Ok(data) = self.innertube.fetch_lyrics_data(&lyrics_id).await
        {
            match data {
                LyricsData::Synced(lines) if !lines.is_empty() => {
                    return Ok(LyricsData::Synced(lines));
                }
                LyricsData::Plain(text) if !text.trim().is_empty() => {
                    ytm_plain = Some(text);
                }
                _ => {}
            }
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LrcLibResp {
            #[serde(default)]
            track_name: Option<String>,
            #[serde(default)]
            artist_name: Option<String>,
            #[serde(default)]
            duration: Option<f64>,
            synced_lyrics: Option<String>,
            plain_lyrics: Option<String>,
        }

        let artist_q = track
            .artist
            .split([',', '•', '·', '&'])
            .next()
            .unwrap_or(&track.artist)
            .trim();
        let clean_title = strip_lyrics_query_noise(&track.title);
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

        // 3. LRCLIB exact match (prefers synced timestamps if YTM only had static text)
        let mut lrclib_exact_plain: Option<String> = None;
        if let Ok(resp) = self
            .http
            .get("https://lrclib.net/api/get")
            .query(&params)
            .header(reqwest::header::USER_AGENT, "GyroTube/1.0")
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
                lrclib_exact_plain = Some(plain);
            }
        }

        // 4. LRCLIB search (prioritizes synced lyrics BEFORE falling back to static text,
        //    validating title/artist similarity and duration ±8s so we never return a wrong song)
        let mut lrclib_search_plain: Option<String> = None;
        let search_query = format!("{artist_q} {clean_title}");
        if let Ok(resp) = self
            .http
            .get("https://lrclib.net/api/search")
            .query(&[("q", search_query.trim())])
            .header(reqwest::header::USER_AGENT, "GyroTube/1.0")
            .send()
            .await
            && resp.status().is_success()
            && let Ok(list) = resp.json::<Vec<LrcLibResp>>().await
        {
            let expected_title = normalize_lyrics_match(&clean_title);
            let expected_artist = normalize_lyrics_match(artist_q);
            let mut best_synced: Option<(f64, Vec<crate::models::SyncedLyricLine>)> = None;

            for item in list {
                let item_title = normalize_lyrics_match(item.track_name.as_deref().unwrap_or(""));
                let item_artist = normalize_lyrics_match(item.artist_name.as_deref().unwrap_or(""));
                let title_ok = expected_title.is_empty()
                    || item_title == expected_title
                    || item_title.contains(&expected_title)
                    || expected_title.contains(&item_title);
                let artist_ok = expected_artist.is_empty()
                    || item_artist.contains(&expected_artist)
                    || expected_artist.contains(&item_artist);
                if !title_ok || !artist_ok {
                    continue;
                }
                let dur_diff = match (track.duration_secs, item.duration) {
                    (Some(exp), Some(got)) if exp > 0 && got > 0.0 => (exp as f64 - got).abs(),
                    _ => 0.0,
                };
                if dur_diff > 8.0 {
                    continue;
                }
                if let Some(synced) = item.synced_lyrics.filter(|s| !s.trim().is_empty()) {
                    let lines = parse_lrc(&synced);
                    if !lines.is_empty()
                        && best_synced
                            .as_ref()
                            .is_none_or(|(best_diff, _)| dur_diff < *best_diff)
                    {
                        best_synced = Some((dur_diff, lines));
                    }
                } else if lrclib_search_plain.is_none()
                    && let Some(plain) = item.plain_lyrics.filter(|s| !s.trim().is_empty())
                {
                    lrclib_search_plain = Some(plain);
                }
            }
            if let Some((_, lines)) = best_synced {
                return Ok(LyricsData::Synced(lines));
            }
        }

        // 5. Official YTM static lyrics (exact video_id)
        if let Some(plain) = ytm_plain {
            return Ok(LyricsData::Plain(plain));
        }

        // 6. LRCLIB exact plain match
        if let Some(plain) = lrclib_exact_plain {
            return Ok(LyricsData::Plain(plain));
        }

        // 7. Fallback ytmapi-rs static lyrics
        if let Ok(plain) = self.fetch_lyrics(&track.id).await {
            return Ok(LyricsData::Plain(plain));
        }

        // 8. Last resort: validated LRCLIB search plain lyrics
        if let Some(plain) = lrclib_search_plain {
            return Ok(LyricsData::Plain(plain));
        }

        Err(String::from("No hay letras disponibles para esta pista."))
    }

    pub async fn fetch_lyrics(&self, video_id: &str) -> Result<String, String> {
        if let Ok(details) = self.innertube.fetch_music_details(video_id).await
            && let Some(id) = details.lyrics_browse_id
            && let Ok(lyrics) = self.innertube.fetch_lyrics(&id).await
            && !lyrics.trim().is_empty()
        {
            return Ok(lyrics);
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

}

fn strip_lyrics_query_noise(title: &str) -> String {
    let mut out = title.trim().to_string();
    for (open, close) in [('(', ')'), ('[', ']')] {
        while let Some(start) = out.find(open) {
            let Some(rel_end) = out[start..].find(close) else {
                break;
            };
            let end = start + rel_end;
            let inner = out[start + 1..end].to_lowercase();
            let is_noise = inner.starts_with("feat")
                || inner.starts_with("ft.")
                || inner.starts_with("ft ")
                || inner.starts_with("con ")
                || inner.starts_with("with ")
                || inner.contains("remaster")
                || inner.contains("single version")
                || inner.contains("album version")
                || inner.contains("official");
            if is_noise {
                out.replace_range(start..=end, " ");
            } else {
                break;
            }
        }
    }
    if let Some(pos) = out.to_lowercase().find(" feat.") {
        out.truncate(pos);
    } else if let Some(pos) = out.to_lowercase().find(" ft.") {
        out.truncate(pos);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_lyrics_match(s: &str) -> String {
    strip_lyrics_query_noise(s)
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

