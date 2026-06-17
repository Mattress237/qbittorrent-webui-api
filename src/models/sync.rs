use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(not(feature = "qBittorrent-5_1"))]
use serde_json::Value as JsonValue;

use crate::models::{ConnectionStatus, TorrentsMap};

/// Main response data object
///
/// Syncronizason object sendt from the qBittorrent server
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct MainData {
    /// Response ID
    pub rid: i32,
    /// Whether the response contains all the data or partial data
    pub full_update: Option<bool>,
    /// Info for categories added since last request
    pub categories: Option<HashMap<String, Category>>,
    /// List of categories removed since last request
    pub categories_removed: Option<Vec<String>>,
    /// List of tags added since last request
    pub tags: Option<Vec<String>>,
    /// List of tags removed since last request
    pub tags_removed: Option<Vec<String>>,
    /// List of Torrents
    ///
    /// Property: torrent hash, value: TorrentInfo
    pub torrents: Option<TorrentsMap>,
    /// List of hashes of torrents removed since last request
    pub torrents_removed: Option<Vec<String>>,
    /// List of trackers
    pub trackers: Option<HashMap<String, Vec<String>>>,
    /// List of trackers removed since last request
    pub trackers_removed: Option<Vec<String>>,
    /// Global transfer info
    pub server_state: Option<ServerState>,
}

/// Category response data object
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Category {
    /// Category name
    pub name: String,
    /// Category save path
    #[serde(rename = "savePath")]
    pub save_path: String,
    // if not set on the server, the value is received as `json:null`
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub download_path: Option<String>,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub inactive_seeding_time_limit: i32,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub seeding_time_limit: i32,
    /// Limit ratio to this value
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub ratio_limit: f64,
    /// Action to take when share limit is reached
    #[cfg(not(feature = "qBittorrent-5_1"))]
    #[serde(deserialize_with = "string_to_share_limit_action")]
    pub share_limit_action: ShareLimitAction,
}

#[cfg(not(feature = "qBittorrent-5_1"))]
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum ShareLimitAction {
    #[default]
    Default = -1,
    Stop = 0,
    Remove = 1,
    RemoveWithContent = 2,
    EnableSuperSeeding = 3,
}

#[cfg(not(feature = "qBittorrent-5_1"))]
impl std::fmt::Display for ShareLimitAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::Stop => write!(f, "Stop"),
            Self::Remove => write!(f, "Remove"),
            Self::RemoveWithContent => write!(f, "RemoveWithContent"),
            Self::EnableSuperSeeding => write!(f, "EnableSuperSeeding"),
        }
    }
}

#[cfg(not(feature = "qBittorrent-5_1"))]
pub fn string_to_share_limit_action<'de, D>(deserializer: D) -> Result<ShareLimitAction, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::String(s) => match s.as_str() {
            "Default" => Ok(ShareLimitAction::Default),
            "Stop" => Ok(ShareLimitAction::Stop),
            "Remove" => Ok(ShareLimitAction::Remove),
            "RemoveWithContent" => Ok(ShareLimitAction::RemoveWithContent),
            "EnableSuperSeeding" => Ok(ShareLimitAction::EnableSuperSeeding),
            _ => Err(serde::de::Error::custom(format!(
                "invalid share limit action: {}",
                s
            ))),
        },
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for share limit action: {:?}",
            v
        ))),
    }
}

