use serde::{Deserialize, Serialize};

use crate::models::ConnectionStatus;

/// Transfer info data object
///
/// This is the data that whuld usually se in the Qbit status bar.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct TransferInfo {
    /// Global download rate (bytes/s)
    pub dl_info_speed: i64,
    /// Data downloaded this session (bytes)
    pub dl_info_data: i64,
    /// Global upload rate (bytes/s)
    pub up_info_speed: i64,
    /// Data uploaded this session (bytes)
    pub up_info_data: i64,
    /// Download rate limit (bytes/s)
    pub dl_rate_limit: i64,
    /// Upload rate limit (bytes/s)
    pub dht_nodes: i64,
    /// DHT nodes connected to
    pub connection_status: ConnectionStatus,
    /// True if torrent queueing is enabled
    pub queueing: Option<bool>,
    /// True if alternative speed limits are enabled
    pub use_alt_speed_limits: Option<bool>,
    /// Transfer list refresh interval (milliseconds)
    pub refresh_interval: Option<bool>,
    /// Last external IPv4 address
    ///
    /// This field has not been documented in the API!
    pub last_external_address_v4: String,
    /// Last external IPv4 address
    ///
    /// This field has not been documented in the API!
    pub last_external_address_v6: String,
}
