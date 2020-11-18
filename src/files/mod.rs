pub mod manifest;
pub mod manifest_json;
pub mod minecraft_instance;

use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static MODS_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("mods"));
pub static CWD: Lazy<PathBuf> =
    Lazy::new(|| std::env::current_dir().unwrap().canonicalize().unwrap());
