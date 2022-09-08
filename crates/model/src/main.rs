use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use model::{config, discover::list_devices, preview::Preview};

fn main() -> Result<()> {
    // Make sure we don't mess with our system while developing
    std::env::set_var("XDG_CONFIG_HOME", "/tmp/test_xkb");
    std::env::set_var("XKB_DEFAULT_OPTIONS", "custom:wombo,caps:ctrl_modifier");

    // Create an example configuration and save it
    let mut model = config::Model::new();
    model.add_special_key("<AC10>", "semicolon colon b b b b");
    model.add_special_key("<AE05>", "5 percent 0x20ac 0x20ac 0x20ac 0x20ac");
    model.export().unwrap();

    // Run preview
    Preview::new()?.read_from_device(&pick_device()?)
}

fn pick_device() -> Result<PathBuf> {
    let devices = list_devices();

    println!("Pick a device to run simulation on:");
    for (n, (path, _device)) in devices.iter().enumerate() {
        println!("{}: {:?}", n, path)
    }
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .context("reading user choice")?;
    let choice: usize = buffer.trim().parse().context("input is not numeric")?;
    if choice >= devices.len() {
        bail!("invalid choice");
    }
    Ok(devices[choice].0.clone())
}
