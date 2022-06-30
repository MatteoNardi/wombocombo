//! Model is the simplified configuration on which Wombo Combo works.
//! When saving, it's serialized and exported to the user ~/.config/xkb
//! folder.
//! At the moment this module is output only.

use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use anyhow::{Context, Result};

pub struct Model {
    special_keys: Vec<SpecialKey>,
}

// Special keys are mapped to the LOCAL_EIGHT_LEVEL type, which allows us
// to specify how the modifiers affect our symbol.
pub struct SpecialKey {
    // The input physical key. eg. "<AC10>"
    keycode: String,

    // The output symbols. eg. vec!["semicolon", "colon", "b"]
    symbols: Vec<String>,
}

// Content of ~/.config/xkb/rules/evdev when exporting configuration
const RULES_EVDEV: &str = r#"
! option                =       symbols
  custom:wombo          =       +custom(wombo)

! include %S/evdev
"#;

// Content of ~/.config/xkb/rules/evdev.xml when exporting configuration
const RULES_EVDEV_XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE xkbConfigRegistry SYSTEM "xkb.dtd">
<xkbConfigRegistry version="1.1">
  <optionList>
    <group allowMultipleSelection="true">
      <configItem>
        <name>custom</name>
        <description>Custom options</description>
      </configItem>
      <option>
      <configItem>
        <name>custom:wombo</name>
        <description>Wombo Combo extension</description>
      </configItem>
      </option>
    </group>
  </optionList>
</xkbConfigRegistry>
"#;

impl Model {
    pub fn new() -> Self {
        let special_keys = Vec::new();
        Self { special_keys }
    }

    pub fn add_special_key(&mut self, keycode: &str, symbols: &str) {
        self.special_keys.push(SpecialKey {
            keycode: keycode.to_string(),
            symbols: symbols.split(' ').map(|x| x.to_string()).collect(),
        })
    }

    pub fn export(&self) -> Result<()> {
        let mut path = dirs::config_dir().context("Finding configuration folder")?;
        path.push("xkb");
        dbg!(&path);

        let rules = path.join("rules");
        create_dir_all(&rules).context("Creating rules folders")?;
        File::create(rules.join("evdev"))
            .context("Creating rules/evdev")?
            .write_all(RULES_EVDEV.as_bytes())?;
        File::create(rules.join("evdev.xml"))
            .context("Creating rules/evdev.xml")?
            .write_all(RULES_EVDEV_XML.as_bytes())?;

        let symbols = path.join("symbols");
        create_dir_all(&symbols).context("Creating symbols folders")?;
        let mut custom_symbols =
            File::create(symbols.join("custom")).context("Creating symbols/custom")?;
        writeln!(custom_symbols, r#"partial alphanumeric_keys"#)?;
        writeln!(custom_symbols, r#"xkb_symbols "wombo" {{"#)?;
        for key in &self.special_keys {
            writeln!(
                custom_symbols,
                r#"    key {} {{ type="LOCAL_EIGHT_LEVEL", symbols[Group1]= [ {} ] }};"#,
                key.keycode,
                key.symbols.join(", ")
            )?;
        }
        writeln!(custom_symbols, r#"}};"#)?;

        Ok(())
    }
}
