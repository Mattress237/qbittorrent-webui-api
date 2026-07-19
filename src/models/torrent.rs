use std::{collections::HashMap, fmt, ops::Deref};

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{MapAccess, Visitor},
};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(not(feature = "qBittorrent-5_1"))]
// https://github.com/qbittorrent/qBittorrent/pull/22989
use crate::models::ShareLimitAction;
#[cfg(feature = "qBittorrent-5_3")]
// https://github.com/qbittorrent/qBittorrent/pull/24043
use crate::models::ShareLimitMode;
use crate::parameters::TorrentState;
use crate::utilities::deserializers;
use crate::utilities::serializers;

/// Represents a torrent and its associated metadata.
///
/// This struct contains detailed information about a torrent, including its
/// download/upload statistics, state, and various properties.
//
// https://github.com/qbittorrent/qBittorrent/blob/master/src/webui/api/serialize/serialize_torrent.cpp
// https://github.com/qbittorrent/qBittorrent/blob/master/src/webui/api/serialize/serialize_torrent.h
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/torrent.cpp
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/torrent.h
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Torrent {
    /// Torrent hash
    pub hash: String,
    /// The SHA-1 hash of the torrent's info dictionary (used in BitTorrent v1).
    pub infohash_v1: String,
    ///  SHA-256 hash of the torrent's info dictionary (used in BitTorrent v2).
    pub infohash_v2: String,
    /// Torrent name
    pub name: String,

    /// True if the torrent has metadata available
    ///
    /// Dependent on this being `true` or `false` fields like `private` may
    /// have a undefinde value and might be using a default value or `None`
    pub has_metadata: bool,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub created_by: String,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub creation_date: i64,
    /// True if torrent is from a private tracker (added in 5.0.0)
    ///
    /// The value will be `None` if the torrent metadata is not available yet.
    /// See issue [#10](https://github.com/Mattress237/qbittorrent-webui-api/issues/10)
    pub private: Option<bool>,
    /// Total size (bytes) of all file in this torrent (including unselected ones)
    pub total_size: i64,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    #[serde(rename = "pieces_num")]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub pieces_count: i32,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub piece_size: i64,

    /// Magnet URI corresponding to this torrent
    pub magnet_uri: String,
    /// Total size (bytes) of files selected for download
    pub size: i64,
    /// Torrent progress (percentage/100)
    pub progress: f64,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub total_wasted: i64,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub pieces_have: i32,
    /// Torrent download speed (bytes/s)
    #[serde(rename = "dlspeed")]
    pub download_speed: i32,
    /// Torrent upload speed (bytes/s)
    #[serde(rename = "upspeed")]
    pub upload_speed: i32,
    /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    #[serde(rename = "priority")]
    pub queue_position: i32,
    /// Number of seeds connected to
    pub num_seeds: i32,
    /// Number of seeds in the swarm
    pub num_complete: i32,
    /// Number of leechers connected to
    pub num_leechs: i32,
    /// Number of leechers in the swarm
    pub num_incomplete: i32,

    /// State that the torrent is currently in.
    pub state: TorrentState,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// True if sequential download is enabled
    #[serde(rename = "seq_dl")]
    pub sequential_download: bool,
    /// True if first last piece are prioritized
    #[serde(rename = "f_l_piece_prio")]
    pub first_last_piece_prio: bool,

    /// Category of the torrent
    pub category: String,
    /// Tags list fore the torrent
    #[serde(deserialize_with = "deserializers::string_to_vec_comma_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_comma_separated")]
    pub tags: Vec<String>, // <== FIX: make list
    /// True if super seeding is enabled
    pub super_seeding: bool,
    /// True if force start is enabled for this torrent
    pub force_start: bool,
    /// Path where this torrent's data is stored
    ///
    /// The parent folder of `content_path`
    pub save_path: String,
    /// Path where the torrent's data is downloaded when incomplet.
    ///
    /// Empty when not used.
    pub download_path: String,
    /// Root path for multifile torrents, absolute file path for singlefile torrents
    pub content_path: String,
    /// The root path of the torrent.
    ///
    /// Empty string if not a folder
    pub root_path: String,
    /// Time (Unix Epoch) when the torrent was added to the client
    pub added_on: i64,
    /// Time (Unix Epoch) when the torrent completed
    pub completion_on: i64,
    /// The first tracker with working status. Returns empty string if no tracker is working.
    pub tracker: String,
    /// Total count of trackers
    pub trackers_count: i32,
    /// Torrent download speed limit (bytes/s). -1 if unlimited.
    #[serde(rename = "dl_limit")]
    pub download_limit: i32,
    /// Torrent upload speed limit (bytes/s). -1 if unlimited.
    #[serde(rename = "up_limit")]
    pub upload_limit: i32,
    /// Amount of data downloaded
    pub downloaded: i64,
    /// Amount of data uploaded
    pub uploaded: i64,
    /// Amount of data downloaded this session
    pub downloaded_session: i64,
    /// Amount of data uploaded this session
    pub uploaded_session: i64,
    #[serde(deserialize_with = "deserializers::from_null_to_default")]
    /// Amount of data left to download (bytes)
    pub amount_left: i64,
    /// Amount of transfer data completed (bytes)
    pub completed: i64,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub connections_count: i32,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22753
    pub connections_limit: i32,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    ///
    /// - `-1` means no limit.
    ///
    /// Uses global limit if `ratio_limit` is set to `-2`.
    pub max_ratio: f64,
    /// The maximum amount of time (minutes) the torrent is allowed to seed before stopped.
    ///
    /// - `-1` means no limit.
    ///
    /// Uses global limit if `seeding_time_limit` is set to `-2`.
    pub max_seeding_time: i32,
    /// The maximum amount of time (minutes) the torrent is allowed to seed while being inactive before stopped.
    ///
    /// - `-1` means no limit.
    ///
    /// Uses global limit if `inactive_seeding_time_limit` is set to `-2`.
    pub max_inactive_seeding_time: i32,
    /// Torrent share ratio. Max ratio value: 9999.
    pub ratio: f64,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    ///
    /// This field is used to override the global setting for this specific torrent.
    ///
    /// - `-2` means the global limit should be used. `max_ratio` will have the
    ///   global setting set.
    /// - `-1` means no limit.
    pub ratio_limit: f64,
    /// Popularity of the torrent
    pub popularity: f64,
    /// The maximum amount of time (minutes) the torrent is allowed to seed before stopped.
    ///
    /// This field is used to override the global setting for this specific torrent.
    ///
    /// - `-2` means the global limit should be used. `max_seeding_time`
    ///   will have the global setting set.
    /// - `-1` means no limit.
    pub seeding_time_limit: i32,
    /// The maximum amount of time (minutes) the torrent is allowed to seed while being inactive before stopped.
    ///
    /// This field is used to override the global setting for this specific torrent.
    ///
    /// - `-2` means the global limit should be used. `max_inactive_seeding_time`
    ///   will have the global setting set.
    /// - `-1` means no limit.
    pub inactive_seeding_time_limit: i32,
    #[cfg(feature = "qBittorrent-5_3")]
    // https://github.com/qbittorrent/qBittorrent/pull/24043
    pub share_limits_mode: ShareLimitMode,
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22989
    pub share_limit_action: ShareLimitAction,
    /// Time (Unix Epoch) when this torrent was last seen complete
    pub seen_complete: i64,
    /// Whether this torrent is managed by Automatic Torrent Management
    pub auto_tmm: bool,
    /// Total active time (seconds)
    pub time_active: i64,
    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i64,
    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: i64,
    /// Percentage of file pieces currently available
    pub availability: f64,
    /// Time until the next tracker reannounce
    pub reannounce: i64,
    /// Torrent comment metadata form the `.torrent` file
    pub comment: String,
}

