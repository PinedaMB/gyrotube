mod backend;
mod gst_deck;
mod models;
mod mpris;
mod ui;

use gtk4::glib;

fn main() -> glib::ExitCode {
    ui::run()
}
