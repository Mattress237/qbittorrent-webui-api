use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::models::ContentLayout;

/// Torrent List/info parameter object
#[derive(Debug, Default, Builder, Clone)]
pub struct TorrentListParams {
    /// Filter torrent list by state. Allowed state filters: TorrentState
    #[builder(setter(strip_option), default)]
    pub filter: Option<FilterTorrentState>,
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

/// Possible Torrent states that can be filtered.
#[derive(Debug, Clone)]
pub enum FilterTorrentState {
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

/// Possible states that any given torrent can be in at a time.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TorrentState {
    /// Some error occurred, applies to paused torrents
    #[serde(rename = "error")]
    Error,
    /// Torrent data files is missing
    #[serde(rename = "missingFiles")]
    MissingFiles,
    /// Torrent is moving to another location
    #[serde(rename = "moving")]
    Moving,
    /// Unknown status
    #[serde(rename = "unknown")]
    Unknown,
    /// Torrent is allocating disk space for download
    #[serde(rename = "allocating")]
    Allocating,
    /// Checking resume data on qBt startup
    #[serde(rename = "checkingResumeData")]
    CheckingResumeData,

    /// Torrent is being seeded and data is being transferred
    #[serde(rename = "uploading")]
    Uploading,
    /// Renamed from paused version in webUI API v2.11.0
    /// Torrent is stopped and has finished downloading
    #[serde(rename = "stoppedUP")]
    StoppedUploading,
    /// Queuing is enabled and torrent is queued for upload
    #[serde(rename = "queuedUP")]
    QueuedUploading,
    /// Torrent is being seeded, but no connection were made
    #[serde(rename = "stalledUP")]
    StalledUploading,
    /// Torrent has finished downloading and is being checked
    #[serde(rename = "checkingUP")]
    CheckingUploading,
    /// Torrent is forced to uploading and ignore queue limit
    #[serde(rename = "forcedUP")]
    ForcedUploading,

    /// Torrent is being downloaded and data is being transferred
    #[serde(rename = "downloading")]
    Downloading,
    /// Torrent has just started downloading and is fetching metadata
    #[serde(rename = "metaDL")]
    MetadataDownloading,
    /// Torrent has just started downloading and is fetching metadata. Queue limit is being ignored
    /// Officiall undocumented
    #[serde(rename = "forcedMetaDL")]
    ForcedMetadataDownloading,
    /// Renamed from paused version in webUI API v2.11.0
    /// Torrent is stopped and has NOT finished downloading
    #[serde(rename = "stoppedDL")]
    StoppedDownloading,
    /// Queuing is enabled and torrent is queued for download
    #[serde(rename = "queuedDL")]
    QueuedDownloading,
    /// Torrent is being downloaded, but no connection were made
    #[serde(rename = "stalledDL")]
    StalledDownloading,
    /// Torrent has NOT finished downloading, and is being checked
    #[serde(rename = "checkingDL")]
    CheckingDownloading,
    /// Torrent is forced to downloading to ignore queue limit
    #[serde(rename = "forcedDL")]
    ForcedDownloading,
}

impl Default for TorrentState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<&str> for TorrentState {
    fn from(value: &str) -> Self {
        match value {
            "error" => Self::Error,
            "missingFiles" => Self::MissingFiles,
            "uploading" => Self::Uploading,
            "stoppedUP" => Self::StoppedUploading,
            "queuedUP" => Self::QueuedUploading,
            "stalledUP" => Self::StalledUploading,
            "checkingUP" => Self::CheckingUploading,
            "forcedUP" => Self::ForcedUploading,
            "allocating" => Self::Allocating,
            "downloading" => Self::Downloading,
            "stoppedDL" => Self::StoppedDownloading,
            "metaDL" => Self::MetadataDownloading,
            "queuedDL" => Self::QueuedDownloading,
            "stalledDL" => Self::StalledDownloading,
            "checkingDL" => Self::CheckingDownloading,
            "forcedDL" => Self::ForcedDownloading,
            "forcedMetaDL" => Self::ForcedMetadataDownloading,
            "checkingResumeData" => Self::CheckingResumeData,
            "moving" => Self::Moving,
            "unknown" => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}
impl From<String> for TorrentState {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl Debug for TorrentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    TorrentState::Error => "Error",
                    TorrentState::MissingFiles => "Missing Files",
                    TorrentState::Moving => "Moving",
                    TorrentState::Unknown => "Unknown",
                    TorrentState::Allocating => "Allocating",
                    TorrentState::CheckingResumeData => "Checking Resume Data",
                    TorrentState::Uploading => "Uploading",
                    TorrentState::StoppedUploading => "Stopped Uploading",
                    TorrentState::QueuedUploading => "Queued Uploading",
                    TorrentState::StalledUploading => "Stalled Uploading",
                    TorrentState::CheckingUploading => "Checking Uploading",
                    TorrentState::ForcedUploading => "Forced Uploading",
                    TorrentState::Downloading => "Downloading",
                    TorrentState::MetadataDownloading => "Metadata Downloading",
                    TorrentState::ForcedMetadataDownloading => "Forced Metadata Downloading",
                    TorrentState::StoppedDownloading => "Stopped Downloading",
                    TorrentState::QueuedDownloading => "Queued Downloading",
                    TorrentState::StalledDownloading => "Stalled Downloading",
                    TorrentState::CheckingDownloading => "Checking Downloading",
                    TorrentState::ForcedDownloading => "Forced Downloading",
                }
            } else {
                match self {
                    TorrentState::Error => "error",
                    TorrentState::MissingFiles => "missingFiles",
                    TorrentState::Moving => "moving",
                    TorrentState::Unknown => "unknown",
                    TorrentState::Uploading => "uploading",
                    TorrentState::StoppedUploading => "stoppedUP",
                    TorrentState::QueuedUploading => "queuedUP",
                    TorrentState::StalledUploading => "stalledUP",
                    TorrentState::CheckingUploading => "checkingUP",
                    TorrentState::ForcedUploading => "forcedUP",
                    TorrentState::Allocating => "allocating",
                    TorrentState::Downloading => "downloading",
                    TorrentState::StoppedDownloading => "stoppedDL",
                    TorrentState::MetadataDownloading => "metaDL",
                    TorrentState::QueuedDownloading => "queuedDL",
                    TorrentState::StalledDownloading => "stalledDL",
                    TorrentState::CheckingDownloading => "checkingDL",
                    TorrentState::ForcedDownloading => "forcedDL",
                    TorrentState::ForcedMetadataDownloading => "forcedMetaDL",
                    TorrentState::CheckingResumeData => "checkingResumeData",
                }
            }
        )
    }
}

