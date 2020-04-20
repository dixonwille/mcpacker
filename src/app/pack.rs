use crate::app::*;
use crate::errors::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct PackParams {
    /// Modify the version before packing.
    #[structopt(short = "v")]
    version: Option<String>,
}

impl Run for PackParams {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
