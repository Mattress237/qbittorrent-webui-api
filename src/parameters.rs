use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::models::ContentLayout;

/// Torrent List/info parameter object
#[derive(Debug, Default, Builder, Clone, Deserialize, Serialize, PartialEq)]
pub struct TorrentListParams {
    /// Filter torrent list by state. Allowed state filters: TorrentState
    #[builder(setter(strip_option), default)]
    pub filter: Option<TorrentState>,
    /// Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category"). Remember to URL-encode the category name. For example, `My category` becomes `My%20category`
    #[builder(setter(into, strip_option), default)]
    pub category: Option<String>,
    /// Get torrents with the given tag (empty string means "without tag"; no "tag" parameter means "any tag". Remember to URL-encode the category name. For example, `My tag` becomes `My%20tag`
    #[builder(setter(into, strip_option), default)]
    pub tag: Option<String>,
    /// Sort torrents by given key. They can be sorted using any field of the response's JSON array (which are documented below) as the sort key.
    #[builder(setter(strip_option), default)]
    pub sort: Option<TorrentSort>,
    /// Enable reverse sorting. Defaults to `false`
    #[builder(default)]
    pub reverse: bool,
    /// Limit the number of torrents returned
    #[builder(setter(into, strip_option), default)]
    pub limit: Option<i64>,
    /// Set offset (if less than 0, offset from end)
    #[builder(setter(into, strip_option), default)]
    pub offset: Option<i64>,
    /// Filter by hashes. Can contain multiple hashes separated by `|`
    #[builder(setter(into, strip_option), default)]
    pub hashes: Option<Vec<String>>,
}

/// Possible Torrent states
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum TorrentState {
    #[default]
    All,
    Downloading,
    Seeding,
    Completed,
    Stopped,
    Active,
    Inactive,
    Running,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored,
}

impl Display for TorrentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => String::from("all"),
                Self::Downloading => String::from("downloading"),
                Self::Seeding => String::from("seeding"),
                Self::Completed => String::from("completed"),
                Self::Stopped => String::from("stopped"),
                Self::Active => String::from("active"),
                Self::Inactive => String::from("inactive"),
                Self::Running => String::from("running"),
                Self::Stalled => String::from("stalled"),
                Self::StalledUploading => String::from("stalled_uploading"),
                Self::StalledDownloading => String::from("stalled_downloading"),
                Self::Errored => String::from("errored"),
            }
        )
    }
}

/// Torrent sort fields
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum TorrentSort {
    /// Time when the torrent was added to the client
    #[default]
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

/// Add torrent parameter object
#[derive(Debug, Default, Builder, Clone, Deserialize, Serialize, PartialEq)]
pub struct AddTorrent {
    /// A list of torrent files or magnet links to be added.
    ///
    /// This field is required and must contain at least one item.
    #[builder(setter(into))]
    pub torrents: AddTorrentType,
    /// Download folder
    #[builder(setter(into, strip_option), default)]
    pub savepath: Option<String>,
    /// Category for the torrent
    #[builder(setter(into, strip_option), default)]
    pub category: Option<String>,
    /// Tags for the torrent, split by `,`
    #[builder(setter(into, strip_option), default)]
    pub tags: Option<Vec<String>>,
    /// Skip hash checking. Possible values are `true`, `false` (default)
    #[builder(default)]
    pub skip_checking: bool,
    /// Add torrents in the paused state. Possible values are `true`, `false` (default)
    #[builder(default)]
    pub paused: bool,
    /// The torrent subfolder layout.
    #[builder(setter(into), default)]
    pub content_layout: ContentLayout,
    /// Rename torrent
    #[builder(setter(into, strip_option), default)]
    pub rename: Option<String>,
    /// Set torrent upload speed limit. Unit in bytes/second
    #[builder(setter(into, strip_option), default)]
    pub up_limit: Option<i64>,
    /// Set torrent download speed limit. Unit in bytes/second
    #[builder(setter(into, strip_option), default)]
    pub dl_limit: Option<i64>,
    /// Set torrent share ratio limit
    #[builder(setter(into, strip_option), default)]
    pub ratio_limit: Option<f32>,
    /// Set torrent seeding time limit. Unit in minutes
    #[builder(setter(into, strip_option), default)]
    pub seeding_time_limit: Option<i64>,
    /// Whether Automatic Torrent Management should be used
    #[builder(default)]
    pub auto_tmm: bool,
    /// Enable sequential download. Possible values are `true`, `false` (default)
    #[builder(default)]
    pub sequential_download: bool,
    /// Prioritize download first last piece. Possible values are `true`, `false` (default)
    #[builder(default)]
    pub first_last_piece_prio: bool,
}

impl AddTorrent {
    pub fn new() -> Self {
        // Although Self::default() could work here, this is done as semi boiler plate for future things if need be.
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum AddTorrentType {
    Links(Vec<String>),
    Files(Vec<TorrentFile>),
}

impl AddTorrentType {
    pub fn is_empty(&self) -> bool {
        match self {
            AddTorrentType::Links(items) => items.is_empty(),
            AddTorrentType::Files(items) => items.is_empty(),
        }
    }
}

impl From<Vec<String>> for AddTorrentType {
    fn from(value: Vec<String>) -> Self {
        Self::Links(value)
    }
}

impl From<Vec<TorrentFile>> for AddTorrentType {
    fn from(value: Vec<TorrentFile>) -> Self {
        Self::Files(value)
    }
}

impl Default for AddTorrentType {
    fn default() -> Self {
        AddTorrentType::Links(vec![])
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct TorrentFile {
    pub filename: String,
    pub data: Vec<u8>,
}
