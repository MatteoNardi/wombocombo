mod imp;

use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application};

use crate::preview::Preview;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create MainWindow")
    }

    pub fn preview(&self) -> Preview {
        self.imp().preview.clone()
    }
}
