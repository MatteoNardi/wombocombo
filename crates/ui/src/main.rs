mod main_window;
mod preview;

use adw::prelude::*;
use gio::glib::clone;
use main_window::MainWindow;

const APP_ID: &str = "org.matteonardi.WomboCombo";

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();

    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    glib::timeout_add_seconds_local_once(4, clone!(@weak app => move || { app.quit(); }));

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = MainWindow::new(app);
    let _preview = window.preview();
    window.present();
}
