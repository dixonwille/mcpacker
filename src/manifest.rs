use crate::options_sorted::*;
use crate::errors::*;
use crate::manifest_json::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub minecraft_version: String,
    pub mod_loader: String,
    pub mod_loader_version: String,
    includes: Option<Vec<String>>, // Can include a jar file not in mod list
    pub mods: Option<Vec<Mod>>,
}

impl Manifest {
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

    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let s = serde_yaml::from_reader(reader)?;
        Ok(s)
    }

    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        serde_yaml::to_writer(writer, &self)?;
        Ok(())
    }

    pub fn get_mods(&self) -> Option<&Vec<Mod>> {
        match &self.mods {
            None => None,
            Some(ref mods) => Some(mods),
        }
    }

    pub fn add_mod(&mut self, m: Mod) {
        self.mods.add(m)
    }

    pub fn add_mods(&mut self, mods: &mut Vec<Mod>) {
        self.mods.add_multiple(mods)
    }

    pub fn remove_mod(&mut self, m: Mod) -> Option<Mod> {
        self.mods.remove_element(m)
    }

    pub fn add_include(&mut self, include: String) {
        self.includes.add(include)
    }

    pub fn add_includes(&mut self, includes: &mut Vec<String>) {
        self.includes.add_multiple(includes)
    }

    pub fn remove_include(&mut self, i: String) -> Option<String> {
        self.includes.remove_element(i)
    }
}

impl From<&ManifestJson> for Manifest {
    fn from(mj: &ManifestJson) -> Self {
        let mut m= Manifest{
            name: mj.name.clone(),
            version: mj.version.clone(),
            author: mj.author.clone(),
            minecraft_version: mj.minecraft.version.clone(),
            mod_loader_version: String::new(),
            mod_loader: String::new(),
            includes: None,
            mods: None,
        };
        if let Some(files) = mj.get_files(){
            let mut modules: Vec<Mod> = Vec::with_capacity(files.len());
            for file in files{
                modules.push(file.into());
            }
            m.mods.add_multiple(&mut modules)
        }
        m
    }
}

impl From<&MinecraftInstance> for Manifest{
    fn from(mi: &MinecraftInstance) -> Self {
        (&mi.manifest).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
    pub project_id: u32,
    pub file_id: u32,
    // Used for verifying the file downloaded
    file_name: String,
    fingerprint: u64,
    file_size: u64,
}

impl From<&FileJson> for Mod {
    fn from(fj: &FileJson) -> Self{
        Mod{
            project_id: fj.project_id,
            file_id: fj.file_id,
            file_name: String::new(),
            fingerprint: 0,
            file_size: 0,
        }
    }
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
