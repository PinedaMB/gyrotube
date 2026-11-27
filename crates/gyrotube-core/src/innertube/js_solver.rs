#[cfg(feature = "js-solver")]
use std::collections::HashMap;
use std::sync::OnceLock;
#[cfg(feature = "js-solver")]
use std::sync::RwLock;
#[cfg(feature = "js-solver")]
use std::time::{Duration, Instant};

#[cfg(feature = "js-solver")]
const PLAYER_JS_TTL: Duration = Duration::from_secs(6 * 60 * 60);
#[cfg(feature = "js-solver")]
const WEB_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36";

#[cfg(feature = "js-solver")]
struct CachedPlayerJs {
    fetched_at: Instant,
    source: String,
}

pub struct JsChallengeSolver {
    #[cfg(feature = "js-solver")]
    player_js: RwLock<Option<CachedPlayerJs>>,
    #[cfg(feature = "js-solver")]
    n_cache: RwLock<HashMap<String, String>>,
}

static GLOBAL_SOLVER: OnceLock<JsChallengeSolver> = OnceLock::new();

impl JsChallengeSolver {
    pub fn global() -> &'static Self {
        GLOBAL_SOLVER.get_or_init(|| Self {
            #[cfg(feature = "js-solver")]
            player_js: RwLock::new(None),
            #[cfg(feature = "js-solver")]
            n_cache: RwLock::new(HashMap::new()),
        })
    }

    pub async fn decipher_format_url(
        &self,
        http: &reqwest::Client,
        direct_url: Option<&str>,
        signature_cipher: Option<&str>,
        transform_n: bool,
    ) -> Result<String, String> {
        #[cfg(not(feature = "js-solver"))]
        {
            let _ = (http, signature_cipher, transform_n);
            return direct_url
                .map(str::to_string)
                .ok_or_else(|| "JS solver feature is disabled".to_string());
        }

        #[cfg(feature = "js-solver")]
        {
            let mut url_str = if let Some(u) = direct_url {
                u.to_string()
            } else if let Some(cipher) = signature_cipher {
                let (base_url, sig, sp) = parse_signature_cipher(cipher)
                    .ok_or_else(|| "Invalid signatureCipher payload".to_string())?;
                let js = self.get_player_js(http).await?;
                let script = extract_sig_decipher_script(&js, &sig)
                    .ok_or_else(|| "Could not locate signature decipher function in base.js".to_string())?;
                let decoded_sig = eval_js_sandboxed(&script, 500)?;
                let mut parsed = reqwest::Url::parse(&base_url)
                    .map_err(|e| format!("Invalid cipher URL: {e}"))?;
                parsed.query_pairs_mut().append_pair(&sp, &decoded_sig);
                parsed.into()
            } else {
                return Err("Format has neither url nor signatureCipher".to_string());
            };

            if transform_n
                && let Ok(mut parsed) = reqwest::Url::parse(&url_str)
            {
                let n_val = parsed
                    .query_pairs()
                    .find(|(k, _)| k == "n")
                    .map(|(_, v)| v.into_owned());
                if let Some(n_input) = n_val {
                    let solved = self.solve_n_param(http, &n_input).await;
                    if let Ok(n_out) = solved
                        && !n_out.is_empty()
                        && n_out != n_input
                    {
                        let pairs: Vec<(String, String)> = parsed
                            .query_pairs()
                            .map(|(k, v)| {
                                if k == "n" {
                                    ("n".to_string(), n_out.clone())
                                } else {
                                    (k.into_owned(), v.into_owned())
                                }
                            })
                            .collect();
                        parsed.query_pairs_mut().clear().extend_pairs(pairs);
                        url_str = parsed.into();
                    }
                }
            }

            Ok(url_str)
        }
    }

    #[cfg(feature = "js-solver")]
    async fn solve_n_param(&self, http: &reqwest::Client, n_input: &str) -> Result<String, String> {
        if let Ok(cache) = self.n_cache.read()
            && let Some(cached) = cache.get(n_input)
        {
            return Ok(cached.clone());
        }

        let js = self.get_player_js(http).await?;
        let script = extract_n_transform_script(&js, n_input)
            .ok_or_else(|| "Could not locate n-transform function in base.js".to_string())?;
        let out = eval_js_sandboxed(&script, 500)?;
        if let Ok(mut cache) = self.n_cache.write() {
            if cache.len() > 256 {
                cache.clear();
            }
            cache.insert(n_input.to_string(), out.clone());
        }
        Ok(out)
    }

    #[cfg(feature = "js-solver")]
    async fn get_player_js(&self, http: &reqwest::Client) -> Result<String, String> {
        if let Ok(guard) = self.player_js.read()
            && let Some(ref cached) = *guard
            && cached.fetched_at.elapsed() < PLAYER_JS_TTL
        {
            return Ok(cached.source.clone());
        }

        let iframe_html = http
            .get("https://www.youtube.com/iframe_api")
            .header("User-Agent", WEB_UA)
            .timeout(Duration::from_secs(6))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch YouTube iframe_api: {e}"))?
            .text()
            .await
            .map_err(|e| format!("Failed to read iframe_api: {e}"))?;

        let js_url = extract_base_js_url(&iframe_html)
            .ok_or_else(|| "Could not extract base.js URL from YouTube iframe_api".to_string())?;

        let source = http
            .get(&js_url)
            .header("User-Agent", WEB_UA)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch YouTube base.js: {e}"))?
            .text()
            .await
            .map_err(|e| format!("Failed to read YouTube base.js: {e}"))?;

        if let Ok(mut guard) = self.player_js.write() {
            *guard = Some(CachedPlayerJs {
                fetched_at: Instant::now(),
                source: source.clone(),
            });
        }
        if let Ok(mut cache) = self.n_cache.write() {
            cache.clear();
        }
        Ok(source)
    }
}

