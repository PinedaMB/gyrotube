mod audio_cache;
mod audio_streamer;
mod backend;
mod gst_deck;
mod i18n;
mod jam;
mod models;
mod mpris;
mod spin_remote;
mod ui;

pub mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

use gtk4::glib;

fn main() -> glib::ExitCode {
    ui::run()
}
