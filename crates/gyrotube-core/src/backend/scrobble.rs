use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::json;

use super::Backend;
use crate::i18n::tr;
use crate::models::{LastFmCreds, Track};

const LISTENBRAINZ_SUBMIT_URL: &str = "https://api.listenbrainz.org/1/submit-listens";
const LISTENBRAINZ_VALIDATE_URL: &str = "https://api.listenbrainz.org/1/validate-token";
const LASTFM_API_URL: &str = "https://ws.audioscrobbler.com/2.0/";
const CLIENT_NAME: &str = "Gyro";

fn is_youtube_noise_tag(inner: &str) -> bool {
    let lower = inner.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return true;
    }
    const EXACT: &[&str] = &[
        "official video",
        "official music video",
        "official audio",
        "official lyric video",
        "official lyrics video",
        "official visualizer",
        "official visualiser",
        "official hd video",
        "official 4k video",
        "official mv",
        "music video",
        "lyric video",
        "lyrics video",
        "lyrics",
        "audio",
        "visualizer",
        "visualiser",
        "hd",
        "hq",
        "4k",
        "mv",
        "video oficial",
        "audio oficial",
        "letra",
        "con letra",
    ];
    if EXACT.contains(&lower.as_str()) {
        return true;
    }
    lower.starts_with("official ")
        && (lower.contains("video")
            || lower.contains("audio")
            || lower.contains("lyric")
            || lower.contains("visuali"))
}

fn extract_feat_body(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    let lower = trimmed.to_ascii_lowercase();
    for prefix in ["feat.", "feat ", "ft.", "ft ", "featuring "] {
        if lower.starts_with(prefix) {
            let rest = trimmed[prefix.len()..].trim();
            if !rest.is_empty() {
                return Some(rest);
            }
        }
    }
    None
}

fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cleans YouTube/YTM noise and duplicated `feat.` suffixes from track titles
/// before scrobbling so ListenBrainz/MusicBrainz MBID matching succeeds more often.
pub(crate) fn clean_scrobble_title(raw: &str) -> String {
    let mut s = collapse_whitespace(raw);
    loop {
        let trimmed = s.trim_end();
        let Some((open, close)) = (if trimmed.ends_with(')') {
            Some(('(', ')'))
        } else if trimmed.ends_with(']') {
            Some(('[', ']'))
        } else {
            None
        }) else {
            break;
        };
        let Some(open_idx) = trimmed.rfind(open) else {
            break;
        };
        if open_idx == 0 {
            break;
        }
        let inner = &trimmed[open_idx + 1..trimmed.len() - close.len_utf8()];
        let prefix = trimmed[..open_idx].trim_end();

        let is_noise = is_youtube_noise_tag(inner);
        let is_duplicate_feat = extract_feat_body(inner).is_some_and(|feat_artist| {
            let prefix_lower = prefix.to_lowercase();
            let feat_lower = feat_artist.to_lowercase();
            prefix_lower.contains(&feat_lower)
                && (prefix_lower.contains("feat.")
                    || prefix_lower.contains("feat ")
                    || prefix_lower.contains("ft.")
                    || prefix_lower.contains("ft ")
                    || prefix_lower.contains("featuring "))
        });

        if is_noise || is_duplicate_feat {
            s = prefix.to_string();
        } else {
            break;
        }
    }

    let cleaned = s.trim().to_string();
    if cleaned.is_empty() {
        raw.trim().to_string()
    } else {
        cleaned
    }
}

pub(crate) fn clean_scrobble_artist(raw: &str) -> String {
    let trimmed = collapse_whitespace(raw);
    for suffix in [" - Topic", " - topic", " – Topic"] {
        if let Some(stripped) = trimmed.strip_suffix(suffix) {
            let clean = stripped.trim();
            if !clean.is_empty() {
                return clean.to_string();
            }
        }
    }
    trimmed
}

fn unix_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Minimal pure-Rust RFC 1321 MD5 for Last.fm `api_sig` signing (zero external crates).
pub(crate) fn md5_hex(input: &[u8]) -> String {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    let mut msg = input.to_vec();
    let bit_len = (msg.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    let (mut a0, mut b0, mut c0, mut d0): (u32, u32, u32, u32) =
        (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    for chunk in msg.as_chunks::<64>().0 {
        let mut m = [0u32; 16];
        for (i, word) in chunk.as_chunks::<4>().0.iter().enumerate() {
            m[i] = u32::from_le_bytes(*word);
        }
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | ((!b) & d), i),
                16..=31 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | (!d)), (7 * i) % 16),
            };
            let temp = d;
            d = c;
            c = b;
            let sum = a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(m[g]);
            b = b.wrapping_add(sum.rotate_left(S[i]));
            a = temp;
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut digest = [0u8; 16];
    digest[0..4].copy_from_slice(&a0.to_le_bytes());
    digest[4..8].copy_from_slice(&b0.to_le_bytes());
    digest[8..12].copy_from_slice(&c0.to_le_bytes());
    digest[12..16].copy_from_slice(&d0.to_le_bytes());

    let mut hex = String::with_capacity(32);
    for byte in digest {
        use std::fmt::Write;
        let _ = write!(&mut hex, "{byte:02x}");
    }
    hex
}

