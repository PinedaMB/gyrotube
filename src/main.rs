mod backend;
mod models;
mod mpris;
mod stream_input;
mod ui;

use gtk4::glib;

fn main() -> glib::ExitCode {
    ui::run()
}
