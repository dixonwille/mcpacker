use crate::app::*;
use crate::errors::Result;
use crate::twitch_api::*;
use fasthash::{murmur2::Hash32, FastHash};
use std::io::Cursor;
use std::path::Path;
use structopt::StructOpt;
use tokio::{fs, io, runtime::Runtime, stream::StreamExt, task};

#[derive(StructOpt, Debug)]
pub struct SyncParams {}

impl SyncParams {
    pub fn run(&self) -> Result<()> {
        let mut rt = Runtime::new()?;
        let mut manifest = get_manifest()?;
        let new_manifest: Manifest = (&get_minecraft_instance()?).into();
        let _ = manifest.sync_mods(&new_manifest);
        manifest.mod_loader = new_manifest.mod_loader;
        manifest.mod_loader_version = new_manifest.mod_loader_version;
        manifest.name = new_manifest.name;
        manifest.to_writer(create_manifest_file()?)?;
        rt.block_on(sync_mod_jars(manifest))?;
        Ok(())
    }
}

async fn sync_mod_jars(manifest: Manifest) -> Result<()> {
    let mut tasks = Vec::new();
    let dir = Path::new(MODS_DIR);
    if dir.is_dir() {
        let mut file_stream = fs::read_dir(dir).await?;
        while let Some(file) = file_stream.next().await {
            let file = file?;
            let file_path = file.path();
            if file_path.is_dir() {
                continue;
            }
            let jar = jar_name(&file_path);
            if jar.is_none() {
                continue;
            }
            let (jar, _) = jar.unwrap();
            let m = manifest.get_mod_by_filename(&jar.file_name().unwrap().to_string_lossy());
            match m {
                Some(m) => tasks.push(task::spawn(verify_file(file_path, m.clone()))),
                None => {
                    if !manifest.include_exists(&file_path) {
                        tasks.push(task::spawn(remove_file(file_path)))
                    }
                }
            }
        }
    }
    if let Some(modules) = manifest.get_mods() {
        for module in modules {
            let path = Path::new(MODS_DIR).join(Path::new(&module.file_name));
            if path.exists() {
                continue;
            }
            let mut disabled_path = module.file_name.clone();
            disabled_path.push_str(".disabled");
            let disabled_path = Path::new(MODS_DIR).join(Path::new(&disabled_path));
            if disabled_path.exists() {
                continue;
            }
            tasks.push(task::spawn(download_mod(module.clone())));
        }
    }
    for t in tasks {
        t.await??;
    }
    Ok(())
}

async fn verify_file(orig: PathBuf, module: Mod) -> Result<()> {
    let f = fs::File::open(&orig).await?;
    let mut r = io::BufReader::new(f);

    // TODO figure out how to hash Async
    let mut cur = Cursor::new(Vec::new());
    let mut buf = io::BufWriter::new(&mut cur);
    let l = io::copy(&mut r, &mut buf).await?;
    if l != module.file_size {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{} is not valid, expected length {} got {}",
                module.file_name, module.file_size, l
            ),
        )
        .into());
    }
    let mut buf = Vec::new();
    for b in cur.into_inner() {
        match b {
            9 | 10 | 13 | 32 => {}
            _ => buf.push(b),
        }
    }
    let h = Hash32::hash_with_seed(buf, 1);

    if h != module.fingerprint {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{} is not valid, expected {} got {}",
                module.file_name, module.fingerprint, h
            ),
        )
        .into());
    }
    Ok(())
}

async fn remove_file(orig: PathBuf) -> Result<()> {
    Ok(fs::remove_file(orig).await?)
}

async fn download_mod(module: Mod) -> Result<()> {
    let twitch = TwitchAPI::new();
    let folder = Path::new(MODS_DIR);
    fs::create_dir_all(folder).await?;
    let path = folder.join(Path::new(&module.file_name));
    // Want to make sure the file handle is closed before verifying the file
    {
        let f = fs::File::create(&path).await?;
        let mut w = io::BufWriter::new(f);
        twitch
            .download(module.project_id, module.file_id, &mut w)
            .await?;
    }
    verify_file(path, module).await
}
