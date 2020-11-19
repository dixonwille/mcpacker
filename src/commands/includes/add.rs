use crate::{
    commands::includes::relative_path,
    files::manifest::{create_manifest_file, get_manifest},
};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Add {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,
}

impl Add {
    pub fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        for path in self.paths.iter() {
            if !path.exists() {
                // TODO Create my own error
                return Err(anyhow!(format!("{} does not exit", path.to_string_lossy())));
            }
            let cpath = relative_path(path)?;
            // Validate that the path is not already included by another
            if let Some(p) = manifest.include_contained(&cpath) {
                println!(
                    "{} already included by {}",
                    cpath.to_string_lossy(),
                    p.to_string_lossy()
                );
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
