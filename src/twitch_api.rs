use crate::manifest::*;
use crate::errors::*;
use reqwest::Client;
use url::Url;
use std::io::{Error, ErrorKind};
use tokio::io;
use futures::stream::TryStreamExt;
use serde::Deserialize;

#[derive(Clone)]
pub struct TwitchAPI {
    client: Client,
}

impl TwitchAPI {
    pub fn new() -> Self {
        TwitchAPI {
            client: Client::new(),
        }
    }

    pub async fn get_mod(&self, project: u32, file: u32) -> Result<Mod> {
        let url = Url::parse(format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}/file/{}", project, file).as_str())?;
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::new(ErrorKind::Other, "incorrect status code").into());
        }
        let fi = resp.json::<FileInfo>().await?;
        Ok(Mod{
            project_id: project,
            file_id: file,
            file_name: fi.file_name,
            file_size: fi.file_length,
            fingerprint: fi.package_fingerprint,
        })
    }

    async fn download_url(&self, project: u32, file: u32) -> Result<String> {
        let url = Url::parse(format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}/file/{}/download-url", project, file).as_str())?;
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::new(ErrorKind::Other, "incorrect status code").into());
        }
        resp.text().await.map_err(|e| e.into())
    }

    pub async fn download<W: io::AsyncWriteExt + std::marker::Unpin>(&self, project: u32, file: u32, w: &mut W) -> Result<()> {
        let url = self.download_url(project, file).await?;
        let url = Url::parse(url.as_str())?;
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::new(ErrorKind::Other, "incorrect status code").into());
        }
        let stream = resp.bytes_stream();
        let stream = io::stream_reader(stream.map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
        let mut stream = io::BufReader::new(stream);
        let _ = io::copy(&mut stream, w).await?;
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FileInfo{
    file_name: String,
    file_length: u64,
    package_fingerprint: u64,
}