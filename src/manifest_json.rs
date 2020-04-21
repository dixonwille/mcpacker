use crate::options_sorted::*;
use crate::errors::*;
use crate::manifest::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::io::{Read, Write};

const MANIFEST_VERSION: u8 = 1;
const MANIFEST_TYPE: &str = "minecraftModpack";
const MANIFEST_OVERRIDES_FOLDER: &str = "overrides";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftInstance {
    pub manifest: ManifestJson,
}

impl MinecraftInstance {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let s = serde_json::from_reader(reader)?;
        Ok(s)
    }
}

impl From<&Manifest> for MinecraftInstance{
    fn from(m: &Manifest) -> Self {
        MinecraftInstance{
            manifest: m.into()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManifestJson {
    pub minecraft: MinecraftJson,
    pub manifest_type: String,
    pub manifest_version: u8,
    pub name: String,
    pub version: String,
    pub author: String,
    overrides: String,
    files: Option<Vec<FileJson>>,
}

impl ManifestJson {
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    pub fn get_files(&self) -> Option<&Vec<FileJson>> {
        match &self.files{
            None=> None,
            Some(ref files) => Some(files)
        }
    }
}

impl From<&Manifest> for ManifestJson {
    fn from(m: &Manifest) -> Self {
        let mut mj = ManifestJson{
            manifest_type: MANIFEST_TYPE.to_string(),
            manifest_version: MANIFEST_VERSION,
            overrides: MANIFEST_OVERRIDES_FOLDER.to_string(),
            files: None,
            name: m.name.clone(),
            version: m.version.clone(),
            author: m.author.clone(),
            minecraft: MinecraftJson{
                version: m.minecraft_version.clone(),
                mod_loaders: vec![ModLoaderJson{
                    id: format!("{}-{}", m.mod_loader, m.mod_loader_version),
                    primary: true,
                }]
            }
        };
        if let Some(mods) = m.get_mods(){
            let mut files: Vec<FileJson> = Vec::with_capacity(mods.len());
            for module in mods{
                files.push(module.into());
            }
            mj.files.add_multiple(&mut files)
        }
        mj
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftJson {
    pub version: String,
    mod_loaders: Vec<ModLoaderJson>,
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
    pub project_id: u32,
    pub file_id: u32,
    required: bool,
}

impl From<&Mod> for FileJson {
    fn from(m: &Mod) -> Self{
        FileJson{
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
