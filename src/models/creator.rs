use std::fmt::{Debug, Display};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// The format of the torrent.
///
/// See [torrent format hybrid v1 and v2](https://www.reddit.com/r/qBittorrent/comments/uiwchy/torrent_format_hybrid_v1_and_v2/) for more information
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum TorrentFormat {
    /// Old version, uses SHA-1 for hashing.
    #[serde(rename = "v1")]
    V1,
    /// New version, uses SHA-256 for hashing
    #[serde(rename = "v2")]
    V2,
    /// Attempts to work with both v1 and v2 torrents.
    #[serde(rename = "hybrid")]
    Hybrid,
}

impl Default for TorrentFormat {
    fn default() -> Self {
        Self::Hybrid
    }
}

impl Display for TorrentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TorrentFormat::V1 => "v1",
                TorrentFormat::V2 => "v2",
                TorrentFormat::Hybrid => "hybrid",
            }
        )
    }
}

/// Everything required to create a new torrent.
#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Builder,
)]
pub struct TorrentCreator {
    /// Source file (or directory) of current torrent. Must be a absolute path
    #[builder(setter(into))]
    pub source_path: String,
    /// Format of the torrent.
    #[builder(setter(into, strip_option), default)]
    pub format: Option<TorrentFormat>,
    /// How big a piece of the file is. (in Bytes). 0 = auto.
    /// Note: If piece size is too big this can cause the torrent to fail to be added.
    #[builder(setter(into, strip_option), default)]
    pub piece_size: Option<TorrentPieceSize>,
    /// Should optimize alignment
    #[builder(default = Some(false))]
    pub optimize_alignment: Option<bool>,
    /// Size limit for padding files
    ///
    /// Used with other clients that are not `LibTorrent2`, shouldn't need to be
    /// changed unless the client is different.
    #[builder(setter(into, strip_option), default = Some(-1))]
    pub padded_file_size_limit: Option<i64>,
    /// Is the torrent private or not? (Won't distrubte on DHT network if private.)
    #[builder(setter(into, strip_option), default)]
    pub private: Option<bool>,
    /// To start seeding the torrent as soon as the file is created.
    #[builder(setter(into, strip_option), default)]
    pub start_seeding: Option<bool>,
    /// The path to save the generated `.torrent` file to.
    #[builder(setter(into, strip_option), default)]
    pub torrent_file_path: Option<String>,
    /// List of trackers
    #[builder(setter(into, strip_option), default)]
    pub trackers: Option<Vec<String>>,
    /// List of url seeds
    #[builder(setter(into, strip_option), default)]
    pub url_seeds: Option<Vec<String>>,
    /// Source metadata field.
    ///
    /// Used for cross-seeding by some private trackers
    #[builder(setter(into, strip_option), default)]
    pub source: Option<String>,
    /// A comment to attach to the torrent.
    #[builder(setter(into, strip_option), default)]
    pub comment: Option<String>,
}

/// A wrapper of a string, used to store the task_id just created.
///
/// Usages should be like a normal string.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TorrentCreatorTask {
    /// The task id related to the torrent just created
    #[serde(rename = "taskID")]
    pub task_id: String,
}

impl From<String> for TorrentCreatorTask {
    fn from(value: String) -> Self {
        Self { task_id: value }
    }
}

impl Display for TorrentCreatorTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.task_id)
    }
}

impl PartialEq<TorrentCreatorTask> for String {
    fn eq(&self, other: &TorrentCreatorTask) -> bool {
        *self == other.task_id
    }
}

impl PartialEq<TorrentCreatorTask> for &str {
    fn eq(&self, other: &TorrentCreatorTask) -> bool {
        *self == other.task_id
    }
}

impl PartialEq<TorrentCreatorTask> for &String {
    fn eq(&self, other: &TorrentCreatorTask) -> bool {
        **self == other.task_id
    }
}

/// How big the chunks of pieces can be in Bytes
///
/// Custom values are allowed, however pre-made values have also been included.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TorrentPieceSize(pub u64);

impl Default for TorrentPieceSize {
    fn default() -> Self {
        Self::auto()
    }
}

