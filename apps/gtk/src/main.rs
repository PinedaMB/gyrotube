pub use gyrotube_core::{audio_cache, audio_streamer, backend, i18n, jam, models, spin_remote};
mod gst_deck;
mod mpris;
mod ui;

pub mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

use gtk4::glib;

fn main() -> glib::ExitCode {
    ui::run()
}
