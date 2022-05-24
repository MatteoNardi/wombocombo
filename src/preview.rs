//! Open an Evdev device in order to preview keyboard configurations.

use std::{ffi::OsString, path::Path};

use anyhow::{Context as AnyhowContext, Result};
use evdev::{Device, InputEventKind, Key};
use xkbcommon::{
    self,
    xkb::{
        self, compose, keysym_get_name, Context, KeyDirection, Keymap, State, COMPILE_NO_FLAGS,
        CONTEXT_NO_FLAGS,
    },
};

const KEYCODE_OFFSET: u16 = 8;
const KEY_STATE_RELEASE: i32 = 0;
const KEY_STATE_REPEAT: i32 = 2;

pub fn run_preview(path: &Path) -> Result<()> {
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

    let with_compose = true;

    let context = Context::new(CONTEXT_NO_FLAGS);
    let keymap = Keymap::new_from_names(&context, "", "", "", "", None, COMPILE_NO_FLAGS)
        .context("creating keymap")?;
    let mut state = State::new(&keymap);
    let compose_table =
        compose::Table::new_from_locale(&context, &OsString::from("C"), COMPILE_NO_FLAGS)
            .map_err(|_| anyhow::anyhow!("error creating compose table"))?;
    let mut compose_state = compose::State::new(&compose_table, COMPILE_NO_FLAGS);

    loop {
        for event in device.fetch_events().context("fetching events")? {
            if let InputEventKind::Key(Key(keycode)) = event.kind() {
                let value = event.value();

                let keycode = (keycode + KEYCODE_OFFSET).into();

                if value == KEY_STATE_REPEAT && !keymap.key_repeats(keycode) {
                    continue;
                }

                if with_compose && value != KEY_STATE_RELEASE {
                    let keysym = state.key_get_one_sym(keycode);
                    compose_state.feed(keysym);
                }

                if value != KEY_STATE_RELEASE {
                    tools_print_keycode_state(&state, &compose_state, keycode);
                }

                if with_compose {
                    let status = compose_state.status();
                    if status == compose::Status::Cancelled || status == compose::Status::Composed {
                        compose_state.reset();
                    }
                }

                let changed = if value == KEY_STATE_RELEASE {
                    state.update_key(keycode, KeyDirection::Up)
                } else {
                    state.update_key(keycode, KeyDirection::Down)
                };

                tools_print_state_changes(changed);
            }
        }
    }
}

fn tools_print_keycode_state(
    state: &State,
    compose_state: &compose::State,
    keycode: u32,
) -> Result<()> {
    let keymap = state.get_keymap();

    let mut syms: Vec<u32> = state.key_get_syms(keycode).into();

    if syms.len() == 0 {
        return Ok(());
    }

    let status = compose_state.status();

    if status == compose::Status::Composing || status == compose::Status::Cancelled {
        return Ok(());
    }

    if status == compose::Status::Composed {
        syms = vec![compose_state.keysym().context("getting keysym")?];
    } else if syms.len() == 1 {
        syms = vec![state.key_get_one_sym(keycode)]
    }

    print!("keycode [ {} ] ", keycode);

    print!("keysyms [ ");
    for keysym in syms {
        print!("{} ", keysym_get_name(keysym));
    }
    print!("] ");

    let s = if status == compose::Status::Composed {
        compose_state.utf8().context("getting utf value")?
    } else {
        state.key_get_utf8(keycode)
    };
    print!("unicode [ {} ] ", s);

    let layout = state.key_get_layout(keycode);
    print!(
        "layout [ {} ({}) ] ",
        keymap.layout_get_name(layout),
        layout
    );

    print!("level [ {} ] ", state.key_get_level(keycode, layout));

    //print!("mods [ ");
    //for (xkb_mod_index_t mod = 0; mod < xkb_keymap_num_mods(keymap); mod++) {
    //    if (xkb_state_mod_index_is_active(state, mod,
    //                                      XKB_STATE_MODS_EFFECTIVE) <= 0)
    //        continue;
    //    if (xkb_state_mod_index_is_consumed2(state, keycode, mod,
    //                                         consumed_mode))
    //        print!("-%s ", xkb_keymap_mod_get_name(keymap, mod));
    //    else
    //        print!("%s ", xkb_keymap_mod_get_name(keymap, mod));
    //}
    //print!("] ");

    //print!("leds [ ");
    //for (xkb_led_index_t led = 0; led < xkb_keymap_num_leds(keymap); led++) {
    //    if (xkb_state_led_index_is_active(state, led) <= 0)
    //        continue;
    //    print!("%s ", xkb_keymap_led_get_name(keymap, led));
    //}
    //print!("] ");

    print!("\n");
    Ok(())
}

fn tools_print_state_changes(changed: u32) {
    if changed == 0 {
        return;
    }

    print!("changed [ ");
    if changed & xkb::STATE_LAYOUT_EFFECTIVE != 0 {
        print!("effective-layout ");
    }
    if changed & xkb::STATE_LAYOUT_DEPRESSED != 0 {
        print!("depressed-layout ");
    }
    if changed & xkb::STATE_LAYOUT_LATCHED != 0 {
        print!("latched-layout ");
    }
    if changed & xkb::STATE_LAYOUT_LOCKED != 0 {
        print!("locked-layout ");
    }
    if changed & xkb::STATE_MODS_EFFECTIVE != 0 {
        print!("effective-mods ");
    }
    if changed & xkb::STATE_MODS_DEPRESSED != 0 {
        print!("depressed-mods ");
    }
    if changed & xkb::STATE_MODS_LATCHED != 0 {
        print!("latched-mods ");
    }
    if changed & xkb::STATE_MODS_LOCKED != 0 {
        print!("locked-mods ");
    }
    if changed & xkb::STATE_LEDS != 0 {
        print!("leds ");
    }
    println!("]");
}
