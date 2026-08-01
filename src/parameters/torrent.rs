// use std::fmt::Display;

use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Torrent List/info parameter object
#[derive(Debug, Default, Builder, Clone, Deserialize, Serialize, PartialEq)]
pub struct TorrentListParams {
    /// Filter torrent list by state. See FilterTorrentState for the allowed filters.
    #[builder(setter(strip_option), default = None)]
    pub filter: Option<FilterTorrentState>,
    /// Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category"). Remember to URL-encode the category name. For example, `My category` becomes `My%20category`
    #[builder(setter(into, strip_option), default = None)]
    pub category: Option<String>,
    /// Get torrents with the given tag (empty string means "without tag"; no "tag" parameter means "any tag"). Remember to URL-encode the category name. For example, `My tag` becomes `My%20tag`
    #[builder(setter(into, strip_option), default = None)]
    pub tag: Option<String>,
    /// Sort torrents by given key. They can be sorted using any field of the response's JSON array (see `TorrentSort`) as the sort key.
    #[builder(setter(strip_option), default = None)]
    pub sort: Option<TorrentSort>,
    /// Enable reverse sorting. Defaults to `false` if not set
    #[builder(setter(strip_option), default = None)]
    pub reverse: Option<bool>,
    /// Limit the number of torrents returned
    #[builder(setter(into, strip_option), default = None)]
    pub limit: Option<i32>,
    /// Set offset (if less than 0, offset from end)
    #[builder(setter(into, strip_option), default = None)]
    pub offset: Option<i32>,
    /// Filter by hashes.
    #[builder(setter(into, strip_option), default = None)]
    pub hashes: Option<Vec<String>>,
    /// Filter by private status.
    #[builder(setter(strip_option), default = None)]
    pub is_private: Option<bool>,
    /// Include torrent files in the response.
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // https://github.com/qbittorrent/qBittorrent/pull/22750
    #[builder(setter(strip_option), default = None)]
    pub include_files: Option<bool>,
    /// Include torrent trackers in the response.
    #[builder(setter(strip_option), default = None)]
    pub include_trackers: Option<bool>,
}

/// Possible Torrent states that can be filtered.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum FilterTorrentState {
    /// Every filter
    All,
    /// Only torrents which are downloading
    Downloading,
    /// Only torrents which are seeding
    Seeding,
    /// Only torrents which are completed
    Completed,
    /// Only torrents which are running (same as active, or checking disk files)
    Running,
    /// Only torrents which are stopped
    Stopped,
    /// Only torrents which are active (downloading, seeding, metadata, etc)
    Active,
    /// Only torrents which are inactive (stopped, stalled, errored)
    Inactive,
    /// Only torrents which are stalled (no data transfer, coverse both `StalledUploading` and `StalledDownloading`)
    Stalled,
    /// Only torrents which are stalled uploading (not seeding any data)
    StalledUploading,
    /// Only torrents which are stalled downloading (not receiving any data)
    StalledDownloading,
    /// Only torrents which are checking disk files
    Checking,
    /// Only torrents which are moving
    Moving,
    /// Only torrents which are errored. (Missing files, failed to write, etc)
    Errored,
}

impl Display for FilterTorrentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => String::from("all"),
                Self::Downloading => String::from("downloading"),
                Self::Seeding => String::from("seeding"),
                Self::Completed => String::from("completed"),
                Self::Running => String::from("running"),
                Self::Stopped => String::from("stopped"),
                Self::Active => String::from("active"),
                Self::Inactive => String::from("inactive"),
                Self::Stalled => String::from("stalled"),
                Self::StalledUploading => String::from("stalled_uploading"),
                Self::StalledDownloading => String::from("stalled_downloading"),
                Self::Checking => String::from("checking"),
                Self::Moving => String::from("moving"),
                Self::Errored => String::from("errored"),
            }
        )
    }
}

