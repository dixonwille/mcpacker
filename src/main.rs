mod manifest;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct App {
    #[structopt(subcommand)]
    cmd: SubCommand
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Initialize a new mcpacker project. Will use minecraftinstance.json if it exists.
    /// If not it doesn't exists, it will prompt for some information to be able to start a new modpack.
    Init{
        /// Name of the mod pack.
        #[structopt(short="n", long="name")]
        name: Option<String>,
        /// Author of the mod pack.
        #[structopt(short="a", long="author")]
        author: Option<String>,
        /// Initial version to use for the mod pack.
        #[structopt(short="v")]
        version: Option<String>,
        /// Minecraft version to use for the mod pack.
        #[structopt(short="m", long="mc_version")]
        mc_verison: Option<String>,
        /// Which mod loader to use.
        #[structopt(short="l", long="loader", default_value="forge")]
        loader: String,
        /// Which version of the mod loader to use [default: latest one found].
        #[structopt(long="loader_version")]
        loader_version: Option<String>,
    },
    /// Syncronize the manifest with the minecraftinstance.json.
    /// Downloads mods that are missing and adds jars to override if not in project list.
    /// This can be assumed as twitch app will remove jar files if mod is uninstalled.
    Sync,
    /// Create the modpack as a zip file.
    Pack{
        /// Modify the version before packing.
        #[structopt(short="v")]
        version: Option<String>,
    },
    /// Modify the override section of the manifest.
    Override(Override),
}

#[derive(StructOpt, Debug)]
enum Override {
    /// Add an override to the manifest.
    Add{
        /// Relative path to file to add to overrides.
        #[structopt(parse(from_os_str))]
        path: PathBuf
    },
    /// Remove an existing override from the manifest.
    #[structopt(name="rm")]
    Remove{
        /// Relative path to remove from the overrides.
        #[structopt(parse(from_os_str))]
        path: PathBuf
    },
}

fn main() -> std::io::Result<()> {
    let app = App::from_args();
    dbg!(app);
    Ok(())
}
