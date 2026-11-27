use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::time::SystemTime;

use crate::backend::validate_video_id;
use crate::models::StreamInfo;

const MAX_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Debug)]
pub struct LocalAudio {
    path: PathBuf,
    _staging: Option<tempfile::TempDir>,
}

#[derive(Default)]
struct AudioCache {
    in_use: HashMap<PathBuf, Weak<LocalAudio>>,
}

fn cache() -> &'static Mutex<AudioCache> {
    static CACHE: OnceLock<Mutex<AudioCache>> = OnceLock::new();
    CACHE.get_or_init(Mutex::default)
}

impl AudioCache {
    fn hold(&mut self, path: PathBuf) -> Arc<LocalAudio> {
        self.in_use.retain(|_, pin| pin.strong_count() != 0);
        if let Some(audio) = self.in_use.get(&path).and_then(Weak::upgrade) {
            return audio;
        }
        let audio = Arc::new(LocalAudio {
            path: path.clone(),
            _staging: None,
        });
        self.in_use.insert(path, Arc::downgrade(&audio));
        audio
    }

    fn trim(&self, dir: &Path, limit: u64) -> io::Result<bool> {
        let mut entries = Vec::new();
        let mut total = 0u64;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if !managed_name(&entry.file_name().to_string_lossy()) {
                continue;
            }
            let metadata = match entry.metadata() {
                Ok(metadata) if metadata.is_file() => metadata,
                Ok(_) => continue,
                Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                Err(e) => return Err(e),
            };
            total = total.saturating_add(metadata.len());
            let path = entry.path();
            if !self
                .in_use
                .get(&path)
                .is_some_and(|pin| pin.strong_count() != 0)
            {
                entries.push((
                    metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    path,
                    metadata.len(),
                ));
            }
        }
        if total <= limit {
            return Ok(true);
        }
        entries.sort_unstable();
        for (_, path, size) in entries {
            if total <= limit {
                break;
            }
            match fs::remove_file(&path) {
                Ok(()) => total = total.saturating_sub(size),
                Err(e) if e.kind() == io::ErrorKind::NotFound => total = total.saturating_sub(size),
                Err(e) => eprintln!("GyroTube: no se pudo liberar caché de audio {path:?}: {e}"),
            }
        }
        Ok(total <= limit)
    }
}

fn managed_name(name: &str) -> bool {
    let Some((id, extension)) = name.split_once('.') else {
        return false;
    };
    validate_video_id(id).is_ok()
        && (matches!(extension, "m4a" | "webm")
            || extension
                .strip_prefix("ytdlp.")
                .is_some_and(audio_extension))
}

pub fn audio_extension(extension: &str) -> bool {
    AUDIO_EXTENSIONS.contains(&extension)
}

/// Returns whether the extension can be decoded by the active playback backend.
/// On desktop (`native-audio`), pure-Rust Symphonia decodes M4A/AAC, MP3, OGG/Vorbis, and FLAC
/// (excluding WebM/Opus so legacy `.webm` cache entries are transparently refreshed as `.m4a`).
pub fn is_playable_extension(extension: &str) -> bool {
    #[cfg(feature = "native-audio")]
    {
        matches!(
            extension,
            "m4a" | "mp4" | "aac" | "mp3" | "ogg" | "oga" | "flac" | "wav"
        )
    }
    #[cfg(not(feature = "native-audio"))]
    {
        audio_extension(extension)
    }
}

// Filesystem modification time records the last time the app used the cache entry.
pub fn find(video_id: &str, dir: &Path) -> Option<StreamInfo> {
    let mut cache = cache().lock().ok()?;
    let legacy = [("m4a", false), ("webm", false)];
    let current = AUDIO_EXTENSIONS.iter().map(|&ext| (ext, true));
    for (extension, downloaded) in legacy.into_iter().chain(current) {
        if !is_playable_extension(extension) {
            continue;
        }
        let prefix = if downloaded { ".ytdlp." } else { "." };
        let path = dir.join(format!("{video_id}{prefix}{extension}"));
        if fs::symlink_metadata(&path)
            .is_ok_and(|m| m.is_file() && (1..=MAX_BYTES).contains(&m.len()))
            && let Ok(file) = fs::File::open(&path)
        {
            let _ = file.set_modified(SystemTime::now());
            return Some(local_stream(cache.hold(path), extension));
        }
    }
    None
}

const AUDIO_EXTENSIONS: &[&str] = &[
    "m4a", "webm", "opus", "ogg", "oga", "mp3", "aac", "wav", "flac", "mp4", "mka", "mkv",
];

