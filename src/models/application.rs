use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Build info response data object.
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildInfo {
    /// QT version
    pub qt: String,
    /// libtorrent version
    pub libtorrent: String,
    /// Boost version
    pub boost: String,
    /// OpenSSL version
    pub openssl: String,
    /// Application bitness (e.g. 64-bit)
    pub bitness: u8,
}
/// Preferences response data object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Preferences {
    // ========== General Settings ==========
    /// Currently selected language (e.g. en_GB for English)
    pub locale: String,
    /// TODO
    pub auto_delete_mode: u8,
    /// True if disk space should be pre-allocated for all files
    pub preallocate_all: bool,
    /// True if ".!qB" should be appended to incomplete files
    pub incomplete_files_ext: bool,

    // ========== Torrent Management ==========
    /// True if Automatic Torrent Management is enabled by default
    pub auto_tmm_enabled: bool,
    /// True if torrent should be relocated when its Category changes
    pub torrent_changed_tmm_enabled: bool,
    /// True if torrent should be relocated when the default save path changes
    pub save_path_changed_tmm_enabled: bool,
    /// True if torrent should be relocated when its Category's save path changes
    pub category_changed_tmm_enabled: bool,
    /// The default layout of the torrent content.
    pub torrent_content_layout: ContentLayout,
    /// The size limit of `.torrent` files
    pub torrent_file_size_limit: u64,
    /// When does the torrent stop
    pub torrent_stop_condition: StopCondition,
    /// What to do with removing torrents.
    pub torrent_content_remove_option: TorrentDeletion,

    // ========== File Paths ==========
    /// Default save path for torrents, separated by slashes
    pub save_path: String,
    /// True if folder for incomplete torrents is enabled
    pub temp_path_enabled: bool,
    /// Path for incomplete torrents, separated by slashes
    pub temp_path: String,
    // Property: directory to watch for torrent files, value: where torrents loaded from this directory should be downloaded to (see list of possible values below). Slashes are used as path separators; multiple key/value pairs can be specified
    pub scan_dirs: HashMap<String, ScanDir>,
    /// Path to directory to copy .torrent files to. Slashes are used as path separators
    pub export_dir: String,
    /// Path to directory to copy .torrent files of completed downloads to. Slashes are used as path separators
    pub export_dir_fin: String,

    // ========== Email Notifications ==========
    /// True if e-mail notification should be enabled
    pub mail_notification_enabled: bool,
    /// e-mail where notifications should originate from
    pub mail_notification_sender: String,
    /// e-mail to send notifications to
    pub mail_notification_email: String,
    /// smtp server for e-mail notifications
    pub mail_notification_smtp: String,
    /// True if smtp server requires SSL connection
    pub mail_notification_ssl_enabled: bool,
    /// True if smtp server requires authentication
    pub mail_notification_auth_enabled: bool,
    /// Username for smtp authentication
    pub mail_notification_username: String,
    /// Password for smtp authentication
    pub mail_notification_password: String,

    // ========== External Programs ==========
    /// True if external program should be run after torrent has finished downloading
    pub autorun_enabled: bool,
    /// Program path/name/arguments to run if `autorun_enabled` is enabled; path is separated by slashes; you can use `%f` and `%n` arguments, which will be expanded by qBittorent as path_to_torrent_file and torrent_name (from the GUI; not the .torrent file name) respectively
    pub autorun_program: String,

    // ========== Queue Management ==========
    /// True if torrent queuing is enabled
    pub queueing_enabled: bool,
    /// Maximum number of active simultaneous downloads
    pub max_active_downloads: i64,
    /// Maximum number of active simultaneous downloads and uploads
    pub max_active_torrents: i64,
    /// Maximum number of active simultaneous uploads
    pub max_active_uploads: i64,
    /// If true torrents w/o any activity (stalled ones) will not be counted towards `max_active_*` limits; see dont_count_slow_torrents for more information
    pub dont_count_slow_torrents: bool,
    /// Download rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_dl_rate_threshold: i64,
    /// Upload rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_ul_rate_threshold: i64,
    /// Seconds a torrent should be inactive before considered "slow"
    pub slow_torrent_inactive_timer: i64,

    // ========== Share Ratio Limits ==========
    /// True if share ratio limit is enabled
    pub max_ratio_enabled: bool,
    /// Get the global share ratio limit
    pub max_ratio: f64,
    /// Action performed when a torrent reaches the maximum share ratio. See list of possible values here below.
    pub max_ratio_act: RatioAct,
    /// True enables max seeding time
    pub max_seeding_time_enabled: bool,
    /// Number of minutes to seed a torrent
    pub max_seeding_time: i64,

    // ========== Connection Settings ==========
    /// Port for incoming connections
    pub listen_port: u16,
    /// True if UPnP/NAT-PMP is enabled
    pub upnp: bool,
    /// True if the port is randomly selected
    ///
    /// NOTE: This is marked as deprecated in the src file
    /// [Github referanse](https://github.com/qbittorrent/qBittorrent/blob/4f94eac235cefa8b83489cb3135dad87fcbed1e3/src/webui/api/appcontroller.cpp#L228)
    pub random_port: bool,
    /// Maximum global number of simultaneous connections
    ///
    /// `-1` means disabled
    pub max_connec: i64,
    /// Maximum number of simultaneous connections per torrent
    ///
    /// `-1` means disabled
    pub max_connec_per_torrent: i64,
    /// Maximum number of upload slots
    ///
    /// `-1` means disabled
    pub max_uploads: i64,
    /// Maximum number of upload slots per torrent
    ///
    /// `-1` means disabled
    pub max_uploads_per_torrent: i64,

    // ========== Speed Limits ==========
    /// Global download speed limit in KiB/s; -1 means no limit is applied
    pub dl_limit: i64,
    /// Global upload speed limit in KiB/s; -1 means no limit is applied
    pub up_limit: i64,
    /// Alternative global download speed limit in KiB/s
    pub alt_dl_limit: i64,
    /// Alternative global upload speed limit in KiB/s
    pub alt_up_limit: i64,

    // ========== Speed Limit Scheduler ==========
    /// True if alternative limits should be applied according to schedule
    pub scheduler_enabled: bool,
    /// Scheduler starting hour
    pub schedule_from_hour: i8,
    /// Scheduler starting minute
    pub schedule_from_min: i8,
    /// Scheduler ending hour
    pub schedule_to_hour: i8,
    /// Scheduler ending minute
    pub schedule_to_min: i8,
    /// Scheduler days. See possible values here below
    pub scheduler_days: SchedulerTime,

    // ========== BitTorrent Protocol ==========
    /// Bittorrent Protocol to use (see list of possible values below)
    pub bittorrent_protocol: BittorrentProtocol,
    /// True if `dl_limit` should be applied to uTP connections; this option is only available in qBittorent built against libtorrent version `0.16.X` and higher
    pub limit_utp_rate: bool,
    /// True if `dl_limit` should be applied to estimated TCP overhead (service data: e.g. packet headers)
    pub limit_tcp_overhead: bool,
    /// True if `dl_limit` should be applied to peers on the LAN
    pub limit_lan_peers: bool,
    /// μTP-TCP mixed mode algorithm (see list of possible values below)
    pub utp_tcp_mixed_mode: UtpTcpMixedMode,

    // ========== Peer Discovery ==========
    /// True if DHT is enabled
    pub dht: bool,
    /// True if PeX is enabled
    pub pex: bool,
    /// True if LSD is enabled
    pub lsd: bool,

    // ========== Encryption & Privacy ==========
    /// See list of possible values here below
    pub encryption: Encryption,
    /// If true anonymous mode will be enabled; read more here; this option is only available in qBittorent built against libtorrent version 0.16.X and higher
    pub anonymous_mode: bool,

    // ========== Proxy Settings ==========
    /// See list of possible values here below
    pub proxy_type: ProxyType,
    /// Proxy IP address or domain name
    pub proxy_ip: String,
    /// Proxy port
    pub proxy_port: u16,
    /// True if peer and web seed connections should be proxified; this option will have any effect only in qBittorent built against libtorrent version 0.16.X and higher
    pub proxy_peer_connections: bool,
    /// True proxy requires authentication; doesn't apply to SOCKS4 proxies
    pub proxy_auth_enabled: bool,
    /// Username for proxy authentication
    pub proxy_username: String,
    /// Password for proxy authentication
    pub proxy_password: String,

    // ========== IP Filtering ==========
    /// True if external IP filter should be enabled
    pub ip_filter_enabled: bool,
    /// Path to IP filter file (.dat, .p2p, .p2b files are supported); path is separated by slashes
    pub ip_filter_path: String,
    /// True if IP filters are applied to trackers
    pub ip_filter_trackers: bool,
    /// List of banned IPs
    #[serde(rename = "banned_IPs")]
    pub banned_ips: String,

    // ========== WebUI Settings ==========
    /// Semicolon-separated list of domains to accept when performing Host header validation
    pub web_ui_domain_list: String,
    /// IP address to use for the WebUI
    pub web_ui_address: String,
    /// WebUI port
    pub web_ui_port: u16,
    /// True if upnp is used for the WebUI port
    pub web_ui_upnp: bool,
    /// WebUI username
    pub web_ui_username: String,
    /// For API ≥ v2.3.0: Plaintext WebUI password. This field is write-only and cannot be read back.
    ///
    /// The password is used exclusively for setting or updating the WebUI password.
    pub web_ui_password: Option<String>,

    // ========== WebUI Security ==========
    /// True if WebUI CSRF protection is enabled
    pub web_ui_csrf_protection_enabled: bool,
    /// True if WebUI clickjacking protection is enabled
    pub web_ui_clickjacking_protection_enabled: bool,
    /// True if WebUI cookie Secure flag is enabled
    pub web_ui_secure_cookie_enabled: bool,
    /// Maximum number of authentication failures before WebUI access ban
    pub web_ui_max_auth_fail_count: i64,
    /// WebUI access ban duration in seconds
    pub web_ui_ban_duration: i64,
    /// Seconds until WebUI is automatically signed off
    pub web_ui_session_timeout: i64,
    /// True if WebUI host header validation is enabled
    pub web_ui_host_header_validation_enabled: bool,
    /// True if authentication challenge for loopback address (127.0.0.1) should be disabled
    pub bypass_local_auth: bool,
    /// True if webui authentication should be bypassed for clients whose ip resides within (at least) one of the subnets on the whitelist
    pub bypass_auth_subnet_whitelist_enabled: bool,
    /// (White)list of ipv4/ipv6 subnets for which webui authentication should be bypassed; list entries are separated by commas
    pub bypass_auth_subnet_whitelist: String,

    // ========== Alternative WebUI ==========
    /// True if an alternative WebUI should be used
    pub alternative_webui_enabled: bool,
    /// File path to the alternative WebUI
    pub alternative_webui_path: String,

    // ========== WebUI HTTPS ==========
    /// True if WebUI HTTPS access is enabled
    pub use_https: bool,
    /// For API ≥ v2.0.1: Path to SSL keyfile
    pub web_ui_https_key_path: String,
    /// For API ≥ v2.0.1: Path to SSL certificate
    pub web_ui_https_cert_path: String,

    // ========== WebUI Custom Headers ==========
    /// For API ≥ v2.5.1: Enable custom http headers
    pub web_ui_use_custom_http_headers_enabled: bool,
    /// For API ≥ v2.5.1: List of custom http headers
    pub web_ui_custom_http_headers: String,

    // ========== Dynamic DNS ==========
    /// True if server DNS should be updated dynamically
    pub dyndns_enabled: bool,
    /// See list of possible values here below
    pub dyndns_service: DyndnsService,
    /// Username for DDNS service
    pub dyndns_username: String,
    /// Password for DDNS service
    pub dyndns_password: String,
    /// Your DDNS domain name
    pub dyndns_domain: String,

    // ========== RSS Settings ==========
    /// RSS refresh interval
    pub rss_refresh_interval: i64,
    /// Max stored articles per RSS feed
    pub rss_max_articles_per_feed: u32,
    /// Enable processing of RSS feeds
    pub rss_processing_enabled: bool,
    /// Enable auto-downloading of torrents from the RSS feeds
    pub rss_auto_downloading_enabled: bool,
    /// For API ≥ v2.5.1: Enable downloading of repack/proper Episodes
    pub rss_download_repack_proper_episodes: bool,
    /// For API ≥ v2.5.1: List of RSS Smart Episode Filters
    pub rss_smart_episode_filters: String,

    // ========== Tracker Settings ==========
    /// Enable automatic adding of trackers to new torrents
    pub add_trackers_enabled: bool,
    /// List of trackers to add to new torrent
    pub add_trackers: String,
    /// Timeout in seconds for a stopped announce request to trackers
    ///
    /// If the value is set to 0, the connections to trackers with the stopped event are suppressed.
    pub stop_tracker_timeout: i64,
    /// TODO
    pub announce_ip: String,
    /// True always announce to all tiers
    pub announce_to_all_tiers: bool,
    /// True always announce to all trackers in a tier
    pub announce_to_all_trackers: bool,

    // ========== Advanced LibTorrent Settings ==========
    /// True if the advanced libtorrent option piece_extent_affinity is enabled
    pub enable_piece_extent_affinity: bool,
    /// Number of asynchronous I/O threads
    pub async_io_threads: u16,
    /// Outstanding memory when checking torrents in MiB
    pub checking_memory_use: u32,
    /// IP Address to bind to. Empty String means All addresses
    pub current_interface_address: String,
    /// Network Interface used
    pub current_network_interface: String,
    /// Disk cache used in MiB
    pub disk_cache: i64,
    /// Disk cache expiry interval in seconds
    pub disk_cache_ttl: i64,
    /// Port used for embedded tracker
    pub embedded_tracker_port: u16,
    /// True enables coalesce reads & writes
    pub enable_coalesce_read_write: bool,
    /// True enables embedded tracker
    pub enable_embedded_tracker: bool,
    /// True allows multiple connections from the same IP address
    pub enable_multi_connections_from_same_ip: bool,
    /// True enables sending of upload piece suggestions
    pub enable_upload_suggestions: bool,
    /// File pool size
    ///
    /// Sets the upper limit on the total number of files this session will keep open.
    /// The reason why files are left open at all is that some anti virus software hooks
    /// on every file close, and scans the file for viruses. deferring the closing of
    /// the files will be the difference between a usable system and a completely hogged
    /// down system. Most operating systems also has a limit on the total number of file
    /// descriptors a process may have open.
    pub file_pool_size: i64,
    /// Maximal outgoing port (0: Disabled)
    pub outgoing_ports_max: u16,
    /// Minimal outgoing port (0: Disabled)
    pub outgoing_ports_min: u16,
    /// True rechecks torrents on completion
    pub recheck_completed_torrents: bool,
    /// True resolves peer countries
    pub resolve_peer_countries: bool,
    /// Save resume data interval in min
    pub save_resume_data_interval: i64,
    /// Send buffer low watermark in KiB
    ///
    /// The minimum send buffer target size (send buffer includes bytes pending being
    /// read from disk). For good and snappy seeding performance, set this fairly high,
    /// to at least fit a few blocks. This is essentially the initial window size
    /// which will determine how fast we can ramp up the send rate
    pub send_buffer_low_watermark: i64,
    /// Send buffer watermark in KiB
    ///
    /// if the send buffer has fewer bytes than send_buffer_watermark, we'll read
    /// another 16 kiB block onto it. If set too small, upload rate capacity will
    /// suffer. If set too high, memory will be wasted. The actual watermark may be
    /// lower than this in case the upload rate is low, this is the upper limit.
    pub send_buffer_watermark: i64,
    /// Send buffer watermark factor in percent
    ///
    /// the current upload rate to a peer is multiplied by this factor to get the
    /// send buffer watermark. The factor is specified as a percentage. i.e.
    /// 50 -> 0.5 This product is clamped to the send_buffer_watermark setting to
    /// not exceed the max. For high speed upload, this should be set to a greater
    /// value than 100. For high capacity connections, setting this higher can
    /// improve upload performance and disk throughput. Setting it too high may
    /// waste RAM and create a bias towards read jobs over write jobs.
    pub send_buffer_watermark_factor: i64,
    /// Socket backlog size
    pub socket_backlog_size: i64,
    /// Upload choking algorithm used (see list of possible values below)
    pub upload_choking_algorithm: UploadChokingAlgorithm,
    /// Upload slots behavior used (see list of possible values below)
    pub upload_slots_behavior: UploadSlotsBehavior,
    /// upnp lease duration (0: Permanent lease)
    ///
    /// The expiration time of upnp port-mappings, specified in seconds. 0 means
    /// permanent lease. Some routers do not support expiration times on port-maps
    /// (nor correctly returning an error indicating lack of support). In those
    /// cases, set this to 0. Otherwise, don't set it any lower than 5 minutes.
    pub upnp_lease_duration: u32,
}

