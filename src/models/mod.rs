use serde::Deserialize;

mod application;
mod log;
mod rss;
mod sync;
mod torrent;
mod transfer;

pub use application::*;
pub use log::*;
pub use rss::*;
pub use sync::*;
pub use torrent::*;
pub use transfer::*;

/// Connection status of the Qbit application
#[derive(Debug, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    Disconnected,
}
