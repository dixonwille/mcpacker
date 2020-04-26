use crate::app::*;
use crate::errors::Result;
use crate::twitch_api::*;
use std::path::Path;
use structopt::StructOpt;
use tokio::{fs, io, runtime::Runtime, task};
use std::io::Cursor;
use fasthash::{murmur2::Hash32, FastHash};

#[derive(StructOpt, Debug)]
pub struct SyncParams {}

impl Run for SyncParams {
    fn run(&self) -> Result<()> {
        let manifest = get_manifest()?;
        if let Some(modules) = manifest.get_mods() {
            let mut rt = Runtime::new()?;
            rt.block_on(download_mods(modules.clone()))?;
            rt.block_on(verify_mods(modules.clone()))?;
        }
        Ok(())
    }
}

async fn download_mods<I: IntoIterator<Item=Mod>>(modules: I) -> Result<()> {
    let mut tasks = Vec::new();
    for module in modules {
        tasks.push(task::spawn(download_mod(module)));
    }
    for t in tasks {
        t.await??;
    }
    Ok(())
}

async fn download_mod(module: Mod) -> Result<()> {
    let twitch = TwitchAPI::new();
    let folder = Path::new("mods");
    fs::create_dir_all(folder).await?;
    let f = fs::File::create(folder.join(Path::new(&module.file_name))).await?;
    let mut w = io::BufWriter::new(f);
    twitch
        .download(module.project_id, module.file_id, &mut w)
        .await
}

async fn verify_mods<I: IntoIterator<Item=Mod>>(modules: I) -> Result<()>{
    let mut tasks = Vec::new();
    for module in modules {
        tasks.push(task::spawn(verify_mod(module)));
    }
    for t in tasks {
        t.await??;
    }
    Ok(())
}

async fn verify_mod(module: Mod) -> Result<()> {
    let folder = Path::new("mods");
    let f = fs::File::open(folder.join(Path::new(&module.file_name))).await?;
    let mut r = io::BufReader::new(f);

    // TODO figure out how to hash Async
    let mut cur = Cursor::new(Vec::new());
    let mut buf = io::BufWriter::new(&mut cur);
    let l = io::copy(&mut r, &mut buf).await?;
    if l != module.file_size {
        return Err(io::Error::new(io::ErrorKind::Other, format!("{} is not valid, expected length {} got {}", module.file_name, module.file_size, l)).into())
    }
    let mut buf = Vec::new();
    for b in cur.into_inner() {
        match b {
            9|10|13|32 => {},
            _ => {buf.push(b)}
        }
    }
    let h = Hash32::hash_with_seed(buf, 1);

    if h as u64 != module.fingerprint {
        return Err(io::Error::new(io::ErrorKind::Other, format!("{} is not valid, expected {} got {}", module.file_name, module.fingerprint, h)).into())
    }
    Ok(())
}
