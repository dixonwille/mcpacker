use anyhow::{anyhow, Context, Result};
use reqwest::Client;
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

    async fn download_url(&self, project: u32, file: u32) -> Result<Url> {
        let url = Url::parse(
            format!(
                "https://addons-ecs.forgesvc.net/api/v2/addon/{}/file/{}/download-url",
                project, file
            )
            .as_str(),
        )
        .expect("could not create get download url");
        let resp = self.client.get(url.clone()).send().await.with_context(|| {
            format!(
                "could not send request to {} for project {} and file {}",
                url, project, file
            )
        })?;
        if !resp.status().is_success() {
            return Err(anyhow!(format!(
                "could not get download url for project {} and file {}: status code {}",
                project,
                file,
                resp.status()
            )));
        }
        let raw = resp.text().await.with_context(|| {
            format!(
                "could not get body of get download request as text for project {} and file {}",
                project, file
            )
        })?;
        Url::parse(&raw).with_context(|| {
            format!(
                "{} is not a valid url for project {} and file {}",
                raw, project, file
            )
        })
    }

    pub async fn download<W: io::AsyncWrite + std::marker::Unpin>(
        &self,
        project: u32,
        file: u32,
        w: &mut W,
    ) -> Result<()> {
        let url = self.download_url(project, file).await?;
        let resp = self.client.get(url.clone()).send().await.with_context(|| {
            format!(
                "could not send request to {} for project {} and file {}",
                url, project, file
            )
        })?;
        if !resp.status().is_success() {
            return Err(anyhow!(format!(
                "could not download file for project {} and file {}: status code {}",
                project,
                file,
                resp.status()
            )));
        }
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let _ = w
                .write_all(
                    chunk
                        .with_context(|| {
                            format!(
                                "failed to read response for project {} and file {}",
                                project, file
                            )
                        })?
                        .as_ref(),
                )
                .await
                .with_context(|| {
                    format!(
                        "failed to write response to writer for project {} and file {}",
                        project, file
                    )
                })?;
        }
        Ok(())
    }
}
