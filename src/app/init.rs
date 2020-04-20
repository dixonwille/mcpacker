use crate::app::*;
use crate::errors::Result;
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

impl Run for InitParams {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