/// Represents a map of torrents, where the key of the `HashMap` is the
/// torrent's hash and the value is the corresponding `Torrent` object.
///
/// This struct is a wrapper around a `HashMap` to provide additional
/// custom deserialization. It will insert the key into the hash filed on
/// the `Torrent` when deserialized
///
/// Its not mean to be used direactly and only as a deserialization object.
///
/// The `TorrentsMap` struct also implements the `Deref` trait, allowing you
/// to use it as if it were a `HashMap` directly.
#[derive(Debug, Serialize, Clone, Default, PartialEq)]
pub struct TorrentsMap(pub HashMap<String, Torrent>);

impl Deref for TorrentsMap {
    type Target = HashMap<String, Torrent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for TorrentsMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TorrentMapVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
struct TorrentMapVisitor;

impl<'de> Visitor<'de> for TorrentMapVisitor {
    type Value = TorrentsMap;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of torrent infohashes to torrent objects")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        #[derive(Deserialize)]
        struct TmpTorrent {
            infohash_v1: String,
            infohash_v2: String,
            name: String,

            has_metadata: bool,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            created_by: String,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            creation_date: i64,
            private: Option<bool>,
            total_size: i64,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            #[serde(rename = "pieces_num")]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            pieces_count: i32,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            piece_size: i64,

