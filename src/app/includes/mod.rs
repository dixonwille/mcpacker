mod add;
mod remove;

use add::*;
use remove::*;

use crate::errors::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Include {
    /// Add an include to the manifest.
    Add(Add),
    /// Remove an existing include from the manifest.
    #[structopt(visible_alias = "rm")]
    Remove(Remove),
}

impl Include {
    pub fn run(&self) -> Result<()> {
        match &self {
            Include::Add(p) => p.run(),
            Include::Remove(p) => p.run(),
        }
    }
}
