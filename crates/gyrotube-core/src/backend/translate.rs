use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde_json::Value;

use super::Backend;

const CLIENTS5_TRANSLATE_URL: &str =
    "https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl=auto";
const GTX_TRANSLATE_URL: &str =
    "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&dt=t";
const MOBILE_TRANSLATE_URL: &str =
    "https://translate.google.com/translate_a/single?client=at&sl=auto&dt=t";
const MYMEMORY_TRANSLATE_URL: &str = "https://api.mymemory.translated.net/get";
const BROWSER_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const ANDROID_TRANSLATE_UA: &str = "GoogleTranslate/6.29.59279 (Linux; U; Android 13; Pixel 7)";
const TRANSLATION_CACHE_TTL: Duration = Duration::from_secs(30 * 60);
const TRANSLATION_CACHE_MAX_ENTRIES: usize = 128;
const GOOGLE_BATCH_MAX_CHARS: usize = 2600;
const MYMEMORY_BATCH_MAX_CHARS: usize = 450;

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

/// Parses `api.mymemory.translated.net/get`:
/// `{"responseData":{"translatedText":"...","detectedLanguage":"de"},"quotaFinished":false,"responseStatus":200}`
pub(crate) fn parse_mymemory_response(body: &str, target_lang: &str) -> Result<Option<(String, String)>, String> {
    let root: Value =
        serde_json::from_str(body).map_err(|e| format!("MyMemory parse error: {e}"))?;
    if root
        .get("quotaFinished")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err("MyMemory quota finished".to_string());
    }
    let status = root
        .get("responseStatus")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(200);
    if !(200..=299).contains(&status) {
        return Err(format!("MyMemory HTTP {status}"));
    }
    let data = root
        .get("responseData")
        .ok_or_else(|| "MyMemory missing responseData".to_string())?;
    let detected = data
        .get("detectedLanguage")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_ascii_lowercase();
    if is_same_lang(&detected, target_lang) {
        return Ok(None);
    }
    let text = data
        .get("translatedText")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    if text.is_empty() {
        return Ok(None);
    }
    Ok(Some((detected, text)))
}

fn build_ipv4_client() -> Option<reqwest::Client> {
    crate::tls::http_client_builder()
        .local_address(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED))
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()
        .ok()
}

impl Backend {
    async fn post_translate_request(
        &self,
        url: &str,
        body_form: &str,
        user_agent: &str,
    ) -> Result<(u16, String), String> {
        let send_once = |client: &reqwest::Client| {
            client
                .post(url)
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded; charset=utf-8",
                )
                .header(reqwest::header::USER_AGENT, user_agent)
                .body(body_form.to_string())
                .timeout(Duration::from_secs(10))
                .send()
        };

        let resp = match send_once(&self.http).await {
            Ok(r) if r.status().as_u16() == 429 => {
                // Retry once over IPv4 in case the ISP IPv6 prefix is rate-limited
                if let Some(c) = build_ipv4_client()
                    && let Ok(v4_resp) = send_once(&c).await
                {
                    v4_resp
                } else {
                    r
                }
            }
            Ok(r) => r,
            Err(first_err) => match build_ipv4_client() {
                Some(c) => send_once(&c)
                    .await
                    .map_err(|e| format!("Translation network error: {e}"))?,
                None => return Err(format!("Translation network error: {first_err}")),
            },
        };

