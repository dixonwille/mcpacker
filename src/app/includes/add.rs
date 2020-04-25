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
            let mut ppath = cpath.clone();
            while let Some(p) = ppath.parent() {
                if manifest.include_exists(p.to_path_buf()) {
                    println!(
                        "{} is already included by {}",
                        cpath.to_string_lossy(),
                        p.to_string_lossy()
                    );
                    return Ok(());
                }
                ppath = p.to_path_buf();
            }
            // Try and add it
            if !manifest.add_include(cpath) {
                println!("{} is already in the include list", path.to_string_lossy());
            }
            // Cleanup other paths that may be included with this new path
            if let Some(includes) = manifest.get_includes() {
                let mut remove: Vec<PathBuf> = Vec::new();
                for include in includes {
                    ppath = include.clone();
                    while let Some(p) = ppath.parent() {
                        if manifest.include_exists(p.to_path_buf()) {
                            remove.push(include.to_path_buf());
                        }
                        ppath = p.to_path_buf();
                    }
                }
                for r in remove {
                    let _ = manifest.remove_include(&r);
                }
            }
        }
        manifest.to_writer(create_manifest_file()?)
    }
}
