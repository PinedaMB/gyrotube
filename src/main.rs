mod backend;
mod models;
mod mpris;
mod ui;

use adw::prelude::*;
use gtk4::glib;
use libadwaita as adw;

const APP_ID: &str = "com.github.gyrotube.GyroTube";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}
