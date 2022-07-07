# Wombo Combo

Wombo Combo will be a keyboard GUI configuration tool for the Linux
[XKB](https://www.x.org/wiki/XKB/) system.

It saves its output to the `~/config/xkb/` folder, allowing configuration
without messing with system files.
These local configurations are compatible with (libxkbcommon)[https://xkbcommon.org/],
which means it won't work on X systems, but only when running Wayland.

## Warning
I'm currently exploring the XKB possibilities and limitations. This is pre-aplha
software, don't use it.

## Instant preview

I'd like Wombo Combo to have instant preview of its changes.
I could open an evdev device directly, but that requires root access.
For now I'll require to change the specific device permissions.
Maybe a PolKit policy would allow us to open that device.

## Useful Links

Some code to parse xkb files:
https://github.com/divvun/xkb-parser

`/usr/include/X11/keysymdef.h` contains the keysym definitions.
Unicode characters don't need one.

`/usr/share/X11/xkb/` contain default components configurations:
compat/geometry/keycodes/rules/symbols/types
Each component can contain multiple variants, defined by
`[default] xkb_keycodes "<name" { ... }`
They can be included with `xfree86(pc102)`
See: https://www.charvolant.org/doug/xkb/html/node4.html
+overrides, /augments the symbols defined in it.
Eg. `us(pc101)+ctrl(swapcaps).`

This compiles the current active configuration: `xkbcomp $DISPLAY output.xkb`


## Gnome Activation

Xkb options can be enabled in gnome with:

```
dconf write /org/gnome/desktop/input-sources/xkb-options "['caps:ctrl_modifier', 'lv3:ralt_switch', 'custom:foo']"
```

When we'll do this directly in the application, we'll require DConf access.
With Flatpak we'll need to add:

```
--filesystem=xdg-run/dconf
--filesystem=~/.config/dconf:ro
--talk-name=ca.desrt.dconf
--env=DCONF_USER_CONFIG_DIR=.config/dconf
```

## Files created

~/.config/xkb/rules/evdev
~/.config/xkb/rules/evdev.xml
~/.config/xkb/symbols/custom