#[cfg_attr(not(feature = "js-solver"), allow(dead_code))]
pub(crate) fn extract_base_js_url(html: &str) -> Option<String> {
    if let Some(pos) = html.find("/s/player/") {
        let after = &html[pos..];
        let end = after.find(['"', '\'', '\\', ' ', '\n', '\r'])?;
        let path = &after[..end];
        if path.ends_with(".js") {
            return Some(format!("https://www.youtube.com{path}"));
        }
        // iframe_api often contains `/s/player/<hash>/www-widgetapi.vflset/www-widgetapi.js`
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 4 {
            let player_hash = parts[3];
            if !player_hash.is_empty() && player_hash.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(format!(
                    "https://www.youtube.com/s/player/{player_hash}/player_ias.vflset/en_US/base.js"
                ));
            }
        }
    }
    None
}

#[cfg_attr(not(feature = "js-solver"), allow(dead_code))]
pub(crate) fn parse_signature_cipher(cipher: &str) -> Option<(String, String, String)> {
    let mut url = None;
    let mut sig = None;
    let mut sp = String::from("sig");

    for (k, v) in reqwest::Url::parse(&format!("https://dummy.invalid/?{cipher}"))
        .ok()?
        .query_pairs()
    {
        match k.as_ref() {
            "url" => url = Some(v.into_owned()),
            "s" => sig = Some(v.into_owned()),
            "sp" if !v.is_empty() => {
                sp = v.into_owned();
            }
            _ => {}
        }
    }
    Some((url?, sig?, sp))
}

