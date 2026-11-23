use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::models::{
    Album, Artist, Playlist, RelatedFeed, StreamInfo, Track, clean_track_title, pick_cover_thumbs,
};

const YTM_CLIENT_NAME: &str = "WEB_REMIX";
const YTM_CLIENT_NAME_HDR: &str = "67";
const YTM_CLIENT_VERSION: &str = "1.20250310.01.00";
const YTM_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36";
const VISITOR_DATA_TTL: Duration = Duration::from_secs(6 * 60 * 60);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackKind {
    Track,
    Video,
    Episode,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct CatalogItem {
    pub track: Track,
    pub kind: TrackKind,
    pub unavailable: bool,
}

#[derive(Clone, Debug)]
pub struct MusicTrackDetails {
    pub item: CatalogItem,
    pub covers: Vec<(String, u32)>,
    pub lyrics_browse_id: Option<String>,
    pub related_browse_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ChartsResult {
    pub trending: Vec<Track>,
    pub top: Vec<Track>,
    pub artists: Vec<Artist>,
    pub available_countries: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ArtistEnrichment {
    pub name: Option<String>,
    pub header_url: Option<String>,
    pub description: Option<String>,
    pub subscriber_count: Option<u64>,
    pub view_counts: HashMap<String, u64>,
    pub tracks: Vec<Track>,
    pub videos: Vec<Track>,
    pub albums: Vec<Album>,
    pub singles: Vec<Album>,
    pub playlists: Vec<Playlist>,
    pub similar_artists: Vec<Artist>,
}

#[derive(Clone, Debug)]
pub struct PlaylistBrowsePage {
    pub title: String,
    pub items: Vec<CatalogItem>,
    pub had_entries: bool,
    pub exhausted: bool,
}

#[derive(Clone)]
pub struct InnerTubeClient {
    http: reqwest::Client,
    visitor_cache: Arc<RwLock<Option<(Instant, String)>>>,
}

impl InnerTubeClient {
    pub fn new(http: reqwest::Client) -> Self {
        Self {
            http,
            visitor_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn visitor_data(&self, force_refresh: bool) -> Option<String> {
        if !force_refresh
            && let Ok(guard) = self.visitor_cache.read()
            && let Some((ts, ref vd)) = *guard
            && ts.elapsed() < VISITOR_DATA_TTL
        {
            return Some(vd.clone());
        }

        let payload = json!({
            "context": {
                "client": {
                    "clientName": YTM_CLIENT_NAME,
                    "clientVersion": YTM_CLIENT_VERSION,
                    "hl": "en",
                    "gl": "US"
                }
            }
        });

        let resp = self
            .http
            .post("https://www.youtube.com/youtubei/v1/visitor_id?prettyPrint=false")
            .timeout(Duration::from_secs(5))
            .header("User-Agent", YTM_USER_AGENT)
            .header("X-YouTube-Client-Name", YTM_CLIENT_NAME_HDR)
            .header("X-YouTube-Client-Version", YTM_CLIENT_VERSION)
            .json(&payload)
            .send()
            .await
            .ok()?
            .json::<Value>()
            .await
            .ok()?;

        let vd = resp["responseContext"]["visitorData"]
            .as_str()
            .filter(|s| !s.is_empty())?
            .to_string();

        if let Ok(mut guard) = self.visitor_cache.write() {
            *guard = Some((Instant::now(), vd.clone()));
        }
        Some(vd)
    }

    async fn ytm_post(&self, endpoint: &str, mut body: Value, country: Option<&str>) -> Result<Value, String> {
        let gl = country.unwrap_or("US");
        let visitor = self.visitor_data(false).await;
        let mut client_ctx = json!({
            "clientName": YTM_CLIENT_NAME,
            "clientVersion": YTM_CLIENT_VERSION,
            "hl": "en",
            "gl": gl
        });
        if let Some(ref vd) = visitor
            && let Some(obj) = client_ctx.as_object_mut()
        {
            obj.insert("visitorData".into(), Value::String(vd.clone()));
        }
        if let Some(obj) = body.as_object_mut() {
            obj.insert("context".into(), json!({ "client": client_ctx }));
        }

        let url = format!("https://music.youtube.com/youtubei/v1/{endpoint}?prettyPrint=false");
        let mut req = self
            .http
            .post(&url)
            .timeout(Duration::from_secs(8))
            .header("User-Agent", YTM_USER_AGENT)
            .header("Origin", "https://music.youtube.com")
            .header("Referer", "https://music.youtube.com/")
            .header("X-YouTube-Client-Name", YTM_CLIENT_NAME_HDR)
            .header("X-YouTube-Client-Version", YTM_CLIENT_VERSION);
        if let Some(ref vd) = visitor {
            req = req.header("X-Goog-Visitor-Id", vd.as_str());
        }

        req.json(&body)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|e| e.without_url().to_string())?
            .json::<Value>()
            .await
            .map_err(|e| e.without_url().to_string())
    }

    pub async fn extract_direct_stream(
        &self,
        video_id: &str,
        profile: &str,
        visitor_data: Option<&str>,
        timeout: Duration,
        require_itag: Option<u64>,
    ) -> Result<(StreamInfo, &'static str, Option<u64>), String> {
        let (user_agent, client_name_hdr, client_version_hdr, mut payload) = match profile {
            "ANDROID_VR" => (
                "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip",
                "28",
                "1.65.10",
                json!({
                    "context": { "client": {
                        "clientName": "ANDROID_VR",
                        "clientVersion": "1.65.10",
                        "deviceMake": "Oculus",
                        "deviceModel": "Quest 3",
                        "androidSdkVersion": 32,
                        "osName": "Android",
                        "osVersion": "12L",
                        "hl": "en",
                        "gl": "US"
                    } },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            "ANDROID_VR_LEGACY" => (
                "com.google.android.apps.youtube.vr.oculus/1.43.32 (Linux; U; Android 12; Quest 2) gzip",
                "28",
                "1.43.32",
                json!({
                    "context": { "client": {
                        "clientName": "ANDROID_VR",
                        "clientVersion": "1.43.32",
                        "deviceMake": "Oculus",
                        "deviceModel": "Quest 2",
                        "androidSdkVersion": 32,
                        "osName": "Android",
                        "osVersion": "12",
                        "hl": "en",
                        "gl": "US"
                    } },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            "TV_EMBEDDED" => (
                "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version",
                "85",
                "2.0",
                json!({
                    "context": {
                        "client": {
                            "clientName": "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
                            "clientVersion": "2.0",
                            "hl": "en",
                            "gl": "US"
                        },
                        "thirdParty": {
                            "embedUrl": "https://www.youtube.com"
                        }
                    },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            "IOS" => (
                "com.google.ios.youtube/20.10.4 (iPhone16,2; U; CPU iOS 18_3_2 like Mac OS X;)",
                "5",
                "20.10.4",
                json!({
                    "context": { "client": {
                        "clientName": "IOS",
                        "clientVersion": "20.10.4",
                        "deviceMake": "Apple",
                        "deviceModel": "iPhone16,2",
                        "osName": "iPhone",
                        "osVersion": "18.3.2.22D82",
                        "hl": "en",
                        "gl": "US"
                    } },
                    "videoId": video_id,
                    "contentCheckOk": true,
                    "racyCheckOk": true,
                }),
            ),
            _ => (
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15",
                "101",
                "1.02",
                json!({
                    "context": { "client": {
                        "clientName": "VISIONOS",
                        "clientVersion": "1.02",
                        "deviceMake": "Apple",
                        "deviceModel": "RealityDevice17,1",
                        "osName": "visionOS",
                        "osVersion": "26.5.23O471"
                    } },
                    "videoId": video_id,
                }),
            ),
        };

        if let Some(vd) = visitor_data
            && let Some(client_obj) = payload["context"]["client"].as_object_mut()
        {
            client_obj.insert("visitorData".to_string(), Value::String(vd.to_string()));
        }

        let mut req = self
            .http
            .post("https://www.youtube.com/youtubei/v1/player?prettyPrint=false")
            .timeout(timeout)
            .header("User-Agent", user_agent)
            .header("X-YouTube-Client-Name", client_name_hdr)
            .header("X-YouTube-Client-Version", client_version_hdr);
        if let Some(vd) = visitor_data {
            req = req.header("X-Goog-Visitor-Id", vd);
        }

        let response: Value = req
            .json(&payload)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|err| err.without_url().to_string())?
            .json()
            .await
            .map_err(|err| err.without_url().to_string())?;

        let adaptive = response["streamingData"]["adaptiveFormats"].as_array();
        let muxed = response["streamingData"]["formats"].as_array();
        if adaptive.is_none() && muxed.is_none() {
            let status = response["playabilityStatus"]["status"]
                .as_str()
                .filter(|status| {
                    status.len() <= 40
                        && status
                            .bytes()
                            .all(|b| b.is_ascii_uppercase() || b == b'_')
                });
            return Err(format!(
                "El reproductor no ofreció audio ({})",
                status.unwrap_or("SIN_FORMATOS")
            ));
        }

        let all_formats: Vec<&Value> = adaptive
            .into_iter()
            .flatten()
            .chain(muxed.into_iter().flatten())
            .collect();

        let candidates: &[(u64, &str)] = &[
            (251, "webm"),
            (140, "m4a"),
            (250, "webm"),
            (249, "webm"),
            (139, "m4a"),
            (18, "mp4"),
        ];

        for &(itag, extension) in candidates {
            if let Some(req_itag) = require_itag
                && itag != req_itag
            {
                continue;
            }
            let Some(format) = all_formats
                .iter()
                .find(|f| f["itag"].as_u64() == Some(itag) && f["url"].is_string())
            else {
                continue;
            };
            let Some(url) = format["url"]
                .as_str()
                .and_then(|url| reqwest::Url::parse(url).ok())
            else {
                continue;
            };
            if url.scheme() != "https"
                || !url
                    .host_str()
                    .is_some_and(|host| host.ends_with(".googlevideo.com"))
            {
                continue;
            }
            let size = format["contentLength"].as_str().and_then(|v| v.parse().ok());
            let kbps = (format["bitrate"].as_u64().unwrap_or(128_000) / 1000).max(96);
            let mut headers = HashMap::new();
            headers.insert("User-Agent".to_string(), user_agent.to_string());
            return Ok((
                StreamInfo {
                    url: url.into(),
                    quality_label: format!("{kbps}kbps ({})", extension.to_uppercase()),
                    user_agent: Some(user_agent.into()),
                    headers,
                    local_audio: None,
                },
                extension,
                size,
            ));
        }
        Err("El reproductor no ofreció una URL directa de audio compatible".into())
    }

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
        let resp = self
            .ytm_post("music/get_search_suggestions", json!({ "input": query }), None)
            .await?;
        Ok(parse_search_suggestions_json(&resp))
    }

    pub async fn fetch_music_details(&self, video_id: &str) -> Result<MusicTrackDetails, String> {
        let body = json!({
            "videoId": video_id,
            "isAudioOnly": true,
            "enablePersistentPlaylistPanel": true
        });
        let resp = self.ytm_post("next", body, None).await?;
        parse_music_details_json(video_id, &resp)
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

    pub async fn fetch_artist_enrichment(&self, artist_id: &str) -> Result<ArtistEnrichment, String> {
        let resp = self
            .ytm_post("browse", json!({ "browseId": artist_id }), None)
            .await?;
        Ok(parse_artist_enrichment_json(artist_id, &resp))
    }
}

// ---------------------------------------------------------------------------
// JSON Parsing helpers (pure functions, unit-testable without network)
// ---------------------------------------------------------------------------

fn runs_text(val: &Value) -> String {
    val["runs"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|r| r["text"].as_str())
                .collect::<String>()
        })
        .or_else(|| val["simpleText"].as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn extract_thumbs(val: &Value) -> Vec<(String, u32)> {
    let arr = val["thumbnails"]
        .as_array()
        .or_else(|| val["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array())
        .or_else(|| val["thumbnail"]["thumbnails"].as_array())
        .or_else(|| val["thumbnail"]["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array());

    arr.map(|list| {
        list.iter()
            .filter_map(|t| {
                let url = t["url"].as_str()?.to_string();
                let width = t["width"].as_u64().unwrap_or(0) as u32;
                Some((url, width))
            })
            .collect()
    })
    .unwrap_or_default()
}

fn parse_duration_str(s: &str) -> Option<u32> {
    let mut parts = s.trim().split(':');
    let a = parts.next()?.parse::<u32>().ok()?;
    let b = parts.next()?.parse::<u32>().ok()?;
    if let Some(c_str) = parts.next() {
        let c = c_str.parse::<u32>().ok()?;
        Some(a * 3600 + b * 60 + c)
    } else {
        Some(a * 60 + b)
    }
}

pub fn parse_compact_count(s: &str) -> Option<u64> {
    let cleaned = s
        .to_lowercase()
        .replace("subscribers", "")
        .replace("suscriptores", "")
        .replace("plays", "")
        .replace("reproducciones", "")
        .replace("views", "")
        .replace("vistas", "")
        .replace("monthly audience", "")
        .replace(',', "")
        .trim()
        .to_string();
    let token = cleaned.split_whitespace().next()?;
    let (num_str, mult) = if let Some(n) = token.strip_suffix('b') {
        (n, 1_000_000_000.0)
    } else if let Some(n) = token.strip_suffix('m') {
        (n, 1_000_000.0)
    } else if let Some(n) = token.strip_suffix('k') {
        (n, 1_000.0)
    } else {
        (token, 1.0)
    };
    let val: f64 = num_str.parse().ok()?;
    if val >= 0.0 {
        Some((val * mult).round() as u64)
    } else {
        None
    }
}

fn classify_video_type(mvt: Option<&str>) -> TrackKind {
    match mvt {
        Some("MUSIC_VIDEO_TYPE_ATV") => TrackKind::Track,
        Some("MUSIC_VIDEO_TYPE_OMV" | "MUSIC_VIDEO_TYPE_UGC" | "MUSIC_VIDEO_TYPE_OFFICIAL_SOURCE_MUSIC") => {
            TrackKind::Video
        }
        Some("MUSIC_VIDEO_TYPE_PODCAST_EPISODE") => TrackKind::Episode,
        _ => TrackKind::Unknown,
    }
}

fn video_type_from_watch_endpoint(ep: &Value) -> Option<&str> {
    ep["watchEndpointMusicSupportedConfigs"]["watchEndpointMusicConfig"]["musicVideoType"].as_str()
}

fn parse_panel_video_renderer(r: &Value) -> Option<(CatalogItem, Vec<(String, u32)>)> {
    let id = r["videoId"].as_str()?.to_string();
    let title = clean_track_title(&runs_text(&r["title"]));
    if title.is_empty() {
        return None;
    }

    let mut artists = Vec::new();
    let mut artist_id = None;
    let mut album = None;
    let mut album_id = None;

    if let Some(runs) = r["longBylineText"]["runs"].as_array() {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "&" || text == "," {
                continue;
            }
            if let Some(bid) = run["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
                if bid.starts_with("UC") {
                    if artist_id.is_none() {
                        artist_id = Some(bid.to_string());
                    }
                    artists.push(text.to_string());
                } else if bid.starts_with("MPREb_") || bid.starts_with("OLAK") {
                    album = Some(text.to_string());
                    album_id = Some(bid.to_string());
                }
            }
        }
    }
    if artists.is_empty() {
        let short = runs_text(&r["shortBylineText"]);
        if !short.is_empty() {
            artists.push(short);
        }
    }
    let artist = if artists.is_empty() {
        String::from("Artista desconocido")
    } else {
        artists.join(", ")
    };

    let duration_secs = parse_duration_str(&runs_text(&r["lengthText"]));
    let covers = extract_thumbs(&r["thumbnail"]);
    let mvt = video_type_from_watch_endpoint(&r["navigationEndpoint"]["watchEndpoint"]);
    let kind = match classify_video_type(mvt) {
        TrackKind::Unknown => TrackKind::Track,
        other => other,
    };
    let unavailable = r["unplayableText"].is_object();

    let track = Track {
        is_music: kind == TrackKind::Track,
        is_video: kind == TrackKind::Video,
        id,
        title,
        artist,
        artist_id,
        album,
        album_id,
        duration_secs,
        view_count: None,
        cover_url: pick_cover_thumbs(&covers, 240),
        cover_large_url: pick_cover_thumbs(&covers, 544),
    };

    Some((
        CatalogItem {
            track,
            kind,
            unavailable,
        },
        covers,
    ))
}

fn parse_responsive_list_item(item: &Value) -> Option<CatalogItem> {
    let video_id = item["playlistItemData"]["videoId"]
        .as_str()
        .or_else(|| {
            item["overlay"]["musicItemThumbnailOverlayRenderer"]["content"]
                ["musicPlayButtonRenderer"]["playNavigationEndpoint"]["watchEndpoint"]["videoId"]
                .as_str()
        })
        .or_else(|| {
            item["flexColumns"][0]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"][0]
                ["navigationEndpoint"]["watchEndpoint"]["videoId"]
                .as_str()
        })?;

    let mvt = item["overlay"]["musicItemThumbnailOverlayRenderer"]["content"]
        ["musicPlayButtonRenderer"]["playNavigationEndpoint"]["watchEndpoint"]
        .as_object()
        .and_then(|_| {
            video_type_from_watch_endpoint(
                &item["overlay"]["musicItemThumbnailOverlayRenderer"]["content"]
                    ["musicPlayButtonRenderer"]["playNavigationEndpoint"]["watchEndpoint"],
            )
        })
        .or_else(|| {
            video_type_from_watch_endpoint(
                &item["flexColumns"][0]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"]
                    [0]["navigationEndpoint"]["watchEndpoint"],
            )
        });

    let kind = match classify_video_type(mvt) {
        TrackKind::Unknown => TrackKind::Track,
        other => other,
    };

    let unavailable = item["musicItemRendererDisplayPolicy"].as_str()
        == Some("MUSIC_ITEM_RENDERER_DISPLAY_POLICY_GREY_OUT");

    let flex = item["flexColumns"].as_array()?;
    let title = flex
        .first()
        .map(|c| runs_text(&c["musicResponsiveListItemFlexColumnRenderer"]["text"]))
        .map(|t| clean_track_title(&t))
        .unwrap_or_default();
    if title.is_empty() {
        return None;
    }

    let mut artists = Vec::new();
    let mut artist_id = None;
    let mut album = None;
    let mut album_id = None;
    let mut duration_secs = None;
    let mut view_count = None;

    for col in flex.iter().skip(1) {
        let Some(runs) = col["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"].as_array()
        else {
            continue;
        };
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "&" || text == "," {
                continue;
            }
            if let Some(bid) = run["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str() {
                if bid.starts_with("UC") {
                    if artist_id.is_none() {
                        artist_id = Some(bid.to_string());
                    }
                    artists.push(text.to_string());
                    continue;
                } else if bid.starts_with("MPREb_") || bid.starts_with("OLAK") {
                    album = Some(text.to_string());
                    album_id = Some(bid.to_string());
                    continue;
                }
            }
            let lower = text.to_lowercase();
            if lower.contains("play") || lower.contains("view") || lower.contains("reproduccion") {
                if view_count.is_none() {
                    view_count = parse_compact_count(text);
                }
            } else if duration_secs.is_none()
                && text.contains(':')
                && let Some(secs) = parse_duration_str(text)
            {
                duration_secs = Some(secs);
            } else if artists.is_empty()
                && !lower.contains("song")
                && !lower.contains("video")
                && !lower.contains("album")
                && !lower.contains("single")
                && !lower.contains("ep")
                && !text.chars().all(|c| c.is_ascii_digit())
            {
                artists.push(text.to_string());
            }
        }
    }

    if duration_secs.is_none()
        && let Some(fixed) = item["fixedColumns"].as_array()
    {
        for col in fixed {
            let text = runs_text(&col["musicResponsiveListItemFixedColumnRenderer"]["text"]);
            if let Some(secs) = parse_duration_str(&text) {
                duration_secs = Some(secs);
                break;
            }
        }
    }

    let covers = extract_thumbs(&item["thumbnail"]);
    let artist = if artists.is_empty() {
        String::from("Artista desconocido")
    } else {
        artists.join(", ")
    };

    Some(CatalogItem {
        track: Track {
            is_music: kind == TrackKind::Track,
            is_video: kind == TrackKind::Video,
            id: video_id.to_string(),
            title,
            artist,
            artist_id,
            album,
            album_id,
            duration_secs,
            view_count,
            cover_url: pick_cover_thumbs(&covers, 240),
            cover_large_url: pick_cover_thumbs(&covers, 544),
        },
        kind,
        unavailable,
    })
}

fn parse_responsive_artist_item(item: &Value) -> Option<Artist> {
    let bid = item["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("UC") {
        return None;
    }
    let flex = item["flexColumns"].as_array()?;
    let name = runs_text(&flex.first()?["musicResponsiveListItemFlexColumnRenderer"]["text"]);
    if name.is_empty() {
        return None;
    }
    let subscriber_count = flex
        .get(1)
        .map(|c| runs_text(&c["musicResponsiveListItemFlexColumnRenderer"]["text"]))
        .and_then(|s| parse_compact_count(&s));
    let covers = extract_thumbs(&item["thumbnail"]);
    Some(Artist {
        id: bid.to_string(),
        name,
        avatar_url: pick_cover_thumbs(&covers, 360),
        subscriber_count,
    })
}

fn parse_two_row_album(r: &Value) -> Option<(Album, bool)> {
    let bid = r["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("MPREb_") && !bid.starts_with("OLAK") {
        return None;
    }
    let title = runs_text(&r["title"]);
    if title.is_empty() {
        return None;
    }
    let sub_runs = r["subtitle"]["runs"].as_array();
    let mut is_single = false;
    let mut year = None;
    let mut artists = Vec::new();
    if let Some(runs) = sub_runs {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "," || text == "&" {
                continue;
            }
            let lower = text.to_lowercase();
            if lower == "single" || lower == "ep" || lower == "sencillo" {
                is_single = true;
                continue;
            }
            if lower == "album" || lower == "álbum" {
                continue;
            }
            if text.len() == 4
                && let Ok(y) = text.parse::<u16>()
                && (1900..=2100).contains(&y)
            {
                year = Some(y);
                continue;
            }
            artists.push(text.to_string());
        }
    }
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    let artist = if artists.is_empty() {
        String::new()
    } else {
        artists.join(", ")
    };
    Some((
        Album {
            id: bid.to_string(),
            title,
            artist,
            year,
            cover_url: pick_cover_thumbs(&covers, 544),
        },
        is_single,
    ))
}

fn parse_two_row_playlist(r: &Value) -> Option<Playlist> {
    let bid = r["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("VL") && !bid.starts_with("PL") && !bid.starts_with("RDCLAK") {
        return None;
    }
    let name = runs_text(&r["title"]);
    if name.is_empty() {
        return None;
    }
    let subtitle = runs_text(&r["subtitle"]);
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    Some(Playlist {
        id: bid.strip_prefix("VL").unwrap_or(bid).to_string(),
        name,
        subtitle: if subtitle.is_empty() {
            String::from("YouTube Music")
        } else {
            subtitle
        },
        cover_url: pick_cover_thumbs(&covers, 544),
    })
}

fn parse_two_row_artist(r: &Value) -> Option<Artist> {
    let bid = r["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()?;
    if !bid.starts_with("UC") {
        return None;
    }
    let name = runs_text(&r["title"]);
    if name.is_empty() {
        return None;
    }
    let sub = runs_text(&r["subtitle"]);
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    Some(Artist {
        id: bid.to_string(),
        name,
        avatar_url: pick_cover_thumbs(&covers, 360),
        subscriber_count: parse_compact_count(&sub),
    })
}

fn parse_two_row_track(r: &Value) -> Option<CatalogItem> {
    let video_id = r["navigationEndpoint"]["watchEndpoint"]["videoId"].as_str()?;
    let title = clean_track_title(&runs_text(&r["title"]));
    if title.is_empty() {
        return None;
    }
    let mvt = video_type_from_watch_endpoint(&r["navigationEndpoint"]["watchEndpoint"]);
    let kind = match classify_video_type(mvt) {
        TrackKind::Unknown => TrackKind::Video,
        other => other,
    };
    let mut artists = Vec::new();
    let mut artist_id = None;
    let mut view_count = None;
    if let Some(runs) = r["subtitle"]["runs"].as_array() {
        for run in runs {
            let text = run["text"].as_str().unwrap_or("").trim();
            if text.is_empty() || text == "•" || text == "," || text == "&" {
                continue;
            }
            if let Some(bid) = run["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str()
                && bid.starts_with("UC")
            {
                if artist_id.is_none() {
                    artist_id = Some(bid.to_string());
                }
                artists.push(text.to_string());
                continue;
            }
            let lower = text.to_lowercase();
            if lower.contains("view") || lower.contains("play") {
                view_count = parse_compact_count(text);
            } else if artists.is_empty() {
                artists.push(text.to_string());
            }
        }
    }
    let covers = extract_thumbs(&r["thumbnailRenderer"]);
    Some(CatalogItem {
        track: Track {
            is_music: kind == TrackKind::Track,
            is_video: kind == TrackKind::Video,
            id: video_id.to_string(),
            title,
            artist: if artists.is_empty() {
                String::from("Artista desconocido")
            } else {
                artists.join(", ")
            },
            artist_id,
            album: None,
            album_id: None,
            duration_secs: None,
            view_count,
            cover_url: pick_cover_thumbs(&covers, 240),
            cover_large_url: pick_cover_thumbs(&covers, 544),
        },
        kind,
        unavailable: false,
    })
}

fn collect_shelves<'a>(val: &'a Value, out: &mut Vec<(String, &'a [Value])>) {
    match val {
        Value::Object(map) => {
            if let Some(shelf) = map.get("musicCarouselShelfRenderer") {
                let title = runs_text(
                    &shelf["header"]["musicCarouselShelfBasicHeaderRenderer"]["title"],
                );
                if let Some(contents) = shelf["contents"].as_array() {
                    out.push((title, contents.as_slice()));
                }
            }
            if let Some(shelf) = map.get("musicShelfRenderer") {
                let title = runs_text(&shelf["title"]);
                if let Some(contents) = shelf["contents"].as_array() {
                    out.push((title, contents.as_slice()));
                }
            }
            if let Some(grid) = map.get("gridRenderer")
                && let Some(items) = grid["items"].as_array()
            {
                let title = runs_text(&grid["header"]["gridHeaderRenderer"]["title"]);
                out.push((title, items.as_slice()));
            }
            for v in map.values() {
                collect_shelves(v, out);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_shelves(v, out);
            }
        }
        _ => {}
    }
}

fn collect_country_codes(val: &Value, out: &mut Vec<String>, seen: &mut HashSet<String>) {
    match val {
        Value::Object(map) => {
            if let Some(sel) = map.get("selectedValues").and_then(|v| v.as_array()) {
                for v in sel {
                    if let Some(code) = v.as_str()
                        && code.len() == 2
                        && code.bytes().all(|b| b.is_ascii_uppercase())
                        && code != "ZZ"
                        && seen.insert(code.to_string())
                    {
                        out.push(code.to_string());
                    }
                }
            }
            for v in map.values() {
                collect_country_codes(v, out, seen);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_country_codes(v, out, seen);
            }
        }
        _ => {}
    }
}

pub fn parse_charts_json(resp: &Value) -> ChartsResult {
    let mut res = ChartsResult::default();
    let mut seen_countries = HashSet::new();
    collect_country_codes(resp, &mut res.available_countries, &mut seen_countries);

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    for (title, items) in shelves {
        let lower = title.to_lowercase();
        let mut shelf_tracks = Vec::new();
        let mut shelf_artists = Vec::new();

        for entry in items {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer") {
                if let Some(item) = parse_responsive_list_item(r) {
                    if !item.unavailable {
                        shelf_tracks.push(item.track);
                    }
                } else if let Some(artist) = parse_responsive_artist_item(r) {
                    shelf_artists.push(artist);
                }
            } else if let Some(r) = entry.get("musicTwoRowItemRenderer") {
                if let Some(item) = parse_two_row_track(r) {
                    if !item.unavailable {
                        shelf_tracks.push(item.track);
                    }
                } else if let Some(artist) = parse_two_row_artist(r) {
                    shelf_artists.push(artist);
                }
            }
        }

        if !shelf_artists.is_empty() && res.artists.is_empty() {
            res.artists = shelf_artists.into_iter().take(12).collect();
            continue;
        }
        if !shelf_tracks.is_empty() {
            if lower.contains("trending") || lower.contains("tendencia") {
                if res.trending.is_empty() {
                    res.trending = shelf_tracks.into_iter().take(12).collect();
                }
            } else if lower.contains("top") || lower.contains("song") || lower.contains("cancion") {
                if res.top.is_empty() {
                    res.top = shelf_tracks.into_iter().take(12).collect();
                } else if res.trending.is_empty() {
                    res.trending = shelf_tracks.into_iter().take(12).collect();
                }
            } else if res.trending.is_empty() {
                res.trending = shelf_tracks.into_iter().take(12).collect();
            } else if res.top.is_empty() {
                res.top = shelf_tracks.into_iter().take(12).collect();
            }
        }
    }

    if res.trending.is_empty() && !res.top.is_empty() {
        res.trending = res.top.clone();
    }
    res
}

pub fn parse_new_albums_json(resp: &Value) -> Vec<Album> {
    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);
    let mut albums = Vec::new();
    let mut seen = HashSet::new();
    for (_title, items) in shelves {
        for entry in items {
            if let Some(r) = entry.get("musicTwoRowItemRenderer")
                && let Some((album, _)) = parse_two_row_album(r)
                && seen.insert(album.id.clone())
            {
                albums.push(album);
            }
        }
    }
    albums
}

pub fn parse_radio_next_json(resp: &Value) -> Vec<CatalogItem> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    collect_panel_videos(resp, &mut out, &mut seen);
    out
}

fn collect_panel_videos(val: &Value, out: &mut Vec<CatalogItem>, seen: &mut HashSet<String>) {
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("playlistPanelVideoRenderer")
                && let Some((item, _)) = parse_panel_video_renderer(r)
                && seen.insert(item.track.id.clone())
            {
                out.push(item);
                return;
            }
            if let Some(wrapper) = map.get("playlistPanelVideoWrapperRenderer") {
                if let Some(primary) = wrapper
                    .get("primaryRenderer")
                    .and_then(|p| p.get("playlistPanelVideoRenderer"))
                    && let Some((item, _)) = parse_panel_video_renderer(primary)
                    && seen.insert(item.track.id.clone())
                {
                    out.push(item);
                }
                return;
            }
            for v in map.values() {
                collect_panel_videos(v, out, seen);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_panel_videos(v, out, seen);
            }
        }
        _ => {}
    }
}

pub fn parse_music_details_json(video_id: &str, resp: &Value) -> Result<MusicTrackDetails, String> {
    let mut lyrics_browse_id = None;
    let mut related_browse_id = None;
    collect_watch_tab_ids(resp, &mut lyrics_browse_id, &mut related_browse_id);

    let mut matched: Option<(CatalogItem, Vec<(String, u32)>)> = None;
    find_panel_video_by_id(resp, video_id, &mut matched);

    let (item, covers) = matched.ok_or_else(|| {
        String::from("No se encontraron detalles de la pista en YouTube Music.")
    })?;

    Ok(MusicTrackDetails {
        item,
        covers,
        lyrics_browse_id,
        related_browse_id,
    })
}

fn collect_watch_tab_ids(
    val: &Value,
    lyrics_id: &mut Option<String>,
    related_id: &mut Option<String>,
) {
    match val {
        Value::Object(map) => {
            if let Some(bid) = map
                .get("browseEndpoint")
                .and_then(|e| e.get("browseId"))
                .and_then(Value::as_str)
            {
                if bid.starts_with("MPLYt_") && lyrics_id.is_none() {
                    *lyrics_id = Some(bid.to_string());
                } else if bid.starts_with("MPTRt_") && related_id.is_none() {
                    *related_id = Some(bid.to_string());
                }
            }
            for v in map.values() {
                collect_watch_tab_ids(v, lyrics_id, related_id);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_watch_tab_ids(v, lyrics_id, related_id);
            }
        }
        _ => {}
    }
}

fn find_panel_video_by_id(
    val: &Value,
    video_id: &str,
    out: &mut Option<(CatalogItem, Vec<(String, u32)>)>,
) {
    if out.is_some() {
        return;
    }
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("playlistPanelVideoRenderer")
                && r["videoId"].as_str() == Some(video_id)
                && let Some(parsed) = parse_panel_video_renderer(r)
            {
                *out = Some(parsed);
                return;
            }
            for v in map.values() {
                find_panel_video_by_id(v, video_id, out);
                if out.is_some() {
                    return;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                find_panel_video_by_id(v, video_id, out);
                if out.is_some() {
                    return;
                }
            }
        }
        _ => {}
    }
}

pub fn parse_related_browse_json(resp: &Value) -> RelatedFeed {
    let mut feed = RelatedFeed::default();
    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    let mut seen_tracks = HashSet::new();
    let mut seen_albums = HashSet::new();
    let mut seen_playlists = HashSet::new();
    let mut seen_artists = HashSet::new();

    for (_title, items) in shelves {
        for entry in items {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer")
                && let Some(item) = parse_responsive_list_item(r)
                && !item.unavailable
                && seen_tracks.insert(item.track.id.clone())
                && feed.tracks.len() < 16
            {
                feed.tracks.push(item.track);
            } else if let Some(r) = entry.get("musicTwoRowItemRenderer") {
                if let Some((album, _)) = parse_two_row_album(r)
                    && seen_albums.insert(album.id.clone())
                    && feed.albums.len() < 12
                {
                    feed.albums.push(album);
                } else if let Some(pl) = parse_two_row_playlist(r)
                    && seen_playlists.insert(pl.id.clone())
                    && feed.playlists.len() < 12
                {
                    feed.playlists.push(pl);
                } else if let Some(artist) = parse_two_row_artist(r)
                    && seen_artists.insert(artist.id.clone())
                    && feed.artists.len() < 12
                {
                    feed.artists.push(artist);
                }
            }
        }
    }
    feed
}

pub fn parse_lyrics_browse_json(resp: &Value) -> Option<String> {
    let mut found = None;
    find_description_shelf(resp, &mut found);
    found.filter(|s| !s.trim().is_empty())
}

fn find_description_shelf(val: &Value, out: &mut Option<String>) {
    if out.is_some() {
        return;
    }
    match val {
        Value::Object(map) => {
            if let Some(shelf) = map.get("musicDescriptionShelfRenderer") {
                let text = runs_text(&shelf["description"]);
                if !text.trim().is_empty() {
                    *out = Some(text);
                    return;
                }
            }
            for v in map.values() {
                find_description_shelf(v, out);
                if out.is_some() {
                    return;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                find_description_shelf(v, out);
                if out.is_some() {
                    return;
                }
            }
        }
        _ => {}
    }
}

pub fn parse_search_suggestions_json(resp: &Value) -> Vec<String> {
    let mut out = Vec::new();
    collect_suggestions(resp, &mut out);
    out
}

fn collect_suggestions(val: &Value, out: &mut Vec<String>) {
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("searchSuggestionRenderer") {
                let text = runs_text(&r["suggestion"]);
                if !text.is_empty() {
                    out.push(text);
                }
                return;
            }
            for v in map.values() {
                collect_suggestions(v, out);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_suggestions(v, out);
            }
        }
        _ => {}
    }
}

fn find_header_title_and_thumbs(resp: &Value) -> (String, Vec<(String, u32)>) {
    let header_candidates = [
        &resp["header"]["musicResponsiveHeaderRenderer"],
        &resp["header"]["musicDetailHeaderRenderer"],
        &resp["header"]["musicImmersiveHeaderRenderer"],
        &resp["contents"]["twoColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]
            ["sectionListRenderer"]["contents"][0]["musicResponsiveHeaderRenderer"],
    ];
    for h in header_candidates {
        if h.is_object() {
            let title = runs_text(&h["title"]);
            let thumbs = extract_thumbs(&h["thumbnail"]);
            if !title.is_empty() {
                return (title, thumbs);
            }
        }
    }
    (String::new(), Vec::new())
}

pub fn parse_album_browse_json(resp: &Value) -> Result<(String, Vec<Track>), String> {
    let (title, covers) = find_header_title_and_thumbs(resp);
    let large_cover = pick_cover_thumbs(&covers, 544);
    let small_cover = pick_cover_thumbs(&covers, 240);

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    let mut tracks = Vec::new();
    for (_shelf_title, items) in shelves {
        for entry in items {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer")
                && let Some(item) = parse_responsive_list_item(r)
                && item.kind == TrackKind::Track
                && !item.unavailable
            {
                let mut tr = item.track;
                if tr.album.is_none() && !title.is_empty() {
                    tr.album = Some(title.clone());
                }
                if large_cover.is_some() {
                    tr.cover_large_url = large_cover.clone();
                }
                if tr.cover_url.is_none() {
                    tr.cover_url = small_cover.clone();
                }
                tracks.push(tr);
            }
        }
    }
    if title.is_empty() && tracks.is_empty() {
        return Err(String::from("Error cargando álbum"));
    }
    Ok((
        if title.is_empty() {
            String::from("Álbum")
        } else {
            title
        },
        tracks,
    ))
}

fn extract_continuation(val: &Value) -> Option<String> {
    match val {
        Value::Object(map) => {
            if let Some(tok) = map
                .get("nextContinuationData")
                .and_then(|d| d.get("continuation"))
                .and_then(Value::as_str)
            {
                return Some(tok.to_string());
            }
            if let Some(tok) = map
                .get("continuationEndpoint")
                .and_then(|e| e.get("continuationCommand"))
                .and_then(|c| c.get("token"))
                .and_then(Value::as_str)
            {
                return Some(tok.to_string());
            }
            for v in map.values() {
                if let Some(tok) = extract_continuation(v) {
                    return Some(tok);
                }
            }
            None
        }
        Value::Array(arr) => arr.iter().find_map(extract_continuation),
        _ => None,
    }
}

pub fn parse_playlist_browse_json(resp: &Value) -> PlaylistBrowsePage {
    let (title, _) = find_header_title_and_thumbs(resp);
    let mut items = Vec::new();
    let mut had_entries = false;

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);
    for (_shelf_title, entries) in shelves {
        for entry in entries {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer") {
                had_entries = true;
                if let Some(item) = parse_responsive_list_item(r) {
                    items.push(item);
                }
            }
        }
    }
    let exhausted = extract_continuation(resp).is_none();
    PlaylistBrowsePage {
        title: if title.is_empty() {
            String::from("Playlist")
        } else {
            title
        },
        items,
        had_entries,
        exhausted,
    }
}

fn parse_playlist_continuation_items(resp: &Value) -> Vec<CatalogItem> {
    let mut items = Vec::new();
    collect_responsive_items_deep(resp, &mut items);
    items
}

fn collect_responsive_items_deep(val: &Value, out: &mut Vec<CatalogItem>) {
    match val {
        Value::Object(map) => {
            if let Some(r) = map.get("musicResponsiveListItemRenderer") {
                if let Some(item) = parse_responsive_list_item(r) {
                    out.push(item);
                }
                return;
            }
            for v in map.values() {
                collect_responsive_items_deep(v, out);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_responsive_items_deep(v, out);
            }
        }
        _ => {}
    }
}

pub fn parse_artist_enrichment_json(artist_id: &str, resp: &Value) -> ArtistEnrichment {
    let mut out = ArtistEnrichment::default();

    let header = if resp["header"]["musicImmersiveHeaderRenderer"].is_object() {
        &resp["header"]["musicImmersiveHeaderRenderer"]
    } else {
        &resp["header"]["musicVisualHeaderRenderer"]
    };

    if header.is_object() {
        let name = runs_text(&header["title"]);
        if !name.is_empty() {
            out.name = Some(name);
        }
        let desc = runs_text(&header["description"]);
        if !desc.is_empty() {
            out.description = Some(desc);
        }
        let sub_text = runs_text(
            &header["subscriptionButton"]["subscribeButtonRenderer"]["subscriberCountText"],
        );
        out.subscriber_count = parse_compact_count(&sub_text);
        let thumbs = extract_thumbs(&header["thumbnail"]);
        out.header_url = pick_cover_thumbs(&thumbs, 1000);
    }

    let mut shelves = Vec::new();
    collect_shelves(resp, &mut shelves);

    let mut seen_tracks = HashSet::new();
    let mut seen_videos = HashSet::new();
    let mut seen_albums = HashSet::new();
    let mut seen_singles = HashSet::new();
    let mut seen_playlists = HashSet::new();
    let mut seen_similar = HashSet::new();

    for (shelf_title, entries) in shelves {
        let lower = shelf_title.to_lowercase();
        let is_singles_shelf =
            lower.contains("single") || lower.contains("sencillo") || lower.contains("ep");

        for entry in entries {
            if let Some(r) = entry.get("musicResponsiveListItemRenderer")
                && let Some(item) = parse_responsive_list_item(r)
            {
                if let Some(vc) = item.track.view_count {
                    out.view_counts.insert(item.track.id.clone(), vc);
                }
                if !item.unavailable {
                    if item.kind == TrackKind::Video {
                        if seen_videos.insert(item.track.id.clone()) {
                            let mut tr = item.track;
                            tr.is_music = true;
                            tr.is_video = true;
                            if tr.artist_id.is_none() {
                                tr.artist_id = Some(artist_id.to_string());
                            }
                            out.videos.push(tr);
                        }
                    } else if seen_tracks.insert(item.track.id.clone()) {
                        out.tracks.push(item.track);
                    }
                }
            } else if let Some(r) = entry.get("musicTwoRowItemRenderer") {
                if let Some((album, is_single_badge)) = parse_two_row_album(r) {
                    if is_singles_shelf || is_single_badge {
                        if seen_singles.insert(album.id.clone()) {
                            out.singles.push(album);
                        }
                    } else if seen_albums.insert(album.id.clone()) {
                        out.albums.push(album);
                    }
                } else if let Some(pl) = parse_two_row_playlist(r) {
                    if seen_playlists.insert(pl.id.clone()) {
                        out.playlists.push(pl);
                    }
                } else if let Some(artist) = parse_two_row_artist(r) {
                    if seen_similar.insert(artist.id.clone()) {
                        out.similar_artists.push(artist);
                    }
                } else if let Some(item) = parse_two_row_track(r)
                    && !item.unavailable
                    && seen_videos.insert(item.track.id.clone())
                {
                    let mut tr = item.track;
                    tr.is_music = true;
                    tr.is_video = true;
                    if tr.artist_id.is_none() {
                        tr.artist_id = Some(artist_id.to_string());
                    }
                    out.videos.push(tr);
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_compact_counts() {
        assert_eq!(parse_compact_count("1.5M plays"), Some(1_500_000));
        assert_eq!(parse_compact_count("420K views"), Some(420_000));
        assert_eq!(parse_compact_count("2.3B subscribers"), Some(2_300_000_000));
        assert_eq!(parse_compact_count("9,812 plays"), Some(9_812));
    }

    #[test]
    fn parses_music_details_tabs_and_track_kind() {
        let sample = json!({
            "contents": {
                "singleColumnMusicWatchNextResultsRenderer": {
                    "tabbedRenderer": {
                        "watchNextTabbedResultsRenderer": {
                            "tabs": [
                                {
                                    "tabRenderer": {
                                        "content": {
                                            "musicQueueRenderer": {
                                                "content": {
                                                    "playlistPanelRenderer": {
                                                        "contents": [
                                                            {
                                                                "playlistPanelVideoRenderer": {
                                                                    "videoId": "audio123456",
                                                                    "title": { "runs": [{ "text": "Song Title" }] },
                                                                    "longBylineText": {
                                                                        "runs": [
                                                                            {
                                                                                "text": "Artist One",
                                                                                "navigationEndpoint": {
                                                                                    "browseEndpoint": { "browseId": "UC12345" }
                                                                                }
                                                                            },
                                                                            { "text": " • " },
                                                                            {
                                                                                "text": "Great Album",
                                                                                "navigationEndpoint": {
                                                                                    "browseEndpoint": { "browseId": "MPREb_abc" }
                                                                                }
                                                                            }
                                                                        ]
                                                                    },
                                                                    "lengthText": { "runs": [{ "text": "3:45" }] },
                                                                    "thumbnail": {
                                                                        "thumbnails": [{ "url": "https://lh3.googleusercontent.com/x=w120-h120", "width": 120 }]
                                                                    },
                                                                    "navigationEndpoint": {
                                                                        "watchEndpoint": {
                                                                            "videoId": "audio123456",
                                                                            "watchEndpointMusicSupportedConfigs": {
                                                                                "watchEndpointMusicConfig": {
                                                                                    "musicVideoType": "MUSIC_VIDEO_TYPE_ATV"
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                {
                                    "tabRenderer": {
                                        "endpoint": { "browseEndpoint": { "browseId": "MPLYt_lyrics_1" } }
                                    }
                                },
                                {
                                    "tabRenderer": {
                                        "endpoint": { "browseEndpoint": { "browseId": "MPTRt_related_1" } }
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        });

        let details = parse_music_details_json("audio123456", &sample).unwrap();
        assert_eq!(details.item.kind, TrackKind::Track);
        assert!(!details.item.unavailable);
        assert_eq!(details.item.track.title, "Song Title");
        assert_eq!(details.item.track.artist, "Artist One");
        assert_eq!(details.item.track.album.as_deref(), Some("Great Album"));
        assert_eq!(details.item.track.duration_secs, Some(225));
        assert_eq!(details.lyrics_browse_id.as_deref(), Some("MPLYt_lyrics_1"));
        assert_eq!(details.related_browse_id.as_deref(), Some("MPTRt_related_1"));
    }

    #[test]
    fn parses_search_suggestions() {
        let sample = json!({
            "contents": [{
                "searchSuggestionsSectionRenderer": {
                    "contents": [
                        { "searchSuggestionRenderer": { "suggestion": { "runs": [{ "text": "ram" }, { "text": "mstein" }] } } },
                        { "searchSuggestionRenderer": { "suggestion": { "runs": [{ "text": "ramon ayala" }] } } }
                    ]
                }
            }]
        });
        assert_eq!(
            parse_search_suggestions_json(&sample),
            vec!["rammstein", "ramon ayala"]
        );
    }

    #[test]
    fn parses_lyrics_description_shelf() {
        let sample = json!({
            "contents": {
                "sectionListRenderer": {
                    "contents": [{
                        "musicDescriptionShelfRenderer": {
                            "description": { "runs": [{ "text": "Line 1\nLine 2" }] }
                        }
                    }]
                }
            }
        });
        assert_eq!(
            parse_lyrics_browse_json(&sample).as_deref(),
            Some("Line 1\nLine 2")
        );
    }
}