/// How the torrent content is laied out.
#[derive(Debug, Serialize, Deserialize)]
pub enum ContentLayout {
    /// Does whatever the server says to do, which by default is Subfolder
    Original,
    /// In cases of batches, will create a separate subfolder automatically of the batch name.
    /// Example: `Save_path/Torrent_name/Torrent_files`
    Subfolder,
    /// In cases of batches, will just place them all in the save_path.
    /// Example: `Save_path/Torrent_files`
    NoSubfolder,
}

impl Default for ContentLayout {
    fn default() -> Self {
        Self::Original
    }
}

impl std::fmt::Display for ContentLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentLayout::Original => write!(f, "Original"),
            ContentLayout::Subfolder => write!(f, "Subfolder"),
            ContentLayout::NoSubfolder => write!(f, "NoSubfolder"),
        }
    }
}

/// When does the torrent stop
#[derive(Debug, Serialize, Deserialize)]
pub enum StopCondition {
    /// Don't stop and go straight to downloading
    None,
    /// Stop after receiving the metadata
    MetadataReceived,
    /// Stop after checking the files.
    FilesChecked,
}

impl Default for StopCondition {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for StopCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StopCondition::None => write!(f, "None"),
            StopCondition::MetadataReceived => write!(f, "MetadataReceived"),
            StopCondition::FilesChecked => write!(f, "FilesChecked"),
        }
    }
}

