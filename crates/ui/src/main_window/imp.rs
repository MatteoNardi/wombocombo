use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

use crate::preview::Preview;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub preview: TemplateChild<Preview>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        let preview = self.preview.clone();
        self.button.connect_clicked(move |_button| {
            // Set the label to "Hello World!" after the button has been clicked on
            preview.buffer().set_text("Hello World!");
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for MainWindow {}

// Trait shared by all windows
impl WindowImpl for MainWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for MainWindow {}
