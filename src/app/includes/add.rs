use crate::app::*;
use crate::errors::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use std::io::{Error, ErrorKind};

#[derive(StructOpt, Debug)]
pub struct Add {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    path: PathBuf, //TODO make a vector for list of paths
}

impl Run for Add {
    fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        if !self.path.exists() {
            // TODO Create my own error
            return Err(Error::new(ErrorKind::NotFound, format!("{} does not exist", self.path.to_string_lossy())).into())
        }
        // TODO make sure it is relative and clean the path
        manifest.add_include(self.path.to_string_lossy().into());
        manifest.to_writer(create_manifest_file()?)
    }
}
