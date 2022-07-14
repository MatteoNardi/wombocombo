//! Open an Evdev device in order to preview keyboard configurations.

use std::{ffi::OsString, fmt, path::Path};

use anyhow::{Context as AnyhowContext, Result};
use evdev::{Device, InputEvent, InputEventKind, Key};
use xkbcommon::{
    self,
    xkb::{
        self, compose, ffi::XKB_STATE_MODS_EFFECTIVE, Context, KeyDirection, Keymap, State,
        COMPILE_NO_FLAGS, CONTEXT_NO_FLAGS,
    },
};

pub const KEYCODE_OFFSET: u16 = 8;
pub const KEY_STATE_RELEASE: i32 = 0;
pub const KEY_STATE_PRESS: i32 = 1;
pub const KEY_STATE_REPEAT: i32 = 2;

pub struct Preview {
    _context: Context,
    keymap: Keymap,
    state: State,
    compose: Option<(compose::Table, compose::State)>,
}

impl Preview {
    pub fn new() -> Result<Self> {
        let context = Context::new(CONTEXT_NO_FLAGS);
        let keymap = Keymap::new_from_names(&context, "", "", "", "", None, COMPILE_NO_FLAGS)
            .context("creating keymap")?;
        let state = State::new(&keymap);
        let compose_table =
            compose::Table::new_from_locale(&context, &OsString::from("C"), COMPILE_NO_FLAGS)
                .map_err(|_| anyhow::anyhow!("error creating compose table"))?;
        let compose_state = compose::State::new(&compose_table, COMPILE_NO_FLAGS);
        Ok(Self {
            _context: context,
            keymap,
            state,
            compose: Some((compose_table, compose_state)),
        })
    }

    pub fn read_from_device(mut self, path: &Path) -> Result<()> {
        let mut device = Device::open(path).with_context(|| format!("opening {path:?}"))?;
        // check if the device has an ENTER key
        if device
            .supported_keys()
            .map_or(false, |keys| keys.contains(Key::KEY_ENTER))
        {
            println!("are you prepared to ENTER the world of evdev?");
        } else {
            println!(":(");
        }

        loop {
            for event in device.fetch_events().context("fetching events")? {
                if let Some(output) = self.process(event)? {
                    println!("{}", output);
                }
            }
        }
    }

    pub fn process(&mut self, event: InputEvent) -> Result<Option<Output>> {
        if let InputEventKind::Key(Key(keycode)) = event.kind() {
            let value = event.value();
            let keycode = (keycode + KEYCODE_OFFSET).into();
            let mut output = Output {
                keycode,
                utf8: String::new(),
                level: 0,
                mods: vec![],
                state_change: 0,
            };

            // skip key repeat if keymap says so
            if value == KEY_STATE_REPEAT && !self.keymap.key_repeats(keycode) {
                return Ok(None);
            }

            // keys produce changes on press, not on release
            if value != KEY_STATE_RELEASE {
                // update compose state with the pressed key
                if let Some((_, compose_state)) = self.compose.as_mut() {
                    let keysym = self.state.key_get_one_sym(keycode);
                    compose_state.feed(keysym);
                }
                // get the effect of the pressed key
                let compose_state = self.compose.as_ref().map(|x| &x.1);
                output.from_state(&self.state, compose_state)?;
            }

            if let Some((_, compose_state)) = self.compose.as_mut() {
                let status = compose_state.status();
                if status == compose::Status::Cancelled || status == compose::Status::Composed {
                    compose_state.reset();
                }
            }

            // update state
            output.state_change = if value == KEY_STATE_RELEASE {
                self.state.update_key(keycode, KeyDirection::Up)
            } else {
                self.state.update_key(keycode, KeyDirection::Down)
            };

            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}

pub struct Output {
    /// keycode originating this change
    pub keycode: u32,
    /// UTF8 output produced
    pub utf8: String,
    /// Keyboard level
    pub level: u32,
    /// Active mods
    pub mods: Vec<String>,
    /// State change
    pub state_change: u32,
}

impl Output {
    fn from_state(&mut self, state: &State, compose_state: Option<&compose::State>) -> Result<()> {
        self.utf8 = state.key_get_utf8(self.keycode);

        if let Some(compose_state) = compose_state {
            match compose_state.status() {
                xkb::Status::Nothing => {}
                xkb::Status::Composing | xkb::Status::Cancelled => {
                    self.utf8 = String::new();
                }
                xkb::Status::Composed => {
                    // override output
                    self.utf8 = compose_state.utf8().context("getting utf value")?;
                }
            }
        }

        let layout = state.key_get_layout(self.keycode);
        self.level = state.key_get_level(self.keycode, layout);
        let keymap = state.get_keymap();

        self.mods = (0..keymap.num_mods())
            .into_iter()
            .filter(|idx| state.mod_index_is_active(*idx, XKB_STATE_MODS_EFFECTIVE))
            .map(|idx| {
                let mod_name = keymap.mod_get_name(idx);
                if state.mod_index_is_consumed(self.keycode, idx) {
                    format!("-{}", mod_name)
                } else {
                    mod_name.to_string()
                }
            })
            .collect();
        Ok(())
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "keycode [ {} ] ", self.keycode)?;
        write!(f, "utf8 [ {} ] ", self.utf8)?;
        write!(f, "level [ {} ] ", self.level)?;
        write!(f, "mods [ ")?;
        for mode in &self.mods {
            write!(f, "{} ", mode)?;
        }
        write!(f, "] ")?;

        let changed = self.state_change;
        if changed == 0 {
            write!(f, "changed [ ")?;
            if changed & xkb::STATE_LAYOUT_EFFECTIVE != 0 {
                write!(f, "effective-layout ")?;
            }
            if changed & xkb::STATE_LAYOUT_DEPRESSED != 0 {
                write!(f, "depressed-layout ")?;
            }
            if changed & xkb::STATE_LAYOUT_LATCHED != 0 {
                write!(f, "latched-layout ")?;
            }
            if changed & xkb::STATE_LAYOUT_LOCKED != 0 {
                write!(f, "locked-layout ")?;
            }
            if changed & xkb::STATE_MODS_EFFECTIVE != 0 {
                write!(f, "effective-mods ")?;
            }
            if changed & xkb::STATE_MODS_DEPRESSED != 0 {
                write!(f, "depressed-mods ")?;
            }
            if changed & xkb::STATE_MODS_LATCHED != 0 {
                write!(f, "latched-mods ")?;
            }
            if changed & xkb::STATE_MODS_LOCKED != 0 {
                write!(f, "locked-mods ")?;
            }
            if changed & xkb::STATE_LEDS != 0 {
                write!(f, "leds ")?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}
