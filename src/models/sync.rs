use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{ConnectionStatus, TorrentsMap};

/// Main response data object
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct MainData {
    /// Response ID
    pub rid: i64,
    /// Whether the response contains all the data or partial data
    pub full_update: Option<bool>,
    /// List of Torrents
    ///
    /// Property: torrent hash, value: TorrentInfo
    pub torrents: Option<TorrentsMap>,
    /// List of hashes of torrents removed since last request
    pub torrents_removed: Option<Vec<String>>,
    /// Info for categories added since last request
    pub categories: Option<HashMap<String, Category>>,
    /// List of categories removed since last request
    pub categories_removed: Option<Vec<String>>,
    /// List of tags added since last request
    pub tags: Option<Vec<String>>,
    /// List of tags removed since last request
    pub tags_removed: Option<Vec<String>>,
    /// Global transfer info
    pub server_state: Option<ServerState>,
    /// List of trackers
    pub trackers: Option<HashMap<String, Vec<String>>>,
}

/// Category response data object
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Category {
    /// Category name
    pub name: String,
    /// Category save path
    #[serde(rename = "savePath")]
    pub save_path: String,
}

/// Server state response data object.
///
/// All of the values are based depending on the last time the request got sent.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct ServerState {
    /// Alltime download
    pub alltime_dl: i64,
    /// Alltime upload
    pub alltime_ul: i64,
    /// Average time in queue in ms
    pub average_time_queue: i64,
    /// Connection status
    pub connection_status: ConnectionStatus,
    /// DHT nodes
    pub dht_nodes: i64,
    /// Download data
    pub dl_info_data: i64,
    /// Download speed
    pub dl_info_speed: i64,
    /// Download rate limit
    pub dl_rate_limit: i64,
    /// Free disk space
    pub free_space_on_disk: i64,
    /// Global ratio
    #[serde(
        deserialize_with = "deserialize_string_to_f64",
        serialize_with = "serialize_f64_to_string"
    )]
    pub global_ratio: f64,
    /// Last external IPv4 address
    pub last_external_address_v4: String,
    /// Last external IPv4 address
    pub last_external_address_v6: String,
    /// Queued IO jobs
    pub queued_io_jobs: i64,
    /// Torrent queueing (application perfence setting)
    pub queueing: bool,
    /// How many times the read cache has been hit
    #[serde(
        deserialize_with = "deserialize_string_to_u64",
        serialize_with = "serialize_u64_to_string"
    )]
    pub read_cache_hits: u64,
    /// How overloaded is the read cache.
    ///
    /// Calculated by read queue size / peer count
    #[serde(
        deserialize_with = "deserialize_string_to_u64",
        serialize_with = "serialize_u64_to_string"
    )]
    pub read_cache_overload: u64,
    /// Refresh Interval
    pub refresh_interval: i64,
    /// Total buffer size
    pub total_buffers_size: i64,
    /// Total peer connections
    pub total_peer_connections: i64,
    /// Total queued size
    pub total_queued_size: i64,
    /// How much data has been wasted in this session. (in bytes)
    ///
    /// Wasted data contains:
    /// - Failed piece checks
    /// - Duplicate Downloads
    pub total_wasted_session: i64,
    /// Upload data
    pub up_info_data: i64,
    /// Upload speed
    pub up_info_speed: i64,
    /// Upload rate limit
    pub up_rate_limit: i64,
    /// Alt speed enabeld
    pub use_alt_speed_limits: bool,
    /// Use subcategories
    pub use_subcategories: bool,
    /// How overloaded is the read cache.
    ///
    /// Calculated by write queue size / peer count
    #[serde(
        deserialize_with = "deserialize_string_to_u64",
        serialize_with = "serialize_u64_to_string"
    )]
    pub write_cache_overload: u64,
}

fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn serialize_f64_to_string<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}

fn serialize_u64_to_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

/// Peers response data object.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct PeersData {
    /// Response ID
    pub rid: i64,
    /// Whether the response contains all the data or partial data
    pub full_update: Option<bool>,
    /// Flags
    pub show_flags: Option<bool>,
    /// List of peers
    pub peers: Option<HashMap<String, Peer>>,
    /// List of removed peers
    pub peers_removed: Option<Vec<String>>,
}

/// Peer response data object.
///
/// All of the values are based on the last time the request got sent.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Peer {
    /// Client used by the peer. (μTorrent, qBittorrent, etc...)
    pub client: Option<String>,
    /// Used connection
    pub connection: Option<String>,
    /// Location
    pub country: Option<String>,
    /// country code
    pub country_code: Option<String>,
    /// Download speed
    pub dl_speed: Option<i64>,
    /// Total downloaded
    pub downloaded: Option<i64>,
    /// Files/contents
    pub files: Option<String>,
    /// Flags
    pub flags: Option<String>,
    /// Flags description
    pub flags_desc: Option<String>,
    /// Connection IP
    pub ip: Option<String>,
    /// Client id
    pub peer_id_client: Option<String>,
    /// Connection port
    pub port: Option<i64>,
    /// How much has the specified peer already downloaded.
    pub progress: Option<f32>,
    /// The ratio of the number of pieces the peer have but you don't have to the total number of pieces you don't have.
    ///
    /// See https://github.com/qbittorrent/qBittorrent/issues/18536 for more information.
    pub relevance: Option<f32>,
    /// Upload speed
    pub up_speed: Option<i64>,
    /// Total uploaded
    pub uploaded: Option<i64>,
}
