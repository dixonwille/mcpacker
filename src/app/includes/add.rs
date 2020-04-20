use crate::app::*;
use crate::errors::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Add {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

impl Run for Add {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
