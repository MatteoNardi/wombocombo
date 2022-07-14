mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Preview(ObjectSubclass<imp::Preview>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Buildable;
}

impl Preview {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create Preview")
    }
}
