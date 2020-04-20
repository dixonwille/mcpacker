use serde::{Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::io::{Write};

use crate::manifest::Manifest;

#[allow(dead_code)]
const MANIFEST_VERSION: u8 = 1;
#[allow(dead_code)]
const MANIFEST_TYPE: &str = "minecraftModpack";
#[allow(dead_code)]
const MANIFEST_OVERRIDES_FOLDER: &str = "overrides";

#[derive(Serialize, Debug)]
pub struct ManifestJson {
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
    #[allow(dead_code)]
    pub fn from_manifest(man: &Manifest) -> Self {
        let mut jman = ManifestJson {
            name: man.name.clone(),
            version: man.version.clone(),
            author: man.author.clone(),
            overrides: MANIFEST_OVERRIDES_FOLDER.to_string(),
            manifest_type: MANIFEST_TYPE.to_string(),
            manifest_version: MANIFEST_VERSION,
            minecraft: MinecraftJson {
                version: man.minecraft_version.clone(),
                mod_loaders: vec![ModLoaderJson {
                    id: format!("{}-{}", man.mod_loader, man.mod_loader_version),
                    primary: true,
                }],
            },
            files: None,
        };
        if let Some(mods) = man.get_mods() {
            let mut files: Vec<FileJson> = Vec::with_capacity(mods.len());
            for m in mods {
                files.push(FileJson {
                    project_id: m.project_id,
                    file_id: m.file_id,
                    required: true,
                })
            }
            files.sort_unstable();
            jman.files = Some(files)
        };
        jman
    }
    
    #[allow(dead_code)]
    pub fn to_writer<W: Write>(&self, writer: W) -> serde_json::Result<()> {
        serde_json::to_writer(writer, &self)
    }
}

#[derive(Serialize, Debug)]
struct MinecraftJson {
    version: String,
    mod_loaders: Vec<ModLoaderJson>,
}

#[derive(Serialize, Debug)]
struct ModLoaderJson {
    id: String,
    primary: bool,
}

#[derive(Serialize, Debug)]
struct FileJson {
    project_id: u32,
    file_id: u32,
    required: bool,
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