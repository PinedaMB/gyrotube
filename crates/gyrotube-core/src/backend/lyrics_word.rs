use std::time::Duration;

use serde_json::Value;

use super::Backend;
use crate::models::{LyricWord, SyncedLyricLine, Track};

const BETTER_LYRICS_URL: &str = "https://lyrics-api.boidu.dev/getLyrics";
const LYRICSPLUS_MIRRORS: [&str; 2] = [
    "https://lyricsplus.prjktla.my.id/v2/lyrics/get",
    "https://lyricsplus.binimum.org/v2/lyrics/get",
];

fn value_as_f64(val: Option<&Value>) -> Option<f64> {
    match val? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

pub(crate) fn parse_ttml_timestamp(raw: &str) -> Option<f64> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(secs_str) = s.strip_suffix('s')
        && !secs_str.ends_with('m')
    {
        return secs_str.trim().parse::<f64>().ok();
    }
    if let Some(ms_str) = s.strip_suffix("ms") {
        return ms_str.trim().parse::<f64>().ok().map(|v| v / 1000.0);
    }
    let parts: Vec<&str> = s.split(':').collect();
    match parts.len() {
        1 => parts[0].parse::<f64>().ok(),
        2 => {
            let mins = parts[0].parse::<f64>().ok()?;
            let secs = parts[1].parse::<f64>().ok()?;
            Some(mins * 60.0 + secs)
        }
        3 => {
            let hours = parts[0].parse::<f64>().ok()?;
            let mins = parts[1].parse::<f64>().ok()?;
            let secs = parts[2].parse::<f64>().ok()?;
            Some(hours * 3600.0 + mins * 60.0 + secs)
        }
        _ => None,
    }
}

fn decode_xml_entities(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}

fn extract_xml_attr<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
    for quote in ['"', '\''] {
        let needle = format!("{name}={quote}");
        if let Some(pos) = attrs.find(&needle) {
            let after = &attrs[pos + needle.len()..];
            if let Some(end) = after.find(quote) {
                return Some(&after[..end]);
            }
        }
    }
    None
}

fn strip_xml_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    decode_xml_entities(&out)
}

pub(crate) fn parse_ttml(xml: &str) -> Option<Vec<SyncedLyricLine>> {
    let mut lines = Vec::new();
    let mut has_word_sync = false;
    let mut cursor = xml;

    while let Some(p_idx) = cursor.find("<p") {
        let after_p = &cursor[p_idx + 2..];
        // Ensure it's `<p ` or `<p>` (not `<param` etc.)
        if !after_p.starts_with(|c: char| c.is_whitespace() || c == '>') {
            cursor = after_p;
            continue;
        }
        let Some(tag_end) = after_p.find('>') else {
            break;
        };
        let p_attrs = &after_p[..tag_end];
        let body_start = &after_p[tag_end + 1..];
        let Some(close_idx) = body_start.find("</p>") else {
            break;
        };
        let p_body = &body_start[..close_idx];
        cursor = &body_start[close_idx + 4..];

        let line_begin = extract_xml_attr(p_attrs, "begin").and_then(parse_ttml_timestamp);
        let line_end = extract_xml_attr(p_attrs, "end").and_then(parse_ttml_timestamp);

        let mut words: Vec<LyricWord> = Vec::new();
        let mut span_cursor = p_body;
        let mut full_text = String::new();

        while let Some(s_idx) = span_cursor.find("<span") {
            let between = &span_cursor[..s_idx];
            if !between.is_empty() && !words.is_empty() {
                let plain_between = strip_xml_tags(between);
                if plain_between.chars().all(|c| c.is_whitespace())
                    && !plain_between.is_empty()
                    && let Some(last) = words.last_mut()
                    && !last.text.ends_with(' ')
                {
                    last.text.push(' ');
                    full_text.push(' ');
                }
            }

            let after_s = &span_cursor[s_idx + 5..];
            let Some(s_tag_end) = after_s.find('>') else {
                break;
            };
            let s_attrs = &after_s[..s_tag_end];
            let s_body_start = &after_s[s_tag_end + 1..];
            let Some(s_close_idx) = s_body_start.find("</span>") else {
                break;
            };
            let s_body = &s_body_start[..s_close_idx];
            span_cursor = &s_body_start[s_close_idx + 7..];

            // Skip nested container spans (e.g., ttm:role="x-bg" wrapping timed child spans)
            if s_body.contains("<span") {
                continue;
            }

            let begin = extract_xml_attr(s_attrs, "begin").and_then(parse_ttml_timestamp);
            let end = extract_xml_attr(s_attrs, "end").and_then(parse_ttml_timestamp);
            let word_text = decode_xml_entities(s_body);

            if let Some(start_secs) = begin
                && !word_text.is_empty()
            {
                let end_secs = end.unwrap_or(start_secs + 0.35).max(start_secs + 0.05);
                full_text.push_str(&word_text);
                words.push(LyricWord {
                    start_secs,
                    end_secs,
                    text: word_text,
                });
            }
        }

        if !words.is_empty() {
            has_word_sync = true;
        }

        let text = if !full_text.trim().is_empty() {
            full_text.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            strip_xml_tags(p_body)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        };
        if text.is_empty() {
            continue;
        }

        let Some(time_secs) = line_begin.or_else(|| words.first().map(|w| w.start_secs)) else {
            continue;
        };
        let end_secs = line_end.or_else(|| words.last().map(|w| w.end_secs));

        lines.push(SyncedLyricLine {
            time_secs,
            end_secs,
            text,
            words,
            romanized: None,
        });
    }

    if has_word_sync && !lines.is_empty() {
        lines.sort_by(|a, b| a.time_secs.total_cmp(&b.time_secs));
        Some(lines)
    } else {
        None
    }
}

