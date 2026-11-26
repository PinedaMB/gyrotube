use serde::Deserialize;
use tauri::State;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub struct MprisStatePayload {
    pub track_id: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub art_url: Option<String>,
    pub duration_secs: f64,
    pub position_secs: f64,
    pub is_playing: bool,
    pub can_go_next: bool,
    pub can_go_previous: bool,
    pub shuffle: bool,
    pub repeat: bool,
    pub volume: f64,
    #[serde(default)]
    pub seeked: bool,
}

#[cfg(target_os = "linux")]
mod linux {
    use std::sync::atomic::Ordering;
    use std::sync::{Arc, RwLock};
    use std::time::Instant;

    use mpris_server::zbus::{Result as ZbusResult, fdo};
    use mpris_server::{
        LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Property,
        RootInterface, Server, Signal, Time, TrackId, Volume,
    };
    use serde::Serialize;
    use tauri::{AppHandle, Emitter, Manager};

    use super::MprisStatePayload;
    use crate::commands::AppCoreState;

    #[derive(Clone, Serialize)]
    #[serde(tag = "kind")]
    enum MprisCommandEvent {
        #[serde(rename = "play")]
        Play,
        #[serde(rename = "pause")]
        Pause,
        #[serde(rename = "play_pause")]
        PlayPause,
        #[serde(rename = "stop")]
        Stop,
        #[serde(rename = "next")]
        Next,
        #[serde(rename = "previous")]
        Previous,
        #[serde(rename = "seek_to")]
        SeekTo { position_secs: f64 },
        #[serde(rename = "seek_by")]
        SeekBy { offset_secs: f64 },
        #[serde(rename = "set_shuffle")]
        SetShuffle { shuffle: bool },
        #[serde(rename = "set_repeat")]
        SetRepeat { repeat: bool },
        #[serde(rename = "set_volume")]
        SetVolume { volume: f64 },
    }

    #[derive(Clone)]
    struct SharedState {
        track_id: Option<String>,
        metadata: Metadata,
        playback_status: PlaybackStatus,
        duration_us: i64,
        base_position_us: i64,
        synced_at: Instant,
        can_go_next: bool,
        can_go_previous: bool,
        can_play: bool,
        can_pause: bool,
        can_seek: bool,
        shuffle: bool,
        loop_status: LoopStatus,
        volume: f64,
    }

    impl Default for SharedState {
        fn default() -> Self {
            Self {
                track_id: None,
                metadata: Metadata::new(),
                playback_status: PlaybackStatus::Stopped,
                duration_us: 0,
                base_position_us: 0,
                synced_at: Instant::now(),
                can_go_next: false,
                can_go_previous: false,
                can_play: false,
                can_pause: false,
                can_seek: false,
                shuffle: false,
                loop_status: LoopStatus::None,
                volume: 1.0,
            }
        }
    }

    impl SharedState {
        fn current_position_us(&self) -> i64 {
            let pos = if self.playback_status == PlaybackStatus::Playing {
                self.base_position_us
                    .saturating_add(self.synced_at.elapsed().as_micros() as i64)
            } else {
                self.base_position_us
            };
            if self.duration_us > 0 {
                pos.clamp(0, self.duration_us)
            } else {
                pos.max(0)
            }
        }
    }

