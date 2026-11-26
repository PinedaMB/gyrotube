#[cfg(not(target_os = "android"))]
use std::collections::HashMap;
use std::time::Duration;

#[cfg(not(target_os = "android"))]
use tokio::process::Command;

use crate::models::StreamInfo;
use super::http::parse_total_bytes_from_headers;

#[cfg(not(target_os = "android"))]
pub(crate) fn ytdlp_command() -> Command {
    let mut cmd = Command::new("yt-dlp");
    // Inside a Linux AppImage, AppRun injects LD_LIBRARY_PATH pointing to bundled libs
    // (often built on Ubuntu), which breaks host Python/yt-dlp/node on other distros (e.g. Fedora).
    if std::env::var_os("APPIMAGE").is_some() {
        cmd.env_remove("LD_LIBRARY_PATH")
            .env_remove("LD_PRELOAD")
            .env_remove("PYTHONHOME")
            .env_remove("PYTHONPATH");
        if let Some(orig) = std::env::var_os("APPIMAGE_ORIG_LD_LIBRARY_PATH")
            && !orig.is_empty()
        {
            cmd.env("LD_LIBRARY_PATH", orig);
        }
    }
    cmd
}

pub(super) async fn fetch_range_block_url(
    client: &reqwest::Client,
    url: &str,
    remote: &StreamInfo,
    start: u64,
    end: u64,
) -> Result<(Vec<u8>, Option<u64>), String> {
    let mut last_err = String::new();
    for attempt in 0..4 {
        let mut r = client
            .get(url)
            .header(reqwest::header::RANGE, format!("bytes={start}-{end}"));
        if let Some(ua) = &remote.user_agent {
            r = r.header(reqwest::header::USER_AGENT, ua);
        }
        for (k, v) in &remote.headers {
            if !k.eq_ignore_ascii_case("range") {
                r = r.header(k.as_str(), v.as_str());
            }
        }
        match r.send().await {
            Ok(resp) if resp.status() == reqwest::StatusCode::PARTIAL_CONTENT => {
                let Some((first, last)) = response_byte_range(resp.headers()) else {
                    return Err(format!("Missing Content-Range for audio block {start}-{end}"));
                };
                if first != start || last < first || last > end {
                    return Err(format!("Unexpected Content-Range for audio block {start}-{end}"));
                }
                let total = parse_total_bytes_from_headers(
                    resp.headers(),
                    true,
                );
                match resp.bytes().await {
                    Ok(b) if b.len() as u64 == last - first + 1 => return Ok((b.to_vec(), total)),
                    Ok(_) => last_err = format!("Incomplete audio block {start}-{end}"),
                    Err(e) => last_err = format!("Error reading block {start}-{end}: {}", e.without_url()),
                }
            }
            Ok(resp) if resp.status() == reqwest::StatusCode::RANGE_NOT_SATISFIABLE => {
                return Ok((Vec::new(), None));
            }
            Ok(resp) => {
                last_err = format!("Block {start}-{end} HTTP {}", resp.status());
            }
            Err(e) => {
                last_err = format!("Block {start}-{end} failed: {}", e.without_url());
            }
        }
        if attempt < 3 {
            tokio::time::sleep(Duration::from_millis(120 * (attempt as u64 + 1))).await;
        }
    }
    Err(last_err)
}

pub(super) fn response_byte_range(headers: &reqwest::header::HeaderMap) -> Option<(u64, u64)> {
    let value = headers.get(reqwest::header::CONTENT_RANGE)?.to_str().ok()?;
    let (range, _) = value.strip_prefix("bytes ")?.split_once('/')?;
    let (first, last) = range.split_once('-')?;
    Some((first.parse().ok()?, last.parse().ok()?))
}

pub(super) async fn verify_audio_tail(
    client: &reqwest::Client,
    remote: &StreamInfo,
    total: u64,
) -> Result<(), String> {
    let last = total - 1;
    let (bytes, reported_total) = tokio::time::timeout(
        Duration::from_secs(4),
        fetch_range_block_url(client, &remote.url, remote, last, last),
    )
    .await
    .map_err(|_| "Timed out checking the end of the audio".to_string())??;
    if bytes.len() != 1 || reported_total != Some(total) {
        return Err("Audio source did not deliver the requested last byte".into());
    }
    Ok(())
}

#[cfg(target_os = "android")]
pub(super) async fn refresh_ytdlp_audio_stream(
    _video_id: &str,
    _itag: Option<&str>,
    _ext: &str,
) -> Option<StreamInfo> {
    None
}

#[cfg(not(target_os = "android"))]
pub(super) async fn refresh_ytdlp_audio_stream(
    video_id: &str,
    itag: Option<&str>,
    ext: &str,
) -> Option<StreamInfo> {
    let format_selector = match itag {
        Some(tag) if !tag.is_empty() => format!("{tag}/bestaudio[ext={ext}]/251/140/bestaudio"),
        _ => format!("bestaudio[ext={ext}]/251/140/bestaudio"),
    };
    let watch_url = format!("https://music.youtube.com/watch?v={video_id}");
    let out = tokio::time::timeout(
        Duration::from_secs(10),
        ytdlp_command()
            .kill_on_drop(true)
            .args([
                "--ignore-config",
                "--no-playlist",
                "--no-warnings",
                "--socket-timeout",
                "5",
                "--retries",
                "1",
                "--dump-single-json",
                "--skip-download",
                "--extractor-args",
                "youtube:player_client=default,-tv_samsung;skip=hls,dash",
                "--js-runtimes",
                "node",
                "--remote-components",
                "ejs:github",
                "-f",
                &format_selector,
                "--",
                &watch_url,
            ])
            .output(),
    )
    .await
    .ok()?
    .ok()?;
    if !out.status.success() {
        return None;
    }
    #[derive(serde::Deserialize)]
    struct FreshAudio {
        url: String,
        #[serde(default)]
        http_headers: HashMap<String, String>,
    }
    let parsed: FreshAudio = serde_json::from_slice(&out.stdout).ok()?;
    if !parsed.url.starts_with("http") {
        return None;
    }
    let ua = parsed
        .http_headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("user-agent"))
        .map(|(_, v)| v.clone());
    Some(StreamInfo {
        url: parsed.url,
        quality_label: ext.to_uppercase(),
        user_agent: ua,
        headers: parsed.http_headers,
        local_audio: None,
    })
}
