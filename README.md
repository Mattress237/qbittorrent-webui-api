[![Crates.io version](https://img.shields.io/crates/v/qbit)](https://crates.io/crates/qbit)
[![Github repo](https://img.shields.io/badge/Github-Repo-orange?logo=github)](https://github.com/Mattress237/qbittorrent-webui-api)
[![docs.rs](https://img.shields.io/docsrs/qbit)](https://docs.rs/qbit)
[![Rust](https://img.shields.io/badge/Rust-stable-brightgreen?logo=rust)](https://www.rust-lang.org/)

# Qbittorrent WebUI Api

Asynchronous Rust wrapper for Qbittorrent Web API, supporting all documented endpoints.

# Supported Qbittorrent versions

Supported Qbittorrent versions: `>=5.2.0`, `>=5.1.2` (with feature flag `qBittorrent-5_1`)
This is the lowest tested versions.

# Usage

Add it using cargo:
```
cargo add qbit
```

Add it manually in your `Cargo.toml`:
``` toml
[dependencies]
qbit = "0.3"
```

## Feature flags
Leaving feature flags unset will default to Qbittorrent `5.2`

For backwards compatibility, and future-proofing, two feature flags are 
available: `qBittorrent-5_3` and `qBittorrent-5_1`.

Enabling `qBittorrent-5_1` will expect the server to be running Qbittorrent 
`5.1`, while `qBittorrent-5_3` will expect the server to be running a prerelease 
version of `5.3`. 

`qBittorrent-5_3` should be expected to be experimental and may 
be unstable as it is in active development.

## Basic usage to get all torrents:
``` rust
use qbit::API;
use qbit::Credentials;

let credentials = Credentials::Login("username".to_string(), "password".to_string());
let mut client = Api::new_login("http://127.0.0.1/", credentials).await.unwrap();

let torrents = client.torrents(None).await.unwrap();
```

# Testing
Quick example of how to set up the codebase for testing.

## Basic codebase tests
Basic testing of the codebase. This should not hit external services.
``` bash
cargo test
```

## Full API tests
Testing of API calls against a running qBittorrent server.
It requires a running qBittorrent server with the web UI enabled. 
It uses environment variables to configure the server URL and credentials.
Environment can be set in the shell or in a `.env` file see `.env.example`.
``` bash
WEBUI_URL="http://localhost" \
WEBUI_PORT="6969" \
WEBUI_USERNAME="admin" \
WEBUI_PASSWORD="adminadmin" \
WEBUI_APIKEY="<APIKEY>" \
TEMP_DIR=".temp" \
SERVER_TEMP_DIR=".temp" \
cargo test -- --include-ignored
```

# Implemented

[WebUI 5.0 documentation](<https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)>)

## Authentication

- [x] Login
- [x] Logout

## Application

- [x] Get application version
- [x] Get API version
- [x] Get build info
- [x] Shutdown application
- [x] Get application preferences
- [x] Set application preferences
- [x] Get default save path
- [x] Get cookies
- [x] Set cookies

## Log

- [x] Get log
- [x] Get peer log

## Sync

- [x] Get main data
- [x] Get torrent peers data // Incomplete documentation for API

## Transfer info

- [x] Get global transfer info
- [x] Get alternative speed limits state
- [x] Toggle alternative speed limits
- [x] Get global download limit
- [x] Set global download limit
- [x] Get global upload limit
- [x] Set global upload limit
- [x] Ban peers

## Torrent management

- [x] Get torrent list
- [x] Get torrent generic properties
- [x] Get torrent trackers
- [x] Get torrent web seeds
- [x] Get torrent contents
- [x] Get torrent pieces' states
- [x] Get torrent pieces' hashes
- [x] Stop torrents
- [x] Start torrents
- [x] Delete torrents
- [x] Recheck torrents
- [x] Reannounce torrents
- [x] Add new torrent
- [x] Add trackers to torrent
- [x] Edit trackers
- [x] Remove trackers
- [x] Add peers
- [x] Increase torrent priority
- [x] Decrease torrent priority
- [x] Maximal torrent priority
- [x] Minimal torrent priority
- [x] Set file priority
- [x] Get torrent download limit
- [x] Set torrent download limit
- [x] Set torrent share limit
- [x] Get torrent upload limit
- [x] Set torrent upload limit
- [x] Set torrent location
- [x] Set torrent name
- [x] Set torrent category
- [x] Get all categories
- [x] Add new category
- [x] Edit category
- [x] Remove categories
- [x] Add torrent tags
- [x] Remove torrent tags
- [x] Get all tags
- [x] Create tags
- [x] Delete tags
- [x] Set automatic torrent management
- [x] Toggle sequential download
- [x] Set first/last piece priority
- [x] Set force start
- [x] Set super seeding
- [x] Rename file   // Unsure if this is for the file or the torrent
- [x] Rename folder // Unsure if this is for the file or the torrent

## RSS (experimental)

- [x] Add folder
- [x] Add feed
- [x] Remove item
- [x] Move item
- [x] Get all items
- [x] Mark as read
- [x] Refresh item
- [x] Set auto-downloading rule
- [x] Rename auto-downloading rule
- [x] Remove auto-downloading rule
- [x] Get all auto-downloading rules
- [x] Get all articles matching a rule

## Search

- [x] Start search
- [x] Stop search
- [x] Get search status
- [x] Get search results
- [x] Delete search
- [x] Get search plugins
- [x] Install search plugin
- [x] Uninstall search plugin
- [x] Enable search plugin
- [x] Update search plugins
