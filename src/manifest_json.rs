use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::io::{Read, Write};

const MANIFEST_VERSION: u8 = 1;
const MANIFEST_TYPE: &str = "minecraftModpack";
const MANIFEST_OVERRIDES_FOLDER: &str = "overrides";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftInstance {
    manifest: ManifestJson,
}

impl MinecraftInstance {
    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        serde_json::from_reader(reader)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
    pub fn to_writer<W: Write>(&self, writer: W) -> serde_json::Result<()> {
        serde_json::to_writer(writer, &self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MinecraftJson {
    version: String,
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