            magnet_uri: String,
            size: i64,
            progress: f64,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            total_wasted: i64,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            pieces_have: i32,
            #[serde(rename = "dlspeed")]
            download_speed: i32,
            #[serde(rename = "upspeed")]
            upload_speed: i32,
            #[serde(rename = "priority")]
            queue_position: i32,
            num_seeds: i32,
            num_complete: i32,
            num_leechs: i32,
            num_incomplete: i32,

            state: TorrentState,
            eta: i64,
            #[serde(rename = "seq_dl")]
            sequential_download: bool,
            #[serde(rename = "f_l_piece_prio")]
            first_last_piece_prio: bool,

            category: String,
            #[serde(deserialize_with = "deserializers::string_to_vec_comma_separated")]
            #[serde(serialize_with = "serializers::vec_to_string_comma_separated")]
            tags: Vec<String>,
            super_seeding: bool,
            force_start: bool,
            save_path: String,
            download_path: String,
            content_path: String,
            root_path: String,
            added_on: i64,
            completion_on: i64,
            tracker: String,
            trackers_count: i32,
            #[serde(rename = "dl_limit")]
            download_limit: i32,
            #[serde(rename = "up_limit")]
            upload_limit: i32,
            downloaded: i64,
            uploaded: i64,
            downloaded_session: i64,
            uploaded_session: i64,
            #[serde(deserialize_with = "deserializers::from_null_to_default")]
            amount_left: i64,
            completed: i64,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            connections_count: i32,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22753
            connections_limit: i32,
            max_ratio: f64,
            max_seeding_time: i32,
            max_inactive_seeding_time: i32,
            ratio: f64,
            ratio_limit: f64,
            popularity: f64,
            seeding_time_limit: i32,
            inactive_seeding_time_limit: i32,
            #[cfg(feature = "qBittorrent-5_3")]
            // https://github.com/qbittorrent/qBittorrent/pull/24043
            share_limits_mode: ShareLimitMode,
            #[cfg(not(feature = "qBittorrent-5_1"))]
            // https://github.com/qbittorrent/qBittorrent/pull/22989
            share_limit_action: ShareLimitAction,
            seen_complete: i64,
            auto_tmm: bool,
            time_active: i64,
            seeding_time: i64,
            last_activity: i64,
            availability: f64,
            reannounce: i64,
            comment: String,
        }

