use std::cell::RefCell;
use std::rc::Rc;

use gtk4::glib;
use mpris_server::{Metadata, PlaybackStatus, Player, Time, TrackId};

use crate::models::Track;

#[derive(Debug, Clone)]
pub enum MprisCommand {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    SeekOffsetUs(i64),
    SetPositionUs(i64),
    Raise,
}

#[derive(Clone)]
pub struct MprisController {
    player: Rc<RefCell<Option<Rc<Player>>>>,
}

impl MprisController {
    pub fn spawn<F>(on_command: F) -> Self
    where
        F: Fn(MprisCommand) + 'static,
    {
        let player_slot: Rc<RefCell<Option<Rc<Player>>>> = Rc::new(RefCell::new(None));
        let slot_clone = Rc::clone(&player_slot);
        let on_cmd = Rc::new(on_command);

        glib::spawn_future_local(async move {
            let Ok(player) = Player::builder("com.github.gyrotube.GyroTube")
                .identity("GyroTube")
                .desktop_entry("com.github.gyrotube.GyroTube")
                .can_play(true)
                .can_pause(true)
                .can_go_next(true)
                .can_go_previous(true)
                .can_seek(true)
                .can_raise(true)
                .build()
                .await
            else {
                return;
            };

            let player = Rc::new(player);

            {
                let cb = Rc::clone(&on_cmd);
                player.connect_play(move |_| cb(MprisCommand::Play));
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_pause(move |_| cb(MprisCommand::Pause));
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_play_pause(move |_| cb(MprisCommand::PlayPause));
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_stop(move |_| cb(MprisCommand::Stop));
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_next(move |_| cb(MprisCommand::Next));
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_previous(move |_| cb(MprisCommand::Previous));
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_seek(move |_, offset| {
                    cb(MprisCommand::SeekOffsetUs(offset.as_micros()));
                });
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_set_position(move |_, _id, pos| {
                    cb(MprisCommand::SetPositionUs(pos.as_micros()));
                });
            }
            {
                let cb = Rc::clone(&on_cmd);
                player.connect_raise(move |_| cb(MprisCommand::Raise));
            }

            *slot_clone.borrow_mut() = Some(Rc::clone(&player));
            player.run().await;
        });

        Self {
            player: player_slot,
        }
    }

    pub fn clear_track(&self) {
        let Some(player) = self.player.borrow().clone() else {
            return;
        };
        glib::spawn_future_local(async move {
            let _ = player.set_metadata(Metadata::default()).await;
            let _ = player.set_playback_status(PlaybackStatus::Stopped).await;
        });
    }

    pub fn update_track(&self, track: &Track) {
        let Some(player) = self.player.borrow().clone() else {
            return;
        };
        let track = track.clone();
        glib::spawn_future_local(async move {
            let mut builder = Metadata::builder()
                .title(track.title)
                .artist([track.artist]);

            if let Some(album) = track.album {
                builder = builder.album(album);
            }
            if let Some(art) = track.cover_large_url.or(track.cover_url) {
                builder = builder.art_url(art);
            }
            if let Some(secs) = track.duration_secs {
                builder = builder.length(Time::from_secs(secs as i64));
            }
            let safe_id: String = track
                .id
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            if let Ok(tid) = TrackId::try_from(format!("/com/github/gyrotube/track/{safe_id}")) {
                builder = builder.trackid(tid);
            }

            let _ = player.set_metadata(builder.build()).await;
        });
    }

    pub fn set_status(&self, playing: bool, stopped: bool) {
        let Some(player) = self.player.borrow().clone() else {
            return;
        };
        let status = if stopped {
            PlaybackStatus::Stopped
        } else if playing {
            PlaybackStatus::Playing
        } else {
            PlaybackStatus::Paused
        };
        glib::spawn_future_local(async move {
            let _ = player.set_playback_status(status).await;
        });
    }

    pub fn set_position_us(&self, pos_us: i64) {
        if let Some(player) = self.player.borrow().as_ref() {
            player.set_position(Time::from_micros(pos_us.max(0)));
        }
    }

    pub fn emit_seeked(&self, pos_us: i64) {
        let Some(player) = self.player.borrow().clone() else {
            return;
        };
        glib::spawn_future_local(async move {
            let _ = player.seeked(Time::from_micros(pos_us.max(0))).await;
        });
    }
}