pub(crate) fn build_lastfm_signature(params: &BTreeMap<String, String>, secret: &str) -> String {
    let mut raw = String::new();
    for (k, v) in params {
        if k == "format" || k == "callback" {
            continue;
        }
        raw.push_str(k);
        raw.push_str(v);
    }
    raw.push_str(secret);
    md5_hex(raw.as_bytes())
}

pub(crate) fn build_listenbrainz_payload(
    track: &Track,
    now_playing: bool,
    timestamp: Option<u64>,
) -> serde_json::Value {
    let artist = clean_scrobble_artist(&track.artist);
    let title = clean_scrobble_title(&track.title);

    let mut additional_info = serde_json::Map::new();
    additional_info.insert("media_player".into(), json!(CLIENT_NAME));
    additional_info.insert("submission_client".into(), json!(CLIENT_NAME));
    if let Some(dur) = track.duration_secs
        && dur > 0
    {
        additional_info.insert("duration".into(), json!(dur));
    }

    let mut meta = serde_json::Map::new();
    meta.insert("artist_name".into(), json!(artist));
    meta.insert("track_name".into(), json!(title));
    if let Some(ref album) = track.album
        && !album.trim().is_empty()
    {
        meta.insert("release_name".into(), json!(album.trim()));
    }
    meta.insert("additional_info".into(), serde_json::Value::Object(additional_info));

    let mut entry = serde_json::Map::new();
    entry.insert("track_metadata".into(), serde_json::Value::Object(meta));
    if !now_playing {
        entry.insert("listened_at".into(), json!(timestamp.unwrap_or_else(unix_now_secs)));
    }

    json!({
        "listen_type": if now_playing { "playing_now" } else { "single" },
        "payload": [serde_json::Value::Object(entry)]
    })
}

impl Backend {
    pub async fn validate_listenbrainz_token(&self, token: &str) -> Result<String, String> {
        let token = token.trim();
        if token.is_empty() {
            return Err(tr("Ingresa un token de ListenBrainz.", "Enter a ListenBrainz token.").into());
        }

        let resp = self
            .http
            .get(LISTENBRAINZ_VALIDATE_URL)
            .header(reqwest::header::AUTHORIZATION, format!("Token {token}"))
            .timeout(Duration::from_secs(6))
            .send()
            .await
            .map_err(|_| {
                tr(
                    "No se pudo conectar con ListenBrainz.",
                    "Could not connect to ListenBrainz.",
                )
                .to_string()
            })?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|_| "Invalid ListenBrainz response".to_string())?;
        if body.get("valid").and_then(|v| v.as_bool()) == Some(true)
            && let Some(user) = body.get("user_name").and_then(|v| v.as_str())
        {
            return Ok(user.to_string());
        }

        Err(tr(
            "Token de ListenBrainz inválido.",
            "Invalid ListenBrainz token.",
        )
        .to_string())
    }

    pub async fn scrobble_listenbrainz(
        &self,
        token: &str,
        track: &Track,
        now_playing: bool,
        timestamp: Option<u64>,
    ) -> Result<(), String> {
        let token = token.trim();
        if token.is_empty() {
            return Ok(());
        }
        let payload = build_listenbrainz_payload(track, now_playing, timestamp);
        let resp = self
            .http
            .post(LISTENBRAINZ_SUBMIT_URL)
            .header(reqwest::header::AUTHORIZATION, format!("Token {token}"))
            .json(&payload)
            .timeout(Duration::from_secs(6))
            .send()
            .await
            .map_err(|e| format!("ListenBrainz error: {e}"))?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let code = resp.status().as_u16();
            let detail = resp
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v.get("error").and_then(|e| e.as_str()).map(str::to_string));
            match detail {
                Some(err) if !err.trim().is_empty() => Err(format!("ListenBrainz HTTP {code}: {err}")),
                _ => Err(format!("ListenBrainz HTTP {code}")),
            }
        }
    }

    pub async fn scrobble_lastfm(
        &self,
        creds: &LastFmCreds,
        track: &Track,
        now_playing: bool,
        timestamp: Option<u64>,
    ) -> Result<(), String> {
        let api_key = creds.api_key.trim();
        let api_secret = creds.api_secret.trim();
        let session_key = creds.session_key.trim();
        if api_key.is_empty() || api_secret.is_empty() || session_key.is_empty() {
            return Ok(());
        }

        let mut params = BTreeMap::new();
        params.insert(
            "method".into(),
            if now_playing {
                "track.updateNowPlaying".into()
            } else {
                "track.scrobble".into()
            },
        );
        params.insert("api_key".into(), api_key.into());
        params.insert("sk".into(), session_key.into());
        params.insert("artist".into(), clean_scrobble_artist(&track.artist));
        params.insert("track".into(), clean_scrobble_title(&track.title));
        if let Some(ref album) = track.album
            && !album.trim().is_empty()
        {
            params.insert("album".into(), album.clone());
        }
        if let Some(dur) = track.duration_secs
            && dur > 0
        {
            params.insert("duration".into(), dur.to_string());
        }
        if !now_playing {
            params.insert(
                "timestamp".into(),
                timestamp.unwrap_or_else(unix_now_secs).to_string(),
            );
        }

        let api_sig = build_lastfm_signature(&params, api_secret);
        params.insert("api_sig".into(), api_sig);
        params.insert("format".into(), "json".into());

        let form_body = encode_form_urlencoded(&params);
        let resp = self
            .http
            .post(LASTFM_API_URL)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(form_body)
            .timeout(Duration::from_secs(6))
            .send()
            .await
            .map_err(|e| format!("Last.fm error: {e}"))?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("Last.fm HTTP {}", resp.status().as_u16()))
        }
    }
}