        while let Some(key) = access.next_key::<String>()? {
            let temp_torrent: TmpTorrent = access.next_value()?;

            let torrent = Torrent {
                hash: key.clone(),
                infohash_v1: temp_torrent.infohash_v1,
                infohash_v2: temp_torrent.infohash_v2,
                name: temp_torrent.name,

                has_metadata: temp_torrent.has_metadata,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                created_by: temp_torrent.created_by,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                creation_date: temp_torrent.creation_date,
                private: temp_torrent.private,
                total_size: temp_torrent.total_size,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                pieces_count: temp_torrent.pieces_count,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                piece_size: temp_torrent.piece_size,

                magnet_uri: temp_torrent.magnet_uri,
                size: temp_torrent.size,
                progress: temp_torrent.progress,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                total_wasted: temp_torrent.total_wasted,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                pieces_have: temp_torrent.pieces_have,
                download_speed: temp_torrent.download_speed,
                upload_speed: temp_torrent.upload_speed,
                queue_position: temp_torrent.queue_position,
                num_seeds: temp_torrent.num_seeds,
                num_complete: temp_torrent.num_complete,
                num_leechs: temp_torrent.num_leechs,
                num_incomplete: temp_torrent.num_incomplete,

                state: temp_torrent.state,
                eta: temp_torrent.eta,
                sequential_download: temp_torrent.sequential_download,
                first_last_piece_prio: temp_torrent.first_last_piece_prio,

                category: temp_torrent.category,
                tags: temp_torrent.tags,
                super_seeding: temp_torrent.super_seeding,
                force_start: temp_torrent.force_start,
                save_path: temp_torrent.save_path,
                download_path: temp_torrent.download_path,
                content_path: temp_torrent.content_path,
                root_path: temp_torrent.root_path,
                added_on: temp_torrent.added_on,
                completion_on: temp_torrent.completion_on,
                tracker: temp_torrent.tracker,
                trackers_count: temp_torrent.trackers_count,
                download_limit: temp_torrent.download_limit,
                upload_limit: temp_torrent.upload_limit,
                downloaded: temp_torrent.downloaded,
                uploaded: temp_torrent.uploaded,
                downloaded_session: temp_torrent.downloaded_session,
                uploaded_session: temp_torrent.uploaded_session,
                amount_left: temp_torrent.amount_left,
                completed: temp_torrent.completed,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                connections_count: temp_torrent.connections_count,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22753
                connections_limit: temp_torrent.connections_limit,
                max_ratio: temp_torrent.max_ratio,
                max_seeding_time: temp_torrent.max_seeding_time,
                max_inactive_seeding_time: temp_torrent.max_inactive_seeding_time,
                ratio: temp_torrent.ratio,
                ratio_limit: temp_torrent.ratio_limit,
                popularity: temp_torrent.popularity,
                seeding_time_limit: temp_torrent.seeding_time_limit,
                inactive_seeding_time_limit: temp_torrent.inactive_seeding_time_limit,
                #[cfg(feature = "qBittorrent-5_3")]
                // https://github.com/qbittorrent/qBittorrent/pull/24043
                share_limits_mode: temp_torrent.share_limits_mode,
                #[cfg(not(feature = "qBittorrent-5_1"))]
                // https://github.com/qbittorrent/qBittorrent/pull/22989
                share_limit_action: temp_torrent.share_limit_action,
                seen_complete: temp_torrent.seen_complete,
                auto_tmm: temp_torrent.auto_tmm,
                time_active: temp_torrent.time_active,
                seeding_time: temp_torrent.seeding_time,
                last_activity: temp_torrent.last_activity,
                availability: temp_torrent.availability,
                reannounce: temp_torrent.reannounce,
                comment: temp_torrent.comment,
            };
            map.insert(key, torrent);
        }

        Ok(TorrentsMap(map))
    }
}

/// Generic Torrent properties.
///
/// This struct provides some generic data and statistics about a torrent.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
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

/// Torrent tracker object
///
/// This struct contains detailed information about a tracker.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Tracker {
    /// Tracker url
    pub url: String,
    /// Tracker status.
    pub status: TrackerStatus,
    /// Tracker priority tier. Lower tier trackers are tried before higher
    /// tiers. Tier numbers are valid when `>= 0`, `< 0` is used as placeholder
    /// when `tier` does not exist for special entries (such as DHT).
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

/// Torrent tracker status
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum TrackerStatus {
    /// Tracker is disabled (used for DHT, PeX, and LSD)
    #[default]
    Disabled = 0,
    /// Tracker has not been contacted yet
    NotContacted = 1,
    /// Tracker has been contacted and is working
    Working = 2,
    /// Tracker is updating
    Updating = 3,
    /// Tracker has been contacted, but it is not working (or doesn't send proper replies)
    NotWorking = 4,
}

/// Web seed for torrent
///
/// Link to torrent that allows the client to download files directly.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct WebSeed {
    /// Web seed URL
    pub url: String,
}

/// Torrent file/content.
///
/// This struct provides detailed information about individual files within a torrent,
/// including their index, name, size, progress, priority, and more.
///
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
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
    /// Is file seeding / completed.
    pub is_seed: Option<bool>,
    /// The first number is the starting piece index and the second number is the ending piece index (inclusive)
    pub piece_range: Vec<i64>,
    /// Percentage of file pieces currently available (percentage/100)
    pub availability: f64,
}

/// File priority enum
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum FilePriority {
    /// Do not download
    DoNotDownload = 0,
    /// Normal priority
    #[default]
    Normal = 1,
    /// High priority
    High = 6,
    /// Maximal priority
    Maximal = 7,
}

