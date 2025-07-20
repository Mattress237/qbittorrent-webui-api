use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Torrent info response object
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentInfo {
    /// Time (Unix Epoch) when the torrent was added to the client
    pub added_on: i64,
    /// Amount of data left to download (bytes)
    pub amount_left: i64,
    /// Whether this torrent is managed by Automatic Torrent Management
    pub auto_tmm: bool,
    /// Percentage of file pieces currently available
    pub availability: f64,
    /// Category of the torrent
    pub category: String,
    /// Amount of transfer data completed (bytes)
    pub completed: i64,
    /// Time (Unix Epoch) when the torrent completed
    pub completion_on: i64,
    /// Absolute path of torrent content (root path for multifile torrents, absolute file path for singlefile torrents)
    pub content_path: String,
    /// Torrent download speed limit (bytes/s). -1 if unlimited.
    pub dl_limit: i64,
    /// Torrent download speed (bytes/s)
    pub dlspeed: i64,
    /// Amount of data downloaded
    pub downloaded: i64,
    /// Amount of data downloaded this session
    pub downloaded_session: i64,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// True if first last piece are prioritized
    pub f_l_piece_prio: bool,
    /// True if force start is enabled for this torrent
    pub force_start: bool,
    /// Torrent hash
    pub hash: Option<String>,
    /// True if torrent is from a private tracker (added in 5.0.0)
    ///
    /// The value will be `None` if the torrent metadata is not available yet.
    /// See issue [#10](https://github.com/Mattress237/qbittorrent-webui-api/issues/10)
    ///
    /// NOTE: Documentation is incorrect. The field name is "private", not "isPrivate".
    pub private: Option<bool>,
    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: i64,
    /// Magnet URI corresponding to this torrent
    pub magnet_uri: String,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    pub max_ratio: f32,
    /// Maximum seeding time (seconds) until torrent is stopped from seeding
    pub max_seeding_time: i64,
    /// Torrent name
    pub name: String,
    /// Number of seeds in the swarm
    pub num_complete: i64,
    /// Number of leechers in the swarm
    pub num_incomplete: i64,
    /// Number of leechers connected to
    pub num_leechs: i64,
    /// Number of seeds connected to
    pub num_seeds: i64,
    /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    pub priority: i64,
    /// Torrent progress (percentage/100)
    pub progress: f32,
    /// Torrent share ratio. Max ratio value: 9999.
    pub ratio: f32,
    /// TODO (what is different from max_ratio?)
    pub ratio_limit: f32,
    /// Time until the next tracker reannounce
    pub reannounce: i64,
    /// Path where this torrent's data is stored
    pub save_path: String,
    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i64,
    /// TODO (what is different from max_seeding_time?) seeding_time_limit is a per torrent setting, when Automatic Torrent Management is disabled, furthermore then max_seeding_time is set to seeding_time_limit for this torrent. If Automatic Torrent Management is enabled, the value is -2. And if max_seeding_time is unset it have a default value -1.
    pub seeding_time_limit: i64,
    /// Time (Unix Epoch) when this torrent was last seen complete
    pub seen_complete: i64,
    /// True if sequential download is enabled
    pub seq_dl: bool,
    /// Total size (bytes) of files selected for download
    pub size: i64,
    /// Torrent state. See table here below for the possible values
    pub state: String,
    /// True if super seeding is enabled
    pub super_seeding: bool,
    /// Comma-concatenated tag list of the torrent
    pub tags: String,
    /// Total active time (seconds)
    pub time_active: i64,
    /// Total size (bytes) of all file in this torrent (including unselected ones)
    pub total_size: i64,
    /// The first tracker with working status. Returns empty string if no tracker is working.
    pub tracker: String,
    /// Torrent upload speed limit (bytes/s). -1 if unlimited.
    pub up_limit: i64,
    /// Amount of data uploaded
    pub uploaded: i64,
    /// Amount of data uploaded this session
    pub uploaded_session: i64,
    /// Torrent upload speed (bytes/s)
    pub upspeed: i64,
}