pub(crate) fn parse_better_lyrics_response(body: &str) -> Option<Vec<SyncedLyricLine>> {
    let trimmed = body.trim();
    if trimmed.starts_with('<') {
        return parse_ttml(trimmed);
    }
    let val: Value = serde_json::from_str(trimmed).ok()?;
    if let Some(ttml) = val.get("ttml").and_then(Value::as_str) {
        return parse_ttml(ttml);
    }
    parse_lyricsplus_v2(trimmed)
}

pub(crate) fn parse_lyricsplus_v2(body: &str) -> Option<Vec<SyncedLyricLine>> {
    let root: Value = serde_json::from_str(body).ok()?;
    let arr = root.get("lyrics").and_then(Value::as_array)?;
    if arr.is_empty() {
        return None;
    }

    let kind = root.get("type").and_then(Value::as_str).unwrap_or("");
    let mut has_word_sync = false;
    let mut lines = Vec::with_capacity(arr.len());

    for item in arr {
        let raw_tokens = item
            .get("syllabus")
            .and_then(Value::as_array)
            .filter(|a| !a.is_empty())
            .or_else(|| item.get("words").and_then(Value::as_array));

        let mut words = Vec::new();
        let mut rebuilt = String::new();

        if let Some(tokens) = raw_tokens {
            let total = tokens.len();
            for (idx, tok) in tokens.iter().enumerate() {
                let raw_text = tok.get("text").and_then(Value::as_str).unwrap_or("");
                if raw_text.is_empty() {
                    continue;
                }
                let start_ms = value_as_f64(tok.get("time"))
                    .or_else(|| value_as_f64(tok.get("startTime")))
                    .unwrap_or(0.0);
                let dur_ms = value_as_f64(tok.get("duration")).unwrap_or_else(|| {
                    value_as_f64(tok.get("endTime"))
                        .map(|end| (end - start_ms).max(60.0))
                        .unwrap_or(280.0)
                });

                let mut text = raw_text.to_string();
                let is_part = tok
                    .get("isPartOfWord")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                // Add trailing space between separate words if the provider omitted whitespace
                if !is_part && idx + 1 < total && !text.ends_with(char::is_whitespace) {
                    let next_text = tokens[idx + 1]
                        .get("text")
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    if !next_text.starts_with(char::is_whitespace)
                        && !next_text.starts_with([',', '.', '!', '?', ';', ':', ')', ']'])
                    {
                        text.push(' ');
                    }
                }

                let start_secs = start_ms / 1000.0;
                let end_secs = start_secs + (dur_ms / 1000.0).max(0.06);
                rebuilt.push_str(&text);
                words.push(LyricWord {
                    start_secs,
                    end_secs,
                    text,
                });
            }
        }

        if !words.is_empty() {
            has_word_sync = true;
        }

        let item_text = item
            .get("text")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or("");
        let text = if !item_text.is_empty() {
            item_text.to_string()
        } else {
            rebuilt.trim().to_string()
        };
        if text.is_empty() {
            continue;
        }

        let time_ms = value_as_f64(item.get("time"))
            .or_else(|| value_as_f64(item.get("startTime")))
            .unwrap_or_else(|| words.first().map(|w| w.start_secs * 1000.0).unwrap_or(0.0));
        let dur_ms = value_as_f64(item.get("duration")).unwrap_or(0.0);
        let time_secs = time_ms / 1000.0;
        let end_secs = if dur_ms > 0.0 {
            Some((time_ms + dur_ms) / 1000.0)
        } else {
            words.last().map(|w| w.end_secs)
        };

        lines.push(SyncedLyricLine {
            time_secs,
            end_secs,
            text,
            words,
            romanized: None,
        });
    }

    if (has_word_sync || kind.eq_ignore_ascii_case("Word")) && !lines.is_empty() {
        lines.sort_by(|a, b| a.time_secs.total_cmp(&b.time_secs));
        Some(lines)
    } else {
        None
    }
}