impl TorrentState {
    /// Returns true if the torrent has been paused.
    pub fn is_stopped(&self) -> bool {
        *self == Self::StoppedUploading || *self == Self::StoppedDownloading
    }
    /// Returns true if the torrent is waiting for peers to either download / upload
    pub fn is_stalled(&self) -> bool {
        *self == Self::StalledUploading || *self == Self::StalledDownloading
    }
    /// Returns true if the torrent is in the queue (queue must be enabled)
    pub fn is_queued(&self) -> bool {
        *self == Self::QueuedUploading || *self == Self::QueuedDownloading
    }
    /// Returns true if the torrent is currently being checked
    pub fn is_checking(&self) -> bool {
        *self == Self::CheckingUploading
            || *self == Self::CheckingDownloading
            || *self == Self::CheckingResumeData
    }
    /// Returns true if the torrent was forced to do something (bypassing the queue)
    pub fn is_forced(&self) -> bool {
        *self == Self::ForcedUploading
            || *self == Self::ForcedDownloading
            || *self == Self::ForcedMetadataDownloading
    }
    /// Returns true if the torrent is in any of the "Uploading" states
    pub fn is_uploading(&self) -> bool {
        *self == Self::Uploading
            || *self == Self::ForcedUploading
            || *self == Self::QueuedUploading
            || *self == Self::StalledUploading
            || *self == Self::StoppedUploading
            || *self == Self::CheckingUploading
    }
    /// Returns true if the torrent is in any of the "Downloading" states
    pub fn is_downloading(&self) -> bool {
        *self == Self::Downloading
            || *self == Self::ForcedDownloading
            || *self == Self::ForcedMetadataDownloading
            || *self == Self::QueuedDownloading
            || *self == Self::StalledDownloading
            || *self == Self::StoppedDownloading
            || *self == Self::CheckingDownloading
            || *self == Self::MetadataDownloading
    }
    /// If an error has occurred within the torrent.
    pub fn is_errored(&self) -> bool {
        *self == Self::Error || *self == Self::MissingFiles
    }
}

/// Torrent sort fields
#[derive(Debug, Clone)]
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
                Self::AddedOn => String::from("added_on"),
                Self::AmountLeft => String::from("amount_left"),
                Self::AutoTmm => String::from("auto_tmm"),
                Self::Availability => String::from("availability"),
                Self::Category => String::from("category"),
                Self::Completed => String::from("completed"),
                Self::CompletionOn => String::from("completion_on"),
                Self::ContentPath => String::from("content_path"),
                Self::DlLimit => String::from("dl_limit"),
                Self::Dlspeed => String::from("dlspeed"),
                Self::Downloaded => String::from("downloaded"),
                Self::DownloadedSession => String::from("downloaded_session"),
                Self::Eta => String::from("eta"),
                Self::FLPiecePrio => String::from("f_l_piece_prio"),
                Self::ForceStart => String::from("force_start"),
                Self::Hash => String::from("hash"),
                Self::Private => String::from("private"),
                Self::LastActivity => String::from("last_activity"),
                Self::MagnetUri => String::from("magnet_uri"),
                Self::MaxRatio => String::from("max_ratio"),
                Self::MaxSeedingTime => String::from("max_seeding_time"),
                Self::Name => String::from("name"),
                Self::NumComplete => String::from("num_complete"),
                Self::NumIncomplete => String::from("num_incomplete"),
                Self::NumLeechs => String::from("num_leechs"),
                Self::NumSeeds => String::from("num_seeds"),
                Self::Priority => String::from("priority"),
                Self::Progress => String::from("progress"),
                Self::Ratio => String::from("ratio"),
                Self::RatioLimit => String::from("ratio_limit"),
                Self::Reannounce => String::from("reannounce"),
                Self::SavePath => String::from("save_path"),
                Self::SeedingTime => String::from("seeding_time"),
                Self::SeedingTimeLimit => String::from("seeding_time_limit"),
                Self::SeenComplete => String::from("seen_complete"),
                Self::SeqDl => String::from("seq_dl"),
                Self::Size => String::from("size"),
                Self::State => String::from("state"),
                Self::SuperSeeding => String::from("super_seeding"),
                Self::Tags => String::from("tags"),
                Self::TimeActive => String::from("time_active"),
                Self::TotalSize => String::from("total_size"),
                Self::Tracker => String::from("tracker"),
                Self::UpLimit => String::from("up_limit"),
                Self::Uploaded => String::from("uploaded"),
                Self::UploadedSession => String::from("uploaded_session"),
                Self::Upspeed => String::from("upspeed"),
            }
        )
    }
}

/// Add torrent parameter object
#[derive(Debug, Default, Builder, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TorrentFile {
    pub filename: String,
    pub data: Vec<u8>,
}
