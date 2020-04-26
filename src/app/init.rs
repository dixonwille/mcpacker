use crate::app::*;
use crate::errors::Result;
use crate::manifest::*;
use crate::twitch_api::*;
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use structopt::StructOpt;
use tokio::runtime::Runtime;
use std::collections::BTreeSet;

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
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("{} already exists", MANIFEST_FILE),
            )
            .into());
        }
        let mut manifest = if minecraft_instance_exists() {
            (&get_minecraft_instance()?).into()
        } else {
            self.prompt_for_manifest()?
        };
        if let Some(modules) = manifest.get_mods() {
            let mut rt = Runtime::new()?;
            let new_modules = rt.block_on(get_twitch_mods(modules.clone()))?;
            manifest.set_mods(new_modules);
        }
        manifest.to_writer(create_manifest_file()?)?;
        Ok(())
    }
}

fn prompt_for_string(prompt: &str) -> Result<String> {
    let mut s = String::new();
    write!(stdout(), "{}: ", prompt)?;
    stdout().flush()?;
    let _ = stdin().read_line(&mut s)?;
    if let Some('\n') = s.chars().next_back() {
        let _ = s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        let _ = s.pop();
    }
    Ok(s)
}

async fn get_twitch_mods(modules: BTreeSet<Mod>) -> Result<BTreeSet<Mod>> {
    let twitch = TwitchAPI::new();
    let mut mods = BTreeSet::new();
    for module in modules {
        let m = twitch.get_mod(module.project_id, module.file_id).await?;
        let _ = mods.insert(m);
    }
    Ok(mods)
}
