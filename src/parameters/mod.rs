//!
//! This module provides the data structures and enums necessary for managing
//! parameters, states, and sorting options.
//!
//! The types defined here are used for serializing parameters to the QbitTorrent WebUI API.
//!

mod application;
mod creator;
mod torrent;

pub use application::*;
pub use creator::*;
pub use torrent::*;
