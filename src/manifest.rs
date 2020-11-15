use crate::compare::*;
use crate::errors::*;
use crate::manifest_json::*;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
fn clean_path(p: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(p.as_ref().to_string_lossy().replace("\\", "/"))
}

#[cfg(not(target_os = "windows"))]
fn clean_path(p: impl AsRef<Path>) -> PathBuf {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub minecraft_version: Version,
    pub mod_loader: String,
    pub mod_loader_version: Version,
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
        self.mods.as_ref().map(|mods| mods)
    }

    pub fn add_mod(&mut self, module: Mod) -> bool {
        match &mut self.mods {
            Some(m) => m.insert(module),
            None => {
                let mut modules: BTreeSet<Mod> = BTreeSet::new();
                let _ = modules.insert(module);
                self.mods = Some(modules);
                true
            }
        }
    }

    pub fn remove_mod(&mut self, module: &Mod) -> bool {
        match &mut self.mods {
            Some(m) => {
                if m.remove(module) {
                    if m.is_empty() {
                        self.mods = None;
                    }
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn get_mod_by_filename(&self, path: impl AsRef<str>) -> Option<&Mod> {
        match &self.mods {
            None => None,
            Some(mods) => {
                for m in mods {
                    if m.file_name == path.as_ref() {
                        return Some(m);
                    }
                }
                None
            }
        }
    }

    fn set_mods(&mut self, modules: BTreeSet<Mod>) {
        self.mods = Some(modules);
    }

    fn clear_mods(&mut self) {
        self.mods = None;
    }

    pub fn get_includes(&self) -> Option<&BTreeSet<PathBuf>> {
        self.includes.as_ref().map(|includes| includes)
    }

    pub fn add_include(&mut self, mut include: PathBuf) -> bool {
        include = clean_path(&include);
        match &mut self.includes {
            Some(i) => i.insert(include),
            None => {
                let mut includes = BTreeSet::new();
                let _ = includes.insert(include);
                self.includes = Some(includes);
                true
            }
        }
    }

    pub fn include_exists(&self, include: impl AsRef<Path>) -> bool {
        let include = clean_path(include);
        match self.includes.as_ref() {
            None => false,
            Some(i) => i.contains(&include),
        }
    }

    pub fn include_contained(&self, include: impl AsRef<Path>) -> Option<PathBuf> {
        let mut ppath = clean_path(include);
        while let Some(p) = ppath.parent() {
            if self.include_exists(p) {
                return Some(p.to_path_buf());
            }
            ppath = p.to_owned();
        }
        None
    }

    pub fn includes_clean(&mut self) {
        if let Some(includes) = &self.includes {
            let mut remove = Vec::new();
            for include in includes {
                if self.include_contained(include).is_some() {
                    remove.push(include.clone());
                }
            }
            for r in remove {
                let _ = self.remove_include(&r);
            }
        }
    }

    pub fn remove_include(&mut self, include: impl AsRef<Path>) -> bool {
        let include = clean_path(include);
        match &mut self.includes {
            Some(i) => {
                if i.remove(&include) {
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

    pub fn sync_mods(&mut self, new: &Manifest) -> (Option<Vec<Mod>>, Option<Vec<Mod>>) {
        let mut rm = Vec::new();
        let mut add = Vec::new();
        match (self.get_mods(), new.get_mods()) {
            (Some(m), Some(n)) => {
                let comp = compare(m.iter(), n.iter());
                for c in comp {
                    match c {
                        Side::Left(m) => rm.push(m.clone()),
                        Side::Right(m) => add.push(m.clone()),
                    }
                }
            }
            (None, Some(n)) => {
                for new in n {
                    add.push(new.clone());
                }
                self.set_mods(n.clone());
            }
            (Some(m), None) => {
                for old in m {
                    rm.push(old.clone());
                }
                self.clear_mods();
            }
            (None, None) => {}
        }
        for m in &rm {
            let _ = self.remove_mod(&m);
        }
        for m in &add {
            let _ = self.add_mod(m.clone());
        }
        match (add.len(), rm.len()) {
            (0, 0) => (None, None),
            (_, 0) => (Some(add), None),
            (0, _) => (None, Some(rm)),
            (_, _) => (Some(add), Some(rm)),
        }
    }
}

impl From<&MinecraftInstance> for Manifest {
    fn from(mi: &MinecraftInstance) -> Self {
        let version = if mi.manifest.is_some() {
            mi.manifest.as_ref().unwrap().version.clone()
        } else {
            mi.game_version.clone()
        };
        let m = mi.base_mod_loader.get_mod_loader();
        let (mod_loader, mod_loader_version) = if let Some(loader) = m {
            (loader.0.to_string(), loader.1)
        } else {
            (String::new(), Version::new(0, 0, 0))
        };
        let mut m = Manifest {
            name: mi.name.clone(),
            version,
            author: mi.custom_author.clone(),
            minecraft_version: mi.game_version.clone(),
            mod_loader,
            mod_loader_version,
            includes: None,
            mods: None,
        };
        if let Some(addons) = mi.installed_addons.as_ref() {
            for addon in addons {
                let _ = m.add_mod(addon.into());
            }
        }
        m
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest {
            name: String::new(),
            version: Version::new(0, 0, 0),
            author: String::new(),
            minecraft_version: Version::new(0, 0, 0),
            mod_loader: String::new(),
            mod_loader_version: Version::new(0, 0, 0),
            includes: None,
            mods: None,
        }
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
    pub file_name: String,
    pub fingerprint: u32,
    pub file_size: u64,
}

impl From<&InstalledAddon> for Mod {
    fn from(ia: &InstalledAddon) -> Self {
        Mod {
            project_id: ia.addon_id,
            file_id: ia.installed_file.id,
            file_name: ia.installed_file.file_name.to_string(),
            file_size: ia.installed_file.file_length,
            fingerprint: ia.installed_file.package_fingerprint,
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
