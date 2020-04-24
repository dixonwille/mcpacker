use crate::app::*;
use crate::errors::Result;
use crate::manifest::*;
use std::io::{Error, ErrorKind, stdin, stdout, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct InitParams {
    /// Name of the mod pack.
    #[structopt(short = "n", long = "name")]
    name: Option<String>,
    /// Author of the mod pack.
    #[structopt(short = "a", long = "author")]
    author: Option<String>,
    /// Initial version to use for the mod pack.
    #[structopt(short = "v")]
    version: Option<String>,
    /// Minecraft version to use for the mod pack.
    #[structopt(short = "m", long = "mc_version")]
    mc_verison: Option<String>,
    /// Which mod loader to use.
    #[structopt(short = "l", long = "loader", default_value = "forge")]
    loader: String,
    /// Which version of the mod loader to use [default: latest one found].
    #[structopt(long = "loader_version")]
    loader_version: Option<String>,
}

impl InitParams {
    fn prompt_for_manifest(&self) -> Result<Manifest> {
        let mut man = Manifest::default();
        man.mod_loader = self.loader.clone();
        match &self.name {
            Some(s) => man.name = s.clone(),
            None => man.name = prompt_for_string("Pack Name")?,
        };
        match &self.author {
            Some(s) => man.author = s.clone(),
            None => man.author = prompt_for_string("Pack Author")?,
        };
        match &self.version {
            Some(s) => man.version = s.clone(),
            None => man.version = prompt_for_string("Pack Version")?,
        };
        match &self.mc_verison {
            Some(s) => man.minecraft_version = s.clone(),
            None => man.minecraft_version = prompt_for_string("Minecraft Version")?,
        };
        match &self.loader_version {
            Some(s) => man.mod_loader_version = s.clone(),
            None => man.mod_loader_version = prompt_for_string("Mod Loader Version")?,
        };
        Ok(man)
    }
}

impl Run for InitParams {
    fn run(&self) -> Result<()> {
        if manifest_exists() {
            // TODO Create my own error
            return Err(Error::new(ErrorKind::AlreadyExists, format!("{} already exists", MANIFEST_FILE)).into())
        }
        let manifest: Manifest;
        if minecraft_instance_exists() {
            manifest = (&get_minecraft_instance()?).into();
        }else {
            manifest = self.prompt_for_manifest()?;
        }
        // TODO Validate and clean the manifest file (Make API calls to twitch app api to add more infor to the mods)
        manifest.to_writer(create_manifest_file()?)?;
        Ok(())
    }
}

fn prompt_for_string(prompt: &str) -> Result<String> {
    let mut s = String::new();
    write!(stdout(), "{}: ", prompt)?;
    stdout().flush()?;
    let _ = stdin().read_line(&mut s)?;
    if let Some('\n')=s.chars().next_back() {
        let _ = s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        let _ = s.pop();
    }
    Ok(s)
}