        let status = resp.status().as_u16();
        let text = resp
            .text()
            .await
            .map_err(|e| format!("Translation read error: {e}"))?;
        Ok((status, text))
    }

    async fn get_translate_request(&self, url: &str) -> Result<(u16, String), String> {
        let send_once = |client: &reqwest::Client| {
            client
                .get(url)
                .header(reqwest::header::USER_AGENT, BROWSER_UA)
                .timeout(Duration::from_secs(10))
                .send()
        };
        let resp = match send_once(&self.http).await {
            Ok(r) => r,
            Err(first_err) => match build_ipv4_client() {
                Some(c) => send_once(&c)
                    .await
                    .map_err(|e| format!("Translation network error: {e}"))?,
                None => return Err(format!("Translation network error: {first_err}")),
            },
        };
        let status = resp.status().as_u16();
        let text = resp
            .text()
            .await
            .map_err(|e| format!("Translation read error: {e}"))?;
        Ok((status, text))
    }

    async fn translate_batch_via_mymemory(
        &self,
        batch: &[&str],
        target: &str,
    ) -> Result<Option<(String, String)>, String> {
        let mut combined_lines: Vec<String> = Vec::with_capacity(batch.len());
        let mut detected_lang = String::new();
        let mut sub_idx = 0;

        while sub_idx < batch.len() {
            let mut sub: Vec<&str> = Vec::new();
            let mut sub_chars = 0usize;
            while sub_idx < batch.len() {
                let line = batch[sub_idx];
                if !sub.is_empty() && sub_chars + line.len() + 1 > MYMEMORY_BATCH_MAX_CHARS {
                    break;
                }
                sub.push(line);
                sub_chars += line.len() + 1;
                sub_idx += 1;
            }

            let joined = sub.join("\n");
            let url = format!(
                "{MYMEMORY_TRANSLATE_URL}?q={}&langpair=autodetect|{}",
                percent_encode(&joined),
                percent_encode(target)
            );
            let (status, text) = self.get_translate_request(&url).await?;
            if !(200..=299).contains(&status) {
                return Err(format!("Translation HTTP {status}"));
            }
            match parse_mymemory_response(&text, target)? {
                Some((lang, translated_block)) => {
                    if detected_lang.is_empty() && !lang.is_empty() {
                        detected_lang = lang;
                    }
                    let split: Vec<&str> = translated_block.split('\n').collect();
                    for i in 0..sub.len() {
                        combined_lines.push(split.get(i).copied().unwrap_or("").to_string());
                    }
                }
                None => {
                    for _ in 0..sub.len() {
                        combined_lines.push(String::new());
                    }
                }
            }

            if sub_idx < batch.len() {
                tokio::time::sleep(Duration::from_millis(80)).await;
            }
        }

        if combined_lines.iter().all(|s| s.trim().is_empty()) {
            Ok(None)
        } else {
            Ok(Some((detected_lang, combined_lines.join("\n"))))
        }
    }

    async fn translate_batch(
        &self,
        batch: &[&str],
        target: &str,
        prefer_mymemory: &mut bool,
    ) -> Result<Option<(String, String)>, String> {
        if *prefer_mymemory {
            return self.translate_batch_via_mymemory(batch, target).await;
        }

        let joined = batch.join("\n");
        let body_form = format!("q={}", percent_encode(&joined));
        let enc_target = percent_encode(target);

        // 1. Primary: clients5.google.com (dict-chrome-ex)
        let c5_url = format!("{CLIENTS5_TRANSLATE_URL}&tl={enc_target}");
        if let Ok((200..=299, text)) = self
            .post_translate_request(&c5_url, &body_form, BROWSER_UA)
            .await
        {
            return Ok(parse_clients5_response(&text, target));
        }

        // 2. Fallback: translate.googleapis.com (gtx)
        let gtx_url = format!("{GTX_TRANSLATE_URL}&tl={enc_target}");
        if let Ok((200..=299, text)) = self
            .post_translate_request(&gtx_url, &body_form, BROWSER_UA)
            .await
        {
            return Ok(parse_google_translate_response(&text, target));
        }

        // 3. Fallback: translate.google.com (client=at, Android UA — separate rate-limit bucket)
        let mobile_url = format!("{MOBILE_TRANSLATE_URL}&tl={enc_target}");
        let last_err = match self
            .post_translate_request(&mobile_url, &body_form, ANDROID_TRANSLATE_UA)
            .await
        {
            Ok((200..=299, text)) => return Ok(parse_google_translate_response(&text, target)),
            Ok((status, _)) => format!("Translation HTTP {status}"),
            Err(err) => err,
        };

        // 4. Independent non-Google fallback: MyMemory
        match self.translate_batch_via_mymemory(batch, target).await {
            Ok(res) => {
                *prefer_mymemory = true;
                Ok(res)
            }
            Err(_) => Err(last_err),
        }
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
        let mut prefer_mymemory = false;

        while idx < unique_lines.len() {
            if idx > 0 {
                tokio::time::sleep(Duration::from_millis(120)).await;
            }
            let mut batch: Vec<&str> = Vec::new();
            let mut chars = 0usize;
            while idx < unique_lines.len() {
                let line = unique_lines[idx].as_str();
                if !batch.is_empty() && chars + line.len() + 1 > GOOGLE_BATCH_MAX_CHARS {
                    break;
                }
                batch.push(line);
                chars += line.len() + 1;
                idx += 1;
            }

            let parsed = self
                .translate_batch(&batch, target, &mut prefer_mymemory)
                .await?;

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
    use super::{parse_clients5_response, parse_google_translate_response, parse_mymemory_response};

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

    #[test]
    fn parses_mymemory_response_and_handles_same_lang_and_quota() {
        let sample = r#"{"responseData":{"translatedText":"Me lo has preguntado\ny no dije nada","detectedLanguage":"de"},"quotaFinished":false,"responseStatus":200}"#;
        let (lang, text) = parse_mymemory_response(sample, "es")
            .expect("valid json")
            .expect("should translate de -> es");
        assert_eq!(lang, "de");
        assert_eq!(text, "Me lo has preguntado\ny no dije nada");
        assert!(parse_mymemory_response(sample, "de").unwrap().is_none());

        let quota = r#"{"responseData":{"translatedText":""},"quotaFinished":true,"responseStatus":200}"#;
        assert!(parse_mymemory_response(quota, "es").is_err());
    }
}
