mod includes;
mod init;
mod pack;
mod sync;

use crate::errors::Result;
use crate::manifest::*;
use crate::manifest_json::*;
use includes::*;
use init::*;
use pack::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use sync::*;

const MINECRAFT_INSTANCE_FILE: &str = "minecraftinstance.json";
const MANIFEST_FILE: &str = ".manifest.yaml";
const MANIFEST_JSON_FILE: &str = "manifest.json";
const MODS_DIR: &str = "mods";

fn minecraft_instance_exists() -> bool {
    std::path::Path::new(MINECRAFT_INSTANCE_FILE).exists()
}

fn manifest_exists() -> bool {
    std::path::Path::new(MANIFEST_FILE).exists()
}

fn get_manifest() -> Result<Manifest> {
    Manifest::from_reader(BufReader::new(File::open(MANIFEST_FILE)?))
}

fn create_manifest_file() -> Result<BufWriter<File>> {
    Ok(BufWriter::new(File::create(MANIFEST_FILE)?))
}

fn get_minecraft_instance() -> Result<MinecraftInstance> {
    MinecraftInstance::from_reader(BufReader::new(File::open(MINECRAFT_INSTANCE_FILE)?))
}

fn clean_path(p: &PathBuf) -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let abs = p.canonicalize()?;
    Ok((abs.strip_prefix(cwd.as_path())?).to_path_buf())
}

fn jar_name(p: &PathBuf) -> Option<(PathBuf, bool)> {
    match p.extension() {
        Some(ext) if ext == "jar" => Some((p.clone(), false)),
        Some(ext) if ext == "disabled" => {
            let parent = p.parent();
            let file_stem = p.file_stem();
            match (parent, file_stem) {
                (Some(par), Some(stem)) => {
                    let new = Path::new(par).join(stem);
                    match new.extension() {
                        Some(new_ext) if new_ext == "jar" => Some((new, true)),
                        _ => None,
                    }
                }
                (_, _) => None,
            }
        }
        _ => None,
    }
}

pub trait Run {
    fn run(&self) -> crate::errors::Result<()>;
}

#[derive(StructOpt, Debug)]
pub struct App {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

impl App {
    pub fn run() -> Result<()> {
        let app = App::from_args();
        app.cmd.run()
    }
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Initialize a new mcpacker project.
    ///
    /// Will use minecraftinstance.json if it exists.
    /// If it doesn't exists, it will prompt for some information to be able to start a new modpack.
    Init(InitParams),
    /// Syncronize the manifest with the minecraftinstance.json.
    ///
    /// Downloads mods that are missing and adds jars to override if not in project list.
    /// This can be assumed as twitch app will remove jar files if mod is uninstalled.
    Sync(SyncParams),
    /// Create the modpack as a zip file.
    Pack(PackParams),
    /// Modify the includes section of the manifest.
    Include(Include),
}

impl Run for SubCommand {
    fn run(&self) -> Result<()> {
        match &self {
            SubCommand::Init(p) => p.run(),
            SubCommand::Sync(p) => p.run(),
            SubCommand::Pack(p) => p.run(),
            SubCommand::Include(p) => p.run(),
        }
    }
}
