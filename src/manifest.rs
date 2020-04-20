use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::io::{Read, Write};
use crate::options_sorted::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub minecraft_version: String,
    pub mod_loader: String,
    pub mod_loader_version: String,
    includes: Option<Vec<String>>, // Can include a jar file not in mod list
    mods: Option<Vec<Mod>>,
}

impl Manifest {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        version: String,
        author: String,
        minecraft_version: String,
        loader: String,
        loader_version: String,
    ) -> Self {
        Manifest {
            name,
            version,
            author,
            minecraft_version,
            mod_loader: loader,
            mod_loader_version: loader_version,
            includes: None,
            mods: None,
        }
    }
    
    #[allow(dead_code)]
    pub fn from_reader<R: Read>(reader: R) -> serde_yaml::Result<Self> {
        serde_yaml::from_reader(reader)
    }

    #[allow(dead_code)]
    pub fn to_writer<W: Write>(&self, writer: W) -> serde_yaml::Result<()> {
        serde_yaml::to_writer(writer, &self)
    }

    pub fn get_mods(&self) -> Option<&Vec<Mod>> {
        match self.mods {
            None => None,
            Some(ref mods) => {
                Some(mods)
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_mod(&mut self, m: Mod) {
        self.mods.add(m)
    }

    #[allow(dead_code)]
    pub fn add_mods(&mut self, mods: &mut Vec<Mod>) {
        self.mods.add_multiple(mods)
    }

    #[allow(dead_code)]
    pub fn remove_mod(&mut self, m: Mod) -> Option<Mod> {
        self.mods.remove_element(m)
    }

    #[allow(dead_code)]
    pub fn add_include(&mut self, include: String) {
        self.includes.add(include)
    }

    #[allow(dead_code)]
    pub fn add_includes(&mut self, includes: &mut Vec<String>) {
        self.includes.add_multiple(includes)
    }

    #[allow(dead_code)]
    pub fn remove_include(&mut self, i: String) -> Option<String> {
        self.includes.remove_element(i)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
    pub project_id: u32,
    pub file_id: u32,
    file_name: String,
    // Used for verifying the file downloaded
    fingerprint: u64,
    file_size: u64,
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.project_id == other.project_id && self.file_id == other.file_id
    }
}

impl Eq for Mod {}

impl Ord for Mod {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.project_id.cmp(&other.project_id) {
            Ordering::Equal => self.file_id.cmp(&other.file_id),
            o => o,
        }
    }
}

impl PartialOrd for Mod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
