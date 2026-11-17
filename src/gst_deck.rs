use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};

use gstreamer as gst;
use gstreamer::prelude::*;
use gtk4::gio;
use gtk4::gio::prelude::FileExt;
use gtk4::glib;

static GST_INIT: OnceLock<Result<(), String>> = OnceLock::new();

fn ensure_gst_init() -> Result<(), String> {
    GST_INIT
        .get_or_init(|| gst::init().map_err(|e| format!("Failed to init GStreamer: {e}")))
        .clone()
}

struct DeckInner {
    playbin: gst::Element,
    _bus_watch: RefCell<Option<gst::bus::BusWatchGuard>>,
    pending_user_agent: Arc<Mutex<Option<String>>>,
    pending_headers: Arc<Mutex<HashMap<String, String>>>,
    is_playing: Cell<bool>,
    is_prepared: Cell<bool>,
    is_ended: Cell<bool>,
    has_stream: Cell<bool>,
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
                source.set_property("timeout", 8u32);
            }
            if source.has_property("retries") {
                source.set_property("retries", 1i32);
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
            is_ended: Cell::new(false),
            has_stream: Cell::new(false),
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
                            for cb in strong.on_prepared.borrow().iter() {
                                cb(&deck);
                            }
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

    pub fn set_stream(
        &self,
        url_or_path: &str,
        user_agent: Option<&str>,
        headers: &HashMap<String, String>,
    ) {
        self.reset_for_new_source();

        if let Ok(mut ua) = self.inner.pending_user_agent.lock() {
            *ua = user_agent.map(str::to_string);
        }
        if let Ok(mut h) = self.inner.pending_headers.lock() {
            *h = headers.clone();
        }

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
        let _ = self.inner.playbin.seek(
            rate,
            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
            gst::SeekType::Set,
            gst::ClockTime::from_nseconds(ns),
            gst::SeekType::None,
            gst::ClockTime::NONE,
        );
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
mod tests {
    use super::*;

    #[test]
    fn clearing_a_deck_removes_its_previous_source() {
        let deck = GStreamerDeck::new();
        deck.inner
            .playbin
            .set_property("uri", "file:///unused-test.wav");
        deck.inner.has_stream.set(true);
        deck.inner.is_remote.set(true);
        deck.clear();
        assert!(!deck.has_stream());
        assert!(!deck.is_remote_stream());
        assert!(!deck.is_prepared());
        assert_eq!(deck.inner.playbin.property::<Option<String>>("uri"), None);
    }
}
