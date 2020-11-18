#![deny(missing_docs, missing_debug_implementations, unused_results)]

//! MCPacker helps maintain Minecarft mod packs.

mod commands;
mod files;
mod utils;

use anyhow::Result;
use commands::{
    author::AuthorParams, bump::BumpParams, includes::Include, init::InitParams, pack::PackParams,
    sync::SyncParams,
};
use structopt::StructOpt;

fn main() -> Result<()> {
    App::run()
}

#[derive(StructOpt, Debug)]
struct App {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

impl App {
    fn run() -> Result<()> {
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
    /// Bump the version of the mod pack.
    Bump(BumpParams),
    /// Set the author of the mod pack.
    ///
    /// This only updates the manifest that only takes effect after a pack.
    /// Will need to import the new pack to see author changes.
    Author(AuthorParams),
}

impl SubCommand {
    fn run(&self) -> Result<()> {
        match &self {
            SubCommand::Init(p) => p.run(),
            SubCommand::Sync(p) => p.run(),
            SubCommand::Pack(p) => p.run(),
            SubCommand::Include(p) => p.run(),
            SubCommand::Bump(p) => p.run(),
            SubCommand::Author(p) => p.run(),
        }
    }
}
