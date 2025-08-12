use core::str;
use url::{self, Url};

use reqwest::{
    Client as ReqwestClient, RequestBuilder,
    header::{self, HeaderMap},
};

use crate::{LoginState, error::Error};

mod application;
mod authentication;
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
            state: tokio::sync::RwLock::new(LoginState::Unknown),
        })
    }

    /// Helper for constructing API URLs
    async fn _build_url(&self, endpoint: &str) -> Result<String, Error> {
        let base_url = self.base_url.read().await;
        let url = format!("{}api/v2/{}", base_url, endpoint);

        Ok(url)
    }

    pub async fn get_sid_cookie(&self) -> Option<String> {
        self.state.read().await.as_cookie()
    }

    pub async fn set_sid_cookie(&mut self, value: impl Into<&str>) -> Result<(), Error> {
        let new_state = self.state.read().await.add_cookie(value.into());

        let mut old_state = self.state.write().await;
        *old_state = new_state;

        Ok(())
    }

    async fn _post(&self, endpoint: &str) -> Result<RequestBuilder, Error> {
        let mut header_map = HeaderMap::new();
        if let Some(cookie) = self.state.read().await.as_cookie() {
            let cookie = format!("SID={}; HttpOnly; SameSite=Strict; path=/", cookie);
            header_map.insert(header::COOKIE, cookie.parse().unwrap());
        }

        let url = self._build_url(endpoint).await?;

        let builder = self.http_client.post(url).headers(header_map);

        Ok(builder)
    }

    async fn _get(&self, endpoint: &str) -> Result<RequestBuilder, Error> {
        let mut header_map = HeaderMap::new();
        if let Some(cookie) = self.state.read().await.as_cookie() {
            let cookie = format!("SID={}; HttpOnly; SameSite=Strict; path=/", cookie);
            header_map.insert(header::COOKIE, cookie.parse().unwrap());
        }

        let url = self._build_url(endpoint).await?;

        let builder = self.http_client.get(url).headers(header_map);

        Ok(builder)
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
