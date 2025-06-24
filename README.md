# Qbittorrent WebUI Api

Rust wrapper for Qbittorrent WebUI API

Supported Qbittorrent version: `5.0`

[WebUI 5.0 documentation](<https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)>)

## Implemented

### Authentication

- [x] Login
- [x] Logout

### Application

- [x] Get application version
- [x] Get API version
- [x] Get build info
- [x] Shutdown application
- [x] Get application preferences
- [x] Set application preferences
- [x] Get default save path
- [ ] Get cookies
- [ ] Set cookies

### Log

- [x] Get log
- [x] Get peer log

### Sync

- [x] Get main data
- [x] Get torrent peers data // Incomplete documentation for API

### Transfer info

- [x] Get global transfer info
- [x] Get alternative speed limits state
- [x] Toggle alternative speed limits
- [x] Get global download limit
- [x] Set global download limit
- [x] Get global upload limit
- [x] Set global upload limit
- [x] Ban peers

### Torrent management

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

### RSS (experimental)

- [x] Add folder
- [x] Add feed
- [x] Remove item
- [x] Move item
- [x] Get all items
- [x] Mark as read
- [x] Refresh item
- [ ] Set auto-downloading rule
- [ ] Rename auto-downloading rule
- [ ] Remove auto-downloading rule
- [ ] Get all auto-downloading rules
- [ ] Get all articles matching a rule

### Search

- [ ] Start search
- [ ] Stop search
- [ ] Get search status
- [ ] Get search results
- [ ] Delete search
- [ ] Get search plugins
- [ ] Install search plugin
- [ ] Uninstall search plugin
- [ ] Enable search plugin
- [ ] Update search plugins
