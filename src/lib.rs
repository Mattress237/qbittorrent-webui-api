#![recursion_limit = "256"] // For serde_json

//! # Qbittorrent Web API wrapper
//!
//! This module provides a wrapper around the Qbit Web API, enabling
//! interaction with the API through a structured and type-safe interface.
//!
//! The wrapper includes functionality for managing authentication states,
//! handling credentials, and interacting with various API endpoints.
//!
//! The library is designed to support Qbittorrent version `5.1`
//!
//! # Example
//!
//! Basic usage to get all torrents
//! ```no_run
//! use qbit::{Api, Credentials};
//!
//! #[tokio::main]
//! async fn main() {
//!     let credentials = Credentials::Login("username".to_string(), "password".to_string());
//!     let client = Api::new_login("http://qBittorrent.server:6969", credentials)
//!         .await
//!         .unwrap();
//!
//!     let torrents = client.torrents(None).await.unwrap();
//!
//!     for torrent in torrents {
//!         println!("{:?}", torrent);
//!     }
//! }
//! ```
//!

mod auth;
mod client;
mod error;
pub(crate) mod utilities;

/// Data object models.
pub mod models;
/// Parameter objects.
pub mod parameters;

pub use auth::Credentials;
pub use auth::LoginState;
pub use client::Api;
pub use error::Error;

#[cfg(all(feature = "qBittorrent-5_1", feature = "qBittorrent-5_3"))]
compile_error!(
    "Features 'qBittorrent-5_1' and 'qBittorrent-5_3' are not intended to be used together. Please use only one of them."
);
