mod backend;
mod gst_deck;
mod models;
mod mpris;
mod ui;

pub mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

use gtk4::glib;

fn main() -> glib::ExitCode {
    ui::run()
}
