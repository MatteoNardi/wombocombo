# Ideas

Maybe opening the evdev device is simpler for showing the keyboard preview!
[](https://xkbcommon.org/doc/current/md_doc_quick_guide.html)
We need to open the device though! ..and that requires root access.
A PolKit policy to open that device would be awesome, but we need it.

# Simulation

I simulate by opening an evdev device (asking root permissions only for that).
I don't care about simulating compose key (it's not what this is used for)

# Links

Some code to parse xkb files:
https://github.com/divvun/xkb-parser

Important files:
/home/matteo/.config/xkb/Readme.md


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

Oh..let's do it the HARD way (which is actually the simplest way since
WomboCombo users won't probably use RMLVO)


This compiles the current active configuration: `xkbcomp $DISPLAY output.xkb`


# Activation

```
dconf write /org/gnome/desktop/input-sources/xkb-options "['caps:ctrl_modifier', 'lv3:ralt_switch', 'custom:foo']"
```

# Files created

~/.config/xkb/rules/evdev
```
! option                =       symbols
  custom:foo            =       +custom(foo)
  custom:bar            =       +custom(baz)

! include %S/evdev
```

~/.config/xkb/rules/evdev.xml
```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE xkbConfigRegistry SYSTEM "xkb.dtd">
<xkbConfigRegistry version="1.1">
  <layoutList>              
    <layout>               
      <configItem>        
        <name>us</name>  
      </configItem>     
      <variantList>                             
        <variant>                               
          <configItem>                          
            <name>banana</name>                 
            <shortDescription>banana</shortDescription>     
            <description>US (Banana)</description>       
          </variant>                                     
      </variantList>                                     
  </layoutList>                                          
  <optionList>                                           
    <group allowMultipleSelection="true">                
      <configItem>                                       
        <name>custom</name>                              
        <description>Custom options</description>        
      </configItem>                                      
      <option>                                           
      <configItem>                                       
        <name>custom:foo</name>                          
        <description>Map Tilde to nothing</description>  
      </configItem>                                      
      </option>                                          
      <option>                                           
      <configItem>                                       
        <name>custom:baz</name>                          
        <description>Map Z to K</description>            
      </configItem>                                      
      </option>                                          
    </group>                                             
  </optionList>                                          
</xkbConfigRegistry>   
```

~/.config/xkb/symbols/custom
```
partial alphanumeric_keys
xkb_symbols "foo" {
    key <AC10> { type="LOCAL_EIGHT_LEVEL", symbols[Group1]= [ semicolon, colon, b, b, b ] };                                                            
};
```