/// Torrent sort fields
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum TorrentSort {
    /// Time when the torrent was added to the client
    AddedOn,
    /// Amount of data left to download
    AmountLeft,
    /// Whether this torrent is managed by Automatic Torrent Management
    AutoTmm,
    /// Percentage of file pieces currently available
    Availability,
    /// Category of the torrent
    Category,
    /// Amount of transfer data completed
    Completed,
    /// Time when the torrent completed
    CompletionOn,
    /// Torrent content path
    ContentPath,
    /// Torrent download speed limit.
    DlLimit,
    /// Torrent download speed
    Dlspeed,
    /// Amount of data downloaded
    Downloaded,
    /// Amount of data downloaded this session
    DownloadedSession,
    /// Torrent ETA
    Eta,
    /// First last piece are prioritized
    FLPiecePrio,
    /// Force start is enabled for this torrent
    ForceStart,
    /// Torrent hash
    Hash,
    /// True if torrent is from a private tracker
    Private,
    /// Last time when a chunk was downloaded/uploaded
    LastActivity,
    /// Magnet URI corresponding to this torrent
    MagnetUri,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    MaxRatio,
    /// Maximum seeding time until torrent is stopped from seeding
    MaxSeedingTime,
    /// Torrent name
    Name,
    /// Number of seeds in the swarm
    NumComplete,
    /// Number of leechers in the swarm
    NumIncomplete,
    /// Number of leechers connected to
    NumLeechs,
    /// Number of seeds connected to
    NumSeeds,
    /// Torrent priority
    Priority,
    /// Torrent progress
    Progress,
    /// Torrent share ratio.
    Ratio,
    /// Maximum share ratio limit for the torrent
    RatioLimit,
    /// Time until the next tracker reannounce
    Reannounce,
    /// Path where this torrent's data is stored
    SavePath,
    /// Torrent elapsed time while complete
    SeedingTime,
    /// Torrent elapsed time while complete limit
    SeedingTimeLimit,
    /// Time when this torrent was last seen complete
    SeenComplete,
    /// True if sequential download is enabled
    SeqDl,
    /// Total size of files selected for download
    Size,
    /// Torrent state.
    State,
    /// Super seeding state
    SuperSeeding,
    /// Tag list of the torrent
    Tags,
    /// Total active time
    TimeActive,
    /// Total size of all file in this torrent. Including unselected ones
    TotalSize,
    /// The first tracker with working status. Empty string if no tracker is working.
    Tracker,
    /// Torrent upload speed limit
    UpLimit,
    /// Amount of data uploaded
    Uploaded,
    /// Amount of data uploaded this session
    UploadedSession,
    /// Torrent upload speed
    Upspeed,
}

impl Display for TorrentSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AddedOn => "added_on",
                Self::AmountLeft => "amount_left",
                Self::AutoTmm => "auto_tmm",
                Self::Availability => "availability",
                Self::Category => "category",
                Self::Completed => "completed",
                Self::CompletionOn => "completion_on",
                Self::ContentPath => "content_path",
                Self::DlLimit => "dl_limit",
                Self::Dlspeed => "dlspeed",
                Self::Downloaded => "downloaded",
                Self::DownloadedSession => "downloaded_session",
                Self::Eta => "eta",
                Self::FLPiecePrio => "f_l_piece_prio",
                Self::ForceStart => "force_start",
                Self::Hash => "hash",
                Self::Private => "private",
                Self::LastActivity => "last_activity",
                Self::MagnetUri => "magnet_uri",
                Self::MaxRatio => "max_ratio",
                Self::MaxSeedingTime => "max_seeding_time",
                Self::Name => "name",
                Self::NumComplete => "num_complete",
                Self::NumIncomplete => "num_incomplete",
                Self::NumLeechs => "num_leechs",
                Self::NumSeeds => "num_seeds",
                Self::Priority => "priority",
                Self::Progress => "progress",
                Self::Ratio => "ratio",
                Self::RatioLimit => "ratio_limit",
                Self::Reannounce => "reannounce",
                Self::SavePath => "save_path",
                Self::SeedingTime => "seeding_time",
                Self::SeedingTimeLimit => "seeding_time_limit",
                Self::SeenComplete => "seen_complete",
                Self::SeqDl => "seq_dl",
                Self::Size => "size",
                Self::State => "state",
                Self::SuperSeeding => "super_seeding",
                Self::Tags => "tags",
                Self::TimeActive => "time_active",
                Self::TotalSize => "total_size",
                Self::Tracker => "tracker",
                Self::UpLimit => "up_limit",
                Self::Uploaded => "uploaded",
                Self::UploadedSession => "uploaded_session",
                Self::Upspeed => "upspeed",
            }
        )
    }
}
