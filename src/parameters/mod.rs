//!
//! This module provides the data structures and enums necessary for managing
//! parameters, states, and sorting options.
//!
//! The types defined here are used for serializing parameters to the QbitTorrent WebUI API.
//!

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

mod application;
mod creator;
mod torrent;

pub use application::*;
pub use creator::*;
pub use torrent::*;

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
