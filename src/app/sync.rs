use crate::app::*;
use crate::errors::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct SyncParams {}

impl Run for SyncParams {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
