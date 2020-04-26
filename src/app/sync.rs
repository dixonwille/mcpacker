use crate::app::*;
use crate::errors::Result;
use crate::twitch_api::*;
use std::path::Path;
use structopt::StructOpt;
use tokio::{fs, io, runtime::Runtime, task};

#[derive(StructOpt, Debug)]
pub struct SyncParams {}

impl Run for SyncParams {
    fn run(&self) -> Result<()> {
        let manifest = get_manifest()?;
        if let Some(modules) = manifest.get_mods() {
            let mut rt = Runtime::new()?;
            rt.block_on(download_mods(modules.clone()))?;
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
