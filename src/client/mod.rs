use core::str;
use std::str::FromStr;

use reqwest::{Client as ReqwestClient, Url};

use crate::error::Error;

mod application;
mod authentication;
mod log;
mod rss;
mod sync;
mod torrent;
mod transfer;

/// Represents a client for interacting with a remote API, handling HTTP requests.
pub struct Api {
    http_client: ReqwestClient,
    base_url: tokio::sync::RwLock<Url>,
}

impl Api {
    /// Creates a new `API` instance.
    fn _new(url: &str) -> Result<Self, Error> {
        let http_client = ReqwestClient::builder().cookie_store(true).build()?;

        let base_url = Url::from_str(url)?;

        Ok(Self {
            http_client: http_client,
            base_url: tokio::sync::RwLock::new(base_url),
        })
    }

    /// Helper for constructing API URLs
    async fn _build_url(&self, endpoint: &str) -> Result<Url, Error> {
        let base_url = self.base_url.read().await;
        let url = base_url.join(endpoint)?;

        Ok(url)
    }
}
