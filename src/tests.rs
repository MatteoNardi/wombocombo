use evdev::{EventType, InputEvent};

use crate::{
    config::Model,
    preview::{Preview, KEYCODE_OFFSET, KEY_STATE_PRESS, KEY_STATE_RELEASE},
};

// test mapping Ctrl+; to Ctrl+b
// The quest for the best tmux prefix. The mapping that started it all.
#[test]
fn ctrl_semicolon_to_ctrl_b() {
    const TEST_FOLDER: &'static str = "/tmp/test_xkb";
    std::env::set_var("XDG_CONFIG_HOME", TEST_FOLDER);
    std::env::set_var("XKB_DEFAULT_OPTIONS", "custom:wombo,caps:ctrl_modifier");
    let _ = std::fs::remove_dir_all(TEST_FOLDER);

    let mut model = Model::new();
    model.add_special_key("<AC10>", "semicolon colon b b b b");
    model.export().unwrap();

    let mut preview = Preview::new().unwrap();
    assert_eq!(
        preview
            .process(InputEvent::new(
                EventType::KEY,
                66 - KEYCODE_OFFSET,
                KEY_STATE_PRESS,
            ))
            .unwrap()
            .unwrap()
            .to_string(),
        "keycode [ 66 ] utf8 [  ] level [ 0 ] mods [ ] ".to_string()
    );
    assert_eq!(
        preview
            .process(InputEvent::new(
                EventType::KEY,
                47 - KEYCODE_OFFSET,
                KEY_STATE_PRESS,
            ))
            .unwrap()
            .unwrap()
            .to_string(),
        "keycode [ 47 ] utf8 [ b ] level [ 4 ] mods [ -Control ] changed [ ]".to_string()
    );
    assert_eq!(
        preview
            .process(InputEvent::new(
                EventType::KEY,
                47 - KEYCODE_OFFSET,
                KEY_STATE_RELEASE,
            ))
            .unwrap()
            .unwrap()
            .to_string(),
        "keycode [ 47 ] utf8 [  ] level [ 0 ] mods [ ] changed [ ]".to_string()
    );
    assert_eq!(
        preview
            .process(InputEvent::new(
                EventType::KEY,
                66 - KEYCODE_OFFSET,
                KEY_STATE_RELEASE,
            ))
            .unwrap()
            .unwrap()
            .to_string(),
        "keycode [ 66 ] utf8 [  ] level [ 0 ] mods [ ] ".to_string()
    );
}
