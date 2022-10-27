mod main_window;
mod preview;
#[rustfmt::skip]
mod config;

use adw::prelude::*;
use gettextrs::{gettext, LocaleCategory};

use crate::config::{
    APP_ID, GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR, PROFILE, RESOURCES_FILE, VERSION,
};

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();

    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("WomboCombo"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = main_window::MainWindow::new(app);
    let _preview = window.preview();
    window.present();

    println!("Hello ({})", APP_ID);
    println!("Version: {} ({})", VERSION, PROFILE);
    println!("Datadir: {}", PKGDATADIR);
}
