use crate::app::*;
use crate::errors::Result;
use crate::manifest_json::*;
use structopt::StructOpt;
use zip::{ZipWriter, write::FileOptions};
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(StructOpt, Debug)]
pub struct PackParams {
    /// Modify the version before packing.
    #[structopt(short = "v")]
    version: Option<String>,
}

impl Run for PackParams {
    fn run(&self) -> Result<()> {
        let manifest: ManifestJson = (&get_manifest()?).into();
        let mut zip_file = ZipWriter::new(BufWriter::new(File::create(manifest.name.clone()+".zip")?));
        zip_file.set_comment("Minecraft ModPack made by MCPacker");
        zip_file.start_file(MANIFEST_JSON_FILE, FileOptions::default())?;
        manifest.to_writer(zip_file.by_ref())?;
        // TODO Loop through includes and add to overrides folder in zip
        let _ = zip_file.finish()?;
        Ok(())
    }
}
