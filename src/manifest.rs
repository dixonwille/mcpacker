use crate::errors::*;
use crate::manifest_json::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub minecraft_version: String,
    pub mod_loader: String,
    pub mod_loader_version: String,
    includes: Option<BTreeSet<PathBuf>>, // Can include a jar file not in mod list
    mods: Option<BTreeSet<Mod>>,
}

impl Manifest {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        serde_yaml::from_reader(reader).map_err(|e| e.into())
    }

    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        serde_yaml::to_writer(writer, &self).map_err(|e| e.into())
    }

    pub fn get_mods(&self) -> Option<&BTreeSet<Mod>> {
        match self.mods.as_ref() {
            None => None,
            Some(ref mods) => Some(mods),
        }
    }

    pub fn add_mod(&mut self, module: Mod) -> bool {
        match &mut self.mods {
            Some(i) => i.insert(module),
            None => {
                let mut modules: BTreeSet<Mod> = BTreeSet::new();
                let _ = modules.insert(module);
                self.mods = Some(modules);
                true
            }
        }
    }

    pub fn get_includes(&self) -> Option<&BTreeSet<PathBuf>> {
        match self.includes.as_ref() {
            None => None,
            Some(ref includes) => Some(includes),
        }
    }

    pub fn add_include(&mut self, include: PathBuf) -> bool {
        match &mut self.includes {
            Some(i) => i.insert(include),
            None => {
                let mut includes: BTreeSet<PathBuf> = BTreeSet::new();
                let _ = includes.insert(include);
                self.includes = Some(includes);
                true
            }
        }
    }

    pub fn include_exists(&self, include: PathBuf) -> bool {
        match self.includes.as_ref() {
            None => false,
            Some(i) => i.contains(&include),
        }
    }

    pub fn include_contained(&self, include: &PathBuf) -> Option<PathBuf> {
        let mut ppath = include.clone();
        while let Some(p) = ppath.parent() {
            if self.include_exists(p.to_path_buf()) {
                return Some(p.to_path_buf());
            }
            ppath = p.to_path_buf();
        };
        return None;
    }

    pub fn includes_clean(&mut self) {
        if let Some(includes) = &self.includes {
            let mut remove: Vec<PathBuf> = Vec::new();
            for include in includes {
                if let Some(_) = self.include_contained(&include) {
                    remove.push(include.clone());
                }
            }
            for r in remove {
                let _ = self.remove_include(&r);
            }
        }
    }

    pub fn remove_include(&mut self, include: &PathBuf) -> bool {
        match &mut self.includes {
            Some(i) => {
                if i.remove(include) {
                    if i.is_empty() {
                        self.includes = None;
                    }
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

impl From<&ManifestJson> for Manifest {
    fn from(mj: &ManifestJson) -> Self {
        let mod_loader_version: String;
        let mod_loader: String;
        match mj.minecraft.get_mod_loader() {
            Some((loader, version)) => {
                mod_loader = loader;
                mod_loader_version = version;
            }
            None => {
                mod_loader = String::new();
                mod_loader_version = String::new();
            }
        };
        let mut m = Manifest {
            name: mj.name.clone(),
            version: mj.version.clone(),
            author: mj.author.clone(),
            minecraft_version: mj.minecraft.version.clone(),
            mod_loader_version,
            mod_loader,
            includes: None,
            mods: None,
        };
        if let Some(files) = mj.get_files() {
            for file in files {
                let _ = m.add_mod(file.into());
            }
        }
        m
    }
}

impl From<&MinecraftInstance> for Manifest {
    fn from(mi: &MinecraftInstance) -> Self {
        (&mi.manifest).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    // Used for verifying the file downloaded
    file_name: String,
    fingerprint: u64,
    file_size: u64,
}

impl From<&FileJson> for Mod {
    fn from(fj: &FileJson) -> Self {
        Mod {
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