/// Server state response data object.
///
/// All of the values are based depending on the last time the request got sent.
// https://github.com/qbittorrent/qBittorrent/blob/master/src/webui/api/synccontroller.cpp
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/sessionstatus.h
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/cachestatus.h
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/session.h
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct ServerState {
    /// Download speed
    pub dl_info_speed: i64,
    /// Download data
    pub dl_info_data: i64,
    /// Upload speed
    pub up_info_speed: i64,
    /// Upload data
    pub up_info_data: i64,
    /// Download rate limit
    pub dl_rate_limit: i32,
    /// Upload rate limit
    pub up_rate_limit: i32,
    /// Alltime download
    pub alltime_dl: i64,
    /// Alltime upload
    pub alltime_ul: i64,
    /// How much data has been wasted in this session. (in bytes)
    ///
    /// Wasted data contains:
    /// - Failed piece checks
    /// - Duplicate Downloads
    pub total_wasted_session: i64,
    /// Global ratio of download / upload
    ///
    /// It is the ratio of download speed to upload speed. It retunes a `-` when
    /// the ratio is unknown or invalid.
    ///
    /// The value should be 0-100 with 2 decimal places.
    // I set this field to a string to show the intended value from the webserver as a percentage
    pub global_ratio: String,
    /// Total peer connections
    pub total_peer_connections: i64,
    /// How many times the read cache has been hit.
    /// It returns percentage calculation of cache hits.
    ///
    /// The value should be 0-100 with 2 decimal places.
    // I set this field to a string to show the intended value from the webserver as a percentage
    pub read_cache_hits: String,
    /// Total buffer size
    pub total_buffers_size: i64,
    /// How overloaded is the write cache.
    /// It returns percentage calculation of cache overload.
    ///
    /// Calculated by write queue size / peer count
    ///
    /// The value should be 0-100 with 2 decimal places.
    // I set this field to a string to show the intended value from the webserver as a percentage
    pub write_cache_overload: String,
    /// How overloaded is the read cache.
    /// It returns percentage calculation of cache overload.
    ///
    /// Calculated by read queue size / peer count
    ///
    /// The value should be 0-100 with 2 decimal places.
    // I set this field to a string to show the intended value from the webserver as a percentage
    pub read_cache_overload: String,
    /// Queued IO jobs
    pub queued_io_jobs: i64,
    /// Average time in queue in ms
    pub average_time_queue: i64,
    /// Total queued size
    pub total_queued_size: i64,
    #[cfg(feature = "qBittorrent-5_3")]
    //[#24152](https://github.com/qbittorrent/qBittorrent/pull/24152)
    /// The time it takes from receiving a request from a peer until we're
    /// sending the response back on the socket.
    pub request_latency: i64,
    /// Last external IPv4 address
    pub last_external_address_v4: String,
    /// Last external IPv4 address
    pub last_external_address_v6: String,
    /// DHT nodes
    pub dht_nodes: i64,
    /// Connection status
    pub connection_status: ConnectionStatus,
    #[cfg(feature = "qBittorrent-5_3")]
    // [#24084](https://github.com/qbittorrent/qBittorrent/pull/24084)
    /// The number of queued tracker announces.
    ///
    /// It shows the number of currently queued tracker announces.
    pub queued_tracker_announces: i64,
    /// Free disk space
    pub free_space_on_disk: i64,
    /// Torrent queueing (application perfence setting)
    pub queueing: bool,
    /// Alt speed enabeld
    pub use_alt_speed_limits: bool,
    /// Refresh Interval
    pub refresh_interval: i32,
    #[cfg(feature = "qBittorrent-5_1")]
    /// Use subcategories
    pub use_subcategories: Option<bool>,
}

/// Peers response data object.
// https://github.com/qbittorrent/qBittorrent/blob/master/src/webui/api/synccontroller.cpp
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct PeersData {
    /// Response ID
    pub rid: i64,
    /// Whether the response contains all the data or partial data
    pub full_update: Option<bool>,
    /// Flags
    pub show_flags: bool,
    /// List of peers
    pub peers: Option<HashMap<String, Peer>>,
    // Cant find the field???
    // /// List of removed peers
    // pub peers_removed: Option<Vec<String>>,
}

/// Peer response data object.
///
/// All of the values are based on the last time the request got sent.
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/peerinfo.h
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Peer {
    /// Client used by the peer. (μTorrent, qBittorrent, etc...)
    pub client: String,
    /// Client id
    pub peer_id_client: String,
    /// How much has the specified peer already downloaded.
    pub progress: f64,
    /// Download speed
    pub dl_speed: i32,
    /// Upload speed
    pub up_speed: i32,
    /// Total downloaded
    pub downloaded: i64,
    /// Total uploaded
    pub uploaded: i64,
    /// Used connection
    pub connection: PeerConnectionType, // make enum
    /// Flags
    pub flags: String,
    /// Flags description
    pub flags_desc: String,
    /// The ratio of the number of pieces the peer have but you don't have to
    /// the total number of pieces you don't have.
    ///
    /// See https://github.com/qbittorrent/qBittorrent/issues/18536 for more information.
    pub relevance: f64,
    #[cfg(feature = "qBittorrent-5_3")]
    // [#23989](https://github.com/qbittorrent/qBittorrent/pull/23989)
    /// Contribution
    ///
    /// Contribution helps users identify how much of a peer's current progress
    /// is directly attributable to this client's uploads
    pub contribution: f64,
    /// Files/contents
    pub files: Option<String>,
    pub i2p_dest: Option<String>,
    /// Connection IP
    pub ip: Option<String>,
    /// Connection port
    pub port: Option<u16>,
    /// Hostname
    pub host_name: Option<String>,
    /// country code
    pub country_code: Option<String>,
    /// Location
    pub country: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PeerConnectionType {
    /// μTP
    UTP,
    #[default]
    BT,
    Web,
}

impl Serialize for PeerConnectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl PeerConnectionType {
    pub fn as_str(&self) -> &str {
        match self {
            PeerConnectionType::UTP => "μTP",
            PeerConnectionType::BT => "BT",
            PeerConnectionType::Web => "Web",
        }
    }
}

// deserialize
impl<'de> Deserialize<'de> for PeerConnectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "μTP" => Ok(PeerConnectionType::UTP),
            "BT" => Ok(PeerConnectionType::BT),
            "Web" => Ok(PeerConnectionType::Web),
            _ => Err(serde::de::Error::custom("invalid peer connection type")),
        }
    }
}
