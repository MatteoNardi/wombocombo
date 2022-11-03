// This requires super-user priviledges.
//
// TODO:
// - Make this a dbus service which runs as root
// - Add dbus activation on the system bus
//   - This requires a rule
// - Use polkit on the server
//   - This requires another rule
// - Pack everything in a flatpak
//
// https://ubuntuforums.org/showthread.php?t=1359397
use evdev::{Device, Key};
use std::path::PathBuf;

pub fn list_devices() -> Vec<(PathBuf, Device)> {
    evdev::enumerate()
        //.filter(|(_, device)| {
        //    device
        //        .supported_keys()
        //        .map_or(false, |keys| keys.contains(Key::KEY_ENTER))
        //})
        .collect()
}
