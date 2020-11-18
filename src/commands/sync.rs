use crate::*;
use crate::errors::Result;
use crate::utils::murmur2::murmurhash2_32;
use crate::utils::twitch_api::*;
use std::path::Path;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::{
    fs,
    io::{self, AsyncReadExt, AsyncWriteExt},
    stream::StreamExt,
    task,
};

#[derive(StructOpt, Debug)]
pub struct SyncParams {}

impl SyncParams {
    pub fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        let new_manifest: Manifest = (&get_minecraft_instance()?).into();
        let _ = manifest.sync_mods(&new_manifest);
        manifest.mod_loader = new_manifest.mod_loader;
        manifest.mod_loader_version = new_manifest.mod_loader_version;
        manifest.name = new_manifest.name;
        manifest.to_writer(create_manifest_file()?)?;
        sync_mod_jars(manifest)?;
        Ok(())
    }
}

#[tokio::main]
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
                Some(m) => tasks.push(task::spawn(verify_file(
                    fs::File::open(file_path).await?,
                    m.clone(),
                ))),
                None => {
                    if !manifest.include_exists(&file_path) {
                        tasks.push(task::spawn(remove_file(file_path)))
                    }
                }
            }
        }
    }
    if let Some(modules) = manifest.get_mods() {
        let twitch = Arc::new(TwitchAPI::new());
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
            tasks.push(task::spawn(download_mod(
                Arc::clone(&twitch),
                module.clone(),
            )));
        }
    }
    let mut was_error = false;
    for t in tasks {
        match t.await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                was_error = true;
                println!("{}", e)
            }
            Err(e) => {
                was_error = true;
                println!("{}", e)
            }
        };
    }
    if was_error {
        return Err(io::Error::new(io::ErrorKind::Other, "there was an error syncing mods").into());
    }
    Ok(())
}

async fn verify_file(mut file: fs::File, module: Mod) -> Result<()> {
    let mut buf = Vec::new();
    // Get file contents into memory to get the length and hash.
    // Hash function used does not support streaming bytes to it.
    // Otherwise, it is best to read chunks at a time instead of all in memory.
    let l = file.read_to_end(&mut buf).await?;
    if l as u64 != module.file_size {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{} is not valid, expected length {} got {}",
                module.file_name, module.file_size, l
            ),
        )
        .into());
    }
    // Compute the hash using the original Murmur2 32 bit algorithm.
    // This hash function does not support streaming bytes so we need the full buff.
    buf.retain(is_not_whitespace);
    let h = murmurhash2_32(buf.as_ref(), 1);
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

fn is_not_whitespace(b: &u8) -> bool {
    let b = *b;
    b != 9 && b != 10 && b != 13 && b != 32
}

async fn remove_file(orig: PathBuf) -> Result<()> {
    Ok(fs::remove_file(orig).await?)
}

async fn download_mod(twitch: Arc<TwitchAPI>, module: Mod) -> Result<()> {
    let folder = Path::new(MODS_DIR);
    fs::create_dir_all(folder).await?;
    let path = folder.join(Path::new(&module.file_name));
    // Want to make sure the file handle is closed before verifying the file
    let f = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .await?;
    let mut w = io::BufWriter::new(f);
    twitch
        .download(module.project_id, module.file_id, &mut w)
        .await?;
    w.flush().await?;
    let mut f = w.into_inner();
    let _ = f.seek(io::SeekFrom::Start(0)).await?; // Need to make sure we start at the beginning of the file
    verify_file(f, module).await
}
