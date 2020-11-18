use crate::errors::*;
use crate::files::manifest_json::ManifestJson;
use semver::Version;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftInstance {
    pub name: String,
    pub custom_author: String,
    pub game_version: Version,
    pub base_mod_loader: BaseModLoader,
    pub manifest: Option<ManifestJson>,
    pub installed_addons: Option<Vec<InstalledAddon>>,
}

impl MinecraftInstance {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        serde_json::from_reader(reader).map_err(|e| e.into())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstalledAddon {
    #[serde(rename = "addonID")]
    pub addon_id: u32,
    pub installed_file: InstalledFile,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstalledFile {
    pub id: u32,
    pub file_name: String,
    pub file_length: u64,
    pub package_fingerprint: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseModLoader {
    pub name: String,
}

impl BaseModLoader {
    pub fn get_mod_loader(&self) -> Option<(&str, Version)> {
        match self.name.rfind('-') {
            None => None,
            Some(idx) => {
                let (loader, version) = self.name.split_at(idx);
                let version = version.trim_start_matches('-');
                let ver = Version::parse(version);
                let ver = match ver {
                    Ok(v) => v,
                    Err(_) => return None,
                };
                Some((loader, ver))
            }
        }
    }
}
