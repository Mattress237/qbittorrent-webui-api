use crate::{
    error::Error,
    models::{MainData, PeersData},
};

impl super::Api {
    /// Get main data
    ///
    /// If the given `rid` is different from the one of last server reply,
    /// `full_update` will be `true`
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-main-data)
    ///
    /// # Arguments
    ///
    /// * `rid` - Response ID. If not provided, `rid=0` will be assumed.
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
    ///     let data = client.main_data(None).await.unwrap();
    ///
    ///     println!("{:#?}", data);
    /// }
    /// ```
    pub async fn main_data(&self, rid: Option<i64>) -> Result<MainData, Error> {
        let mut query = vec![];
        if let Some(rid) = rid {
            query.push(("rid", rid));
        }

        let data = self
            ._get("sync/maindata")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<MainData>()
            .await?;

        Ok(data)
    }

    /// Get torrent peers data
    ///
    /// Fetches peer data changes since the last request. If the given `rid` is different from the one of last server reply,
    /// `full_update` will be `true`
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-torrent-peers-data)
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash.
    /// * `rid` - Response ID. If not provided, `rid=0` will be assumed.
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
    ///     let data = client.peers_data("hash", None).await.unwrap();
    ///
    ///     println!("{:#?}", data);
    /// }
    /// ```
    pub async fn peers_data(&self, hash: &str, rid: Option<i64>) -> Result<PeersData, Error> {
        let mut query = vec![];
        query.push(("hash", hash.to_string()));
        if let Some(rid) = rid {
            query.push(("rid", rid.to_string()));
        }

        let data = self
            ._get("sync/torrentPeers")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<PeersData>()
            .await?;

        Ok(data)
    }
}
