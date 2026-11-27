use super::*;
use super::http::*;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};


async fn spawn_slow_range_origin(payload: Arc<Vec<u8>>, cutoff: Option<u64>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        loop {
            let Ok((socket, _)) = listener.accept().await else {
                break;
            };
            let data = payload.clone();
            tokio::spawn(async move {
                let (read_half, mut write_half) = socket.into_split();
                let mut reader = BufReader::new(read_half);
                let mut range_hdr: Option<String> = None;
                loop {
                    let mut line = String::new();
                    let Ok(n) = reader.read_line(&mut line).await else {
                        return;
                    };
                    if n == 0 || line == "\r\n" || line == "\n" {
                        break;
                    }
                    if let Some((k, v)) = line.split_once(':')
                        && k.trim().eq_ignore_ascii_case("range")
                    {
                        range_hdr = Some(v.trim().to_string());
                    }
                }

                let total = data.len() as u64;
                let (start, end) = range_hdr
                    .as_deref()
                    .and_then(parse_range_header)
                    .map(|(s, e)| (s, e.unwrap_or(total - 1).min(total - 1)))
                    .unwrap_or((0, total - 1));

                if cutoff.is_some_and(|limit| end >= limit) {
                    let _ = write_half.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n").await;
                    return;
                }

                if start >= total {
                    let _ = write_half
                        .write_all(b"HTTP/1.1 416 Range Not Satisfiable\r\nContent-Length: 0\r\n\r\n")
                        .await;
                    return;
                }

                if start > 0 && end > start {
                    // Delay subsequent multi-byte blocks (not 1-byte tail probes) so the test
                    // can verify Block 0 plays before full download completes.
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }

                let slice = &data[start as usize..=end as usize];
                let hdr = format!(
                    "HTTP/1.1 206 Partial Content\r\nContent-Type: audio/webm\r\nContent-Range: bytes {start}-{end}/{total}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    slice.len()
                );
                let _ = write_half.write_all(hdr.as_bytes()).await;
                let _ = write_half.write_all(slice).await;
            });
        }
    });
    port
}

#[tokio::test]
async fn streams_audio_in_blocks_before_full_download_completes() {
    let total_size = 900 * 1024; // 900 KB (> 256 KB initial block + 512 KB second block)
    let payload: Arc<Vec<u8>> =
        Arc::new((0..total_size).map(|i| (i % 251) as u8).collect());
    let origin_port = spawn_slow_range_origin(payload.clone(), None).await;

    let temp = tempfile::tempdir().unwrap();
    let streamer = AudioStreamer::start_new().await.unwrap();
    let remote = StreamInfo {
        url: format!("http://127.0.0.1:{origin_port}/audio.webm"),
        quality_label: "160kbps (WEBM)".to_string(),
        user_agent: Some("GyroTubeTest/1.0".to_string()),
        headers: HashMap::new(),
        local_audio: None,
        loudness_db: None,
    };

    let started = std::time::Instant::now();
    let local_stream = streamer
        .start_chunked_http_stream("dQw4w9WgXcQ", remote, "webm", None, temp.path())
        .await
        .unwrap();

    // start_chunked_http_stream must return as soon as Block 0 is ready (< 150ms),
    // without waiting for the delayed subsequent blocks (which take > 500ms).
    assert!(
        started.elapsed() < Duration::from_millis(200),
        "Expected stream to become ready before subsequent blocks finished, took {:?}",
        started.elapsed()
    );

    let client = reqwest::Client::new();
    let full_bytes = client
        .get(&local_stream.url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    assert_eq!(full_bytes.len(), total_size);
    assert_eq!(&full_bytes[..], &payload[..]);
}

#[tokio::test]
async fn supports_byte_range_requests_on_progressive_stream() {
    let total_size = 512 * 1024;
    let payload: Arc<Vec<u8>> =
        Arc::new((0..total_size).map(|i| (i % 239) as u8).collect());
    let origin_port = spawn_slow_range_origin(payload.clone(), None).await;

    let temp = tempfile::tempdir().unwrap();
    let streamer = AudioStreamer::start_new().await.unwrap();
    let remote = StreamInfo {
        url: format!("http://127.0.0.1:{origin_port}/audio.webm"),
        quality_label: "160kbps (WEBM)".to_string(),
        user_agent: None,
        headers: HashMap::new(),
        local_audio: None,
        loudness_db: None,
    };

    let local_stream = streamer
        .start_chunked_http_stream("3JZ_D3ELwOQ", remote, "webm", None, temp.path())
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let resp = client
        .get(&local_stream.url)
        .header(reqwest::header::RANGE, "bytes=1000-4999")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), reqwest::StatusCode::PARTIAL_CONTENT);
    let body = resp.bytes().await.unwrap();
    assert_eq!(body.len(), 4000);
    assert_eq!(&body[..], &payload[1000..5000]);
}

