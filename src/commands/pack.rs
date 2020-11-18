use crate::files::{
    manifest::get_manifest,
    manifest_json::{ManifestJson, MANIFEST_JSON_FILE, MANIFEST_OVERRIDES_FOLDER},
};
use anyhow::Result;
use std::{
    collections::HashSet,
    fs::{read_dir, File},
    io::{copy, BufReader, BufWriter, Seek, Write},
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use zip::{write::FileOptions, ZipWriter};

#[derive(StructOpt, Debug)]
pub struct PackParams {}

impl PackParams {
    pub fn run(&self) -> Result<()> {
        let manifest = get_manifest()?;
        let manifest_json: ManifestJson = (&manifest).into();
        let mut zip_file = ZipWriter::new(BufWriter::new(File::create(
            manifest.name.clone() + ".zip",
        )?));
        zip_file.set_comment("Minecraft ModPack made by MCPacker");
        zip_file.start_file(MANIFEST_JSON_FILE.to_string_lossy(), FileOptions::default())?;
        manifest_json.to_writer(BufWriter::new(zip_file.by_ref()))?;
        if let Some(includes) = manifest.get_includes() {
            let mut zi = ZipInclude::new();
            for include in includes {
                zi.path_to_zip(zip_file.by_ref(), include)?;
            }
        }
        let _ = zip_file.finish()?;
        Ok(())
    }
}

struct ZipInclude {
    included: HashSet<PathBuf>,
}

impl ZipInclude {
    fn new() -> Self {
        ZipInclude {
            included: HashSet::new(),
        }
    }

    fn path_to_zip<W: Write + Seek>(&mut self, z: &mut ZipWriter<W>, p: &PathBuf) -> Result<()> {
        if p.is_dir() {
            let over = Path::new(MANIFEST_OVERRIDES_FOLDER).to_path_buf().join(p);
            if !self.included.insert(over.clone()) {
                println!("already added {} to archive", over.to_string_lossy());
                return Ok(());
            }
            z.add_directory(over.to_string_lossy(), FileOptions::default())?;
            for entry in read_dir(p)? {
                let path = entry?.path();
                self.path_to_zip(z, &path)?;
            }
        } else if p.is_file() {
            let over = Path::new(MANIFEST_OVERRIDES_FOLDER).to_path_buf().join(p);
            if !self.included.insert(over.clone()) {
                println!("already added {} to archive", over.to_string_lossy());
                return Ok(());
            }
            z.start_file(over.to_string_lossy(), FileOptions::default())?;
            let _ = copy(&mut BufReader::new(File::open(p)?), z.by_ref())?;
        } else {
            println!("unsure what to do with include {}", p.to_string_lossy())
        }
        Ok(())
    }
}