impl From<u64> for TorrentPieceSize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TorrentPieceSize {
    /// Leave it up to qbittorrent to decided.
    pub fn auto() -> Self {
        Self(0)
    }
    /// Pieces in 16 KibiBytes (16 * 1024)
    pub fn k16() -> Self {
        Self(16384)
    }
    /// Pieces in 32 KibiBytes (32 * 1024)
    pub fn k32() -> Self {
        Self(32768)
    }
    /// Pieces in 64 KibiBytes (64 * 1024)
    pub fn k64() -> Self {
        Self(65536)
    }
    /// Pieces in 128 KibiBytes (128 * 1024)
    pub fn k128() -> Self {
        Self(131072)
    }
    /// Pieces in 256 KibiBytes (256 * 1024)
    pub fn k256() -> Self {
        Self(262144)
    }
    /// Pieces in 512 KibiBytes (512 * 1024)
    pub fn k512() -> Self {
        Self(524288)
    }
    /// Pieces in 1 MebiBytes (1 * 1024 * 1024)
    pub fn m1() -> Self {
        Self(1048576)
    }
    /// Pieces in 2 MebiBytes (2 * 1024 * 1024)
    pub fn m2() -> Self {
        Self(2097152)
    }
    /// Pieces in 4 MebiBytes (4 * 1024 * 1024)
    pub fn m4() -> Self {
        Self(4194304)
    }
    /// Pieces in 8 MebiBytes (8 * 1024 * 1024)
    pub fn m8() -> Self {
        Self(8388608)
    }
    /// Pieces in 16 MebiBytes (16 * 1024 * 1024)
    pub fn m16() -> Self {
        Self(16777216)
    }
    /// Pieces in 32 MebiBytes (32 * 1024 * 1024)
    pub fn m32() -> Self {
        Self(33554432)
    }
    /// Pieces in 64 MebiBytes (64 * 1024 * 1024)
    pub fn m64() -> Self {
        Self(67108864)
    }
    /// Pieces in 128 MebiBytes (128 * 1024 * 1024)
    pub fn m128() -> Self {
        Self(134217728)
    }
    /// Pieces in 256 MebiBytes (256 * 1024 * 1024)
    pub fn m256() -> Self {
        Self(268435456)
    }
}

/// The current status of the task
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskStatus {
    /// The task failed to complete, see `error_message` of `TorrentCreatorTaskStatus` for the reason why
    Failed,
    /// The task is in the queue waiting to be processed
    Queued,
    /// The task is current being processed
    Running,
    /// The task has finished processing successfully.
    Finished,
}

/// Information about a created torrent
///
/// Depending on the TaskStatus depends on which fields may or may not be included.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentCreatorTaskStatus {
    /// The format of the torrent.
    pub format: Option<TorrentFormat>,
    /// An error message as to why the torrent failed to be created
    pub error_message: Option<String>,
    /// The comment attached to the torrent
    pub comment: Option<String>,
    /// Should optimize alignment
    pub optimize_alignment: Option<bool>,
    /// Size limit for padding files
    ///
    /// Used with other clients that are not `LibTorrent2`, shouldn't need to be
    /// changed unless the client is different.
    pub padded_file_size_limit: Option<i64>,
    /// How big the pieces of the torrent is.
    pub piece_size: TorrentPieceSize,
    /// Is the torrent private
    pub private: bool,
    /// The path to the file / folder the torrent is uploading
    pub source_path: String,
    /// The current status of the task
    pub status: TaskStatus,
    /// The task id of the torrent
    #[serde(rename = "taskID")]
    pub task_id: String,
    /// The time this task got added
    pub time_added: String,
    /// The time this task finished
    pub time_finished: Option<String>,
    /// The time this task started being processed
    pub time_started: Option<String>,
    /// Source metadata field.
    ///
    /// Used for cross-seeding by some private trackers
    pub source: Option<String>,
    /// List of trackers
    pub trackers: Vec<String>,
    /// List of URL seeds
    pub url_seeds: Vec<String>,
}