/// What to do when removing content files upon removing a torrent.
#[derive(Debug, Serialize, Deserialize)]
pub enum TorrentDeletion {
    /// Erase from disk permanatly
    Delete,
    /// Attempts to move to Trash/Wastebin if possible.
    MoveToTrash,
}

impl Default for TorrentDeletion {
    fn default() -> Self {
        Self::Delete
    }
}

impl std::fmt::Display for TorrentDeletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delete => write!(f, "Delete"),
            Self::MoveToTrash => write!(f, "MoveToTrash"),
        }
    }
}

/// Scan dir types
#[derive(Debug)]
pub enum ScanDir {
    MonitoredFolder,
    DefaultSavePath,
    OtherPath(String),
}

impl<'de> Deserialize<'de> for ScanDir {
    fn deserialize<D>(deserializer: D) -> Result<ScanDir, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Int(u8),
            Str(String),
        }

        match Helper::deserialize(deserializer)? {
            Helper::Int(0) => Ok(ScanDir::MonitoredFolder),
            Helper::Int(1) => Ok(ScanDir::DefaultSavePath),
            Helper::Str(s) => Ok(ScanDir::OtherPath(s)),
            _ => Err(serde::de::Error::custom(
                "unexpected value, expected 0, 1, or a string (path)",
            )),
        }
    }
}

