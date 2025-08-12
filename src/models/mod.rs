use serde::{Deserialize, Serialize};

mod application;
mod log;
mod rss;
mod search;
mod sync;
mod torrent;
mod transfer;

pub use application::*;
pub use log::*;
pub use rss::*;
pub use search::*;
pub use sync::*;
pub use torrent::*;
pub use transfer::*;

/// Connection status of the Qbit application
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    #[default]
    Disconnected,
}
