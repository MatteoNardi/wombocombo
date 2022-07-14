mod main_window;

use gtk::prelude::*;
use gtk::{self, Application};
use main_window::MainWindow;

const APP_ID: &str = "org.matteonardi.WomboCombo";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = MainWindow::new(app);
    window.present();
}
