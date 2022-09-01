mod main_window;
mod preview;

use gio::glib::clone;
use gtk::prelude::*;
use gtk::{self, Application};
use main_window::MainWindow;

const APP_ID: &str = "org.matteonardi.WomboCombo";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    glib::timeout_add_seconds_local_once(4, clone!(@weak app => move || { app.quit(); }));

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = MainWindow::new(app);
    let _preview = window.preview();
    window.present();
}
