mod imp;

use glib::Object;
use gtk::{gio, glib, Application};

use crate::preview::Preview;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application, preview: Preview) -> Self {
        // TODO: find how to pass down preview
        Object::new(&[("application", app)]).expect("Failed to create MainWindow")
    }
}
