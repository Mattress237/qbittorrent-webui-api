use core::str;
use url::{self, Url};

use reqwest::{Client as ReqwestClient, RequestBuilder, header};

use crate::{Credentials, LoginState, error::Error};

mod application;
mod authentication;
mod creator;
mod log;
mod rss;
mod search;
mod sync;
mod torrent;
mod transfer;

/// Represents a client for interacting with a remote API, handling HTTP requests.
#[derive(Debug)]
pub struct Api {
    http_client: ReqwestClient,
    base_url: tokio::sync::RwLock<Url>,
    state: tokio::sync::RwLock<LoginState>,
}

impl Api {
    /// Creates a new `API` instance.
    pub fn new(url: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            http_client: ReqwestClient::new(),
            base_url: tokio::sync::RwLock::new(Url::parse(&url.into())?),
            state: tokio::sync::RwLock::new(LoginState::new()),
        })
    }

    /// Helper for constructing API URLs
    async fn _build_url(&self, endpoint: &str) -> Result<String, Error> {
        let base_url = self.base_url.read().await;
        let url = format!("{}api/v2/{}", base_url, endpoint);

        Ok(url)
    }

    /// Returns the current session identifier cookie (if it exists).
    pub async fn get_sid_cookie(&self) -> Option<String> {
        self.state.read().await.sid_cookie.clone()
    }

    /// Sets the current session identifier cookie.
    ///
    /// This will also change the state of the client.
    pub async fn set_sid_cookie(&mut self, value: impl Into<&str>) -> Result<(), Error> {
        self.state.write().await.sid_cookie = Some(value.into().to_string());

        Ok(())
    }

    async fn _post(&self, endpoint: &str) -> Result<RequestBuilder, Error> {
        let url = self._build_url(endpoint).await?;

        let builder = self._insert_auth(self.http_client.post(url)).await;

        Ok(builder)
    }

    async fn _get(&self, endpoint: &str) -> Result<RequestBuilder, Error> {
        let url = self._build_url(endpoint).await?;

        let builder = self._insert_auth(self.http_client.get(url)).await;

        Ok(builder)
    }

    async fn _insert_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        #[cfg(not(feature = "qBittorrent-5_1"))]
        if let Some(Credentials::APIKey(key)) = self.state.read().await.credentials.clone() {
            return builder.header(header::AUTHORIZATION, format!("Bearer {}", key));
        }
        if let Some(cookie) = self.state.read().await.sid_cookie.clone() {
            let cookie = format!("{}; HttpOnly; SameSite=Strict; path=/", cookie);
            return builder.header(header::COOKIE, cookie);
        }
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::ParseError::RelativeUrlWithoutBase;

    #[tokio::test]
    async fn url_with_trailing() {
        let result = Api::new("http://127.0.0.1:8090/");

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn url_without_trailing() {
        let result = Api::new("http://127.0.0.1:8090");

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn url_without_port() {
        let result = Api::new("http://127.0.0.1/");

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn url_without_base() {
        let result = Api::new("127.0.0.1:8090");

        assert!(result.is_err());
        let err = result.err().unwrap();

        assert!(matches!(err, Error::UrlParseError(RelativeUrlWithoutBase)));
    }
}
