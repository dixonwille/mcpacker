use crate::files::{
    manifest::{create_manifest_file, Manifest, MANIFEST_FILE},
    minecraft_instance::{get_minecraft_instance, MINECRAFT_INSTANCE_FILE},
};
use anyhow::{anyhow, Result};
use semver::Version;
use std::io::{stdin, stdout, Write};
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
    version: Option<Version>,
    /// Minecraft version to use for the mod pack.
    #[structopt(short = "m", long = "mc_version")]
    mc_verison: Option<Version>,
    /// Which mod loader to use.
    #[structopt(short = "l", long = "loader", default_value = "forge")]
    loader: String,
    /// Which version of the mod loader to use.
    #[structopt(long = "loader_version")]
    loader_version: Option<Version>,
}

impl InitParams {
    fn prompt_for_manifest(&self) -> Result<Manifest> {
        let mut man = Manifest::default();
        man.mod_loader = self.loader.clone();
        match &self.name {
            Some(s) => man.name = s.clone(),
            None => man.name = prompt_for_string("Pack Name"),
        };
        match &self.author {
            Some(s) => man.author = s.clone(),
            None => man.author = prompt_for_string("Pack Author"),
        };
        match &self.version {
            Some(s) => man.version = s.clone(),
            None => man.version = prompt_for_version("Pack Version", 3)?,
        };
        match &self.mc_verison {
            Some(s) => man.minecraft_version = s.clone(),
            None => man.minecraft_version = prompt_for_version("Minecraft Version", 3)?,
        };
        match &self.loader_version {
            Some(s) => man.mod_loader_version = s.clone(),
            None => man.mod_loader_version = prompt_for_version("Mod Loader Version", 3)?,
        };
        Ok(man)
    }
}

impl InitParams {
    pub fn run(&self) -> Result<()> {
        if MANIFEST_FILE.exists() {
            return Err(anyhow!(
                "{} already exists",
                MANIFEST_FILE.to_string_lossy()
            ));
        }
        let manifest = if MINECRAFT_INSTANCE_FILE.exists() {
            (&get_minecraft_instance()?).into()
        } else {
            self.prompt_for_manifest()?
        };
        manifest.to_writer(create_manifest_file()?)?;
        Ok(())
    }
}

fn prompt_for_string(prompt: &str) -> String {
    let mut s = String::new();
    write!(stdout(), "{}: ", prompt).expect("could not write to prompt");
    stdout().flush().expect("could not flush to prompt");
    let _ = stdin().read_line(&mut s).expect("could not read line");
    if let Some('\n') = s.chars().next_back() {
        let _ = s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        let _ = s.pop();
    }
    s
}

fn prompt_for_version(prompt: &str, retries: u8) -> Result<Version> {
    let mut cur: u8 = 0;
    let mut last_err = None;
    while cur < retries {
        if cur != 0 {
            eprintln!(" please retry");
        }
        match Version::parse(&prompt_for_string(prompt)) {
            Ok(ver) => return Ok(ver),
            Err(e) => {
                eprint!("not a valid version");
                last_err = Some(e);
            }
        };
        cur = cur + 1;
    }
    match last_err {
        Some(e) => {
            eprintln!(" too many attempts");
            Err(anyhow::Error::new(e).context("not a valid version"))
        }
        None => Ok(Version::new(0, 0, 0)),
    }
}
