use evdev::{Device, Key};
use std::path::PathBuf;

pub fn list_devices() -> Vec<(PathBuf, Device)> {
    evdev::enumerate()
        .filter(|(_, device)| {
            device
                .supported_keys()
                .map_or(false, |keys| keys.contains(Key::KEY_ENTER))
        })
        .collect()
}
