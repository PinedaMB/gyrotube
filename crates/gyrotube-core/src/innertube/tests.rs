#![allow(clippy::module_inception)]
use super::*;

#[cfg(test)]
mod tests {
    #![allow(clippy::module_inception)]
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

    #[test]
    fn preferred_profile_moves_to_front_and_deduplicates() {
        let client = InnerTubeClient::new(reqwest::Client::new());
        assert_eq!(client.ordered_profiles()[0], "ANDROID_VR");

        client.record_working_profile("VISIONOS");
        let ordered = client.ordered_profiles();
        assert_eq!(ordered[0], "VISIONOS");
        assert_eq!(ordered.len(), DEFAULT_PLAYER_PROFILES.len());
        let unique: HashSet<_> = ordered.iter().copied().collect();
        assert_eq!(unique.len(), ordered.len());
    }

    #[tokio::test]
    async fn details_cache_avoids_duplicate_lookups() {
        let dead_http = crate::tls::http_client_builder()
            .timeout(Duration::from_millis(1))
            .build()
            .unwrap();
        let client = InnerTubeClient::new(dead_http);
        let sample = MusicTrackDetails {
            item: CatalogItem {
                track: Track {
                    is_music: true,
                    is_video: false,
                    id: "cached12345".into(),
                    title: "Cached Song".into(),
                    artist: "Artist".into(),
                    artist_id: None,
                    album: None,
                    album_id: None,
                    duration_secs: Some(200),
                    view_count: None,
                    cover_url: None,
                    cover_large_url: None,
                },
                kind: TrackKind::Track,
                unavailable: false,
            },
            covers: vec![],
            lyrics_browse_id: Some("MPLYt_cached".into()),
            related_browse_id: Some("MPTRt_cached".into()),
            counterparts: vec![],
        };
        client
            .details_cache
            .write()
            .unwrap()
            .insert("cached12345".into(), (Instant::now(), sample));

        let hit = client.fetch_music_details("cached12345").await.unwrap();
        assert_eq!(hit.item.track.title, "Cached Song");
        assert_eq!(hit.related_browse_id.as_deref(), Some("MPTRt_cached"));
    }

    #[test]
    fn parses_wrapped_song_and_video_counterparts_both_ways() {
        let wrapped = json!({
            "playlistPanelVideoWrapperRenderer": {
                "primaryRenderer": {
                    "playlistPanelVideoRenderer": {
                        "videoId": "atvSong1234",
                        "title": { "runs": [{ "text": "Studio Song" }] },
                        "shortBylineText": { "runs": [{ "text": "Artist" }] },
                        "navigationEndpoint": {
                            "watchEndpoint": {
                                "videoId": "atvSong1234",
                                "watchEndpointMusicSupportedConfigs": {
                                    "watchEndpointMusicConfig": { "musicVideoType": "MUSIC_VIDEO_TYPE_ATV" }
                                }
                            }
                        }
                    }
                },
                "counterpart": [{
                    "counterpartRenderer": {
                        "playlistPanelVideoRenderer": {
                            "videoId": "omvVid12345",
                            "title": { "runs": [{ "text": "Studio Song (Official Video)" }] },
                            "shortBylineText": { "runs": [{ "text": "Artist" }] },
                            "navigationEndpoint": {
                                "watchEndpoint": {
                                    "videoId": "omvVid12345",
                                    "watchEndpointMusicSupportedConfigs": {
                                        "watchEndpointMusicConfig": { "musicVideoType": "MUSIC_VIDEO_TYPE_OMV" }
                                    }
                                }
                            }
                        }
                    }
                }]
            }
        });

        let from_song = parse_music_details_json("atvSong1234", &wrapped).unwrap();
        assert_eq!(from_song.item.kind, TrackKind::Track);
        assert_eq!(from_song.counterparts.len(), 1);
        assert_eq!(from_song.counterparts[0].track.id, "omvVid12345");
        assert_eq!(from_song.counterparts[0].kind, TrackKind::Video);

        let from_video = parse_music_details_json("omvVid12345", &wrapped).unwrap();
        assert_eq!(from_video.item.kind, TrackKind::Video);
        assert_eq!(from_video.counterparts.len(), 1);
        assert_eq!(from_video.counterparts[0].track.id, "atvSong1234");
        assert_eq!(from_video.counterparts[0].kind, TrackKind::Track);
    }

    #[test]
    fn parses_android_music_timed_lyrics() {
        let sample = json!({
            "contents": {
                "elementRenderer": {
                    "newElement": {
                        "type": {
                            "componentType": {
                                "model": {
                                    "timedLyricsModel": {
                                        "lyricsData": {
                                            "timedLyricsData": [
                                                { "lyricLine": "First line", "cueRange": { "startTimeMilliseconds": "12500" } },
                                                { "lyricLine": "Second line", "cueRange": { "startTimeMilliseconds": "18200" } }
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        match parse_timed_lyrics_browse_json(&sample) {
            Some(LyricsData::Synced(lines)) => {
                assert_eq!(lines.len(), 2);
                assert!((lines[0].time_secs - 12.5).abs() < 0.001);
                assert_eq!(lines[0].text, "First line");
                assert!((lines[1].time_secs - 18.2).abs() < 0.001);
            }
            other => panic!("expected Synced lyrics, got {other:?}"),
        }
    }
}

#[cfg(test)]
mod live_tests {
    #![allow(clippy::module_inception)]
use super::*;

    fn live_client() -> InnerTubeClient {
        crate::tls::ensure_crypto_provider();
        let http = crate::tls::http_client_builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        InnerTubeClient::new(http)
    }

    #[tokio::test]
    #[ignore = "Live InnerTube smoke test (requires internet)"]
    async fn smoke_all_innertube_endpoints() {
        let client = live_client();
        let vd = client.visitor_data(true).await;
        println!("visitor_data: {:?}", vd.as_deref().map(|s| &s[..s.len().min(16)]));
        assert!(vd.is_some());

        let charts = client.fetch_charts(None).await.expect("charts");
        println!(
            "charts: trending={} top={} artists={} countries={}",
            charts.trending.len(),
            charts.top.len(),
            charts.artists.len(),
            charts.available_countries.len()
        );
        assert!(!charts.trending.is_empty());

        let albums = client.fetch_new_albums().await.expect("new_albums");
        println!("new_albums: {}", albums.len());
        assert!(!albums.is_empty());

        let sample_id = &charts.trending[0].id;
        let details = client.fetch_music_details(sample_id).await.expect("music_details");
        println!(
            "details: {} (lyrics={:?}, related={:?})",
            details.item.track.title, details.lyrics_browse_id, details.related_browse_id
        );

        let radio = client.fetch_radio_track(sample_id).await.expect("radio");
        println!("radio: {} tracks", radio.len());
        assert!(!radio.is_empty());

        if let Some(ref rel_id) = details.related_browse_id {
            let related = client.fetch_related(rel_id).await.expect("related");
            println!(
                "related: tracks={} albums={} playlists={} artists={}",
                related.tracks.len(),
                related.albums.len(),
                related.playlists.len(),
                related.artists.len()
            );
        }

        let mut any_stream_ok = false;
        for profile in client.ordered_profiles() {
            let res = client
                .extract_direct_stream(sample_id, profile, vd.as_deref(), Duration::from_secs(6), None)
                .await;
            println!("stream[{profile}]: {:?}", res.as_ref().map(|(s, ext, _)| (&s.quality_label, ext)));
            if res.is_ok() {
                any_stream_ok = true;
            }
        }
        assert!(any_stream_ok, "At least one InnerTube stream profile must succeed");
    }
}
