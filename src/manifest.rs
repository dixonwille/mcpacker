use std::path::PathBuf;
use serde::{Serialize, Deserialize};

const MANIFEST_VERSION: u8 = 1;
const MANIFEST_TYPE: &str = "minecraftModpack";
const MANIFEST_OVERRIDES_FOLDER: &str = "overrides";

#[derive(Serialize, Deserialize, Debug)]
struct ManifestJson{
    minecraft: MinecraftJson,
    manifest_type: String,
    manifest_version: u8,
    name: String,
    version: String,
    author: String, 
    overrides: String,
    files: Option<Vec<FileJson>>,
}

impl ManifestJson {
    pub fn from_file(path: &PathBuf) -> Self{
        ManifestJson{
            manifest_type: MANIFEST_TYPE.to_string(),
            manifest_version: MANIFEST_VERSION,
            files: None,
            overrides: MANIFEST_OVERRIDES_FOLDER.to_string(),
            author: String::from("Test"),
            name: String::from("Test"),
            version: String::from("v0.1.0"),
            minecraft: MinecraftJson{
                version: String::from("1.15.2"),
                mod_loaders: vec!{
                    ModLoaderJson{
                        id: String::from("forge-31.1.43"),
                        primary: true,
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MinecraftJson{
    version: String,
    mod_loaders: Vec<ModLoaderJson>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModLoaderJson {
    id: String,
    primary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileJson {
    project_id: u32,
    file_id: u32,
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    name: String,
    version: String,
    author: String,
    minecraft_version: String,
    mod_loader: String,
    mod_loader_version: String,
    overrides: Vec<String>, // Can include a jar file not in mod list
    mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mod {
    project_id: u32,
    file_id: u32,
    file_name: String,
    // Used for verifying the file downloaded
    fingerprint: u64,
    file_size: u64,
}