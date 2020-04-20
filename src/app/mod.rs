mod includes;
mod init;
mod pack;
mod sync;

use includes::*;
use init::*;
use pack::*;
use sync::*;

use crate::errors::Result;
use structopt::StructOpt;

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
    /// Initialize a new mcpacker project. Will use minecraftinstance.json if it exists.
    /// If not it doesn't exists, it will prompt for some information to be able to start a new modpack.
    Init(InitParams),
    /// Syncronize the manifest with the minecraftinstance.json.
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
