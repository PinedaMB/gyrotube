use super::*;
use super::parsers::*;

    #[tokio::test]
    async fn playback_reuses_a_url_published_by_an_in_flight_preload() {
        use std::future::{Future, poll_fn};
        use std::task::Poll;

        let backend = Backend::new();
        let id = "timingTest_";
        let preload = backend.audio_lock(id, AudioWork::Resolve).unwrap();
        let guard = preload.lock().await;
        let mut playback = std::pin::pin!(backend.resolve_audio_stream(id));
        assert!(poll_fn(|cx| Poll::Ready(playback.as_mut().poll(cx).is_pending())).await);
        let stream = StreamInfo {
            url: "https://example.com/preloaded-audio".into(),
            quality_label: "OPUS".into(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };
        backend
            .stream_cache
            .write()
            .unwrap()
            .insert(id.into(), (Instant::now(), stream.clone()));
        drop(guard);
        assert_eq!(playback.await.unwrap().url, stream.url);
    }

    #[test]
    fn unverified_tracks_cannot_use_a_cached_url() {
        let backend = Backend::new();
        let mut track: Track = serde_json::from_value(serde_json::json!({
            "id":"video123456", "title":"Song", "artist":"Artist"
        }))
        .unwrap();
        backend.stream_cache.write().unwrap().insert(
            track.id.clone(),
            (
                Instant::now(),
                StreamInfo {
                    url: "https://example.com/audio".into(),
                    quality_label: "OPUS".into(),
                    user_agent: None,
                    headers: HashMap::new(),
                    local_audio: None,
                },
            ),
        );
        assert!(backend.cached_track_stream(&track).is_none());
        track.is_music = true;
        assert!(backend.cached_track_stream(&track).is_some());
    }

    #[test]
    fn cached_url_for_a_search_result_requires_a_fresh_exact_id_proof() {
        let backend = Backend::new();
        let track: Track = serde_json::from_value(serde_json::json!({
            "id": "timingTest_", "title": "Song", "artist": "Artist"
        })).unwrap();
        let url = "https://example.com/audio";
        backend.stream_cache.write().unwrap().insert(
            track.id.clone(),
            (Instant::now(), StreamInfo {
                url: url.into(),
                quality_label: "OPUS".into(),
                user_agent: None,
                headers: HashMap::new(),
                local_audio: None,
            }),
        );
        assert!(backend.cached_track_stream(&track).is_none());

        let mut proof = track.clone();
        proof.is_music = true;
        let mut other = track.clone();
        other.id = "anotherTest".into();
        backend.music_proofs.write().unwrap().insert(
            other.id.clone(),
            (Instant::now(), other),
        );
        assert!(backend.cached_track_stream(&track).is_none());

        backend.music_proofs.write().unwrap().insert(
            track.id.clone(),
            (Instant::now(), proof.clone()),
        );
        assert_eq!(backend.cached_track_stream(&track).unwrap().url, url);
        backend.music_proofs.write().unwrap().insert(
            track.id.clone(),
            (Instant::now() - MUSIC_PROOF_TTL, proof),
        );
        assert!(backend.cached_track_stream(&track).is_none());
    }

    #[tokio::test]
    async fn cached_audio_cannot_bypass_failed_catalog_verification() {
        let mut backend = Backend::new();
        let dead_http = crate::tls::http_client_builder()
            .timeout(Duration::from_millis(1))
            .build()
            .unwrap();
        backend.innertube = InnerTubeClient::new(dead_http);
        let mut track: Track = serde_json::from_value(serde_json::json!({
            "id":"video123456", "title":"Song", "artist":"Artist"
        }))
        .unwrap();
        backend.stream_cache.write().unwrap().insert(
            track.id.clone(),
            (
                Instant::now(),
                StreamInfo {
                    url: "https://example.com/audio".into(),
                    quality_label: "OPUS".into(),
                    user_agent: None,
                    headers: HashMap::new(),
                    local_audio: None,
                },
            ),
        );
        assert!(backend.resolve_track_stream(&track).await.is_err());
        track.is_music = true;
        assert_eq!(
            backend.resolve_track_stream(&track).await.unwrap().url,
            "https://example.com/audio"
        );
    }

    #[test]
    fn guest_claims_do_not_override_catalog_metadata_or_type() {
        let claimed: Track = serde_json::from_value(serde_json::json!({
            "id":"audio123456", "title":"Forged", "artist":"Forged",
            "verified_catalog_audio":true, "cover_url":"http://127.0.0.1/private"
        }))
        .unwrap();
        let mut item = CatalogItem {
            track: Track {
                is_music: true,
                is_video: false,
                id: claimed.id.clone(),
                title: "Catalog song".into(),
                artist: "Artista desconocido".into(),
                artist_id: None,
                album: None,
                album_id: None,
                duration_secs: None,
                view_count: None,
                cover_url: None,
                cover_large_url: None,
            },
            kind: TrackKind::Video,
            unavailable: false,
        };
        assert!(verified_music_track(&claimed.id, &item).is_err());
        item.kind = TrackKind::Episode;
        assert!(verified_music_track(&claimed.id, &item).is_err());
        item.kind = TrackKind::Track;
        item.unavailable = true;
        assert!(verified_music_track(&claimed.id, &item).is_err());
        item.unavailable = false;
        assert!(verified_music_track("other123456", &item).is_err());
        let verified = verified_music_track(&claimed.id, &item).unwrap();
        assert_eq!(verified.title, "Catalog song");
        assert!(verified.is_music);
        assert_eq!(verified.cover_url, None);
    }

    #[test]
    fn playlist_browse_requests_require_exactly_one_vl_prefix() {
        assert_eq!(playlist_browse_id("OLAK-test").get_raw(), "VLOLAK-test");
        assert_eq!(playlist_browse_id("VLOLAK-test").get_raw(), "VLOLAK-test");
    }

    #[test]
    fn audio_playlist_excludes_clips_and_unavailable_songs() {
        let song = serde_json::json!({"Song": {
            "video_id":"audio123456", "track_no":1,
            "album":{"name":"Album", "id":"MPRE-test"},
            "duration":"3:00", "library_management":null, "title":"Song",
            "artists":[{"name":"Artist", "id":null}], "like_status":"INDIFFERENT",
            "thumbnails":[], "explicit":"NotExplicit", "is_available":true,
            "playlist_id":"OLAK-test"
        }});
        let mut unavailable = song.clone();
        unavailable["Song"]["is_available"] = serde_json::json!(false);
        let video = serde_json::json!({"Video": {
            "video_id":"video123456", "track_no":1, "duration":"4:00", "title":"Song",
            "channel_name":"Artist", "channel_id":"UC-test", "like_status":"INDIFFERENT",
            "thumbnails":[], "is_available":true, "playlist_id":"OLAK-test"
        }});
        let items: Vec<PlaylistItem> =
            serde_json::from_value(serde_json::json!([video, song, unavailable])).unwrap();
        let tracks = tracks_from_ytm_playlist(items.clone());
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].id, "audio123456");
        assert_eq!(tracks[0].duration_secs, Some(180));
        assert!(tracks[0].is_music);
        assert!(!tracks[0].is_video);

        let with_videos = tracks_from_ytm_playlist_with_videos(items);
        assert_eq!(with_videos.len(), 2);
        assert_eq!(with_videos[0].id, "video123456");
        assert!(with_videos[0].is_music);
        assert!(with_videos[0].is_video);
        assert_eq!(with_videos[1].id, "audio123456");
        assert!(!with_videos[1].is_video);
    }

    #[test]
    fn playlists_with_filtered_or_unchecked_entries_are_not_reported_as_empty() {
        assert!(
            playlist_audio_result("Lista".into(), vec![], false, true)
                .unwrap()
                .1
                .is_empty()
        );
        let filtered = playlist_audio_result("Lista".into(), vec![], true, true).unwrap_err();
        let incomplete = playlist_audio_result("Lista".into(), vec![], true, false).unwrap_err();
        assert_ne!(filtered, incomplete);
        assert!(playlist_audio_result("Lista".into(), vec![], false, false).is_err());

        let mut item = CatalogItem {
            track: Track {
                is_music: true,
                is_video: false,
                id: "audio123456".into(),
                title: "Song".into(),
                artist: "Artista desconocido".into(),
                artist_id: None,
                album: None,
                album_id: None,
                duration_secs: None,
                view_count: None,
                cover_url: None,
                cover_large_url: None,
            },
            kind: TrackKind::Video,
            unavailable: false,
        };
        assert!(!is_playlist_audio(&item));
        item.kind = TrackKind::Track;
        let tracks = [&item]
            .into_iter()
            .filter(|t| is_playlist_audio(t))
            .map(|t| t.track.clone())
            .collect();
        let (_, tracks) = playlist_audio_result("Lista".into(), tracks, true, false).unwrap();
        assert_eq!(tracks[0].id, "audio123456");
        item.unavailable = true;
        assert!(!is_playlist_audio(&item));
    }

    #[tokio::test]
    #[ignore = "Requires YouTube Music access"]
    async fn live_album_uses_audio_playlist() {
        let backend = Backend::new();
        let ytm = backend.primary().await.unwrap();
        let albums = ytm
            .search_albums("La Casa De Mis Sueños Santiago Benavides")
            .await
            .unwrap();
        let album = albums
            .iter()
            .find(|a| a.title.to_lowercase() == "la casa de mis sueños")
            .expect("album from saved session");
        let id = album.album_id.get_raw();
        let browse = ytm.get_album(AlbumID::from_raw(id)).await.unwrap();
        let audio_playlist = browse.audio_playlist_id.expect("album audio playlist");
        let expected = tracks_from_ytm_playlist(
            ytm.get_playlist_tracks(playlist_browse_id(audio_playlist.get_raw()))
                .await
                .unwrap(),
        );
        let (title, tracks) = backend.fetch_album_tracks(id).await.unwrap();
        assert!(!tracks.is_empty());
        for track in &tracks {
            assert!(expected.iter().any(|audio| audio.id == track.id));
            assert!(track.is_music);
        }
        let audio = tracks
            .iter()
            .find(|t| t.title.to_lowercase() == "me habla de amor")
            .unwrap();
        let old = backend
            .innertube
            .fetch_music_details("zuFs_gXr_Wk")
            .await
            .unwrap();
        let current = backend.innertube.fetch_music_details(&audio.id).await.unwrap();
        eprintln!(
            "Álbum: {title}; ID anterior: {} ({:?}); ID de audio: {} ({:?})",
            old.item.track.id, old.item.kind, current.item.track.id, current.item.kind
        );
        assert_eq!(current.item.kind, TrackKind::Track);
        if old.item.kind == TrackKind::Video {
            assert_ne!(old.item.track.id, audio.id);
        }
    }

    #[tokio::test]
    async fn home_cache_stores_multiple_countries_independently() {
        let backend = Backend::new();
        let make_feed = |cc: Option<&str>| HomeFeed {
            trending: vec![],
            top: vec![],
            new_albums: vec![],
            top_artists: vec![],
            country: cc.map(str::to_string),
            available_countries: vec!["MX".into(), "US".into()],
        };
        {
            let mut guard = backend.home_cache.write().unwrap();
            guard.insert(None, (Instant::now(), make_feed(None)));
            guard.insert(Some("MX".into()), (Instant::now(), make_feed(Some("MX"))));
        }
        let global = backend.fetch_home_feed_for_country(None, false).await.unwrap();
        let mx = backend.fetch_home_feed_for_country(Some("mx"), false).await.unwrap();
        assert_eq!(global.country, None);
        assert_eq!(mx.country.as_deref(), Some("MX"));
    }

    #[test]
    fn reuses_home_snapshot_only_while_fresh() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("home.json");
        assert!(load_home_cache(&path).is_none());
        let feed = HomeFeed {
            trending: vec![],
            top: vec![],
            new_albums: vec![],
            top_artists: vec![],
            country: None,
            available_countries: vec![],
        };
        std::fs::write(&path, serde_json::to_vec(&feed).unwrap()).unwrap();
        assert!(load_home_cache(&path).is_some());
        let file = std::fs::File::options().write(true).open(&path).unwrap();
        file.set_times(
            std::fs::FileTimes::new().set_modified(
                std::time::SystemTime::now() - HOME_CACHE_TTL - Duration::from_secs(1),
            ),
        )
        .unwrap();
        assert!(load_home_cache(&path).is_none());
        std::fs::write(&path, b"interrupted write").unwrap();
        assert!(load_home_cache(&path).is_none());
    }

    #[test]
    fn preloaded_url_survives_a_song_and_expires_safely() {
        let mut stream = StreamInfo {
            url: "https://example.com/audio".into(),
            quality_label: "OPUS".into(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };
        assert!(stream_url_is_fresh(
            Instant::now() - Duration::from_secs(240),
            &stream
        ));
        assert!(!stream_url_is_fresh(
            Instant::now() - STREAM_CACHE_TTL,
            &stream
        ));
        stream.url.push_str("?expire=123");
        assert!(!stream_url_is_fresh(Instant::now(), &stream));
    }

    #[test]
    fn extracts_audio_url_and_required_headers() {
        let stream = stream_from_ytdlp_json(
            br#"{
            "url":"https://rr1.googlevideo.com/videoplayback?expire=123",
            "ext":"webm","vcodec":"none","acodec":"opus",
            "http_headers":{"User-Agent":"test-agent","Referer":"https://music.youtube.com/"}
        }"#,
        )
        .unwrap();
        assert!(stream.url.starts_with("https://rr1.googlevideo.com/"));
        assert_eq!(stream.quality_label, "160kbps (WEBM)");
        assert_eq!(stream.user_agent.as_deref(), Some("test-agent"));
        assert_eq!(stream.headers["Referer"], "https://music.youtube.com/");
    }

    #[test]
    fn rejects_non_audio_or_non_http_extractor_output() {
        for (url, video, audio) in [
            ("file:///etc/passwd", "none", "opus"),
            ("https://example.com/", "avc1", "opus"),
            ("https://example.com/", "none", "none"),
        ] {
            let raw = serde_json::json!({"url":url,"vcodec":video,"acodec":audio,"ext":"webm"});
            assert!(stream_from_ytdlp_json(&serde_json::to_vec(&raw).unwrap()).is_err());
        }
        assert!(stream_from_ytdlp_json(b"{}").is_err());
    }

    #[test]
    fn rejects_invalid_ids_before_cache_or_process_access() {
        for id in ["../secret", "--help", "1234567890/", "", "123456789012"] {
            assert!(validate_video_id(id).is_err());
        }
        assert!(validate_video_id("Abc_123-xyz").is_ok());
    }

    #[test]
    fn selects_largest_cover_without_assuming_thumbnail_order() {
        let thumbs: Vec<ytmapi_rs::common::Thumbnail> = serde_json::from_value(serde_json::json!([
            {"url":"large", "width":800, "height":800},
            {"url":"small", "width":60, "height":60}
        ]))
        .unwrap();
        assert_eq!(best_ytm_thumb(&thumbs).as_deref(), Some("large"));
    }

    #[tokio::test]
    #[ignore = "Requires YouTube access and yt-dlp in PATH"]
    async fn live_search_cover_and_audio() {
        let backend = Backend::new();
        let start = Instant::now();
        let results = backend.search_all("Daft Punk Get Lucky").await.unwrap();
        let track = results.tracks.first().expect("music result");
        eprintln!("Búsqueda: {:?}, pista: {}", start.elapsed(), track.id);
        let cover = track
            .cover_large_url
            .as_deref()
            .or(track.cover_url.as_deref())
            .unwrap();
        let image = backend.fetch_image_bytes(cover).await.unwrap();
        assert!(image.len() > 256);
        let cold_start = Instant::now();
        let stream = backend.resolve_track_stream(track).await.unwrap();
        eprintln!("Resolución inicial de audio: {:?}", cold_start.elapsed());
        assert!(!stream.url.is_empty());
        let start = Instant::now();
        assert_eq!(
            backend.resolve_track_stream(track).await.unwrap().url,
            stream.url
        );
        eprintln!("Resolución con URL precargada: {:?}", start.elapsed());
    }

    #[test]
    fn find_cached_audio_reuses_local_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file = dir.path().join("dQw4w9WgXcQ.m4a");
        std::fs::write(&file, b"audio-bytes").expect("write cached audio");

        let found = audio_cache::find("dQw4w9WgXcQ", dir.path()).expect("cached audio");
        assert_eq!(found.url, file.to_string_lossy());
        assert_eq!(found.quality_label, "128kbps (M4A)");
    }

    #[test]
    fn parses_mm_ss_and_sub_counts() {
        assert_eq!(parse_mm_ss("3:45"), Some(225));
        assert_eq!(parse_mm_ss("1:02:03"), Some(3723));
        assert_eq!(parse_sub_count("9.4M subscribers"), Some(9_400_000));
        assert_eq!(parse_sub_count("120K"), Some(120_000));
        assert_eq!(parse_sub_count("176K monthly audience"), Some(176_000));
        assert_eq!(parse_sub_count("176 mil suscriptores"), Some(176_000));
        assert_eq!(parse_sub_count("1,5 M de suscriptores"), Some(1_500_000));
        assert_eq!(parse_sub_count("721 monthly listeners"), Some(721));
    }

    #[test]
    fn parses_mood_playlists_from_raw_json() {
        let json = serde_json::json!({
            "header": {
                "musicHeaderRenderer": {
                    "title": { "runs": [{ "text": "Cumbia" }] }
                }
            },
            "contents": {
                "singleColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "gridRenderer": {
                                            "items": [{
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Cumbia Sonidera" }] },
                                                    "subtitle": { "runs": [{ "text": "YouTube Music" }] },
                                                    "navigationEndpoint": {
                                                        "browseEndpoint": { "browseId": "VLRDCLAK5uy_test123" }
                                                    },
                                                    "thumbnailRenderer": {
                                                        "musicThumbnailRenderer": {
                                                            "thumbnail": {
                                                                "thumbnails": [
                                                                    { "url": "https://lh3.googleusercontent.com/abc=w226-h226", "width": 226, "height": 226 },
                                                                    { "url": "https://lh3.googleusercontent.com/abc=w544-h544", "width": 544, "height": 544 }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            }
        });

        let (name, playlists) = parse_mood_playlists_json(&json);
        assert_eq!(name, "Cumbia");
        assert_eq!(playlists.len(), 1);
        assert_eq!(playlists[0].id, "RDCLAK5uy_test123");
        assert_eq!(playlists[0].name, "Cumbia Sonidera");
        assert_eq!(playlists[0].subtitle, "YouTube Music");
        assert!(playlists[0].cover_url.is_some());
    }

    #[test]
    fn parses_official_watch_counterpart_song_to_video_and_back() {
        let make_renderer = |id: &str, title: &str, mtype: &str, with_album: bool| {
            let mut byline_runs = vec![serde_json::json!({
                "text": "Daft Punk",
                "navigationEndpoint": {
                    "browseEndpoint": {
                        "browseId": "UC_daftpunk123",
                        "browseEndpointContextSupportedConfigs": {
                            "browseEndpointContextMusicConfig": {
                                "pageType": "MUSIC_PAGE_TYPE_ARTIST"
                            }
                        }
                    }
                }
            })];
            if with_album {
                byline_runs.push(serde_json::json!({ "text": " • " }));
                byline_runs.push(serde_json::json!({
                    "text": "Random Access Memories",
                    "navigationEndpoint": {
                        "browseEndpoint": {
                            "browseId": "MPREb_ram123",
                            "browseEndpointContextSupportedConfigs": {
                                "browseEndpointContextMusicConfig": {
                                    "pageType": "MUSIC_PAGE_TYPE_ALBUM"
                                }
                            }
                        }
                    }
                }));
            }
            serde_json::json!({
                "videoId": id,
                "title": { "runs": [{ "text": title }] },
                "lengthText": { "runs": [{ "text": "4:08" }] },
                "longBylineText": { "runs": byline_runs },
                "shortBylineText": { "runs": [{ "text": "Daft Punk" }] },
                "navigationEndpoint": {
                    "watchEndpoint": {
                        "videoId": id,
                        "watchEndpointMusicSupportedConfigs": {
                            "watchEndpointMusicConfig": {
                                "musicVideoType": mtype
                            }
                        }
                    }
                },
                "thumbnail": {
                    "thumbnails": [
                        { "url": "https://lh3.googleusercontent.com/thumb=w226-h226", "width": 226, "height": 226 }
                    ]
                }
            })
        };

        let watch_json = serde_json::json!({
            "contents": {
                "singleColumnMusicWatchNextResultsRenderer": {
                    "tabbedRenderer": {
                        "watchNextTabbedResultsRenderer": {
                            "tabs": [{
                                "tabRenderer": {
                                    "content": {
                                        "musicQueueRenderer": {
                                            "content": {
                                                "playlistPanelRenderer": {
                                                    "contents": [{
                                                        "playlistPanelVideoWrapperRenderer": {
                                                            "primaryRenderer": {
                                                                "playlistPanelVideoRenderer": make_renderer(
                                                                    "atvSong1234",
                                                                    "Get Lucky",
                                                                    "MUSIC_VIDEO_TYPE_ATV",
                                                                    true
                                                                )
                                                            },
                                                            "counterpart": [{
                                                                "counterpartRenderer": {
                                                                    "playlistPanelVideoRenderer": make_renderer(
                                                                        "omvVid56789",
                                                                        "Get Lucky (Official Video)",
                                                                        "MUSIC_VIDEO_TYPE_OMV",
                                                                        false
                                                                    )
                                                                }
                                                            }]
                                                        }
                                                    }]
                                                }
                                            }
                                        }
                                    }
                                }
                            }]
                        }
                    }
                }
            }
        });

        let song = Track {
            is_music: true,
            is_video: false,
            id: "atvSong1234".into(),
            title: "Get Lucky".into(),
            artist: "Daft Punk".into(),
            artist_id: Some("UC_daftpunk123".into()),
            album: Some("Random Access Memories".into()),
            album_id: Some("MPREb_ram123".into()),
            duration_secs: Some(248),
            view_count: None,
            cover_url: None,
            cover_large_url: None,
        };

        // Song -> Video
        let mv = parse_watch_counterpart_json(&watch_json, &song, true)
            .expect("should resolve official OMV counterpart");
        assert_eq!(mv.id, "omvVid56789");
        assert!(mv.is_video);
        assert!(mv.is_music);
        assert_eq!(mv.title, "Get Lucky");
        assert_eq!(mv.artist, "Daft Punk");
        assert_eq!(mv.artist_id.as_deref(), Some("UC_daftpunk123"));
        assert_eq!(mv.duration_secs, Some(248));

        // Video -> Studio Audio (even when ATV is in primaryRenderer and OMV is in counterpart)
        let back_to_song = parse_watch_counterpart_json(&watch_json, &mv, false)
            .expect("should resolve official ATV counterpart");
        assert_eq!(back_to_song.id, "atvSong1234");
        assert!(!back_to_song.is_video);
        assert!(back_to_song.is_music);
        assert_eq!(back_to_song.album.as_deref(), Some("Random Access Memories"));
        assert_eq!(back_to_song.album_id.as_deref(), Some("MPREb_ram123"));

        // Non-matching track ID returns None so caller falls back to search
        let other = Track {
            id: "otherId9999".into(),
            ..song
        };
        assert!(parse_watch_counterpart_json(&watch_json, &other, true).is_none());
    }
