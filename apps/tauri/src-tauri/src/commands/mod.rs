mod audio;
mod catalog;
mod persistence;
mod spin;
mod updater;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};

use gyrotube_core::audio_cache::LocalAudio;
use gyrotube_core::backend::Backend;
use gyrotube_core::jam::{JamRequest, JamServerHandle, JamSnapshot};
use gyrotube_core::native_audio::{AudioController, DecoderHandle};
use gyrotube_core::spin_remote::{
    GuestSession, HostPublisher, RemoteRole, RemoteSyncPayload, RemoteUpdate,
};
use serde::Serialize;
use tokio::sync::Notify;

pub use audio::*;
pub use catalog::*;
pub use persistence::*;
pub use spin::*;
pub use updater::*;

pub struct SpinState {
    pub snapshot: Arc<RwLock<JamSnapshot>>,
    pub server: Option<JamServerHandle>,
    pub cmd_tx: async_channel::Sender<JamRequest>,
    pub remote_role: RemoteRole,
    pub host_publisher: Option<HostPublisher>,
    pub guest_session: Option<GuestSession>,
    pub remote_tx: async_channel::Sender<RemoteUpdate>,
}

pub struct NativeDeckSlot {
    pub decoder: Option<DecoderHandle>,
    pub track_id: Option<String>,
    pub stream_url: Option<String>,
    pub local_path: Option<std::path::PathBuf>,
    pub lease: Option<Arc<LocalAudio>>,
    pub bpm: f32,
}

impl Default for NativeDeckSlot {
    fn default() -> Self {
        Self {
            decoder: None,
            track_id: None,
            stream_url: None,
            local_path: None,
            lease: None,
            bpm: 120.0,
        }
    }
}

pub struct NativeAudioState {
    pub controller: Option<AudioController>,
    pub deck_a: NativeDeckSlot,
    pub deck_b: NativeDeckSlot,
}

impl NativeAudioState {
    pub fn slot_mut(&mut self, deck: gyrotube_core::native_audio::DeckId) -> &mut NativeDeckSlot {
        match deck {
            gyrotube_core::native_audio::DeckId::A => &mut self.deck_a,
            gyrotube_core::native_audio::DeckId::B => &mut self.deck_b,
        }
    }
}

pub struct AppCoreState {
    pub backend: Backend,
    pub active_audio_lease: Mutex<Option<Arc<LocalAudio>>>,
    pub native_audio: Mutex<NativeAudioState>,
    pub tick_notify: Arc<Notify>,
    pub spin: Mutex<SpinState>,
    pub background_playback: Arc<AtomicBool>,
    #[cfg_attr(not(desktop), allow(dead_code))]
    pub quitting: Arc<AtomicBool>,
}

impl AppCoreState {
    pub fn new() -> (
        Self,
        async_channel::Receiver<JamRequest>,
        async_channel::Receiver<RemoteUpdate>,
    ) {
        let (cmd_tx, cmd_rx) = async_channel::unbounded();
        let (remote_tx, remote_rx) = async_channel::unbounded();
        let controller = match AudioController::start() {
            Ok(ctrl) => Some(ctrl),
            Err(err) => {
                eprintln!("[gyrotube-tauri] Native cpal audio unavailable (headless/no device): {err}");
                None
            }
        };
        let state = Self {
            backend: Backend::new(),
            active_audio_lease: Mutex::new(None),
            native_audio: Mutex::new(NativeAudioState {
                controller,
                deck_a: NativeDeckSlot::default(),
                deck_b: NativeDeckSlot::default(),
            }),
            tick_notify: Arc::new(Notify::new()),
            spin: Mutex::new(SpinState {
                snapshot: Arc::new(RwLock::new(JamSnapshot::default())),
                server: None,
                cmd_tx,
                remote_role: RemoteRole::Idle,
                host_publisher: None,
                guest_session: None,
                remote_tx,
            }),
            background_playback: Arc::new(AtomicBool::new(true)),
            quitting: Arc::new(AtomicBool::new(false)),
        };
        (state, cmd_rx, remote_rx)
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind")]
pub enum SpinCommandEvent {
    #[serde(rename = "add_track")]
    AddTrack {
        video_id: String,
        guest_name: String,
        track: Option<Box<gyrotube_core::models::Track>>,
    },
    #[serde(rename = "toggle_play_pause")]
    TogglePlayPause,
    #[serde(rename = "next_track")]
    NextTrack,
    #[serde(rename = "prev_track")]
    PrevTrack,
    #[serde(rename = "move_track")]
    MoveTrack { from: usize, to: usize },
}

#[derive(Clone, Serialize)]
pub struct SpinRemoteSyncEvent {
    pub target: String,
    pub payload: Option<RemoteSyncPayload>,
    pub error: Option<String>,
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    let trimmed = url.trim();
    if !trimmed.starts_with("https://") && !trimmed.starts_with("http://") {
        return Err("URL inválida".into());
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(trimmed)
            .spawn()
            .map_err(|e| format!("No se pudo abrir el navegador: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(trimmed)
            .spawn()
            .map_err(|e| format!("No se pudo abrir el navegador: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", trimmed])
            .spawn()
            .map_err(|e| format!("No se pudo abrir el navegador: {e}"))?;
        Ok(())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err("No soportado en esta plataforma".into())
    }
}