/// Pices state
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum PiecesState {
    /// The piece has not yet been downloaded
    #[default]
    NotDownloaded = 0,
    /// The piece is in the progress of being downloaded
    Downloading = 1,
    /// The piece has been downloaded.
    Downloaded = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use serde_json::{Number, Value, json};

    mod serialization {
        use super::*;

        #[test]
        fn test_serialize_torrent() {
            let mut torrent = Torrent::default();
            torrent.added_on = 6969;
            torrent.name = "serialization.txt".to_string();
            torrent.hash = "aaaaaaaaaaaaaaaaaaaa".to_string();

            let res = serde_json::to_string(&torrent);
            assert!(res.is_ok());

            let data = res.unwrap();
            assert!(data.contains("\"name\":\"serialization.txt\""));
            assert!(data.contains("\"added_on\":6969"));
            assert!(data.contains("\"hash\":\"aaaaaaaaaaaaaaaaaaaa\""));
        }
    }

    mod deserialization {
        use super::*;

        fn get_base_json() -> serde_json::Value {
            #[allow(unused_mut)]
            let mut base = json!({
                "hash": "ffffffffffffffffffffffffffffffffffffffff",
                "infohash_v1": "ffffffffffffffffffffffffffffffffffffffff",
                "infohash_v2": "",
                "name": "file.pdf",
                "has_metadata": true,
                "private": false,
                "total_size": 702545920,
                "magnet_uri": "magnet:?xt=urn:btih:ffffffffffffffffffffffffffffffffffffffff",
                "size": 702545920,
                "progress": 0.4545,
                "dlspeed": 0,
                "upspeed": 0,
                "priority": 13,
                "num_seeds": 5,
                "num_complete": 3,
                "num_leechs": 4,
                "num_incomplete": 56,
                "state": "stoppedDL",
                "eta": 6000,
                "seq_dl": false,
                "f_l_piece_prio": false,
                "category": "cat1",
                "tags": "tag1,tag2",
                "super_seeding": false,
                "force_start": false,
                "save_path": "/downloads",
                "download_path": "/path/to/downloads/",
                "content_path": "/path/to/downloads/file.pdf",
                "root_path": "/root",
                "added_on": 1000,
                "completion_on": -1,
                "tracker": "http://file:6969/announce",
                "trackers_count": 1,
                "dl_limit": 0,
                "up_limit": 0,
                "downloaded": 198550558,
                "uploaded": 0,
                "downloaded_session": 0,
                "uploaded_session": 0,
                "amount_left": 2000,
                "completed": 3000,
                "max_ratio": 0.25,
                "max_seeding_time": -1,
                "max_inactive_seeding_time": -1,
                "ratio": 0.1,
                "ratio_limit": 0.25,
                "popularity": 0.7,
                "seeding_time_limit": -1,
                "inactive_seeding_time_limit": -1,
                "seen_complete": 7000,
                "auto_tmm": false,
                "time_active": 150,
                "seeding_time": 5000,
                "last_activity": 1781720956,
                "availability": 0.3,
                "reannounce": 67,
                "comment": "This is a comment",
            });

            #[cfg(not(feature = "qBittorrent-5_1"))]
            {
                base["created_by"] = Value::String("Billy".to_string());
                base["creation_date"] = Value::Number(Number::from(-1));
                base["pieces_num"] = Value::Number(Number::from(2680));
                base["piece_size"] = Value::Number(Number::from(262144));
                base["total_wasted"] = Value::Number(Number::from(0));
                base["pieces_have"] = Value::Number(Number::from(738));
                base["connections_count"] = Value::Number(Number::from(2));
                base["connections_limit"] = Value::Number(Number::from(100));
                base["share_limit_action"] = Value::String("EnableSuperSeeding".to_string());
            }

            #[cfg(feature = "qBittorrent-5_3")]
            {
                base["share_limits_mode"] = Value::String("MatchAny".to_string());
            }

            base
        }

        #[test]
        fn test_deserialize_torrent() {
            let base = get_base_json();

            let res: Result<Torrent, serde_json::Error> = serde_json::from_value(base);
            assert!(res.is_ok());
            let torrent = res.unwrap();
            assert_eq!(
                torrent.hash,
                "ffffffffffffffffffffffffffffffffffffffff".to_string()
            );
            assert_eq!(
                torrent.infohash_v1,
                "ffffffffffffffffffffffffffffffffffffffff".to_string()
            );
            assert_eq!(torrent.infohash_v2, "".to_string());
            assert_eq!(torrent.name, "file.pdf".to_string());
        }
    }
}