impl Backend {
    async fn fetch_better_lyrics_ttml(
        &self,
        track: &Track,
        artist_q: &str,
        clean_title: &str,
    ) -> Option<Vec<SyncedLyricLine>> {
        let mut params = vec![
            ("s", clean_title.to_string()),
            ("a", artist_q.to_string()),
        ];
        if let Some(dur) = track.duration_secs
            && dur > 0
        {
            params.push(("d", dur.to_string()));
        }

        let resp = self
            .http
            .get(BETTER_LYRICS_URL)
            .query(&params)
            .header(reqwest::header::USER_AGENT, "GyroTube/1.0")
            .timeout(Duration::from_secs(4))
            .send()
            .await
            .ok()?;

        if !resp.status().is_success() {
            return None;
        }
        let body = resp.text().await.ok()?;
        parse_better_lyrics_response(&body)
    }

    async fn fetch_lyricsplus_word_lyrics(
        &self,
        track: &Track,
        artist_q: &str,
        clean_title: &str,
    ) -> Option<Vec<SyncedLyricLine>> {
        let mut params = vec![
            ("title", clean_title.to_string()),
            ("artist", artist_q.to_string()),
        ];
        if let Some(dur) = track.duration_secs
            && dur > 0
        {
            params.push(("duration", dur.to_string()));
        }

        for mirror in LYRICSPLUS_MIRRORS {
            let Ok(resp) = self
                .http
                .get(mirror)
                .query(&params)
                .header(reqwest::header::USER_AGENT, "GyroTube/1.0")
                .timeout(Duration::from_secs(3))
                .send()
                .await
            else {
                continue;
            };
            if !resp.status().is_success() {
                continue;
            }
            if let Ok(body) = resp.text().await
                && let Some(lines) = parse_lyricsplus_v2(&body)
            {
                return Some(lines);
            }
        }
        None
    }

    pub(crate) async fn fetch_word_synced_lyrics(
        &self,
        track: &Track,
        artist_q: &str,
        clean_title: &str,
    ) -> Option<Vec<SyncedLyricLine>> {
        if clean_title.trim().is_empty() {
            return None;
        }

        // Query Better Lyrics (TTML) and LyricsPlus V2 in parallel for maximum coverage & speed
        let (bl_res, lp_res) = tokio::join!(
            self.fetch_better_lyrics_ttml(track, artist_q, clean_title),
            self.fetch_lyricsplus_word_lyrics(track, artist_q, clean_title)
        );
        bl_res.or(lp_res)
    }
}

