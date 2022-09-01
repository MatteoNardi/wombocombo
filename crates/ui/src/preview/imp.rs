use adw::subclass::prelude::*;

use adw::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/preview.ui")]
pub struct Preview {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Preview {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyPreview";
    type Type = super::Preview;
    type ParentType = gtk::TextView;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Preview {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        //TextViewExt::buffer();
        obj.buffer().set_text("hello world from wombo");
    }
}

// Trait shared by all widgets
impl WidgetImpl for Preview {}

impl TextViewImpl for Preview {}
