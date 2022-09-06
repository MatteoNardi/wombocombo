mod main_window;
mod preview;

use adw::prelude::*;

const APP_ID: &str = "org.matteonardi.WomboCombo";

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();

    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = main_window::MainWindow::new(app);
    let _preview = window.preview();
    window.present();
}
