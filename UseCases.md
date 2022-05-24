# Use Cases

This is a list of use cases achievable using xkb configuration.

## Map Super+alphakey to other key

https://geekhack.org/index.php?topic=53137.msg

```
type "SHIFT+SUPER" {
     modifiers= Shift+Super;
     map[Shift]= Level2;
     map[Super]= Level3;
     map[Shift+Super]= Level3;
preserve[Shift+Super]= Shift;
     level_name[Level1]= "Base";
     level_name[Level2]= "Shift";
     level_name[Level3]= "Super";
};
```


```
key <AD02> {
     type= "SHIFT+SUPER",
     symbols[Group1]= [   w,             W,             NoSymbol  ],
    actions[Group1]= [   NoAction(), NoAction(),  RedirectKey(key=<UP>, clearMods=Super) ]
};
```
