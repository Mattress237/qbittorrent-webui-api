use std::{collections::HashMap, fmt, ops::Deref};

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{MapAccess, Visitor},
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::parameters::TorrentState;

/// Torrent info response object
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Torrent {
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
    /// Torrent comment metadata form the `.torrent` file
    pub comment: String,
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
    /// TODO: Need to look into the purposes of the field.
    pub download_path: String,
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
    /// True if the torrent has metadata available
    pub has_metadata: bool,
    /// Torrent hash
    pub hash: String,
    /// TODO: need doc block
    /// I dont know if this is the right filed: https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-share-limit
    pub inactive_seeding_time_limit: i64,
    /// The SHA-1 hash of the torrent's info dictionary (used in BitTorrent v1).
    pub infohash_v1: String,
    ///  SHA-256 hash of the torrent's info dictionary (used in BitTorrent v2).
    pub infohash_v2: String,
    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: i64,
    /// TODO: need doc block
    /// I dont know if this is the right filed: https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-torrent-share-limit
    pub max_inactive_seeding_time: i64,
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
    /// Popularity of the torrent
    pub popularity: f64,
    /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    pub priority: i64,
    /// True if torrent is from a private tracker (added in 5.0.0)
    ///
    /// The value will be `None` if the torrent metadata is not available yet.
    /// See issue [#10](https://github.com/Mattress237/qbittorrent-webui-api/issues/10)
    ///
    /// NOTE: Documentation is incorrect. The field name is "private", not "isPrivate".
    pub private: Option<bool>,
    /// Torrent progress (percentage/100)
    pub progress: f32,
    /// Torrent share ratio. Max ratio value: 9999.
    pub ratio: f32,
    /// TODO (what is different from max_ratio?)
    pub ratio_limit: f32,
    /// Time until the next tracker reannounce
    pub reannounce: i64,
    /// TODO: need doc block
    pub root_path: String,
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
    /// State that the torrent is currently in.
    pub state: TorrentState,
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
    /// Totall count of trackers
    pub trackers_count: i64,
    /// Torrent upload speed limit (bytes/s). -1 if unlimited.
    pub up_limit: i64,
    /// Amount of data uploaded
    pub uploaded: i64,
    /// Amount of data uploaded this session
    pub uploaded_session: i64,
    /// Torrent upload speed (bytes/s)
    pub upspeed: i64,
}

#[derive(Debug, Serialize, Clone)]
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
            added_on: i64,
            amount_left: i64,
            auto_tmm: bool,
            availability: f64,
            category: String,
            comment: String,
            completed: i64,
            completion_on: i64,
            content_path: String,
            dl_limit: i64,
            dlspeed: i64,
            download_path: String,
            downloaded: i64,
            downloaded_session: i64,
            eta: i64,
            f_l_piece_prio: bool,
            force_start: bool,
            has_metadata: bool,
            inactive_seeding_time_limit: i64,
            infohash_v1: String,
            infohash_v2: String,
            last_activity: i64,
            max_inactive_seeding_time: i64,
            magnet_uri: String,
            max_ratio: f32,
            max_seeding_time: i64,
            name: String,
            num_complete: i64,
            num_incomplete: i64,
            num_leechs: i64,
            num_seeds: i64,
            popularity: f64,
            priority: i64,
            private: Option<bool>,
            progress: f32,
            ratio: f32,
            ratio_limit: f32,
            reannounce: i64,
            root_path: String,
            save_path: String,
            seeding_time: i64,
            seeding_time_limit: i64,
            seen_complete: i64,
            seq_dl: bool,
            size: i64,
            state: TorrentState,
            super_seeding: bool,
            tags: String,
            time_active: i64,
            total_size: i64,
            tracker: String,
            trackers_count: i64,
            up_limit: i64,
            uploaded: i64,
            uploaded_session: i64,
            upspeed: i64,
        }

        while let Some(key) = access.next_key::<String>()? {
            let temp_torrent: TmpTorrent = access.next_value()?;

            let torrent = Torrent {
                hash: key.clone(),
                added_on: temp_torrent.added_on,
                amount_left: temp_torrent.amount_left,
                auto_tmm: temp_torrent.auto_tmm,
                availability: temp_torrent.availability,
                category: temp_torrent.category,
                comment: temp_torrent.comment,
                completed: temp_torrent.completed,
                completion_on: temp_torrent.completion_on,
                content_path: temp_torrent.content_path,
                dl_limit: temp_torrent.dl_limit,
                dlspeed: temp_torrent.dlspeed,
                download_path: temp_torrent.download_path,
                downloaded: temp_torrent.downloaded,
                downloaded_session: temp_torrent.downloaded_session,
                eta: temp_torrent.eta,
                f_l_piece_prio: temp_torrent.f_l_piece_prio,
                force_start: temp_torrent.force_start,
                has_metadata: temp_torrent.has_metadata,
                inactive_seeding_time_limit: temp_torrent.inactive_seeding_time_limit,
                infohash_v1: temp_torrent.infohash_v1,
                infohash_v2: temp_torrent.infohash_v2,
                last_activity: temp_torrent.last_activity,
                max_inactive_seeding_time: temp_torrent.max_inactive_seeding_time,
                magnet_uri: temp_torrent.magnet_uri,
                max_ratio: temp_torrent.max_ratio,
                max_seeding_time: temp_torrent.max_seeding_time,
                name: temp_torrent.name,
                num_complete: temp_torrent.num_complete,
                num_incomplete: temp_torrent.num_incomplete,
                num_leechs: temp_torrent.num_leechs,
                num_seeds: temp_torrent.num_seeds,
                popularity: temp_torrent.popularity,
                priority: temp_torrent.priority,
                private: temp_torrent.private,
                progress: temp_torrent.progress,
                ratio: temp_torrent.ratio,
                ratio_limit: temp_torrent.ratio_limit,
                reannounce: temp_torrent.reannounce,
                root_path: temp_torrent.root_path,
                save_path: temp_torrent.save_path,
                seeding_time: temp_torrent.seeding_time,
                seeding_time_limit: temp_torrent.seeding_time_limit,
                seen_complete: temp_torrent.seen_complete,
                seq_dl: temp_torrent.seq_dl,
                size: temp_torrent.size,
                state: temp_torrent.state,
                super_seeding: temp_torrent.super_seeding,
                tags: temp_torrent.tags,
                time_active: temp_torrent.time_active,
                total_size: temp_torrent.total_size,
                tracker: temp_torrent.tracker,
                trackers_count: temp_torrent.trackers_count,
                up_limit: temp_torrent.up_limit,
                uploaded: temp_torrent.uploaded,
                uploaded_session: temp_torrent.uploaded_session,
                upspeed: temp_torrent.upspeed,
            };
            map.insert(key, torrent);
        }

        Ok(TorrentsMap(map))
    }
}

/// Generic torrent properties
#[derive(Debug, Deserialize, Serialize, Clone)]
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
#[derive(Debug, Deserialize, Serialize, Clone)]
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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebSeed {
    /// Web seed URL
    pub url: String,
}

/// Torrent file/content data object
#[derive(Debug, Deserialize, Serialize, Clone)]
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
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone)]
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
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone)]
#[repr(u8)]
pub enum PiecesState {
    NotDownloaded = 0,
    Downloading = 1,
    Downloaded = 2,
}
