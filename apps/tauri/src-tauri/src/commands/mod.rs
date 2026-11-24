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
use gyrotube_core::spin_remote::{
    GuestSession, HostPublisher, RemoteRole, RemoteSyncPayload, RemoteUpdate,
};
use serde::Serialize;

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

pub struct AppCoreState {
    pub backend: Backend,
    pub active_audio_lease: Mutex<Option<Arc<LocalAudio>>>,
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
        let state = Self {
            backend: Backend::new(),
            active_audio_lease: Mutex::new(None),
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
        track: Option<gyrotube_core::models::Track>,
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