#[cfg(feature = "js-solver")]
fn extract_balanced_block(src: &str, open_brace_idx: usize) -> Option<&str> {
    let bytes = src.as_bytes();
    if bytes.get(open_brace_idx) != Some(&b'{') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str: Option<u8> = None;
    let mut escaped = false;

    for (rel, &b) in bytes[open_brace_idx..].iter().enumerate() {
        if let Some(q) = in_str {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == q {
                in_str = None;
            }
            continue;
        }
        match b {
            b'"' | b'\'' => in_str = Some(b),
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&src[open_brace_idx..=open_brace_idx + rel]);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(feature = "js-solver")]
pub(crate) fn extract_sig_decipher_script(base_js: &str, sig: &str) -> Option<String> {
    // Locate signature function body containing `split("")` and `join("")`
    let needle = "a=a.split(\"\")";
    let pos = base_js.find(needle)?;
    let prefix = &base_js[..pos];
    let fn_start = prefix.rfind("function(")?;
    let open_brace = base_js[fn_start..].find('{')? + fn_start;
    let body = extract_balanced_block(base_js, open_brace)?;

    // Find helper object name referenced inside body (e.g. `Ab.cd(a,12)`)
    let mut helper_def = String::new();
    for stmt in body.split(';') {
        if let Some(dot) = stmt.find('.') {
            let obj = stmt[..dot].trim_start_matches(['{', '\n', ' ']);
            if !obj.is_empty()
                && obj != "a"
                && obj.len() <= 6
                && obj
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
            {
                let decl = format!("var {obj}={{");
                if let Some(h_pos) = base_js.find(&decl)
                    && let Some(h_brace) = base_js[h_pos..].find('{')
                    && let Some(h_block) = extract_balanced_block(base_js, h_pos + h_brace)
                {
                    helper_def = format!("var {obj}={h_block};");
                    break;
                }
            }
        }
    }

    let sig_json = serde_json::to_string(sig).ok()?;
    Some(format!(
        "{helper_def}var __decipher=function(a){body};__decipher({sig_json});"
    ))
}

#[cfg(feature = "js-solver")]
pub(crate) fn extract_n_transform_script(base_js: &str, n_val: &str) -> Option<String> {
    let marker = "enhanced_except_";
    let marker_pos = base_js.find(marker)?;
    let window_start = marker_pos.saturating_sub(25_000);
    let prefix = &base_js[window_start..marker_pos];
    let rel_fn = prefix.rfind("function(a){var b=a.split(\"\")")?;
    let abs_fn = window_start + rel_fn;
    let open_brace = base_js[abs_fn..].find('{')? + abs_fn;
    let body = extract_balanced_block(base_js, open_brace)?;
    let n_json = serde_json::to_string(n_val).ok()?;
    Some(format!("var __ntrans=function(a){body};__ntrans({n_json});"))
}

/// Executes a JavaScript snippet inside an isolated QuickJS runtime with strict
/// 16 MiB memory cap and wall-clock timeout (protects against infinite loops).
#[cfg(feature = "js-solver")]
pub(crate) fn eval_js_sandboxed(script: &str, timeout_ms: u64) -> Result<String, String> {
    use rquickjs::{Context, Runtime};

    let rt = Runtime::new().map_err(|e| format!("QuickJS runtime init failed: {e}"))?;
    rt.set_memory_limit(16 * 1024 * 1024);
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    rt.set_interrupt_handler(Some(Box::new(move || Instant::now() >= deadline)));

    let ctx = Context::full(&rt).map_err(|e| format!("QuickJS context init failed: {e}"))?;
    ctx.with(|ctx| {
        ctx.eval::<String, _>(script)
            .map_err(|e| format!("QuickJS eval error: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::{extract_base_js_url, parse_signature_cipher};

    #[test]
    fn parses_signature_cipher_and_base_js_url() {
        let cipher = "s=abc123&sp=sig&url=https%3A%2F%2Frr1.googlevideo.com%2Fvideoplayback%3Fid%3D1";
        let (url, sig, sp) = parse_signature_cipher(cipher).expect("should parse cipher");
        assert_eq!(url, "https://rr1.googlevideo.com/videoplayback?id=1");
        assert_eq!(sig, "abc123");
        assert_eq!(sp, "sig");

        let iframe = r#"var scriptUrl = 'https://www.youtube.com/s/player/a1b2c3d4/www-widgetapi.vflset/www-widgetapi.js';"#;
        assert_eq!(
            extract_base_js_url(iframe).as_deref(),
            Some("https://www.youtube.com/s/player/a1b2c3d4/www-widgetapi.vflset/www-widgetapi.js")
        );
    }

    #[cfg(feature = "js-solver")]
    #[test]
    fn quickjs_sandbox_executes_decipher_and_aborts_infinite_loop() {
        use super::eval_js_sandboxed;

        let res = eval_js_sandboxed(
            r#"var f=function(a){return a.split("").reverse().join("");};f("oryg");"#,
            500,
        )
        .expect("should evaluate JS");
        assert_eq!(res, "gyro");

        let err = eval_js_sandboxed("while(true){}", 40);
        assert!(err.is_err(), "infinite loop must be interrupted by timeout");
    }
}
