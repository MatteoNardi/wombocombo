mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::{gio, glib};

use crate::preview::Preview;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create MainWindow")
    }

    pub fn preview(&self) -> Preview {
        self.imp().preview.clone()
    }
}
