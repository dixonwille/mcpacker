use crate::files::manifest::{Manifest, Mod};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::BTreeSet, io::Write, path::PathBuf};

pub static MANIFEST_JSON_FILE: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("manifest.json"));

const MANIFEST_VERSION: u8 = 1;
const MANIFEST_TYPE: &str = "minecraftModpack";
pub const MANIFEST_OVERRIDES_FOLDER: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("overrides"));

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManifestJson {
    pub minecraft: MinecraftJson,
    pub manifest_type: String,
    pub manifest_version: u8,
    pub name: String,
    pub version: Version,
    pub author: String,
    overrides: String,
    files: Option<BTreeSet<FileJson>>,
}

impl ManifestJson {
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        serde_json::to_writer(writer, &self)
            .with_context(|| "could not serialize from MinecrafJson")
    }

    pub fn add_file(&mut self, file: FileJson) -> bool {
        match &mut self.files {
            Some(i) => i.insert(file),
            None => {
                let mut files: BTreeSet<FileJson> = BTreeSet::new();
                let _ = files.insert(file);
                self.files = Some(files);
                true
            }
        }
    }
}

impl From<&Manifest> for ManifestJson {
    fn from(m: &Manifest) -> Self {
        let mut mj = ManifestJson {
            manifest_type: MANIFEST_TYPE.to_string(),
            manifest_version: MANIFEST_VERSION,
            overrides: MANIFEST_OVERRIDES_FOLDER.to_string_lossy().into(),
            files: None,
            name: m.name.clone(),
            version: m.version.clone(),
            author: m.author.clone(),
            minecraft: MinecraftJson {
                version: m.minecraft_version.clone(),
                mod_loaders: Vec::new(),
            },
        };
        mj.minecraft
            .set_mod_loader(&m.mod_loader, &m.mod_loader_version);
        if let Some(mods) = m.get_mods() {
            for module in mods {
                let _ = mj.add_file(module.into());
            }
        }
        mj
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftJson {
    pub version: Version,
    mod_loaders: Vec<ModLoaderJson>,
}

impl MinecraftJson {
    pub fn set_mod_loader(&mut self, name: impl AsRef<str>, version: &Version) {
        self.mod_loaders = vec![ModLoaderJson {
            id: format!("{}-{}", name.as_ref(), version),
            primary: true,
        }]
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ModLoaderJson {
    id: String,
    primary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileJson {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    required: bool,
}

impl From<&Mod> for FileJson {
    fn from(m: &Mod) -> Self {
        FileJson {
            project_id: m.project_id,
            file_id: m.file_id,
            required: true,
        }
    }
}

impl PartialEq for FileJson {
    fn eq(&self, other: &Self) -> bool {
        self.project_id == other.project_id && self.file_id == other.file_id
    }
}

impl Eq for FileJson {}

impl Ord for FileJson {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.project_id.cmp(&other.project_id) {
            Ordering::Equal => self.file_id.cmp(&other.file_id),
            o => o,
        }
    }
}

impl PartialOrd for FileJson {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
