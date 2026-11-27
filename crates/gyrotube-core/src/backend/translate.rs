use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde_json::Value;

use super::Backend;

const CLIENTS5_TRANSLATE_URL: &str =
    "https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl=auto";
const GTX_TRANSLATE_URL: &str =
    "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&dt=t";
const BROWSER_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const TRANSLATION_CACHE_TTL: Duration = Duration::from_secs(30 * 60);
const TRANSLATION_CACHE_MAX_ENTRIES: usize = 128;

fn percent_encode(input: &str) -> String {
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

fn is_same_lang(detected: &str, target_lang: &str) -> bool {
    let d = detected.trim().to_ascii_lowercase();
    let t = target_lang.trim().to_ascii_lowercase();
    let d_base = d.split(['-', '_']).next().unwrap_or(&d);
    let t_base = t.split(['-', '_']).next().unwrap_or(&t);
    !d_base.is_empty() && d_base == t_base
}

/// Parses `clients5.google.com/translate_a/t`: `[["translated text", "detected_lang"]]`
pub(crate) fn parse_clients5_response(body: &str, target_lang: &str) -> Option<(String, String)> {
    let root: Value = serde_json::from_str(body).ok()?;
    let arr = root.as_array()?;
    let first = arr.first()?;
    if let Some(pair) = first.as_array() {
        let text = pair.first()?.as_str()?.to_string();
        let detected = pair
            .get(1)
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_ascii_lowercase();
        if is_same_lang(&detected, target_lang) {
            return None;
        }
        return Some((detected, text));
    }
    if let Some(text) = first.as_str() {
        return Some((String::new(), text.to_string()));
    }
    None
}

/// Parses `translate.googleapis.com/translate_a/single`: `[[["tr", "orig", ...]], null, "lang"]`
pub(crate) fn parse_google_translate_response(
    body: &str,
    target_lang: &str,
) -> Option<(String, String)> {
    let root: Value = serde_json::from_str(body).ok()?;
    let arr = root.as_array()?;
    let detected = arr
        .get(2)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_ascii_lowercase();
    if is_same_lang(&detected, target_lang) {
        return None;
    }

    let segments = arr.first()?.as_array()?;
    let mut translated = String::new();
    for seg in segments {
        if let Some(chunk) = seg.as_array().and_then(|a| a.first()).and_then(Value::as_str) {
            translated.push_str(chunk);
        }
    }
    Some((detected, translated))
}

impl Backend {
    async fn post_translate_request(
        &self,
        url: &str,
        body_form: &str,
    ) -> Result<(u16, String), String> {
        let send_once = |client: &reqwest::Client| {
            client
                .post(url)
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded; charset=utf-8",
                )
                .header(reqwest::header::USER_AGENT, BROWSER_UA)
                .body(body_form.to_string())
                .timeout(Duration::from_secs(10))
                .send()
        };

        let resp = match send_once(&self.http).await {
            Ok(r) => r,
            Err(first_err) => {
                let ipv4_client = crate::tls::http_client_builder()
                    .local_address(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED))
                    .connect_timeout(Duration::from_secs(5))
                    .timeout(Duration::from_secs(10))
                    .build();
                match ipv4_client {
                    Ok(c) => send_once(&c)
                        .await
                        .map_err(|e| format!("Translation network error: {e}"))?,
                    Err(_) => return Err(format!("Translation network error: {first_err}")),
                }
            }
        };

        let status = resp.status().as_u16();
        let text = resp
            .text()
            .await
            .map_err(|e| format!("Translation read error: {e}"))?;
        Ok((status, text))
    }

    pub async fn translate_lyrics_lines(
        &self,
        cache_key: &str,
        lines: &[String],
        target_lang: &str,
    ) -> Result<Vec<String>, String> {
        if lines.is_empty() {
            return Ok(Vec::new());
        }
        let target = target_lang.trim().to_ascii_lowercase();
        let target = if target.is_empty() { "es" } else { target.as_str() };
        let full_key = (cache_key.to_string(), target.to_string());

        if !cache_key.is_empty()
            && let Ok(guard) = self.translation_cache.read()
            && let Some((ts, cached)) = guard.get(&full_key)
            && ts.elapsed() < TRANSLATION_CACHE_TTL
            && cached.len() == lines.len()
        {
            return Ok(cached.clone());
        }

        // Deduplicate unique non-empty lines to minimize payload size & avoid HTTP 429 rate limits
        let mut unique_lines: Vec<String> = Vec::new();
        let mut line_to_unique: HashMap<String, usize> = HashMap::new();
        let mut index_map: Vec<Option<usize>> = Vec::with_capacity(lines.len());
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                index_map.push(None);
                continue;
            }
            if let Some(&u_idx) = line_to_unique.get(trimmed) {
                index_map.push(Some(u_idx));
            } else {
                let u_idx = unique_lines.len();
                unique_lines.push(trimmed.to_string());
                line_to_unique.insert(trimmed.to_string(), u_idx);
                index_map.push(Some(u_idx));
            }
        }

        let mut unique_translated: Vec<String> = Vec::with_capacity(unique_lines.len());
        let mut idx = 0;
        let mut any_different = false;

        while idx < unique_lines.len() {
            let mut batch: Vec<&str> = Vec::new();
            let mut chars = 0usize;
            while idx < unique_lines.len() {
                let line = unique_lines[idx].as_str();
                if !batch.is_empty() && chars + line.len() + 1 > 1400 {
                    break;
                }
                batch.push(line);
                chars += line.len() + 1;
                idx += 1;
            }

            let joined = batch.join("\n");
            let body_form = format!("q={}", percent_encode(&joined));

            // 1. Primary: clients5.google.com (dict-chrome-ex)
            let c5_url = format!("{CLIENTS5_TRANSLATE_URL}&tl={}", percent_encode(target));
            let parsed = match self.post_translate_request(&c5_url, &body_form).await {
                Ok((200..=299, text)) => Ok(parse_clients5_response(&text, target)),
                Ok((status, _)) => {
                    // 2. Fallback: translate.googleapis.com (gtx) with browser UA
                    let gtx_url = format!("{GTX_TRANSLATE_URL}&tl={}", percent_encode(target));
                    match self.post_translate_request(&gtx_url, &body_form).await {
                        Ok((200..=299, text)) => Ok(parse_google_translate_response(&text, target)),
                        Ok((fb_status, _)) => Err(format!("Translation HTTP {fb_status}")),
                        Err(_) => Err(format!("Translation HTTP {status}")),
                    }
                }
                Err(err) => Err(err),
            }?;

            if let Some((_, translated_block)) = parsed {
                let split: Vec<&str> = translated_block.split('\n').collect();
                for (i, orig) in batch.iter().enumerate() {
                    let tr_line = split.get(i).copied().unwrap_or("").trim();
                    if !tr_line.is_empty() && !tr_line.eq_ignore_ascii_case(orig.trim()) {
                        any_different = true;
                        unique_translated.push(tr_line.to_string());
                    } else {
                        unique_translated.push(String::new());
                    }
                }
            } else {
                for _ in &batch {
                    unique_translated.push(String::new());
                }
            }
        }

        let translated_out: Vec<String> = if any_different {
            index_map
                .into_iter()
                .map(|opt| {
                    opt.and_then(|u| unique_translated.get(u).cloned())
                        .unwrap_or_default()
                })
                .collect()
        } else {
            vec![String::new(); lines.len()]
        };

        if !cache_key.is_empty()
            && let Ok(mut guard) = self.translation_cache.write()
        {
            guard.retain(|_, (ts, _)| ts.elapsed() < TRANSLATION_CACHE_TTL);
            if guard.len() >= TRANSLATION_CACHE_MAX_ENTRIES
                && let Some(oldest) = guard
                    .iter()
                    .min_by_key(|(_, (ts, _))| *ts)
                    .map(|(k, _)| k.clone())
            {
                guard.remove(&oldest);
            }
            guard.insert(full_key, (Instant::now(), translated_out.clone()));
        }

        Ok(translated_out)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_clients5_response, parse_google_translate_response};

    #[test]
    fn parses_clients5_response_and_skips_same_language() {
        let sample = r#"[["tienes\nme tienes","de"]]"#;
        let (lang, text) = parse_clients5_response(sample, "es").expect("should translate de -> es");
        assert_eq!(lang, "de");
        assert_eq!(text, "tienes\nme tienes");
        assert!(parse_clients5_response(sample, "de").is_none());
    }

    #[test]
    fn parses_google_translate_segments_and_skips_same_language() {
        let sample = r#"[[["Tú tienes\n","Du hast\n",null,null,1],["Tú me tienes","Du hast mich",null,null,1]],null,"de"]"#;
        let (lang, text) =
            parse_google_translate_response(sample, "es").expect("should translate de -> es");
        assert_eq!(lang, "de");
        assert_eq!(text, "Tú tienes\nTú me tienes");
        assert!(parse_google_translate_response(sample, "de").is_none());
    }
}
