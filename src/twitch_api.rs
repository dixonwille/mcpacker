use crate::errors::*;
use futures::stream::TryStreamExt;
use reqwest::Client;
use std::io::{Error, ErrorKind};
use tokio::io;
use url::Url;

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

    async fn download_url(&self, project: u32, file: u32) -> Result<String> {
        let url = Url::parse(
            format!(
                "https://addons-ecs.forgesvc.net/api/v2/addon/{}/file/{}/download-url",
                project, file
            )
            .as_str(),
        )?;
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::new(ErrorKind::Other, "incorrect status code").into());
        }
        resp.text().await.map_err(|e| e.into())
    }

    pub async fn download<W: io::AsyncWriteExt + std::marker::Unpin>(
        &self,
        project: u32,
        file: u32,
        w: &mut W,
    ) -> Result<()> {
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
