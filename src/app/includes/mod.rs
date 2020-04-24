mod add;
mod remove;

use add::*;
use remove::*;

use crate::app::*;
use crate::errors::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Include {
    /// Add an include to the manifest.
    Add(Add),
    /// Remove an existing include from the manifest.
    #[structopt(alias = "rm")]
    Remove(Remove),
}

impl Run for Include {
    fn run(&self) -> Result<()> {
        match &self {
            Include::Add(p) => p.run(),
            Include::Remove(p) => p.run(),
        }
    }
}
