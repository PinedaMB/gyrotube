use std::time::{Duration, Instant};

use ring::digest;
use serde::Deserialize;

use super::Backend;
use super::parsers::validate_video_id;
use crate::models::SponsorSegment;

const SPONSORBLOCK_CACHE_TTL: Duration = Duration::from_secs(30 * 60);
const SPONSORBLOCK_CACHE_MAX_ENTRIES: usize = 256;
const SPONSORBLOCK_API_BASE: &str = "https://sponsor.ajay.app/api/skipSegments";

#[derive(Deserialize)]
struct SbVideoEntry {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(default)]
    segments: Vec<SbSegmentEntry>,
}

#[derive(Deserialize)]
struct SbSegmentEntry {
    segment: [f64; 2],
    #[serde(default)]
    category: String,
    #[serde(rename = "actionType", default)]
    action_type: Option<String>,
}

pub(crate) fn sha256_prefix_4(video_id: &str) -> String {
    let hash = digest::digest(&digest::SHA256, video_id.as_bytes());
    let bytes = hash.as_ref();
    format!("{:02x}{:02x}", bytes[0], bytes[1])
}

pub(crate) fn parse_sponsorblock_response(video_id: &str, body: &str) -> Vec<SponsorSegment> {
    let Ok(entries) = serde_json::from_str::<Vec<SbVideoEntry>>(body) else {
        return Vec::new();
    };
    let mut out: Vec<SponsorSegment> = entries
        .into_iter()
        .filter(|e| e.video_id == video_id)
        .flat_map(|e| e.segments.into_iter())
        .filter(|s| s.action_type.as_deref().unwrap_or("skip") == "skip")
        .filter_map(|s| {
            let [start, end] = s.segment;
            if start.is_finite() && end.is_finite() && start >= 0.0 && end > start + 0.5 {
                Some(SponsorSegment {
                    start,
                    end,
                    category: if s.category.is_empty() {
                        "music_offtopic".to_string()
                    } else {
                        s.category
                    },
                })
            } else {
                None
            }
        })
        .collect();
    out.sort_by(|a, b| a.start.total_cmp(&b.start));
    out
}

impl Backend {
    pub async fn fetch_sponsorblock_segments(
        &self,
        track_id: &str,
    ) -> Result<Vec<SponsorSegment>, String> {
        let track_id = track_id.trim();
        validate_video_id(track_id)?;

        if let Ok(guard) = self.sponsorblock_cache.read()
            && let Some((ts, cached)) = guard.get(track_id)
            && ts.elapsed() < SPONSORBLOCK_CACHE_TTL
        {
            return Ok(cached.clone());
        }

        let prefix = sha256_prefix_4(track_id);
        let url = format!("{SPONSORBLOCK_API_BASE}/{prefix}?categories=%5B%22music_offtopic%22%5D");

        let segments = match self
            .http
            .get(&url)
            .timeout(Duration::from_secs(4))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => match resp.text().await {
                Ok(body) => parse_sponsorblock_response(track_id, &body),
                Err(_) => Vec::new(),
            },
            Ok(resp) if resp.status().as_u16() == 404 => Vec::new(),
            _ => return Ok(Vec::new()),
        };

        if let Ok(mut guard) = self.sponsorblock_cache.write() {
            guard.retain(|_, (ts, _)| ts.elapsed() < SPONSORBLOCK_CACHE_TTL);
            if guard.len() >= SPONSORBLOCK_CACHE_MAX_ENTRIES
                && let Some(oldest) = guard
                    .iter()
                    .min_by_key(|(_, (ts, _))| *ts)
                    .map(|(k, _)| k.clone())
            {
                guard.remove(&oldest);
            }
            guard.insert(track_id.to_string(), (Instant::now(), segments.clone()));
        }

        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_sponsorblock_response, sha256_prefix_4};

    #[test]
    fn sha256_prefix_is_four_lowercase_hex_chars() {
        let p = sha256_prefix_4("dQw4w9WgXcQ");
        assert_eq!(p.len(), 4);
        assert!(p.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }

    #[test]
    fn parses_and_filters_only_matching_skip_segments() {
        let payload = r#"[
            {
                "videoID": "other123456",
                "segments": [
                    { "segment": [0.0, 20.0], "category": "music_offtopic", "actionType": "skip" }
                ]
            },
            {
                "videoID": "dQw4w9WgXcQ",
                "segments": [
                    { "segment": [180.5, 210.0], "category": "music_offtopic", "actionType": "skip" },
                    { "segment": [0.0, 14.2], "category": "music_offtopic", "actionType": "skip" },
                    { "segment": [20.0, 20.2], "category": "music_offtopic", "actionType": "skip" },
                    { "segment": [50.0, 60.0], "category": "music_offtopic", "actionType": "mute" }
                ]
            }
        ]"#;

        let segs = parse_sponsorblock_response("dQw4w9WgXcQ", payload);
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0].start, 0.0);
        assert_eq!(segs[0].end, 14.2);
        assert_eq!(segs[1].start, 180.5);
        assert_eq!(segs[1].end, 210.0);
    }
}
