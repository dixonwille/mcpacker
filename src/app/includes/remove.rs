use crate::app::*;
use crate::errors::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Remove {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    path: PathBuf, //TODO make vector for list of paths
}

impl Run for Remove {
    fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        // TODO make sure it is relative and clean the path
        let _ = manifest.remove_include(self.path.to_string_lossy().into());
        manifest.to_writer(create_manifest_file()?)
    }
}
