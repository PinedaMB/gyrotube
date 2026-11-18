use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use gstreamer as gst;
use gstreamer::prelude::*;
use gtk4::gio;
use gtk4::gio::prelude::FileExt;
use gtk4::glib;

use crate::backend::log_audio_timing;
use crate::models::StreamInfo;

static GST_INIT: OnceLock<Result<(), String>> = OnceLock::new();

fn ensure_gst_init() -> Result<(), String> {
    GST_INIT
        .get_or_init(|| gst::init().map_err(|e| format!("Failed to init GStreamer: {e}")))
        .clone()
}

#[derive(Clone)]
pub(crate) struct AudioStartup {
    pub id: String,
    pub selected_at: Instant,
}

struct DeckInner {
    playbin: gst::Element,
    _bus_watch: RefCell<Option<gst::bus::BusWatchGuard>>,
    pending_user_agent: Arc<Mutex<Option<String>>>,
    pending_headers: Arc<Mutex<HashMap<String, String>>>,
    is_playing: Cell<bool>,
    is_prepared: Cell<bool>,
    startup: RefCell<Option<(AudioStartup, Instant)>>,
    is_ended: Cell<bool>,
    has_stream: Cell<bool>,
    local_audio: RefCell<Option<Arc<crate::audio_cache::LocalAudio>>>,
    is_remote: Cell<bool>,
    rate: Cell<f64>,
    last_error: RefCell<Option<String>>,
    on_prepared: RefCell<Vec<Box<dyn Fn(&GStreamerDeck)>>>,
    on_playing: RefCell<Vec<Box<dyn Fn(&GStreamerDeck)>>>,
    on_timestamp: RefCell<Vec<Box<dyn Fn(&GStreamerDeck)>>>,
    on_duration: RefCell<Vec<Box<dyn Fn(&GStreamerDeck)>>>,
    on_ended: RefCell<Vec<Box<dyn Fn(&GStreamerDeck)>>>,
    on_error: RefCell<Vec<Box<dyn Fn(&GStreamerDeck)>>>,
}

#[derive(Clone)]
pub struct GStreamerDeck {
    inner: Rc<DeckInner>,
}