fn percent_encode_form(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for &b in input.as_bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else if b == b' ' {
            out.push('+');
        } else {
            use std::fmt::Write;
            let _ = write!(&mut out, "%{b:02X}");
        }
    }
    out
}

fn encode_form_urlencoded(params: &BTreeMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode_form(k), percent_encode_form(v)))
        .collect::<Vec<_>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        build_lastfm_signature, build_listenbrainz_payload, clean_scrobble_artist,
        clean_scrobble_title, md5_hex,
    };
    use crate::models::Track;

    #[test]
    fn md5_matches_rfc1321_vectors() {
        assert_eq!(md5_hex(b""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(md5_hex(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(
            md5_hex(b"message digest"),
            "f96b697d7cb7938d525a2f31aaf161d0"
        );
    }

    #[test]
    fn cleans_scrobble_titles_and_artists() {
        assert_eq!(
            clean_scrobble_title("SALMO 23 feat. Marco Barrientos (feat. Marco Barrientos)"),
            "SALMO 23 feat. Marco Barrientos"
        );
        assert_eq!(
            clean_scrobble_title("Song (feat. Artist) (feat. Artist)"),
            "Song (feat. Artist)"
        );
        assert_eq!(
            clean_scrobble_title("Feel (Official Music Video) [4K]"),
            "Feel"
        );
        assert_eq!(
            clean_scrobble_title("DARE (feat. Shaun Ryder) (Official Audio)"),
            "DARE (feat. Shaun Ryder)"
        );
        assert_eq!(
            clean_scrobble_title("Bohemian Rhapsody (Remastered 2011)"),
            "Bohemian Rhapsody (Remastered 2011)"
        );
        assert_eq!(
            clean_scrobble_title("Hotel California (Live)"),
            "Hotel California (Live)"
        );
        assert_eq!(
            clean_scrobble_artist("Robbie Williams - Topic"),
            "Robbie Williams"
        );
        assert_eq!(
            clean_scrobble_artist("Un Corazón, Kim Richards"),
            "Un Corazón, Kim Richards"
        );
    }

    #[test]
    fn builds_valid_lastfm_signature_and_listenbrainz_payload() {
        let mut params = BTreeMap::new();
        params.insert("api_key".into(), "key123".into());
        params.insert("method".into(), "track.scrobble".into());
        params.insert("format".into(), "json".into());
        assert_eq!(
            build_lastfm_signature(&params, "sec"),
            md5_hex(b"api_keykey123methodtrack.scrobblesec")
        );

        let track = Track {
            is_music: true,
            is_video: false,
            id: "dQw4w9WgXcQ".into(),
            title: "Never Gonna Give You Up (Official Video)".into(),
            artist: "Rick Astley - Topic".into(),
            artist_id: None,
            album: Some("Whenever You Need Somebody".into()),
            album_id: None,
            duration_secs: Some(213),
            view_count: None,
            cover_url: None,
            cover_large_url: None,
        };
        let payload = build_listenbrainz_payload(&track, false, Some(1700000000));
        assert_eq!(payload["listen_type"], "single");
        assert_eq!(payload["payload"][0]["listened_at"], 1700000000);
        let meta = &payload["payload"][0]["track_metadata"];
        assert_eq!(meta["artist_name"], "Rick Astley");
        assert_eq!(meta["track_name"], "Never Gonna Give You Up");
        assert_eq!(meta["additional_info"]["media_player"], "Gyro");
        assert_eq!(meta["additional_info"]["submission_client"], "Gyro");
        assert_eq!(meta["additional_info"]["duration"], 213);
        assert!(meta["additional_info"].get("music_service").is_none());
        assert!(meta["additional_info"].get("origin_url").is_none());

        let playing_now = build_listenbrainz_payload(&track, true, Some(1700000000));
        assert_eq!(playing_now["listen_type"], "playing_now");
        assert!(playing_now["payload"][0].get("listened_at").is_none());
    }
}