impl Serialize for ScanDir {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ScanDir::MonitoredFolder => serializer.serialize_u8(0),
            ScanDir::DefaultSavePath => serializer.serialize_i8(1),
            ScanDir::OtherPath(s) => serializer.serialize_str(s),
        }
    }
}

/// Ratio actions
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RatioAct {
    PauseTorrent = 0,
    RemoveTorrent = 1,
}

/// Bittorrent protocols
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum BittorrentProtocol {
    Tcpμtp = 0,
    Tcp = 1,
    MicroTransportProtocol = 2,
}

/// Scheduler
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SchedulerTime {
    /// Every day
    Day = 0,
    /// Every Weekday
    Weekday = 1,
    /// Every Weekend
    Weekend = 2,
    /// Every Monday
    Monday = 3,
    /// Every Tuesday
    Tuesday = 4,
    /// Every Wednesday
    Wednesday = 5,
    /// Every Thursday
    Thursday = 6,
    /// Every Friday
    Friday = 7,
    /// Every Saturday
    Saturday = 8,
    /// Every Sunday
    Sunday = 9,
}

/// Encryption states
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Encryption {
    Prefer = 0,
    ForceOn = 1,
    ForceOff = 2,
}

/// Proxy types states
#[derive(Debug)]
pub enum ProxyType {
    Disabled,
    Other(String),
    HttpWithoutAuth,
    Socks5WithoutAuth,
    HttpWithAuth,
    Socks5WithAuth,
    Socks4WithoutAuth,
}