    fn sanitize_track_object_path(raw_id: &str) -> TrackId {
        let mut clean = String::with_capacity(raw_id.len());
        for ch in raw_id.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                clean.push(ch);
            } else {
                clean.push('_');
            }
        }
        if clean.is_empty() {
            clean.push('0');
        }
        let path = format!("/com/github/gyrotube/track/{clean}");
        TrackId::try_from(path).unwrap_or(TrackId::NO_TRACK)
    }

    fn build_metadata(payload: &MprisStatePayload) -> Metadata {
        let Some(ref raw_id) = payload.track_id else {
            return Metadata::new();
        };
        let mut builder = Metadata::builder().trackid(sanitize_track_object_path(raw_id));
        if payload.duration_secs.is_finite() && payload.duration_secs > 0.0 {
            builder = builder.length(Time::from_micros(
                (payload.duration_secs * 1_000_000.0).round() as i64,
            ));
        }
        if let Some(ref title) = payload.title
            && !title.is_empty()
        {
            builder = builder.title(title.clone());
        }
        if let Some(ref artist) = payload.artist
            && !artist.is_empty()
        {
            builder = builder.artist(vec![artist.clone()]);
        }
        if let Some(ref album) = payload.album
            && !album.is_empty()
        {
            builder = builder.album(album.clone());
        }
        if let Some(ref art) = payload.art_url
            && !art.is_empty()
        {
            builder = builder.art_url(art.clone());
        }
        builder.build()
    }

    struct GyroTubeMprisPlayer {
        app: AppHandle,
        state: Arc<RwLock<SharedState>>,
    }

    impl GyroTubeMprisPlayer {
        fn emit_cmd(&self, cmd: MprisCommandEvent) {
            let _ = self.app.emit("mpris:command", cmd);
        }
    }

    impl RootInterface for GyroTubeMprisPlayer {
        async fn raise(&self) -> fdo::Result<()> {
            if let Some(win) = self.app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.unminimize();
                let _ = win.set_focus();
            }
            Ok(())
        }

        async fn quit(&self) -> fdo::Result<()> {
            let core = self.app.state::<AppCoreState>();
            core.quitting.store(true, Ordering::SeqCst);
            self.app.exit(0);
            Ok(())
        }

        async fn can_quit(&self) -> fdo::Result<bool> {
            Ok(true)
        }

        async fn fullscreen(&self) -> fdo::Result<bool> {
            Ok(false)
        }

        async fn set_fullscreen(&self, _fullscreen: bool) -> ZbusResult<()> {
            Ok(())
        }

        async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
            Ok(false)
        }

        async fn can_raise(&self) -> fdo::Result<bool> {
            Ok(true)
        }

        async fn has_track_list(&self) -> fdo::Result<bool> {
            Ok(false)
        }

        async fn identity(&self) -> fdo::Result<String> {
            Ok("Gyro".into())
        }

        async fn desktop_entry(&self) -> fdo::Result<String> {
            Ok(std::env::var("FLATPAK_ID").unwrap_or_else(|_| "Gyro".into()))
        }

        async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
            Ok(vec![])
        }

        async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
            Ok(vec![])
        }
    }

    impl PlayerInterface for GyroTubeMprisPlayer {
        async fn next(&self) -> fdo::Result<()> {
            self.emit_cmd(MprisCommandEvent::Next);
            Ok(())
        }

        async fn previous(&self) -> fdo::Result<()> {
            self.emit_cmd(MprisCommandEvent::Previous);
            Ok(())
        }

        async fn pause(&self) -> fdo::Result<()> {
            self.emit_cmd(MprisCommandEvent::Pause);
            Ok(())
        }

        async fn play_pause(&self) -> fdo::Result<()> {
            self.emit_cmd(MprisCommandEvent::PlayPause);
            Ok(())
        }

        async fn stop(&self) -> fdo::Result<()> {
            self.emit_cmd(MprisCommandEvent::Stop);
            Ok(())
        }

        async fn play(&self) -> fdo::Result<()> {
            self.emit_cmd(MprisCommandEvent::Play);
            Ok(())
        }

        async fn seek(&self, offset: Time) -> fdo::Result<()> {
            let offset_secs = offset.as_micros() as f64 / 1_000_000.0;
            if let Ok(mut st) = self.state.write() {
                let next_us = st.current_position_us().saturating_add(offset.as_micros());
                st.base_position_us = if st.duration_us > 0 {
                    next_us.clamp(0, st.duration_us)
                } else {
                    next_us.max(0)
                };
                st.synced_at = Instant::now();
            }
            self.emit_cmd(MprisCommandEvent::SeekBy { offset_secs });
            Ok(())
        }

        async fn set_position(&self, track_id: TrackId, position: Time) -> fdo::Result<()> {
            let target_us = position.as_micros().max(0);
            let accepted = if let Ok(mut st) = self.state.write() {
                let matches = st
                    .track_id
                    .as_deref()
                    .map(sanitize_track_object_path)
                    .is_some_and(|expected| expected == track_id);
                if matches {
                    st.base_position_us = if st.duration_us > 0 {
                        target_us.min(st.duration_us)
                    } else {
                        target_us
                    };
                    st.synced_at = Instant::now();
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if accepted {
                self.emit_cmd(MprisCommandEvent::SeekTo {
                    position_secs: target_us as f64 / 1_000_000.0,
                });
            }
            Ok(())
        }

        async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
            Ok(())
        }

        async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
            Ok(self
                .state
                .read()
                .map(|s| s.playback_status)
                .unwrap_or(PlaybackStatus::Stopped))
        }

        async fn loop_status(&self) -> fdo::Result<LoopStatus> {
            Ok(self
                .state
                .read()
                .map(|s| s.loop_status)
                .unwrap_or(LoopStatus::None))
        }

        async fn set_loop_status(&self, loop_status: LoopStatus) -> ZbusResult<()> {
            let repeat = !matches!(loop_status, LoopStatus::None);
            self.emit_cmd(MprisCommandEvent::SetRepeat { repeat });
            Ok(())
        }

        async fn rate(&self) -> fdo::Result<PlaybackRate> {
            Ok(1.0)
        }

        async fn set_rate(&self, _rate: PlaybackRate) -> ZbusResult<()> {
            Ok(())
        }

        async fn shuffle(&self) -> fdo::Result<bool> {
            Ok(self.state.read().map(|s| s.shuffle).unwrap_or(false))
        }

        async fn set_shuffle(&self, shuffle: bool) -> ZbusResult<()> {
            self.emit_cmd(MprisCommandEvent::SetShuffle { shuffle });
            Ok(())
        }

        async fn metadata(&self) -> fdo::Result<Metadata> {
            Ok(self
                .state
                .read()
                .map(|s| s.metadata.clone())
                .unwrap_or_default())
        }

        async fn volume(&self) -> fdo::Result<Volume> {
            Ok(self.state.read().map(|s| s.volume).unwrap_or(1.0))
        }

        async fn set_volume(&self, volume: Volume) -> ZbusResult<()> {
            let clamped = if volume.is_finite() {
                volume.clamp(0.0, 1.0)
            } else {
                1.0
            };
            self.emit_cmd(MprisCommandEvent::SetVolume { volume: clamped });
            Ok(())
        }

        async fn position(&self) -> fdo::Result<Time> {
            let pos_us = self
                .state
                .read()
                .map(|s| s.current_position_us())
                .unwrap_or(0);
            Ok(Time::from_micros(pos_us))
        }

        async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
            Ok(1.0)
        }

        async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
            Ok(1.0)
        }

        async fn can_go_next(&self) -> fdo::Result<bool> {
            Ok(self.state.read().map(|s| s.can_go_next).unwrap_or(false))
        }

        async fn can_go_previous(&self) -> fdo::Result<bool> {
            Ok(self
                .state
                .read()
                .map(|s| s.can_go_previous)
                .unwrap_or(false))
        }

        async fn can_play(&self) -> fdo::Result<bool> {
            Ok(self.state.read().map(|s| s.can_play).unwrap_or(false))
        }

        async fn can_pause(&self) -> fdo::Result<bool> {
            Ok(self.state.read().map(|s| s.can_pause).unwrap_or(false))
        }

        async fn can_seek(&self) -> fdo::Result<bool> {
            Ok(self.state.read().map(|s| s.can_seek).unwrap_or(false))
        }

        async fn can_control(&self) -> fdo::Result<bool> {
            Ok(true)
        }
    }

    pub struct MprisHandle {
        tx: async_channel::Sender<MprisStatePayload>,
    }

    impl MprisHandle {
        pub fn start(app: AppHandle) -> Self {
            let (tx, rx) = async_channel::unbounded::<MprisStatePayload>();
            tauri::async_runtime::spawn(async move {
                let state = Arc::new(RwLock::new(SharedState::default()));
                let player = GyroTubeMprisPlayer {
                    app,
                    state: Arc::clone(&state),
                };
                let Ok(server) = Server::new("Gyro", player).await else {
                    return;
                };

                while let Ok(payload) = rx.recv().await {
                    let new_meta = build_metadata(&payload);
                    let has_track = payload.track_id.is_some();
                    let new_status = if !has_track {
                        PlaybackStatus::Stopped
                    } else if payload.is_playing {
                        PlaybackStatus::Playing
                    } else {
                        PlaybackStatus::Paused
                    };
                    let new_duration_us = if payload.duration_secs.is_finite()
                        && payload.duration_secs > 0.0
                    {
                        (payload.duration_secs * 1_000_000.0).round() as i64
                    } else {
                        0
                    };
                    let new_pos_us = if payload.position_secs.is_finite()
                        && payload.position_secs >= 0.0
                    {
                        (payload.position_secs * 1_000_000.0).round() as i64
                    } else {
                        0
                    };
                    let new_loop = if payload.repeat {
                        LoopStatus::Playlist
                    } else {
                        LoopStatus::None
                    };
                    let new_vol = if payload.volume.is_finite() {
                        payload.volume.clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    let new_can_seek = has_track && new_duration_us > 0;

                    let (changed, emit_seeked) = {
                        let Ok(mut st) = state.write() else {
                            continue;
                        };
                        let prev_pos_us = st.current_position_us();
                        let track_changed = st.track_id != payload.track_id;
                        let mut props = Vec::new();

                        if st.metadata != new_meta {
                            st.metadata = new_meta.clone();
                            props.push(Property::Metadata(new_meta));
                        }
                        if st.playback_status != new_status {
                            st.playback_status = new_status;
                            props.push(Property::PlaybackStatus(new_status));
                        }
                        if st.can_go_next != payload.can_go_next {
                            st.can_go_next = payload.can_go_next;
                            props.push(Property::CanGoNext(payload.can_go_next));
                        }
                        if st.can_go_previous != payload.can_go_previous {
                            st.can_go_previous = payload.can_go_previous;
                            props.push(Property::CanGoPrevious(payload.can_go_previous));
                        }
                        if st.can_play != has_track {
                            st.can_play = has_track;
                            props.push(Property::CanPlay(has_track));
                        }
                        if st.can_pause != has_track {
                            st.can_pause = has_track;
                            props.push(Property::CanPause(has_track));
                        }
                        if st.can_seek != new_can_seek {
                            st.can_seek = new_can_seek;
                            props.push(Property::CanSeek(new_can_seek));
                        }
                        if st.shuffle != payload.shuffle {
                            st.shuffle = payload.shuffle;
                            props.push(Property::Shuffle(payload.shuffle));
                        }
                        if st.loop_status != new_loop {
                            st.loop_status = new_loop;
                            props.push(Property::LoopStatus(new_loop));
                        }
                        if (st.volume - new_vol).abs() > 0.005 {
                            st.volume = new_vol;
                            props.push(Property::Volume(new_vol));
                        }

                        let big_jump = !track_changed && (new_pos_us - prev_pos_us).abs() > 1_500_000;
                        st.track_id = payload.track_id;
                        st.duration_us = new_duration_us;
                        st.base_position_us = new_pos_us;
                        st.synced_at = Instant::now();

                        (props, payload.seeked || big_jump)
                    };

                    if !changed.is_empty() {
                        let _ = server.properties_changed(changed).await;
                    }
                    if emit_seeked {
                        let _ = server
                            .emit(Signal::Seeked {
                                position: Time::from_micros(new_pos_us),
                            })
                            .await;
                    }
                }
            });
            Self { tx }
        }

        pub fn sync(&self, payload: MprisStatePayload) {
            let _ = self.tx.try_send(payload);
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod linux {
    use tauri::AppHandle;

    use super::MprisStatePayload;

    pub struct MprisHandle;

    impl MprisHandle {
        pub fn start(_app: AppHandle) -> Self {
            Self
        }

        pub fn sync(&self, _payload: MprisStatePayload) {}
    }
}

pub use linux::MprisHandle;

#[tauri::command]
pub fn mpris_sync_state(state: State<'_, MprisHandle>, payload: MprisStatePayload) {
    state.sync(payload);
}