impl GStreamerDeck {
    pub fn new() -> Self {
        let _ = ensure_gst_init();
        let playbin = gst::ElementFactory::make("playbin")
            .build()
            .expect("Failed to create GStreamer playbin element");
        playbin.set_property_from_str("flags", "audio+soft-volume");
        if playbin.has_property("buffer-size") {
            playbin.set_property("buffer-size", 64 * 1024i32);
        }
        if playbin.has_property("buffer-duration") {
            playbin.set_property("buffer-duration", 500_000_000i64);
        }

        let pending_user_agent: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let pending_headers: Arc<Mutex<HashMap<String, String>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let ua_for_signal = Arc::clone(&pending_user_agent);
        let headers_for_signal = Arc::clone(&pending_headers);
        playbin.connect("source-setup", false, move |values| {
            let Some(source) = values.get(1).and_then(|v| v.get::<gst::Element>().ok()) else {
                return None;
            };

            if source.has_property("iradio-mode") {
                source.set_property("iradio-mode", false);
            }
            if source.has_property("timeout") {
                source.set_property("timeout", 30u32);
            }
            if source.has_property("retries") {
                source.set_property("retries", 3i32);
            }

            if let Ok(guard) = ua_for_signal.lock()
                && let Some(ref ua) = *guard
                && source.has_property("user-agent")
            {
                source.set_property("user-agent", ua);
            }
            if let Ok(hguard) = headers_for_signal.lock()
                && !hguard.is_empty()
                && source.has_property("extra-headers")
            {
                let mut builder = gst::Structure::builder("extra-headers");
                let mut has_any = false;
                for (k, v) in hguard.iter() {
                    if !k.eq_ignore_ascii_case("user-agent")
                        && !k.eq_ignore_ascii_case("accept-encoding")
                        && !k.eq_ignore_ascii_case("host")
                        && !k.eq_ignore_ascii_case("connection")
                    {
                        builder = builder.field(k.as_str(), v.as_str());
                        has_any = true;
                    }
                }
                if has_any {
                    source.set_property("extra-headers", builder.build());
                }
            }
            None
        });

        let inner = Rc::new(DeckInner {
            playbin: playbin.clone(),
            _bus_watch: RefCell::new(None),
            pending_user_agent,
            pending_headers,
            is_playing: Cell::new(false),
            is_prepared: Cell::new(false),
            startup: RefCell::new(None),
            is_ended: Cell::new(false),
            has_stream: Cell::new(false),
            local_audio: RefCell::new(None),
            is_remote: Cell::new(false),
            rate: Cell::new(1.0),
            last_error: RefCell::new(None),
            on_prepared: RefCell::new(Vec::new()),
            on_playing: RefCell::new(Vec::new()),
            on_timestamp: RefCell::new(Vec::new()),
            on_duration: RefCell::new(Vec::new()),
            on_ended: RefCell::new(Vec::new()),
            on_error: RefCell::new(Vec::new()),
        });

        let weak = Rc::downgrade(&inner);
        let bus_watch = playbin.bus().and_then(|bus| {
            bus.add_watch_local(move |_, msg| {
                let Some(strong) = weak.upgrade() else {
                    return gst::glib::ControlFlow::Break;
                };
                let deck = GStreamerDeck {
                    inner: Rc::clone(&strong),
                };
                match msg.view() {
                    gst::MessageView::AsyncDone(_) => {
                        if !strong.is_prepared.get() {
                            strong.is_prepared.set(true);
                            deck.trace_prepared();
                            for cb in strong.on_prepared.borrow().iter() {
                                cb(&deck);
                            }
                        }
                        for cb in strong.on_duration.borrow().iter() {
                            cb(&deck);
                        }
                    }
                    gst::MessageView::DurationChanged(_) => {
                        for cb in strong.on_duration.borrow().iter() {
                            cb(&deck);
                        }
                    }
                    gst::MessageView::StateChanged(sc)
                        if sc.src() == Some(strong.playbin.upcast_ref()) =>
                    {
                        let playing_now = sc.current() == gst::State::Playing;
                        if strong.is_playing.get() != playing_now {
                            strong.is_playing.set(playing_now);
                            for cb in strong.on_playing.borrow().iter() {
                                cb(&deck);
                            }
                        }
                        if matches!(sc.current(), gst::State::Paused | gst::State::Playing)
                            && !strong.is_prepared.get()
                        {
                            strong.is_prepared.set(true);
                            deck.trace_prepared();
                            for cb in strong.on_prepared.borrow().iter() {
                                cb(&deck);
                            }
                        }
                        if playing_now
                            && let Some((timing, _)) = strong.startup.borrow_mut().take()
                        {
                            log_audio_timing(
                                &timing.id,
                                "seleccion-a-gstreamer-playing",
                                timing.selected_at,
                            );
                        }
                    }
                    gst::MessageView::Eos(_) => {
                        strong.is_playing.set(false);
                        strong.is_ended.set(true);
                        for cb in strong.on_playing.borrow().iter() {
                            cb(&deck);
                        }
                        for cb in strong.on_ended.borrow().iter() {
                            cb(&deck);
                        }
                    }
                    gst::MessageView::Error(err) => {
                        strong.is_playing.set(false);
                        let already_errored = strong.last_error.borrow().is_some();
                        if !already_errored {
                            *strong.last_error.borrow_mut() = Some(err.error().to_string());
                            for cb in strong.on_playing.borrow().iter() {
                                cb(&deck);
                            }
                            for cb in strong.on_error.borrow().iter() {
                                cb(&deck);
                            }
                        }
                    }
                    _ => {}
                }
                gst::glib::ControlFlow::Continue
            })
            .ok()
        });

        // SAFETY: inner is exclusively owned here before returning.
        *inner._bus_watch.borrow_mut() = bus_watch;

        let weak_tick = Rc::downgrade(&inner);
        glib::timeout_add_local(std::time::Duration::from_millis(250), move || {
            let Some(strong) = weak_tick.upgrade() else {
                return glib::ControlFlow::Break;
            };
            if strong.is_playing.get() {
                let deck = GStreamerDeck {
                    inner: Rc::clone(&strong),
                };
                for cb in strong.on_timestamp.borrow().iter() {
                    cb(&deck);
                }
            }
            glib::ControlFlow::Continue
        });

        Self { inner }
    }

    fn reset_for_new_source(&self) {
        let _ = self.inner.playbin.set_state(gst::State::Null);
        self.inner.local_audio.borrow_mut().take();
        self.inner.startup.borrow_mut().take();
        self.inner.is_playing.set(false);
        self.inner.is_prepared.set(false);
        self.inner.is_ended.set(false);
        self.inner.rate.set(1.0);
        *self.inner.last_error.borrow_mut() = None;
    }

    pub fn clear(&self) {
        self.pause();
        self.reset_for_new_source();
        self.inner.playbin.set_property("uri", Option::<&str>::None);
        self.inner.has_stream.set(false);
        self.inner.is_remote.set(false);
    }

