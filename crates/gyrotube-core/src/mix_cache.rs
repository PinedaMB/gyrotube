use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::backend::validate_video_id;
use crate::models::cache_dir;

const MAX_MIX_CACHE_ENTRIES: usize = 500;
const MIX_CACHE_FILENAME: &str = "mix-analysis.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackMixProfile {
    pub version: u8,
    pub duration_secs: f64,
    pub intro_end_secs: f64,
    pub outro_start_secs: f64,
    pub bpm: Option<f64>,
    pub bpm_confidence: f64,
    pub beat_offset_secs: f64,
    #[serde(default)]
    pub peaks: Vec<f32>,
    #[serde(default)]
    pub analyzed_at: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct MixCacheStore {
    entries: HashMap<String, TrackMixProfile>,
}

fn store_lock() -> &'static Mutex<Option<MixCacheStore>> {
    static STORE: OnceLock<Mutex<Option<MixCacheStore>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(None))
}

fn cache_file_path() -> Option<PathBuf> {
    cache_dir().map(|d| d.join(MIX_CACHE_FILENAME))
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn load_from_path(path: &Path) -> MixCacheStore {
    std::fs::read(path)
        .ok()
        .and_then(|raw| serde_json::from_slice(&raw).ok())
        .unwrap_or_default()
}

fn save_to_path(path: &Path, store: &MixCacheStore) -> std::io::Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| std::io::Error::other("Mix cache path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let mut staged = tempfile::NamedTempFile::new_in(dir)?;
    serde_json::to_writer(&mut staged, store).map_err(std::io::Error::other)?;
    staged.flush()?;
    staged.persist(path).map_err(|err| err.error)?;
    Ok(())
}

fn trim_lru(store: &mut MixCacheStore, max_entries: usize) {
    if store.entries.len() <= max_entries {
        return;
    }
    let mut order: Vec<(u64, String)> = store
        .entries
        .iter()
        .map(|(k, v)| (v.analyzed_at, k.clone()))
        .collect();
    order.sort_unstable();
    let remove_count = store.entries.len().saturating_sub(max_entries);
    for (_, key) in order.into_iter().take(remove_count) {
        store.entries.remove(&key);
    }
}

pub fn get_profile(video_id: &str) -> Option<TrackMixProfile> {
    validate_video_id(video_id).ok()?;
    let path = cache_file_path()?;
    let mut guard = store_lock().lock().ok()?;
    let store = guard.get_or_insert_with(|| load_from_path(&path));
    store.entries.get(video_id).cloned()
}

pub fn save_profile(video_id: &str, mut profile: TrackMixProfile) -> Result<(), String> {
    validate_video_id(video_id)?;
    let Some(path) = cache_file_path() else {
        return Ok(());
    };
    if profile.analyzed_at == 0 {
        profile.analyzed_at = now_unix_secs();
    }
    let mut guard = store_lock()
        .lock()
        .map_err(|_| "Mix cache lock poisoned".to_string())?;
    let store = guard.get_or_insert_with(|| load_from_path(&path));
    store.entries.insert(video_id.to_string(), profile);
    trim_lru(store, MAX_MIX_CACHE_ENTRIES);
    save_to_path(&path, store).map_err(|e| format!("Failed to persist mix cache: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile(ts: u64, bpm: Option<f64>) -> TrackMixProfile {
        TrackMixProfile {
            version: 1,
            duration_secs: 210.0,
            intro_end_secs: 1.2,
            outro_start_secs: 196.0,
            bpm,
            bpm_confidence: 0.78,
            beat_offset_secs: 0.12,
            peaks: vec![0.2, 0.8, 0.5],
            analyzed_at: ts,
        }
    }

    #[test]
    fn atomic_roundtrip_and_lru_eviction() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mix-analysis.json");

        let mut store = MixCacheStore::default();
        store
            .entries
            .insert("aaaaaaaaaaa".into(), sample_profile(10, Some(120.0)));
        store
            .entries
            .insert("bbbbbbbbbbb".into(), sample_profile(30, Some(124.0)));
        store
            .entries
            .insert("ccccccccccc".into(), sample_profile(20, None));

        trim_lru(&mut store, 2);
        assert_eq!(store.entries.len(), 2);
        assert!(!store.entries.contains_key("aaaaaaaaaaa"));
        assert!(store.entries.contains_key("bbbbbbbbbbb"));
        assert!(store.entries.contains_key("ccccccccccc"));

        save_to_path(&path, &store).unwrap();
        let loaded = load_from_path(&path);
        assert_eq!(loaded.entries.len(), 2);
        assert_eq!(loaded.entries["bbbbbbbbbbb"].bpm, Some(124.0));
        assert_eq!(loaded.entries["ccccccccccc"].bpm, None);
    }
}
