use std::collections::HashMap;

use crate::{
    error::Error,
    models::{LogItem, LogPeers, LogType},
};

impl super::Api {
    /// Retrieves the main log (application log)
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-log)
    ///
    /// # Arguments
    ///
    /// * `last_known_id` - Exclude messages with "message id" <= `last_known_id` (default: `-1`)
    /// * `log_types` - List of desierd log types. (default: all). Doesn't matter if multiple of the same type are provided as only one will be counted in the end.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::models::LogType;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let types = vec![LogType::Critical, LogType::Warning];
    ///     let log = client.log(None, Some(types)).await.unwrap();
    ///
    ///     for item in log {
    ///         println!("{:?}", item);
    ///     }
    /// }
    /// ```
    pub async fn log(
        &self,
        last_known_id: Option<i64>,
        log_types: Option<Vec<LogType>>,
    ) -> Result<Vec<LogItem>, Error> {
        let mut query = HashMap::new();
        if let Some(last_known_id) = last_known_id {
            query.insert("last_known_id".to_string(), last_known_id.to_string());
        }
        if let Some(log_types) = log_types {
            log_types.iter().for_each(|log_type| {
                query.insert(log_type.to_string(), "true".to_string());
            });
        }

        let log = self
            ._get("log/main")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<LogItem>>()
            .await?;

        Ok(log)
    }

    /// Retrieves the peer log
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-peer-log)
    ///
    /// # Arguments
    ///
    /// * `last_known_id` - Exclude messages with "message id" <= `last_known_id` (default: `-1`)
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
    ///     let log = client.peer_log(None).await.unwrap();
    ///
    ///     for item in log {
    ///         println!("{:?}", item);
    ///     }
    /// }
    /// ```
    pub async fn peer_log(&self, last_known_id: Option<i64>) -> Result<Vec<LogPeers>, Error> {
        let mut query = HashMap::new();
        if let Some(last_known_id) = last_known_id {
            query.insert("last_known_id".to_string(), last_known_id.to_string());
        }

        let log = self
            ._get("log/peers")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<LogPeers>>()
            .await?;

        Ok(log)
    }
}
