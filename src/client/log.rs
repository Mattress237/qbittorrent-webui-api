use std::collections::HashMap;

use crate::{
    error::Error,
    insert_optional,
    models::{LogItem, LogPeers, LogType},
};

impl super::Api {
    /// Get log
    ///
    /// Retrieves the main log of the qBittorrent application.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-log)
    ///
    /// # Arguments
    ///
    /// * `last_known_id` - Exclude messages with "message id" <= `last_known_id` (default: `-1`)
    /// * `log_types` - List of desierd log types. (default: all)
    ///
    pub async fn log(
        &self,
        last_known_id: Option<i64>,
        log_types: Option<Vec<LogType>>,
    ) -> Result<Vec<LogItem>, Error> {
        let mut query = HashMap::new();
        insert_optional!(
            query,
            "last_known_id".to_string(),
            last_known_id,
            |v: i64| v.to_string()
        );

        if let Some(log_types) = log_types {
            for log_type in log_types {
                query.insert(log_type.to_string(), "true".to_string());
            }
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

    /// Get peer log
    ///
    /// Retrieves the peer log of the qBittorrent application.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-peer-log)
    ///
    /// # Arguments
    ///
    /// * `last_known_id` - Exclude messages with "message id" <= `last_known_id` (default: `-1`)
    ///
    pub async fn peer_log(&self, last_known_id: Option<i64>) -> Result<Vec<LogPeers>, Error> {
        let mut query = HashMap::new();
        insert_optional!(query, "last_know_id", last_known_id, |v: i64| v.to_string());

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