    fn trace_prepared(&self) {
        if let Some((timing, started)) = self.inner.startup.borrow().as_ref() {
            log_audio_timing(&timing.id, "preparacion-gstreamer", *started);
        }
    }

    pub(crate) fn startup_timing(&self) -> Option<AudioStartup> {
        self.inner
            .startup
            .borrow()
            .as_ref()
            .map(|(timing, _)| timing.clone())
    }

    pub fn set_stream(&self, stream: &StreamInfo) {
        self.set_stream_for_startup(stream, None);
    }

    pub(crate) fn set_stream_for_startup(&self, stream: &StreamInfo, timing: Option<AudioStartup>) {
        self.reset_for_new_source();
        if let Some(timing) = timing {
            log_audio_timing(&timing.id, "seleccion-a-fuente", timing.selected_at);
            *self.inner.startup.borrow_mut() = Some((timing, Instant::now()));
        }
        *self.inner.local_audio.borrow_mut() = stream.local_audio.clone();

        if let Ok(mut ua) = self.inner.pending_user_agent.lock() {
            *ua = stream.user_agent.clone();
        }
        if let Ok(mut h) = self.inner.pending_headers.lock() {
            *h = stream.headers.clone();
        }

        let url_or_path = stream.url.as_str();
        let is_remote = url_or_path.starts_with("http://") || url_or_path.starts_with("https://");
        let uri = if is_remote {
            url_or_path.to_string()
        } else {
            gio::File::for_path(url_or_path).uri().to_string()
        };

        self.inner.playbin.set_property("uri", &uri);
        self.inner.has_stream.set(true);
        self.inner.is_remote.set(is_remote);
        let _ = self.inner.playbin.set_state(gst::State::Paused);
    }

    pub fn has_stream(&self) -> bool {
        self.inner.has_stream.get()
    }

    pub fn is_remote_stream(&self) -> bool {
        self.inner.is_remote.get()
    }

    pub fn play(&self) {
        self.inner.is_ended.set(false);
        let _ = self.inner.playbin.set_state(gst::State::Playing);
        if !self.inner.is_playing.get() {
            self.inner.is_playing.set(true);
            for cb in self.inner.on_playing.borrow().iter() {
                cb(self);
            }
        }
    }

    pub fn pause(&self) {
        let _ = self.inner.playbin.set_state(gst::State::Paused);
        if self.inner.is_playing.get() {
            self.inner.is_playing.set(false);
            for cb in self.inner.on_playing.borrow().iter() {
                cb(self);
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        self.inner.is_playing.get()
    }

    pub fn is_prepared(&self) -> bool {
        self.inner.is_prepared.get()
    }

    pub fn is_prerolled(&self) -> bool {
        let (result, current, pending) = self.inner.playbin.state(gst::ClockTime::ZERO);
        matches!(result, Ok(gst::StateChangeSuccess::Success))
            && current == gst::State::Paused
            && pending == gst::State::VoidPending
    }

    pub fn is_ended(&self) -> bool {
        self.inner.is_ended.get()
    }

    pub fn error(&self) -> Option<String> {
        self.inner.last_error.borrow().clone()
    }

    pub fn timestamp(&self) -> i64 {
        self.inner
            .playbin
            .query_position::<gst::ClockTime>()
            .map(|t| t.useconds() as i64)
            .unwrap_or(0)
    }

    pub fn duration(&self) -> i64 {
        self.inner
            .playbin
            .query_duration::<gst::ClockTime>()
            .map(|t| t.useconds() as i64)
            .unwrap_or(0)
    }

    pub fn seek(&self, target_us: i64) {
        let ns = (target_us.max(0) as u64).saturating_mul(1_000);
        let rate = self.inner.rate.get();
        let target = gst::ClockTime::from_nseconds(ns);
        let ok = self
            .inner
            .playbin
            .seek(
                rate,
                gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
                gst::SeekType::Set,
                target,
                gst::SeekType::None,
                gst::ClockTime::NONE,
            )
            .is_ok();
        if !ok {
            let _ = self.inner.playbin.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT | gst::SeekFlags::SNAP_NEAREST,
                target,
            );
        }
    }

    pub fn set_volume(&self, volume: f64) {
        self.inner
            .playbin
            .set_property("volume", volume.clamp(0.0, 1.0));
    }

    #[allow(dead_code)]
    pub fn set_rate(&self, rate: f64) {
        let clamped = rate.clamp(0.5, 2.0);
        self.inner.rate.set(clamped);
        let pos_ns = (self.timestamp().max(0) as u64).saturating_mul(1_000);
        let _ = self.inner.playbin.seek(
            clamped,
            gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
            gst::SeekType::Set,
            gst::ClockTime::from_nseconds(pos_ns),
            gst::SeekType::None,
            gst::ClockTime::NONE,
        );
    }

    pub fn connect_prepared_notify<F: Fn(&Self) + 'static>(&self, f: F) {
        self.inner.on_prepared.borrow_mut().push(Box::new(f));
    }

