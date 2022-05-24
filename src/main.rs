mod config;
mod preview;

use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    // Make sure we don't mess with our system while developing
    std::env::set_var("XDG_CONFIG_HOME", "/tmp/test_xkb");
    std::env::set_var("XKB_DEFAULT_OPTIONS", "custom:wombo,caps:ctrl_modifier");

    // Create an example configuration and save it
    let mut model = config::Model::new();
    model.add_special_key(
        "<AC10>".to_string(),
        vec![
            "semicolon".to_string(),
            "colon".to_string(),
            "b".to_string(),
            "b".to_string(),
            "b".to_string(),
            "b".to_string(),
        ],
    );
    model.add_special_key(
        "<AE05>".to_string(),
        vec![
            "5".to_string(),
            "percent".to_string(),
            "0x20ac".to_string(),
            "0x20ac".to_string(),
            "0x20ac".to_string(),
            "0x20ac".to_string(),
        ],
    );
    model.export().unwrap();

    // Run preview
    let real_device = pick_device()?;
    preview::run_preview(&real_device)?;
    Ok(())
}

fn pick_device() -> Result<PathBuf> {
    let devices = list_devices()?;

    println!("Pick a device to run simulation on:");
    for (n, device) in devices.iter().enumerate() {
        println!("{}: {:?}", n, device.path())
    }
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .context("reading user choice")?;
    let choice: usize = buffer.trim().parse().context("input is not numeric")?;
    if choice >= devices.len() {
        bail!("invalid choice");
    }
    Ok(devices[choice].path())
}

const DEVICES: &'static str = "/dev/input/by-id/";

fn list_devices() -> Result<Vec<fs::DirEntry>> {
    fs::read_dir(DEVICES)
        .with_context(|| format!("listing devices at {DEVICES}"))?
        .collect::<Result<_, _>>()
        .context("accessing device")
}
