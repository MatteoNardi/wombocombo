use anyhow::{Context, Result};
use std::fs;

pub const DEVICES_BY_PATH: &'static str = "/dev/input/by-path/";
pub const DEVICES_BY_ID: &'static str = "/dev/input/by-id/";

pub fn list_devices() -> Result<Vec<fs::DirEntry>> {
    let search = |path| fs::read_dir(path).with_context(|| format!("listing devices at {path}"));
    search(DEVICES_BY_ID)?
        .chain(search(DEVICES_BY_PATH)?)
        .collect::<Result<_, _>>()
        .context("accessing device")
}
