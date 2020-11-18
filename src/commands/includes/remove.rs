use crate::{
    commands::includes::relative_path,
    files::manifest::{create_manifest_file, get_manifest},
};
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Remove {
    /// Relative path to file to add to includes.
    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,
}

impl Remove {
    pub fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        for path in self.paths.iter() {
            if !manifest.remove_include(&relative_path(path)?) {
                println!("{} was not in the includes", path.to_string_lossy());
            }
        }
        manifest.to_writer(create_manifest_file()?)
    }
}