/// Generic torrent properties
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentProperties {
    /// Torrent save path
    pub save_path: String,
    /// Torrent creation date (Unix timestamp)
    pub creation_date: i64,
    /// Torrent piece size (bytes)
    pub piece_size: i64,
    /// Torrent comment
    pub comment: String,
    /// Total data wasted for torrent (bytes)
    pub total_wasted: i64,
    /// Total data uploaded for torrent (bytes)
    pub total_uploaded: i64,
    /// Total data uploaded this session (bytes)
    pub total_uploaded_session: i64,
    /// Total data downloaded for torrent (bytes)
    pub total_downloaded: i64,
    /// Total data downloaded this session (bytes)
    pub total_downloaded_session: i64,
    /// Torrent upload limit (bytes/s)
    pub up_limit: i64,
    /// Torrent download limit (bytes/s)
    pub dl_limit: i64,
    /// Torrent elapsed time (seconds)
    pub time_elapsed: i64,
    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i64,
    /// Torrent connection count
    pub nb_connections: i64,
    /// Torrent connection count limit
    pub nb_connections_limit: i64,
    /// Torrent share ratio
    pub share_ratio: f32,
    /// When this torrent was added (unix timestamp)
    pub addition_date: i64,
    /// Torrent completion date (unix timestamp)
    pub completion_date: i64,
    /// Torrent creator
    pub created_by: String,
    /// Torrent average download speed (bytes/second)
    pub dl_speed_avg: i64,
    /// Torrent download speed (bytes/second)
    pub dl_speed: i64,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// Last seen complete date (unix timestamp)
    pub last_seen: i64,
    /// Number of peers connected to
    pub peers: i64,
    /// Number of peers in the swarm
    pub peers_total: i64,
    /// Number of pieces owned
    pub pieces_have: i64,
    /// Number of pieces of the torrent
    pub pieces_num: i64,
    /// Number of seconds until the next announce
    pub reannounce: i64,
    /// Number of seeds connected to
    pub seeds: i64,
    /// Number of seeds in the swarm
    pub seeds_total: i64,
    /// Torrent total size (bytes)
    pub total_size: i64,
    /// Torrent average upload speed (bytes/second)
    pub up_speed_avg: i64,
    /// Torrent upload speed (bytes/second)
    pub up_speed: i64,
    /// True if torrent is from a private tracker (added in 5.0.0)
    ///
    /// The value will be `null` if the torrent metadata is not available yet.
    /// See issue [#10](https://github.com/Mattress237/qbittorrent-webui-api/issues/10)
    pub private: Option<bool>,
}

/// Torrent tracker data object
#[derive(Debug, Deserialize, Serialize)]
pub struct Tracker {
    /// Tracker url
    pub url: String,
    /// Tracker status. See the table below for possible values
    pub status: i64,
    /// Tracker priority tier. Lower tier trackers are tried before higher tiers. Tier numbers are valid when `>= 0`, `< 0` is used as placeholder when `tier` does not exist for special entries (such as DHT).
    pub tier: i64,
    /// Number of peers for current torrent, as reported by the tracker
    pub num_peers: i64,
    /// Number of seeds for current torrent, asreported by the tracker
    pub num_seeds: i64,
    /// Number of leeches for current torrent, as reported by the tracker
    pub num_leeches: i64,
    /// Number of completed downloads for current torrent, as reported by the tracker
    pub num_downloaded: i64,
    /// Tracker message (there is no way of knowing what this message is - it's up to tracker admins)
    pub msg: String,
}

/// Web seed data object
#[derive(Debug, Deserialize, Serialize)]
pub struct WebSeed {
    /// Web seed URL
    pub url: String,
}

/// Torrent file/content data object
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentContent {
    /// File index
    pub index: i64,
    /// File name (including relative path)
    pub name: String,
    /// File size (bytes)
    pub size: i64,
    /// File progress (percentage/100)
    pub progress: f64,
    /// File priority.
    pub priority: FilePriority,
    /// True if file is seeding/complete
    pub is_seed: Option<bool>,
    /// The first number is the starting piece index and the second number is the ending piece index (inclusive)
    pub piece_range: Vec<i64>,
    /// Percentage of file pieces currently available (percentage/100)
    pub availability: f64,
}

/// File priority enum
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FilePriority {
    /// Do not download
    DoNotDownload = 0,
    /// Normal priority
    Normal = 1,
    /// High priority
    High = 6,
    /// Maximal priority
    Maximal = 7,
}

/// Pices state
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PiecesState {
    NotDownloaded = 0,
    Downloading = 1,
    Downloaded = 2,
}
