use crate::errors::Result;
use crate::app::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct AuthorParams {
    author: String
}

impl AuthorParams {
    pub fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        manifest.author = self.author.clone();
        manifest.to_writer(create_manifest_file()?)?;
        Ok(())
    }
}