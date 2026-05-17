//! [HTTP API] definitions of [SRS].
//!
//! [SRS]: https://ossrs.io/
//! [1]: https://ossrs.io/lts/en-us/docs/v5/doc/http-api
#![allow(unused_imports)]

mod client;
mod common;
mod error;
mod feature;
mod meminfos;
mod response;
mod rusages;
mod self_proc_stats;
mod stream;
mod summary;
mod system_proc_stats;
mod vhost;

pub use client::Client;
pub use common::{Hls, Kbps, Publish};
pub use error::SrsClientError;
pub use response::{SrsClientResp, SrsClientRespData};
pub use stream::{Audio, Stream, Video};
pub use vhost::Vhost;

use reqwest::{Client as ReqwestClient, Response as ReqwestResponse};
use url::Url;

/// Client for performing requests to [HTTP API][1] of spawned [SRS].
///
/// [SRS]: https://ossrs.io/
/// [1]: https://ossrs.io/lts/en-us/docs/v5/doc/http-api
#[derive(Clone, Debug)]
pub struct SrsClient {
    http_client: ReqwestClient,
    base_url: Url,
}

impl SrsClient {
    /// Build [`SrsClient`] for future call to [HTTP API][1] API of spawned [SRS]. .
    ///
    /// # Errors
    ///
    /// If incorrect `base_url` passed
    ///
    /// [SRS]: https://ossrs.io/
    /// [1]: https://ossrs.io/lts/en-us/docs/v5/doc/http-api
    pub fn build<S: Into<String>>(base_url: S) -> Result<Self, SrsClientError> {
        let base_url = Url::parse(&base_url.into())
            .and_then(|url| url.join("/api/v1/"))
            .map_err(SrsClientError::IncorrectBaseUrl)?;
        tracing::debug!("base_url: {base_url}");
        Ok(Self {
            http_client: ReqwestClient::new(),
            base_url,
        })
    }

    async fn get(&self, url: &str) -> Result<ReqwestResponse, SrsClientError> {
        self.http_client
            .get(
                self.base_url
                    .join(url)
                    .map_err(SrsClientError::IncorrectApiUrl)?,
            )
            .send()
            .await
            .map_err(SrsClientError::RequestFailed)
    }

    async fn delete(&self, url: &str) -> Result<ReqwestResponse, SrsClientError> {
        self.http_client
            .delete(
                self.base_url
                    .join(url)
                    .map_err(SrsClientError::IncorrectApiUrl)?,
            )
            .send()
            .await
            .map_err(SrsClientError::RequestFailed)
    }

    async fn process_resp(&self, resp: ReqwestResponse) -> Result<SrsClientResp, SrsClientError> {
        if !resp.status().is_success() {
            return Err(SrsClientError::BadStatus(resp.status()));
        }
        // tracing::debug!(url = resp.url().to_string(), "processing request");
        tracing::debug!("processing request to: {}", resp.url());
        let resp = resp
            .json::<SrsClientResp>()
            .await
            .map_err(SrsClientError::DeserializeError)?;
        Ok(resp)
    }