pub fn finish(
    dir: &Path,
    video_id: &str,
    path: PathBuf,
    extension: &str,
    staging: tempfile::TempDir,
) -> StreamInfo {
    let destination = dir.join(format!("{video_id}.ytdlp.{extension}"));
    if let Ok(mut cache) = cache().lock()
        && let Ok(metadata) = fs::metadata(&path)
        && let Some(room) = MAX_BYTES.checked_sub(metadata.len())
        && fs::symlink_metadata(&destination)
            .map(|m| m.is_file() && m.len() == 0)
            .unwrap_or_else(|e| e.kind() == io::ErrorKind::NotFound)
    {
        match cache.trim(dir, room) {
            Ok(true) => match fs::rename(&path, &destination) {
                Ok(()) => {
                    if let Ok(file) = fs::File::open(&destination) {
                        let _ = file.set_modified(SystemTime::now());
                    }
                    return local_stream(cache.hold(destination), extension);
                }
                Err(e) => eprintln!("GyroTube: no se pudo guardar audio en caché: {e}"),
            },
            Ok(false) => {}
            Err(e) => eprintln!("GyroTube: no se pudo medir la caché de audio: {e}"),
        }
    }
    // A full cache must not make the playback fallback fail.
    local_stream(
        Arc::new(LocalAudio {
            path,
            _staging: Some(staging),
        }),
        extension,
    )
}

pub fn trim_existing(dir: &Path) {
    if let Ok(cache) = cache().lock()
        && let Err(e) = cache.trim(dir, MAX_BYTES)
    {
        eprintln!("GyroTube: no se pudo limpiar la caché de audio: {e}");
    }
}

fn local_stream(audio: Arc<LocalAudio>, extension: &str) -> StreamInfo {
    let quality_label = match extension.to_ascii_lowercase().as_str() {
        "webm" | "opus" => "160kbps (WEBM)".to_string(),
        "m4a" | "mp4" | "aac" => "128kbps (M4A)".to_string(),
        _ => extension.to_uppercase(),
    };
    StreamInfo {
        url: audio.path.to_string_lossy().into_owned(),
        quality_label,
        user_agent: None,
        headers: HashMap::new(),
        local_audio: Some(audio),
        loudness_db: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn eviction_uses_last_access_and_skips_audio_held_by_the_player() {
        let dir = tempfile::tempdir().unwrap();
        let pinned = dir.path().join("abcdefghijk.m4a");
        let oldest = dir.path().join("bcdefghijkl.ytdlp.webm");
        let reused = dir.path().join("cdefghijklm.ytdlp.mp3");
        let unrelated = dir.path().join("other.json");
        let staging = tempfile::tempdir_in(dir.path()).unwrap();
        let partial = staging.path().join("audio.webm.part");
        for (n, path) in [&pinned, &reused, &oldest, &unrelated, &partial]
            .into_iter()
            .enumerate()
        {
            fs::write(path, [0; 4]).unwrap();
            fs::File::open(path)
                .unwrap()
                .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(60 + n as u64))
                .unwrap();
        }

        let playing = find("abcdefghijk", dir.path()).unwrap();
        let _ = find("cdefghijklm", dir.path()).unwrap();
        fs::File::open(&pinned)
            .unwrap()
            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(60))
            .unwrap();
        assert!(cache().lock().unwrap().trim(dir.path(), 8).unwrap());
        assert!(!oldest.exists());
        assert!(reused.exists() && pinned.exists() && unrelated.exists() && partial.exists());

        drop(playing);
        assert!(cache().lock().unwrap().trim(dir.path(), 4).unwrap());
        assert!(!pinned.exists());
        assert!(reused.exists());
    }

    #[test]
    fn a_download_that_cannot_fit_stays_available_until_playback_releases_it() {
        let dir = tempfile::tempdir().unwrap();
        let pinned = dir.path().join("abcdefghijk.m4a");
        // Sparse file to exercise the production quota without writing a GiB.
        fs::File::create(&pinned)
            .unwrap()
            .set_len(MAX_BYTES)
            .unwrap();
        let playing = find("abcdefghijk", dir.path()).unwrap();

        let staging = tempfile::tempdir_in(dir.path()).unwrap();
        let downloaded = staging.path().join("audio.m4a");
        fs::write(&downloaded, b"track").unwrap();
        let stream = finish(
            dir.path(),
            "bcdefghijkl",
            downloaded.clone(),
            "m4a",
            staging,
        );
        assert_eq!(stream.url, downloaded.to_string_lossy());
        let deck_reference = stream.local_audio.clone();
        drop(stream);
        assert!(downloaded.exists());
        assert!(pinned.exists());
        drop(deck_reference);
        assert!(!downloaded.exists());

        drop(playing);
        let staging = tempfile::tempdir_in(dir.path()).unwrap();
        let downloaded = staging.path().join("audio.m4a");
        fs::write(&downloaded, b"track").unwrap();
        let stream = finish(dir.path(), "bcdefghijkl", downloaded, "m4a", staging);
        assert!(!pinned.exists());
        assert!(Path::new(&stream.url).is_file());
        assert!(find("bcdefghijkl", dir.path()).is_some());
    }
}
