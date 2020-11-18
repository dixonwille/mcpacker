mod add;
mod remove;

use crate::files::CWD;
use add::Add;
use anyhow::Result;
use remove::Remove;
use std::path::PathBuf;
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

fn relative_path(p: &PathBuf) -> Result<PathBuf> {
    let abs = p.canonicalize()?;
    Ok((abs.strip_prefix(CWD.as_path())?).to_path_buf())
}
