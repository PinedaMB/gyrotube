mod backend;
mod models;
mod mpris;
mod ui;

use gtk4::glib;

fn main() -> glib::ExitCode {
    ui::run()
}
