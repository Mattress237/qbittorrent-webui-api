use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Log item data object
///
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/logger.h
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct LogItem {
    /// ID of the message
    pub id: i32,
    /// Type of the message
    #[serde(rename = "type")]
    pub log_type: LogType,
    /// Seconds since epoch
    ///
    /// (Note: switched from milliseconds to seconds in v4.5.0)
    pub timestamp: i64,
    /// Text of the message
    pub message: String,
}

/// Peer log item data object
///
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/logger.h
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct LogPeers {
    /// ID of the peer
    pub id: i32,
    /// Whether or not the peer was blocked
    pub blocked: bool,
    /// Seconds since epoch
    pub timestamp: i64,
    /// IP of the peer
    pub ip: String,
    /// Reason of the block
    pub reason: String,
}

/// Log types
///
/// Filter log types by severity levels
///
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/logger.h
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(i8)]
pub enum LogType {
    /// Include all log types
    #[default]
    All = -1,
    /// Include normal messages
    Normal = 1,
    /// Include Information messages
    Info = 2,
    /// Include Warning messages
    Warning = 4,
    /// Include Critical messages
    Critical = 8,
}

impl Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogType::All => "all",
                LogType::Normal => "normal",
                LogType::Info => "info",
                LogType::Warning => "warning",
                LogType::Critical => "critical",
            }
        )
    }
}
