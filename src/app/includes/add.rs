use crate::app::*;
use crate::errors::Result;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Add {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,
}

impl Run for Add {
    fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        for path in self.paths.iter() {
            if !path.exists() {
                // TODO Create my own error
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("{} does not exist", path.to_string_lossy()),
                )
                .into());
            }
            let cpath = clean_path(path)?;
            // Validate that the path is not already included by another
            if let Some(p) = manifest.include_contained(&cpath){
                println!("{} already included by {}", cpath.to_string_lossy(), p.to_string_lossy());
                return Ok(());
            }
            // Try and add it
            if !manifest.add_include(cpath) {
                println!("{} is already in the include list", path.to_string_lossy());
                return Ok(());
            }
            // Cleanup other paths that may be included with this new path
            manifest.includes_clean();
        }
        manifest.to_writer(create_manifest_file()?)
    }
}
