use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

/// The format of the torrent.
///
/// See [torrent format hybrid v1 and v2](https://www.reddit.com/r/qBittorrent/comments/uiwchy/torrent_format_hybrid_v1_and_v2/) for more information
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug, Default)]
pub enum TorrentFormat {
    /// Old version, uses SHA-1 for hashing.
    #[serde(rename = "v1")]
    V1,
    /// New version, uses SHA-256 for hashing
    #[serde(rename = "v2")]
    V2,
    /// Attempts to work with both v1 and v2 torrents.
    #[serde(rename = "hybrid")]
    #[default]
    Hybrid,
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

/// How big the chunks of pieces can be in Bytes
///
/// Custom values are allowed, however pre-made values have also been included.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TorrentPieceSize(pub i32);

impl Default for TorrentPieceSize {
    fn default() -> Self {
        Self::auto()
    }
}

impl From<i32> for TorrentPieceSize {
    fn from(value: i32) -> Self {
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
    /// The task failed to complete, see `error_message` of `TorrentCreatorTask` for the reason why
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
pub struct TorrentCreatorTask {
    /// The task id of the torrent
    #[serde(rename = "taskID")]
    pub task_id: String,
    /// The path to the file / folder the torrent is uploading
    pub source_path: String,
    /// How big the pieces of the torrent is.
    pub piece_size: TorrentPieceSize,
    // https://github.com/qbittorrent/qBittorrent/pull/24346
    #[cfg(feature = "qBittorrent-5_3")]
    /// Whether to ignore dotfiles when creating the torrent.
    pub ignore_dotfiles: bool,
    /// Is the torrent private
    pub private: bool,
    /// The time this task got added
    pub time_added: String,
    /// The format of the torrent.
    pub format: Option<TorrentFormat>,
    /// Should optimize alignment
    pub optimize_alignment: Option<bool>,
    /// Size limit for padding files
    ///
    /// Used with other clients that are not `LibTorrent2`, shouldn't need to be
    /// changed unless the client is different.
    pub padded_file_size_limit: Option<i32>,
    /// The current status of the task
    pub status: TaskStatus,
    /// The comment attached to the torrent
    pub comment: Option<String>,
    /// The path to the torrent file
    pub torrent_file_path: Option<String>,
    /// Source metadata field.
    ///
    /// Used for cross-seeding by some private trackers
    pub source: Option<String>,
    /// List of trackers
    pub trackers: Vec<String>,
    /// List of URL seeds
    pub url_seeds: Vec<String>,
    /// The time this task started being processed
    pub time_started: Option<String>,
    /// The time this task finished
    pub time_finished: Option<String>,
    /// An error message as to why the torrent failed to be created
    pub error_message: Option<String>,
    /// Progress of the task
    ///
    /// Only available when the task is in progress
    // Note: In the source code this typed as a `int`
    pub progress: Option<i32>,
}