/// Ensures every synced line has word-level timing:
/// - Preserves real word/syllable timestamps whenever provided by Better Lyrics / LyricsPlus / ELRC.
/// - For line-only sources (YTM, LRCLIB, Apple Line), distributes sequential non-overlapping
///   per-word timing weighted by character length over a brisk sweep window (~0.75–1.85 s)
///   so the line sweeps continuously from left to right without overlapping words or dragging.
pub(crate) fn ensure_word_timings(lines: &mut [SyncedLyricLine]) {
    let total_lines = lines.len();
    for i in 0..total_lines {
        if !lines[i].words.is_empty() {
            continue;
        }
        let start = lines[i].time_secs;
        let next_start = lines.get(i + 1).map(|l| l.time_secs);
        let char_count = lines[i].text.chars().count().max(1) as f64;
        let est_dur = (char_count * 0.075).clamp(1.2, 5.2);

        let end = match lines[i].end_secs {
            Some(e) if e > start + 0.25 => e,
            _ => match next_start {
                Some(ns) if ns > start + 0.25 => (start + est_dur).min(ns - 0.05).max(start + 0.25),
                _ => start + est_dur,
            },
        };
        lines[i].end_secs = Some(end);

        let tokens: Vec<&str> = lines[i].text.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let span = (end - start).max(0.25);
        // Sequential sweep window: brisk left-to-right progression without waiting the full line span
        let sweep_span = (char_count * 0.045)
            .clamp(0.75, 1.85)
            .min(span * 0.85)
            .max(0.25);
        let sweep_end = start + sweep_span;

        let weights: Vec<f64> = tokens
            .iter()
            .map(|w| (w.chars().count() as f64).max(1.0) + 0.4)
            .collect();
        let total_weight: f64 = weights.iter().sum();

        let count = tokens.len();
        let mut cursor = start;
        let mut words = Vec::with_capacity(count);
        for (idx, (tok, w)) in tokens.iter().zip(weights.iter()).enumerate() {
            let dur = (sweep_span * (*w / total_weight)).max(0.05);
            let word_end = if idx + 1 == count {
                sweep_end.max(cursor + 0.05)
            } else {
                (cursor + dur).min(sweep_end)
            };
            let mut text = (*tok).to_string();
            if idx + 1 < count {
                text.push(' ');
            }
            words.push(LyricWord {
                start_secs: cursor,
                end_secs: word_end,
                text,
            });
            cursor = word_end;
        }
        lines[i].words = words;
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_better_lyrics_response, parse_lyricsplus_v2, parse_ttml};

    #[test]
    fn parses_word_level_syllabus_from_lyricsplus_v2() {
        let json = r#"{
            "type": "Word",
            "lyrics": [
                {
                    "time": 12000.5,
                    "duration": 2400.0,
                    "text": "Hello world",
                    "syllabus": [
                        { "time": 12000.5, "duration": 900, "text": "Hello" },
                        { "time": 13000, "duration": 1400, "text": "world" }
                    ]
                }
            ]
        }"#;
        let lines = parse_lyricsplus_v2(json).expect("should parse word-synced lyrics");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].words.len(), 2);
        assert_eq!(lines[0].words[0].text, "Hello ");
        assert_eq!(lines[0].words[1].text, "world");
    }

    #[test]
    fn parses_apple_music_ttml_with_preserved_spaces_and_entities() {
        let ttml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <tt xmlns="http://www.w3.org/ns/ttml">
          <body>
            <div>
              <p begin="00:03.520" end="00:07.120">
                <span begin="00:03.520" end="00:04.100">You&apos;re</span>
                <span begin="00:04.100" end="00:04.450">the</span>
                <span begin="00:04.450" end="00:05.100">light</span>
              </p>
            </div>
          </body>
        </tt>"#;
        let lines = parse_ttml(ttml).expect("should parse TTML");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "You're the light");
        assert_eq!(lines[0].words.len(), 3);
        assert_eq!(lines[0].words[0].text, "You're ");
        assert_eq!(lines[0].words[2].text, "light");
    }

    #[test]
    fn parses_better_lyrics_json_wrapped_ttml() {
        let json = r#"{"ttml":"<tt><body><div><p begin=\"0:01.000\" end=\"0:03.000\"><span begin=\"0:01.000\" end=\"0:02.000\">Hi</span> <span begin=\"0:02.000\" end=\"0:03.000\">there</span></p></div></body></tt>"}"#;
        let lines = parse_better_lyrics_response(json).expect("should unwrap TTML");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].words.len(), 2);
    }

    #[test]
    fn rejects_line_only_payload_so_ytm_stays_primary_for_line_sync() {
        let json = r#"{
            "type": "Line",
            "lyrics": [
                { "time": 1000, "duration": 2000, "text": "Plain line" }
            ]
        }"#;
        assert!(parse_lyricsplus_v2(json).is_none());
    }
}
