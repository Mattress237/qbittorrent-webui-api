use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::models::{TorrentFormat, TorrentPieceSize};
use crate::utilities::serializers::option_vec_to_string_pipe_separated;

/// Everything required to create a new torrent.
// https://github.com/qbittorrent/qBittorrent/blob/master/src/webui/api/torrentcreatorcontroller.cpp
#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Builder,
)]
pub struct CreateTorrent {
    // https://github.com/qbittorrent/qBittorrent/pull/24346
    #[cfg(feature = "qBittorrent-5_3")]
    /// Whether to ignore dotfiles when creating the torrent.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreDotfiles")]
    pub ignore_dotfiles: Option<bool>,
    /// Is the torrent private or not? (Won't distrubte on DHT network if private.)
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// Format of the torrent.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TorrentFormat>,
    /// Should optimize alignment
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "optimizeAlignment")]
    pub optimize_alignment: Option<bool>,
    /// Size limit for padding files
    ///
    /// Used with other clients that are not `LibTorrent2`, shouldn't need to be
    /// changed unless the client is different.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paddedFileSizeLimit")]
    pub padded_file_size_limit: Option<i32>,
    /// How big a piece of the file is. (in Bytes). 0 = auto.
    /// Note: If piece size is too big this can cause the torrent to fail to be added.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pieceSize")]
    pub piece_size: Option<TorrentPieceSize>,
    /// Source file (or directory) of current torrent. Must be a absolute path
    #[builder(setter(into))]
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    /// The path to save the generated `.torrent` file to.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "torrentFilePath")]
    pub torrent_file_path: Option<String>,
    /// A comment to attach to the torrent.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Source metadata field.
    ///
    /// Used for cross-seeding by some private trackers
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// List of trackers
    #[builder(setter(into, strip_option), default = None)]
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_vec_to_string_pipe_separated"
    )]
    pub trackers: Option<Vec<String>>,
    /// List of url seeds
    #[builder(setter(into, strip_option), default = None)]
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_vec_to_string_pipe_separated"
    )]
    #[serde(rename = "urlSeeds")]
    pub url_seeds: Option<Vec<String>>,

    /// To start seeding the torrent as soon as the file is created.
    #[builder(setter(into, strip_option), default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startSeeding")]
    pub start_seeding: Option<bool>,
}
