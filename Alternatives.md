# Alternatives

Here is a walkthrough of alternative software for keyboard configuration:
- karabiner. Mac only. Virtual device-based
- [input-remapper](https://github.com/sezanzeb/input-remapper). uinput-based
- kmonad. uinput-based.
- sxhkd. uinput-based.
- QMK. firmware. Used by ergodox ez.

# Why choosing Wombo Combo?

Most solutions grab control of the real keyboard device and forward the modified
events to a uinput virtual keyboard.
Wombo Combo generates configuration for the existing configuration system (XKB).

The advantages of this approach are:
- You don't need to keep a daemon running.
- You can uninstall Wombo Combo once you've made your configuration.
- Root privileges are not required

The disadvantages are:
- Less flexibility with what we can do.
- We can't really make combos or multi-sequence combinations (making the Wombo
  Combo name horribly misleading)

## Karabiner

- Simple modifications: change a key to another key
- Use more complex rules
  - Modifier flags + key to keys:
        Change control-m to return key.
        Change control-h to delete key.
  - Key to modifier flags + keys:
        Change caps lock to command+control+option+shift key
  - Post other key events when a key pressed alone:
        Post escape key when you press left control key alone.
  - Change keys in specific apps:
        Change left command key to left control key in virtual machines.
  - Execute shell command:
        Open Activity Monitor by right shift+a.
- GokuRakuJoudo config generator DSL
- Limit to some devices
- Doesn't change keyboard type/layout. You use Mac Os for that.
- application-specific rules
- [downloadable community rules](https://ke-complex-modifications.pqrs.org/)
- f-mode? make f (or any other alpha) a modifier
- emoji mode as z-mode

[This video](https://egghead.io/talks/egghead-save-your-hands-and-save-your-time-rethinking-how-to-use-a-keyboard)
shows some really interesting use-cases.

Karabiner-Elements treats input events by the following order:

    Catch events from hardware.
    Apply Simple Modifications.
    Apply Complex Modifications.
    Apply Function Keys Modifications. (change f1…f12 keys to media controls)
    Post events to applications via a virtual keyboard.


## Input remapper


## QMK

- Auto Shift: Tap a key, but hold it slightly longer and you get its shifted state.
- [Key override](https://docs.qmk.fm/#/feature_key_overrides) is similar to what
  we can achieve with xkb.
- Layers
  - Special keys:
    - tap sends key, holding changes layer 
    - toggle layer

## TODO:
- checkout libei (maybe emulator could be a solution?)
