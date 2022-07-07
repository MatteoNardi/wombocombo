use evdev::{EventType, InputEvent};

use crate::{
    config::Model,
    keycodes::{self, Keycode},
    preview::{Preview, KEYCODE_OFFSET, KEY_STATE_PRESS, KEY_STATE_RELEASE},
};

// test mapping Ctrl+; to Ctrl+b
// The quest for the best tmux prefix. The mapping that started it all.
#[test]
fn ctrl_semicolon_to_ctrl_b() {
    TestRunner::new(|model: &mut Model| {
        model.add_special_key("<AC10>", "semicolon colon b b b b");
    })
    .press(keycodes::CAPS, "")
    .press(keycodes::AC10, "b")
    .release(keycodes::CAPS, "")
    .release(keycodes::AC10, "");
}

struct TestRunner {
    preview: Preview,
}

impl TestRunner {
    fn new(mut setup: impl FnMut(&mut Model)) -> Self {
        const TEST_FOLDER: &'static str = "/tmp/test_xkb";
        std::env::set_var("XDG_CONFIG_HOME", TEST_FOLDER);
        std::env::set_var("XKB_DEFAULT_OPTIONS", "custom:wombo,caps:ctrl_modifier");
        let _ = std::fs::remove_dir_all(TEST_FOLDER);

        let mut model = Model::new();
        setup(&mut model);
        model.export().unwrap();

        let preview = Preview::new().unwrap();
        Self { preview }
    }

    /// Set key as pressed and check we've produced the expected results
    fn press(&mut self, key: Keycode, expectation: &str) -> &mut Self {
        self.send(key, KEY_STATE_PRESS, expectation);
        self
    }

    /// Set key as depressed and check we've produced the expected results
    fn release(&mut self, key: Keycode, expectation: &str) -> &mut Self {
        self.send(key, KEY_STATE_RELEASE, expectation);
        self
    }

    fn send(&mut self, key: Keycode, status: i32, expectation: &str) {
        let output = self
            .preview
            .process(InputEvent::new(
                EventType::KEY,
                key - KEYCODE_OFFSET,
                status,
            ))
            .unwrap()
            .unwrap();
        println!("{}", output.to_string());
        assert_eq!(output.keycode, key as u32);
        assert_eq!(output.utf8, expectation);
    }
}
