use super::*;

impl InnerTubeClient {
    async fn post_player(
        &self,
        video_id: &str,
        profile: &str,
        visitor_data: Option<&str>,
        timeout: Duration,
    ) -> Result<(Value, &'static str), String> {
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
            "WEB" => (
                YTM_USER_AGENT,
                "1",
                "2.20250310.01.00",
                json!({
                    "context": { "client": {
                        "clientName": "WEB",
                        "clientVersion": "2.20250310.01.00",
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

        Ok((response, user_agent))
    }

    pub async fn extract_direct_stream(
        &self,
        video_id: &str,
        profile: &str,
        visitor_data: Option<&str>,
        timeout: Duration,
        require_itag: Option<u64>,
    ) -> Result<(StreamInfo, &'static str, Option<u64>), String> {
        let (response, user_agent) = self
            .post_player(video_id, profile, visitor_data, timeout)
            .await?;

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

        #[cfg(feature = "native-audio")]
        let candidates: &[(u64, &str)] = match self.audio_quality() {
            AudioQuality::High => &[
                (141, "m4a"),
                (251, "webm"),
                (140, "m4a"),
                (250, "webm"),
                (139, "m4a"),
                (18, "mp4"),
            ],
            AudioQuality::Normal => &[
                (250, "webm"),
                (140, "m4a"),
                (249, "webm"),
                (251, "webm"),
                (139, "m4a"),
                (18, "mp4"),
            ],
            AudioQuality::Low => &[
                (249, "webm"),
                (139, "m4a"),
                (250, "webm"),
                (140, "m4a"),
                (18, "mp4"),
            ],
        };

        #[cfg(all(target_os = "macos", not(feature = "native-audio")))]
        let candidates: &[(u64, &str)] = match self.audio_quality() {
            AudioQuality::High => &[
                (141, "m4a"),
                (140, "m4a"),
                (139, "m4a"),
                (18, "mp4"),
                (251, "webm"),
            ],
            AudioQuality::Normal => &[
                (140, "m4a"),
                (139, "m4a"),
                (18, "mp4"),
                (250, "webm"),
            ],
            AudioQuality::Low => &[
                (139, "m4a"),
                (140, "m4a"),
                (18, "mp4"),
                (249, "webm"),
            ],
        };

        #[cfg(not(any(target_os = "macos", feature = "native-audio")))]
        let candidates: &[(u64, &str)] = match self.audio_quality() {
            AudioQuality::High => &[
                (251, "webm"),
                (140, "m4a"),
                (250, "webm"),
                (249, "webm"),
                (139, "m4a"),
                (18, "mp4"),
            ],
            AudioQuality::Normal => &[
                (250, "webm"),
                (249, "webm"),
                (140, "m4a"),
                (251, "webm"),
                (139, "m4a"),
                (18, "mp4"),
            ],
            AudioQuality::Low => &[
                (249, "webm"),
                (139, "m4a"),
                (250, "webm"),
                (140, "m4a"),
                (251, "webm"),
                (18, "mp4"),
            ],
        };

        for &(itag, extension) in candidates {
            if let Some(req_itag) = require_itag
                && itag != req_itag
            {
                continue;
            }
            let Some(format) = all_formats.iter().find(|f| {
                f["itag"].as_u64() == Some(itag)
                    && (f["url"].is_string()
                        || f["signatureCipher"].is_string()
                        || f["cipher"].is_string())
            }) else {
                continue;
            };
            let direct_url = format["url"].as_str();
            let sig_cipher = format["signatureCipher"]
                .as_str()
                .or_else(|| format["cipher"].as_str());
            let resolved_url = if let Some(u) = direct_url
                && profile != "WEB"
            {
                u.to_string()
            } else {
                let Ok(deciphered) = js_solver::JsChallengeSolver::global()
                    .decipher_format_url(&self.http, direct_url, sig_cipher, profile == "WEB")
                    .await
                else {
                    continue;
                };
                deciphered
            };
            let Some(url) = reqwest::Url::parse(&resolved_url).ok() else {
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
            let kbps = (format["bitrate"].as_u64().unwrap_or(128_000) / 1000).max(48);
            let loudness_db = response["playerConfig"]["audioConfig"]["loudnessDb"]
                .as_f64()
                .map(|v| v as f32)
                .filter(|v| v.is_finite());
            let mut headers = HashMap::new();
            headers.insert("User-Agent".to_string(), user_agent.to_string());
            return Ok((
                StreamInfo {
                    url: url.into(),
                    quality_label: format!("{kbps}kbps ({})", extension.to_uppercase()),
                    user_agent: Some(user_agent.into()),
                    headers,
                    local_audio: None,
                    loudness_db,
                },
                extension,
                size,
            ));
        }
        Err("El reproductor no ofreció una URL directa de audio compatible".into())
    }

    pub async fn extract_direct_video_stream(
        &self,
        video_id: &str,
        max_height: u32,
        visitor_data: Option<&str>,
    ) -> Result<DirectVideoCandidate, String> {
        for profile in self.ordered_profiles() {
            let Ok((json, user_agent)) = self
                .post_player(video_id, profile, visitor_data, Duration::from_secs(6))
                .await
            else {
                continue;
            };
            let adaptive = json["streamingData"]["adaptiveFormats"].as_array();
            let muxed = json["streamingData"]["formats"].as_array();
            let all_formats: Vec<&Value> = adaptive
                .into_iter()
                .flatten()
                .chain(muxed.into_iter().flatten())
                .collect();

            let mut best_webm: Option<(u32, u64, String, u32, u32)> = None;
            let mut best_mp4: Option<(u32, u64, String, u32, u32)> = None;
            for f in all_formats {
                let Some(mime) = f["mimeType"].as_str() else { continue };
                let is_webm_vp9 = mime.starts_with("video/webm")
                    && (mime.contains("vp9") || mime.contains("vp09") || mime.contains("vp8"));
                let is_mp4_h264 = mime.starts_with("video/mp4") && mime.contains("avc1");
                if !is_webm_vp9 && !is_mp4_h264 {
                    continue;
                }
                let Some(url_str) = f["url"].as_str() else { continue };
                let Some(parsed) = reqwest::Url::parse(url_str).ok() else { continue };
                if parsed.scheme() != "https"
                    || !parsed
                        .host_str()
                        .is_some_and(|h| h.ends_with(".googlevideo.com"))
                {
                    continue;
                }
                let height = f["height"].as_u64().unwrap_or(0) as u32;
                let width = f["width"].as_u64().unwrap_or(0) as u32;
                if height == 0 || height > max_height {
                    continue;
                }
                let bitrate = f["bitrate"].as_u64().unwrap_or(0);
                let slot = if is_webm_vp9 {
                    &mut best_webm
                } else {
                    &mut best_mp4
                };
                let replace = match slot {
                    None => true,
                    Some((bh, bb, _, _, _)) => height > *bh || (height == *bh && bitrate > *bb),
                };
                if replace {
                    *slot = Some((height, bitrate, parsed.into(), width, height));
                }
            }

            #[cfg(target_os = "linux")]
            let chosen = best_webm
                .map(|v| (v, "video/webm"))
                .or_else(|| best_mp4.map(|v| (v, "video/mp4")));
            #[cfg(not(target_os = "linux"))]
            let chosen = best_mp4
                .map(|v| (v, "video/mp4"))
                .or_else(|| best_webm.map(|v| (v, "video/webm")));

            if let Some(((_height, _, raw_url, width, height), mime_type)) = chosen {
                let probe_ok = self
                    .http
                    .get(&raw_url)
                    .timeout(Duration::from_secs(4))
                    .header("User-Agent", user_agent)
                    .header("Range", "bytes=0-0")
                    .send()
                    .await
                    .is_ok_and(|r| r.status().is_success() || r.status().as_u16() == 206);
                if !probe_ok {
                    continue;
                }
                self.record_working_profile(profile);
                return Ok(DirectVideoCandidate {
                    url: raw_url,
                    user_agent: user_agent.to_string(),
                    mime_type,
                    width,
                    height,
                });
            }
        }
        Err("Video no disponible para esta pista".into())
    }

}