    /// [Kicks off][1] a client connected to [SRS] server by its `id`.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    ///
    /// [SRS]: https://ossrs.io/
    /// [1]: https://ossrs.io/lts/en-us/docs/v5/doc/http-api#kickoff-client
    pub async fn kickoff_client<T: Into<String>>(
        self,
        id: T,
    ) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.delete(&format!("clients/{}/", id.into())).await?;
        self.process_resp(resp).await
    }

    /// Retrieves the server version.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_version(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("versions").await?;
        self.process_resp(resp).await
    }

    /// Manages all vhosts or a specified vhost.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_vhosts(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("vhosts").await?;
        self.process_resp(resp).await
    }

    /// Manages a specified vhost.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_vhost<T: Into<String>>(self, id: T) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get(&format!("vhosts/{}", id.into())).await?;
        self.process_resp(resp).await
    }

    /// Retrieves all vhosts as a typed list.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_vhost_list(self) -> Result<Vec<Vhost>, SrsClientError> {
        let response = self.get_vhosts().await?;
        match response.data {
            SrsClientRespData::Vhosts { vhosts } => Ok(vhosts),
            _ => Ok(Vec::new()),
        }
    }

    /// Retrieves a specified vhost as a typed item.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_vhost_item<T: Into<String>>(
        self,
        id: T,
    ) -> Result<Option<Vhost>, SrsClientError> {
        let response = self.get_vhost(id).await?;
        match response.data {
            SrsClientRespData::Vhost { vhost } => Ok(Some(vhost)),
            _ => Ok(None),
        }
    }

    /// Manages all streams or a specified stream.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_streams(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("streams").await?;
        self.process_resp(resp).await
    }

    /// Manages all streams using SRS pagination.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_streams_page(
        self,
        start: i64,
        count: i64,
    ) -> Result<SrsClientResp, SrsClientError> {
        let resp = self
            .get(&format!("streams?start={start}&count={count}"))
            .await?;
        self.process_resp(resp).await
    }

    /// Manages a specified stream.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_stream<T: Into<String>>(self, id: T) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get(&format!("streams/{}", id.into())).await?;
        self.process_resp(resp).await
    }

    /// Retrieves all streams as a typed list.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_stream_list(self) -> Result<Vec<Stream>, SrsClientError> {
        let response = self.get_streams().await?;
        match response.data {
            SrsClientRespData::Streams { streams } => Ok(streams),
            _ => Ok(Vec::new()),
        }
    }

    /// Retrieves a paginated stream response as a typed list.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_stream_page_list(
        self,
        start: i64,
        count: i64,
    ) -> Result<Vec<Stream>, SrsClientError> {
        let response = self.get_streams_page(start, count).await?;
        match response.data {
            SrsClientRespData::Streams { streams } => Ok(streams),
            _ => Ok(Vec::new()),
        }
    }

    /// Retrieves a specified stream as a typed item.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_stream_item<T: Into<String>>(
        self,
        id: T,
    ) -> Result<Option<Stream>, SrsClientError> {
        let response = self.get_stream(id).await?;
        match response.data {
            SrsClientRespData::Stream { stream } => Ok(Some(stream)),
            _ => Ok(None),
        }
    }

    /// Manages all clients or a specified client, default query top 10 clients.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_clients(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("clients").await?;
        self.process_resp(resp).await
    }

    /// Manages all clients using SRS pagination.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_clients_page(
        self,
        start: i64,
        count: i64,
    ) -> Result<SrsClientResp, SrsClientError> {
        let resp = self
            .get(&format!("clients?start={start}&count={count}"))
            .await?;
        self.process_resp(resp).await
    }

    /// Manages a specified client.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_client<T: Into<String>>(self, id: T) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get(&format!("clients/{}", id.into())).await?;
        self.process_resp(resp).await
    }

    /// Retrieves all clients as a typed list.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_client_list(self) -> Result<Vec<Client>, SrsClientError> {
        let response = self.get_clients().await?;
        match response.data {
            SrsClientRespData::Clients { clients } => Ok(clients),
            _ => Ok(Vec::new()),
        }
    }

    /// Retrieves a paginated client response as a typed list.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_client_page_list(
        self,
        start: i64,
        count: i64,
    ) -> Result<Vec<Client>, SrsClientError> {
        let response = self.get_clients_page(start, count).await?;
        match response.data {
            SrsClientRespData::Clients { clients } => Ok(clients),
            _ => Ok(Vec::new()),
        }
    }

    /// Retrieves a specified client as a typed item.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_client_item<T: Into<String>>(
        self,
        id: T,
    ) -> Result<Option<Client>, SrsClientError> {
        let response = self.get_client(id).await?;
        match response.data {
            SrsClientRespData::Client { client } => Ok(Some(client)),
            _ => Ok(None),
        }
    }

    /// Retrieves the supported features of SRS.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_features(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("features").await?;
        self.process_resp(resp).await
    }

    /// Retrieves the rusage of SRS.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_rusages(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("rusages").await?;
        self.process_resp(resp).await
    }

    /// Retrieves the self process stats.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_self_proc_stats(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("self_proc_stats").await?;
        self.process_resp(resp).await
    }

    /// Retrieves the system process stats.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_system_proc_stats(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("system_proc_stats").await?;
        self.process_resp(resp).await
    }

    /// Retrieves the meminfo of system.
    ///
    /// # Errors
    ///
    /// If API request cannot be performed, or fails. See [`SrsClientError`](enum@SrsClientError)
    /// for details.
    pub async fn get_meminfos(self) -> Result<SrsClientResp, SrsClientError> {
        let resp = self.get("meminfos").await?;
        self.process_resp(resp).await
    }
}
