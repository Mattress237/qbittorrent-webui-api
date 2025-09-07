//!
//! This module contains the core models used throughout the application.
//!
//! The models defined here are shared across various components and
//! providing a consistent structure for data representation and serialization.
//!

use serde::{Deserialize, Serialize};

mod application;
mod creator;
mod log;
mod rss;
mod search;
mod sync;
mod torrent;
mod transfer;

pub use application::*;
pub use creator::*;
pub use log::*;
pub use rss::*;
pub use search::*;
pub use sync::*;
pub use torrent::*;
pub use transfer::*;

/// Connection status of the Qbit application
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum ConnectionStatus {
    /// The client is connected to the other cliet / server
    #[serde(rename = "connected")]
    Connected,
    /// The client is connected to the other cliet / server but through a firewall
    #[serde(rename = "firewalled")]
    Firewalled,
    /// The client is not connected to the other cliet / server (or was and then disconnectede)
    #[serde(rename = "disconnected")]
    #[default]
    Disconnected,
}
