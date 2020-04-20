use crate::app::*;
use crate::errors::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Remove {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

impl Run for Remove {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