#[tokio::test]
async fn rejects_sources_that_serve_the_start_but_block_the_rest() {
    let total = 900 * 1024;
    let payload = Arc::new(vec![0; total as usize]);
    let limited = spawn_slow_range_origin(payload.clone(), Some(768 * 1024)).await;
    let complete = spawn_slow_range_origin(payload, None).await;
    let mut remote = StreamInfo {
        url: format!("http://127.0.0.1:{limited}/audio.webm"),
        quality_label: "WEBM".into(),
        user_agent: None,
        headers: HashMap::new(),
        local_audio: None,
        loudness_db: None,
    };
    let client = reqwest::Client::new();
    assert!(fetch_range_block_url(&client, &remote.url, &remote, 0, INITIAL_HTTP_BLOCK_BYTES - 1).await.is_ok());
    assert!(verify_audio_tail(&client, &remote, total).await.unwrap_err().contains("403"));
    let temp = tempfile::tempdir().unwrap();
    let streamer = AudioStreamer::start_new().await.unwrap();
    let err = streamer
        .start_chunked_http_stream("cutoffTest1", remote.clone(), "webm", None, temp.path())
        .await
        .unwrap_err();
    assert!(err.contains("403"), "expected 403 rejection, got: {err}");
    remote.url = format!("http://127.0.0.1:{complete}/audio.webm");
    verify_audio_tail(&client, &remote, total).await.unwrap();
}

#[test]
fn extracts_clen_query_param_from_googlevideo_url() {
    let url = "https://rr2---sn-abc.googlevideo.com/videoplayback?expire=1700000000&clen=3845120&mime=audio%2Fwebm";
    assert_eq!(parse_clen_from_url(url), Some(3_845_120));
}

#[tokio::test]
async fn a_stream_that_fails_after_buffering_cannot_be_reused() {
    let streamer = AudioStreamer::start_new().await.unwrap();
    let key = "test-1";
    let url = reqwest::Url::parse(&streamer.local_url(key, "webm")).unwrap();
    let progress = ChunkProgress {
        written: READY_THRESHOLD_BYTES,
        total_bytes: Some(2 * READY_THRESHOLD_BYTES),
        done: false,
        error: None,
    };
    let (tx, mut rx) = watch::channel(progress.clone());
    streamer.sessions.write().unwrap().insert(key.into(), Arc::new(StreamSession {
        file_path: PathBuf::new(),
        mime_type: "audio/webm".into(),
        progress_rx: rx.clone(),
        loudness_db: None,
    }));
    assert!(streamer.can_reuse_url(&url));
    tx.send_replace(ChunkProgress {
        done: true,
        error: Some("network interrupted".into()),
        ..progress
    });
    assert!(!streamer.can_reuse_url(&url));
    assert_eq!(wait_until_initial_chunk_ready(&mut rx).await.unwrap_err(), "network interrupted");
    let response = reqwest::Client::new().get(url).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::BAD_GATEWAY);
}

/// Live throughput + byte-exactness check for the chunked `/proxy/` endpoint.
/// Run with: GYRO_PROXY_TEST_URL=<googlevideo url> GYRO_PROXY_TEST_REF=<local ref file>
/// cargo test -p gyrotube-core chunked_proxy_live -- --ignored --nocapture
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn chunked_proxy_live_throughput_and_integrity() {
    let Ok(remote) = std::env::var("GYRO_PROXY_TEST_URL") else {
        return;
    };
    crate::tls::ensure_crypto_provider();
    let streamer = super::AudioStreamer::start_new().await.expect("streamer");
    let local = streamer.serve_remote_proxy("proxytest", &remote, "Mozilla/5.0", "video/webm");

    let client = reqwest::Client::new();
    let t0 = std::time::Instant::now();
    let mut resp = client
        .get(&local)
        .header("Range", "bytes=0-")
        .send()
        .await
        .expect("proxy request");
    println!(
        "status={} content-range={:?} content-length={:?}",
        resp.status(),
        resp.headers().get("content-range"),
        resp.headers().get("content-length")
    );
    let mut body: Vec<u8> = Vec::new();
    while t0.elapsed() < std::time::Duration::from_secs(12) {
        match resp.chunk().await {
            Ok(Some(c)) => body.extend_from_slice(&c),
            _ => break,
        }
    }
    let secs = t0.elapsed().as_secs_f64();
    println!(
        "proxy bytes={} in {:.2}s => {:.0} KB/s",
        body.len(),
        secs,
        body.len() as f64 / 1024.0 / secs
    );

    if let Ok(ref_path) = std::env::var("GYRO_PROXY_TEST_REF") {
        let reference = std::fs::read(ref_path).expect("ref file");
        let n = body.len().min(reference.len());
        assert!(n > 0);
        assert_eq!(&body[..n], &reference[..n], "proxy bytes differ from reference");
        println!("integrity OK over {n} bytes");
    }

    // Mid-file seek like <video> does
    let mut seek = client
        .get(&local)
        .header("Range", "bytes=5000000-")
        .send()
        .await
        .expect("seek request");
    println!("seek status={} content-range={:?}", seek.status(), seek.headers().get("content-range"));
    let mut seek_body = Vec::new();
    while seek_body.len() < 3_000_000 {
        match seek.chunk().await {
            Ok(Some(c)) => seek_body.extend_from_slice(&c),
            _ => break,
        }
    }
    if let Ok(ref_path) = std::env::var("GYRO_PROXY_TEST_REF") {
        let reference = std::fs::read(ref_path).expect("ref file");
        let n = seek_body.len().min(reference.len().saturating_sub(5_000_000));
        assert_eq!(&seek_body[..n], &reference[5_000_000..5_000_000 + n], "seek bytes differ");
        println!("seek integrity OK over {n} bytes");
    }
}
