use reqwest::multipart;

use crate::{error::Error, models::TransferInfo};

impl super::Api {
    /// Get global transfer info
    ///
    /// This method returns info you usually see in qBt status bar.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-global-transfer-info)
    ///
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
    /// The response is 1 if alternative speed limits are enabled, 0 otherwise.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-alternative-speed-limits-state)
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
    /// * `peers` - The peer to ban, or multiple peers. Each peer is a colon-separated `host:port`
    ///
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
