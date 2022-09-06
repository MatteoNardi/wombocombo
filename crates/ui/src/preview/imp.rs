use adw::{subclass::prelude::*, ComboRow};

use adw::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, StringList, TextView};
use model::discover::list_devices;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/preview.ui")]
pub struct Preview {
    // TODO: fill it with available devices
    // TODO: Add actual preview
    #[template_child]
    pub text: TemplateChild<TextView>,
    #[template_child]
    pub devices: TemplateChild<StringList>,
    #[template_child]
    pub combo: TemplateChild<ComboRow>,
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

        match list_devices() {
            Ok(devices) => {
                for device in &devices {
                    self.devices.append(&device.file_name().to_string_lossy());
                }

                let text_widget = self.text.clone();
                self.combo.connect_selected_notify(move |combo| {
                    let device = devices[combo.selected() as usize].path();
                    let d = device.to_string_lossy();
                    text_widget.buffer().set_text(&format!("Device {d}"));
                });
            }
            Err(_) => todo!(),
        }
    }
}

// Trait shared by all widgets
impl WidgetImpl for Preview {}

impl BoxImpl for Preview {}
