mod includes;
mod init;
mod pack;
mod sync;

use includes::*;
use init::*;
use pack::*;
use sync::*;

use crate::errors::Result;
use crate::manifest::*;
use crate::manifest_json::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use structopt::StructOpt;

const MINECRAFT_INSTANCE_FILE: &str = "minecraftinstance.json";
const MANIFEST_FILE: &str = ".manifest.yaml";
const MANIFEST_JSON_FILE: &str = "manifest.json";

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