impl<'de> Deserialize<'de> for ProxyType {
    fn deserialize<D>(deserializer: D) -> Result<ProxyType, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Int(i8),
            Str(String),
        }

        match Helper::deserialize(deserializer)? {
            Helper::Int(-1) => Ok(ProxyType::Disabled),
            Helper::Str(s) => Ok(ProxyType::Other(s)),
            Helper::Int(1) => Ok(ProxyType::HttpWithoutAuth),
            Helper::Int(2) => Ok(ProxyType::Socks5WithoutAuth),
            Helper::Int(3) => Ok(ProxyType::HttpWithAuth),
            Helper::Int(4) => Ok(ProxyType::Socks5WithAuth),
            Helper::Int(5) => Ok(ProxyType::Socks4WithoutAuth),
            _ => Err(serde::de::Error::custom(
                "unexpected value, expected 1 to 5, -1 or a string",
            )),
        }
    }
}

impl Serialize for ProxyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ProxyType::Disabled => serializer.serialize_i8(-1),
            ProxyType::HttpWithoutAuth => serializer.serialize_i8(1),
            ProxyType::Socks5WithoutAuth => serializer.serialize_i8(2),
            ProxyType::HttpWithAuth => serializer.serialize_i8(3),
            ProxyType::Socks5WithAuth => serializer.serialize_i8(4),
            ProxyType::Socks4WithoutAuth => serializer.serialize_i8(5),
            ProxyType::Other(s) => serializer.serialize_str(s),
        }
    }
}

/// Dyndns servcice types
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum DyndnsService {
    Dydns = 0,
    Noip = 1,
}

/// Upload choking algorithm
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    RoundRobin = 0,
    FastestUpload = 1,
    AntiLeech = 2,
}

/// Upload slots behavior
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    Fixed = 0,
    UploadRate = 1,
}

/// Mix mode UTP / TCP
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    PreferTcp = 0,
    PeerProportional = 1,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Cookie {
    /// The name of the cookie.
    pub name: String,
    /// The domain associated with the cookie.
    pub domain: String,
    /// The path associated with the cookie.
    pub path: String,
    /// The value stored in the cookie.
    pub value: String,
    /// The expiration date of the cookie, represented as seconds since the Unix epoch.
    #[serde(rename = "expirationDate")]
    pub expiration: i64,
}