    pub fn connect_playing_notify<F: Fn(&Self) + 'static>(&self, f: F) {
        self.inner.on_playing.borrow_mut().push(Box::new(f));
    }

    pub fn connect_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) {
        self.inner.on_timestamp.borrow_mut().push(Box::new(f));
    }

    pub fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) {
        self.inner.on_duration.borrow_mut().push(Box::new(f));
    }

    pub fn connect_ended_notify<F: Fn(&Self) + 'static>(&self, f: F) {
        self.inner.on_ended.borrow_mut().push(Box::new(f));
    }

    pub fn connect_error_notify<F: Fn(&Self) + 'static>(&self, f: F) {
        self.inner.on_error.borrow_mut().push(Box::new(f));
    }
}

#[cfg(test)]
pub(crate) fn preroll_and_seek_for_test(
    stream: &StreamInfo,
) -> Result<(std::time::Duration, i64), gst::StateChangeError> {
    let deck = GStreamerDeck::new();
    let sink = gst::ElementFactory::make("fakesink").build().unwrap();
    deck.inner.playbin.set_property("audio-sink", &sink);
    let start = std::time::Instant::now();
    deck.set_stream(stream);
    let _ = deck.inner.playbin.state(gst::ClockTime::from_seconds(12));
    let result = (|| {
        if !deck.is_prerolled() {
            return Err(gst::StateChangeError);
        }
        let prepared = start.elapsed();
        let target = gst::ClockTime::from_seconds(30);
        deck.inner.playbin.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE, target)
            .map_err(|_| gst::StateChangeError)?;
        let (state, _, _) = deck.inner.playbin.state(gst::ClockTime::from_seconds(12));
        state?;
        let position = deck.timestamp();
        if (position - target.useconds() as i64).abs() > 1_000_000 {
            return Err(gst::StateChangeError);
        }
        Ok((prepared, position))
    })();
    deck.clear();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_is_ready_after_preroll_and_clear_removes_its_source() {
        let deck = GStreamerDeck::new();
        let sink = gst::ElementFactory::make("fakesink").build().unwrap();
        deck.inner.playbin.set_property("audio-sink", &sink);
        assert!(!deck.is_prerolled());

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("silence.wav");
        // PCM mono, 8 kHz, 16-bit, one second of silence.
        let mut wav = b"RIFF".to_vec();
        wav.extend_from_slice(&16_036_u32.to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt \x10\x00\x00\x00\x01\x00\x01\x00");
        wav.extend_from_slice(&8_000_u32.to_le_bytes());
        wav.extend_from_slice(&16_000_u32.to_le_bytes());
        wav.extend_from_slice(b"\x02\x00\x10\x00data");
        wav.extend_from_slice(&16_000_u32.to_le_bytes());
        wav.resize(wav.len() + 16_000, 0);
        std::fs::write(&path, wav).unwrap();

        let stream = StreamInfo {
            url: path.to_string_lossy().into_owned(),
            quality_label: "WAV".into(),
            user_agent: None,
            headers: HashMap::new(),
            local_audio: None,
        };
        deck.set_stream_for_startup(
            &stream,
            Some(AudioStartup { id: "local-test".into(), selected_at: Instant::now() }),
        );
        let (result, _, _) = deck.inner.playbin.state(gst::ClockTime::from_seconds(3));
        assert_eq!(result, Ok(gst::StateChangeSuccess::Success));
        assert!(deck.is_prerolled());
        deck.play();
        assert!(deck.startup_timing().is_some());
        let context = gst::glib::MainContext::default();
        let deadline = Instant::now() + std::time::Duration::from_secs(3);
        while deck.startup_timing().is_some() && Instant::now() < deadline {
            context.iteration(false);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        assert!(deck.startup_timing().is_none(), "GStreamer did not report Playing");
        deck.inner.is_remote.set(true);
        deck.clear();
        assert!(!deck.is_prerolled());
        assert!(!deck.has_stream());
        assert!(!deck.is_remote_stream());
        assert!(!deck.is_prepared());
        assert_eq!(deck.inner.playbin.property::<Option<String>>("uri"), None);
    }
}
