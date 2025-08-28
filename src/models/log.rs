use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Log item data object
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct LogItem {
    /// ID of the message
    pub id: i64,
    /// Text of the message
    pub message: String,
    /// Seconds since epoch
    ///
    /// (Note: switched from milliseconds to seconds in v4.5.0)
    pub timestamp: i64,
    /// Type of the message
    #[serde(rename = "type")]
    pub log_type: LogType,
}

/// Log types
///
/// Log levels used by the logger
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum LogType {
    #[default]
    Normal = 1,
    Info = 2,
    Warning = 4,
    Critical = 8,
}

impl Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogType::Normal => "normal",
                LogType::Info => "info",
                LogType::Warning => "warning",
                LogType::Critical => "critical",
            }
        )
    }
}

/// Peer log item data object
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct LogPeers {
    /// ID of the peer
    pub id: i64,
    /// IP of the peer
    pub ip: String,
    /// Seconds since epoch
    pub timestamp: i64,
    /// Whether or not the peer was blocked
    pub blocked: bool,
    /// Reason of the block
    pub reason: String,
}
