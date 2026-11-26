use std::time::Duration;

use serde_json::Value;

use super::Backend;
use crate::i18n::tr;
use crate::models::{ExternalPlaylistPreview, ExternalTrackQuery, Track};

const SPOTIFY_EMBED_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub(crate) fn extract_spotify_playlist_id(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix("spotify:playlist:") {
        let id = rest.split([':', '?', '&', '/']).next().unwrap_or("");
        return validate_spotify_id(id);
    }

    if let Some(pos) = trimmed.find("/playlist/") {
        let after = &trimmed[pos + "/playlist/".len()..];
        let id = after.split(['?', '#', '/', '&']).next().unwrap_or("");
        return validate_spotify_id(id);
    }

    // Also allow pasting a bare Spotify playlist ID
    if trimmed.len() == 22 && trimmed.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Some(trimmed.to_string());
    }

    None
}

fn validate_spotify_id(id: &str) -> Option<String> {
    if (10..=40).contains(&id.len()) && id.chars().all(|c| c.is_ascii_alphanumeric()) {
        Some(id.to_string())
    } else {
        None
    }
}

pub(crate) fn parse_spotify_embed_html(html: &str) -> Result<ExternalPlaylistPreview, String> {
    let marker = "__NEXT_DATA__";
    let marker_pos = html
        .find(marker)
        .ok_or_else(|| tr(
            "No se pudieron leer los datos de la playlist de Spotify. Verifica que sea pública.",
            "Could not read Spotify playlist data. Make sure the playlist is public.",
        ).to_string())?;
    let after_marker = &html[marker_pos..];
    let json_start = after_marker
        .find('>')
        .map(|i| i + 1)
        .ok_or_else(|| "Malformed Spotify embed script tag".to_string())?;
    let after_open = &after_marker[json_start..];
    let json_end = after_open
        .find("</script>")
        .ok_or_else(|| "Unterminated Spotify embed script tag".to_string())?;
    let json_str = after_open[..json_end].trim();

    let root: Value = serde_json::from_str(json_str)
        .map_err(|_| tr(
            "No se pudo interpretar la respuesta de Spotify.",
            "Could not parse Spotify response.",
        ).to_string())?;

    let entity = root
        .pointer("/props/pageProps/state/data/entity")
        .ok_or_else(|| tr(
            "Playlist no encontrada o privada en Spotify.",
            "Spotify playlist not found or private.",
        ).to_string())?;

    let name = entity
        .get("name")
        .or_else(|| entity.get("title"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Spotify Playlist")
        .to_string();

    let cover_url = entity
        .pointer("/visualIdentity/image/0/url")
        .or_else(|| entity.pointer("/coverArt/sources/0/url"))
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let mut tracks = Vec::new();
    if let Some(list) = entity.get("trackList").and_then(Value::as_array) {
        for item in list {
            let title = item
                .get("title")
                .and_then(Value::as_str)
                .map(str::trim)
                .unwrap_or("");
            if title.is_empty() {
                continue;
            }
            let artist = item
                .get("subtitle")
                .and_then(Value::as_str)
                .map(str::trim)
                .unwrap_or("")
                .replace('\u{00a0}', " ");
            let duration_secs = item
                .get("duration")
                .and_then(Value::as_u64)
                .map(|ms| (ms / 1000) as u32)
                .filter(|&s| s > 0);
            tracks.push(ExternalTrackQuery {
                title: title.to_string(),
                artist,
                duration_secs,
            });
        }
    }

    if tracks.is_empty() {
        return Err(tr(
            "La playlist de Spotify está vacía o no contiene pistas accesibles.",
            "The Spotify playlist is empty or has no accessible tracks.",
        ).to_string());
    }

    Ok(ExternalPlaylistPreview {
        name,
        cover_url,
        tracks,
    })
}

fn normalize_match_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_space = true;
    for ch in input.chars() {
        let lower = match ch {
            'Á' | 'À' | 'Â' | 'Ä' | 'Ã' | 'á' | 'à' | 'â' | 'ä' | 'ã' => 'a',
            'É' | 'È' | 'Ê' | 'Ë' | 'é' | 'è' | 'ê' | 'ë' => 'e',
            'Í' | 'Ì' | 'Î' | 'Ï' | 'í' | 'ì' | 'î' | 'ï' => 'i',
            'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' | 'ó' | 'ò' | 'ô' | 'ö' | 'õ' => 'o',
            'Ú' | 'Ù' | 'Û' | 'Ü' | 'ú' | 'ù' | 'û' | 'ü' => 'u',
            'Ñ' | 'ñ' => 'n',
            'Ç' | 'ç' => 'c',
            c => c.to_ascii_lowercase(),
        };
        if lower.is_alphanumeric() {
            out.push(lower);
            prev_space = false;
        } else if !prev_space {
            out.push(' ');
            prev_space = true;
        }
    }
    out.trim().to_string()
}

pub(crate) fn score_match_candidate(
    query_title: &str,
    query_artist: &str,
    query_duration: Option<u32>,
    cand: &Track,
) -> i32 {
    let mut score = 0;

    if cand.is_music && !cand.is_video {
        score += 35;
    } else if cand.is_video {
        score -= 10;
    }

    let qt = normalize_match_text(query_title);
    let ct = normalize_match_text(&cand.title);
    if !qt.is_empty() && !ct.is_empty() {
        if qt == ct {
            score += 55;
        } else if ct.contains(&qt) || qt.contains(&ct) {
            score += 35;
        } else {
            let qt_words: Vec<&str> = qt.split_whitespace().collect();
            let hits = qt_words.iter().filter(|w| ct.contains(*w)).count();
            if hits > 0 {
                score += ((hits * 30) / qt_words.len().max(1)) as i32;
            }
        }
    }

    let qa = normalize_match_text(query_artist);
    let ca = normalize_match_text(&cand.artist);
    if !qa.is_empty() && !ca.is_empty() {
        if qa == ca || ca.contains(&qa) || qa.contains(&ca) {
            score += 35;
        } else {
            let qa_words: Vec<&str> = qa.split_whitespace().filter(|w| w.len() > 1).collect();
            let hits = qa_words.iter().filter(|w| ca.contains(*w)).count();
            if hits > 0 {
                score += ((hits * 28) / qa_words.len().max(1)) as i32;
            }
        }
    }

    if let (Some(qd), Some(cd)) = (query_duration, cand.duration_secs) {
        let diff = qd.abs_diff(cd);
        if diff <= 5 {
            score += 25;
        } else if diff <= 15 {
            score += 12;
        } else if diff > 45 {
            score -= 20;
        }
    }

    score
}

impl Backend {
    pub async fn fetch_spotify_playlist(
        &self,
        input: &str,
    ) -> Result<ExternalPlaylistPreview, String> {
        let playlist_id = extract_spotify_playlist_id(input).ok_or_else(|| {
            tr(
                "Enlace de playlist de Spotify no válido.",
                "Invalid Spotify playlist link.",
            )
            .to_string()
        })?;

        let url = format!("https://open.spotify.com/embed/playlist/{playlist_id}");
        let resp = self
            .http
            .get(&url)
            .header(reqwest::header::USER_AGENT, SPOTIFY_EMBED_UA)
            .header(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.9")
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|_| {
                tr(
                    "No se pudo conectar con Spotify. Verifica tu conexión.",
                    "Could not connect to Spotify. Check your internet connection.",
                )
                .to_string()
            })?;

        if !resp.status().is_success() {
            return Err(tr(
                "No se pudo cargar la playlist de Spotify (puede ser privada o no existir).",
                "Could not load the Spotify playlist (it may be private or not exist).",
            )
            .to_string());
        }

        let html = resp
            .text()
            .await
            .map_err(|e| format!("Error leyendo respuesta de Spotify: {e}"))?;
        parse_spotify_embed_html(&html)
    }

    pub async fn match_external_track(
        &self,
        title: &str,
        artist: &str,
        duration_secs: Option<u32>,
    ) -> Result<Option<Track>, String> {
        let clean_title = title.trim();
        let clean_artist = artist.trim();
        if clean_title.is_empty() {
            return Ok(None);
        }

        let query = if clean_artist.is_empty() {
            clean_title.to_string()
        } else {
            format!("{clean_title} {clean_artist}")
        };

        let results = self.search_all(&query).await?;
        let best = results
            .tracks
            .into_iter()
            .map(|t| {
                let s = score_match_candidate(clean_title, clean_artist, duration_secs, &t);
                (s, t)
            })
            .max_by_key(|(s, _)| *s);

        match best {
            Some((score, track)) if score >= 30 => Ok(Some(track)),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_spotify_playlist_id, parse_spotify_embed_html, score_match_candidate};
    use crate::models::Track;

    #[test]
    fn extracts_playlist_id_from_all_spotify_formats() {
        assert_eq!(
            extract_spotify_playlist_id("https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M?si=abc123"),
            Some("37i9dQZF1DXcBWIGoYBM5M".to_string())
        );
        assert_eq!(
            extract_spotify_playlist_id("https://open.spotify.com/intl-es/playlist/37i9dQZF1DXcBWIGoYBM5M"),
            Some("37i9dQZF1DXcBWIGoYBM5M".to_string())
        );
        assert_eq!(
            extract_spotify_playlist_id("spotify:playlist:37i9dQZF1DXcBWIGoYBM5M"),
            Some("37i9dQZF1DXcBWIGoYBM5M".to_string())
        );
        assert_eq!(
            extract_spotify_playlist_id("https://open.spotify.com/track/37i9dQZF1DXcBWIGoYBM5M"),
            None
        );
    }

    #[test]
    fn parses_next_data_from_spotify_embed_html() {
        let html = r#"<html><head><script id="__NEXT_DATA__" type="application/json">{"props":{"pageProps":{"state":{"data":{"entity":{"name":"Road Trip","coverArt":{"sources":[{"url":"https://i.scdn.co/image/abc"}]},"trackList":[{"title":"Blinding Lights","subtitle":"The Weeknd","duration":200040},{"title":"Take On Me","subtitle":"a-ha","duration":225280}]}}}}}}</script></head></html>"#;
        let preview = parse_spotify_embed_html(html).expect("should parse embed HTML");
        assert_eq!(preview.name, "Road Trip");
        assert_eq!(preview.cover_url.as_deref(), Some("https://i.scdn.co/image/abc"));
        assert_eq!(preview.tracks.len(), 2);
        assert_eq!(preview.tracks[0].title, "Blinding Lights");
        assert_eq!(preview.tracks[0].artist, "The Weeknd");
        assert_eq!(preview.tracks[0].duration_secs, Some(200));
    }

    #[test]
    fn prefers_exact_studio_music_candidate() {
        let studio = Track {
            is_music: true,
            is_video: false,
            id: "aaaaaaaaaaa".into(),
            title: "Blinding Lights".into(),
            artist: "The Weeknd".into(),
            artist_id: None,
            album: None,
            album_id: None,
            duration_secs: Some(200),
            view_count: None,
            cover_url: None,
            cover_large_url: None,
        };
        let remix_video = Track {
            is_music: false,
            is_video: true,
            id: "bbbbbbbbbbb".into(),
            title: "Blinding Lights (1 Hour Looped)".into(),
            artist: "Random Channel".into(),
            artist_id: None,
            album: None,
            album_id: None,
            duration_secs: Some(3600),
            view_count: None,
            cover_url: None,
            cover_large_url: None,
        };

        assert!(
            score_match_candidate("Blinding Lights", "The Weeknd", Some(200), &studio)
                > score_match_candidate("Blinding Lights", "The Weeknd", Some(200), &remix_video)
        );
    }
}
