use reqwest::multipart;

use crate::{error::Error, models::TransferInfo};

impl super::Api {
    /// Get global transfer info
    ///
    /// This method returns info you usually see in qBt status bar.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-global-transfer-info)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let info = client.global_transfer_info().await.unwrap();
    ///
    ///     println!("{:?}", info);
    /// }
    /// ```
    pub async fn global_transfer_info(&self) -> Result<TransferInfo, Error> {
        let info = self
            ._get("transfer/info")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<TransferInfo>()
            .await?;

        Ok(info)
    }

    /// Get alternative speed limits state
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-alternative-speed-limits-state)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let state = client.alternative_speed_limit().await.unwrap();
    ///
    ///     println!("Alternative: {}", state);
    /// }
    /// ```
    pub async fn alternative_speed_limit(&self) -> Result<bool, Error> {
        let is_active = self
            ._get("transfer/speedLimitsMode")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<u8>()
            .await?;

        Ok(is_active != 0)
    }

    /// Toggle alternative speed limits
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#toggle-alternative-speed-limits)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.toggle_alternative_speed_limit().await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn toggle_alternative_speed_limit(&self) -> Result<(), Error> {
        self._post("transfer/toggleSpeedLimitsMode")
            .await?
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get global download limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-global-download-limit)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let limit = client.global_download_limit().await.unwrap();
    ///
    ///     println!("Download limit: {}", limit);
    /// }
    /// ```
    pub async fn global_download_limit(&self) -> Result<u64, Error> {
        let limites = self
            ._get("transfer/downloadLimit")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<u64>()
            .await?;

        Ok(limites)
    }

    /// Set global download limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-global-download-limit)
    ///
    /// # Arguments
    ///
    /// * `limit` - The global download speed limit to set in bytes/second. `0` if no limit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.set_global_download_limit(1337).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn set_global_download_limit(&self, limit: u64) -> Result<(), Error> {
        let form = multipart::Form::new().text("limit", limit.to_string());

        self._post("transfer/setDownloadLimit")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get global upload limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-global-upload-limit)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let limit = client.global_upload_limit().await.unwrap();
    ///
    ///     println!("Upload limit: {}", limit);
    /// }
    /// ```
    pub async fn global_upload_limit(&self) -> Result<u64, Error> {
        let limites = self
            ._get("transfer/uploadLimit")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<u64>()
            .await?;

        Ok(limites)
    }

    /// Set global upload limit
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-global-upload-limit)
    ///
    /// # Arguments
    ///
    /// * `limit` - The global upload speed limit to set in bytes/second. `0` if no limit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.set_global_upload_limit(1337).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn set_global_upload_limit(&self, limit: u64) -> Result<(), Error> {
        let form = multipart::Form::new().text("limit", limit.to_string());

        self._post("transfer/setUploadLimit")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Ban peers
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#ban-peers)
    ///
    /// # Arguments
    ///
    /// * `peers` - The peer to ban, or multiple peers. Each peer is `host:port`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let peers = vec!["alice".to_string(), "bob".to_string()];
    ///     let result = client.peers_ban(peers).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn peers_ban(&self, peers: Vec<String>) -> Result<(), Error> {
        let form = multipart::Form::new().text("peers", peers.join("|"));

        self._post("transfer/banPeers")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
