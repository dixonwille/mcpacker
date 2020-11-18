use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::io::{Error, ErrorKind};
use tokio::{
    io::{self, AsyncWriteExt},
    stream::StreamExt,
};
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
        )
        .expect("could not create get download url");
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("could not get download url"));
        }
        resp.text()
            .await
            .with_context(|| "could not get body of get download request as text")
    }

    pub async fn download<W: io::AsyncWrite + std::marker::Unpin>(
        &self,
        project: u32,
        file: u32,
        w: &mut W,
    ) -> Result<()> {
        let url = Url::parse(self.download_url(project, file).await?.as_str())?;
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::new(ErrorKind::Other, "incorrect status code").into());
        }
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let _ = w.write_all(chunk?.as_ref()).await?;
        }
        Ok(())
    }
}
