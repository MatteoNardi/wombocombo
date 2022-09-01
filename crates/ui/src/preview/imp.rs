use adw::subclass::prelude::*;

use adw::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, TextView};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/preview.ui")]
pub struct Preview {
    // TODO: find out why this doesn't activate
    // TODO: fill it with available devices
    // TODO: Add actual preview
    #[template_child]
    pub text: TemplateChild<TextView>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Preview {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyPreview";
    type Type = super::Preview;
    type ParentType = gtk::Box;

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
        self.text.buffer().set_text("hello world from wombo");
    }
}

// Trait shared by all widgets
impl WidgetImpl for Preview {}

impl BoxImpl for Preview {}
