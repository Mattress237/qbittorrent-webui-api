use std::{collections::HashMap, fmt::Display};

use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Build info response data object.
///
/// Contains version information of software used to run qbittorrent.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Builder)]
pub struct Preferences {
    // ========== General Settings ==========
    /// Currently selected language (e.g. en_GB for English)
    pub locale: String,
    /// When (and should) the `.torrent` file be deleted after added.
    pub auto_delete_mode: AutoDeleteMode,
    /// Should disk space be pre-allocated for all files?
    pub preallocate_all: bool,
    /// Should `.!qb` be added to incomplete files?
    pub incomplete_files_ext: bool,
    /// Should unchecked files be added to the `.unwanted` folder?
    ///
    /// See https://github.com/qbittorrent/qBittorrent/issues/13531 for an argument on the subject.
    pub use_unwanted_folder: bool,
    /// Customise the name of the app instance
    pub app_instance_name: String,
    /// How often should the UI refresh to get new updates? (in ms)
    pub refresh_interval: u64,
    /// Should the client external IP be shown in the status bar?
    pub status_bar_external_ip: bool,
    /// Show a confirmation message before deleting a torrent. Does not apply to the API
    pub confirm_torrent_deletion: bool,
    /// To delete content files alongside the torrent. A "cache" setting
    ///
    /// NOTE: In the webui, this setting is only visible by checking the icon next to the `Also remove content files`
    /// checkbox upon deleting a file.
    pub delete_torrent_content_files: bool,
    /// Show a confirmation message before rechecking a torrent. Does not apply to the API
    pub confirm_torrent_recheck: bool,
    /// Allow using of sub-categories. Sub-categories are made by adding `/` between the parent and child.
    pub use_subcategories: bool,
    /// Memory usage limit of Physical RAM in MiB
    ///
    /// Note: Requires Libtorrent >= 2.0.0
    pub memory_working_set_limit: u64,

    // ========== Torrent Management ==========
    /// Should `Automatic Torrent Mangament` be enabled for new torrents by default?
    pub auto_tmm_enabled: bool,
    /// Should the torrent be relocated or switched to manual mode when category is changed?
    ///
    /// True = Relocated, False = Manual Mode
    pub torrent_changed_tmm_enabled: bool,
    /// Should the affected torrents be relocated or switched to manual mode when the default save/incomplete path is changed?
    ///
    /// True = Relocated, False = Manual Mode
    pub save_path_changed_tmm_enabled: bool,
    /// Should the affected torrents be relocated or switched to manual mode when it's category save path has changed?
    ///
    /// True = Relocated, False = Manual Mode
    pub category_changed_tmm_enabled: bool,
    /// The default layout of the torrent content.
    pub torrent_content_layout: ContentLayout,
    /// The size limit of `.torrent` files
    pub torrent_file_size_limit: u64,
    /// When does the torrent stop
    pub torrent_stop_condition: StopCondition,
    /// What to do with removing torrents.
    pub torrent_content_remove_option: TorrentDeletion,
    /// If the torrent exists, do we merge trackers with it or fail to add the torrent altogether?
    pub merge_trackers: bool,
    /// Use the path specified by the category even if the torrent is in manual mode.
    pub use_category_paths_in_manual_mode: bool,
    /// Number of connection attempts made per second.
    ///
    /// If number < 0, a default of 200 will be made.
    pub connection_speed: i64,
    /// How many torrents can be actively checking at one time.
    pub max_active_checking_torrents: i64,

    // ========== File Paths ==========
    /// Default save path for torrents
    pub save_path: String,
    /// Should another path be used for incomplete torrents?
    pub temp_path_enabled: bool,
    /// The path to use for incomplete torrents.
    pub temp_path: String,
    /// Directories to scan for `.torrent` files. `ScanDir` enum is used to overwrite the default save path of adding torrents.
    pub scan_dirs: HashMap<String, ScanDir>,
    /// Path to copy `.torrent` files to.
    pub export_dir: String,
    /// Path to copy `.torrent` files of completed downloads to.
    pub export_dir_fin: String,
    /// Is the filename blacklist enabled?
    pub excluded_file_names_enabled: bool,
    /// Blacklist filter file names from being downloaded from the torrent.
    ///
    /// Files matching any of this list will have the priority set to `Do Not Download` by default. (newline separator)
    ///
    /// The follow wildcards can be used:
    /// - *: Any character
    /// - ?: Any Single character
    /// - [...]: Sets of characters
    pub excluded_file_names: String,

    // ========== Email Notifications ==========
    /// Should email notifications be sent after a download is finished?
    pub mail_notification_enabled: bool,
    /// e-mail where notifications should originate from
    ///
    /// Client default: qBittorrent_notification@example.com
    pub mail_notification_sender: String,
    /// e-mail to send notifications to
    pub mail_notification_email: String,
    /// smtp server for e-mail notifications
    pub mail_notification_smtp: String,
    /// Does the smtp server require a secure connection (SSL)?
    pub mail_notification_ssl_enabled: bool,
    /// Does the smtp server require authentication?
    pub mail_notification_auth_enabled: bool,
    /// Username for smtp authentication
    pub mail_notification_username: String,
    /// Password for smtp authentication
    pub mail_notification_password: String,

    // ========== External Programs ==========
    /// Should an external program be run after a torrent has completed?
    pub autorun_enabled: bool,
    /// Program path/name/arguments to run if `autorun_enabled` is enabled
    ///
    /// Supported parameters (case sensitive)
    /// - %N: Torrent Name
    /// - %L: Torrent Category
    /// - %G: Torrent Tags (CSV)
    /// - %F: Torrent Content Path (same as root path for multi-file torrents)
    /// - %R: Torrent Root Path (first torrent subdirectory path)
    /// - %D: Torrent Save Path
    /// - %C: Number of files in torrent
    /// - %Z: Torrent Size (bytes)
    /// - %T: Current Tracker of Torrent
    /// - %I: Torrent Hash v1
    /// - %J: Torrent Hash v2
    /// - %K: Torrent ID
    ///
    /// Tip: Encapsulate parameter with quotation marks to avoid text being cut off at whitespace (e.g., "%N")
    ///
    /// # Example
    /// ```sh
    /// ./path/to/some/program.sh "%N" "%C"
    /// ```
    pub autorun_program: String,
    /// Should an external program be run after a torrent has been added?
    pub autorun_on_torrent_added_enabled: bool,
    /// Program path/name/argumets to run if `autorun_on_torrent_added_enabled` is enabled.
    ///
    /// See `autorun_program` for the supported parameters, tips and examples.
    pub autorun_on_torrent_added_program: String,
    /// Enables Mark-of-the-web. Tells external programs that this file is potentiall unsafe.
    ///
    /// Windows (MOTW) and Mac (quarantine) only
    pub mark_of_the_web: bool,
    /// Python executable path. For use in stuff like search engine plugins which require python.
    ///
    /// Will attempt to find and use a system wide one if nothing is specified.
    pub python_executable_path: String,

    // ========== Queue Management ==========
    /// Is torrent queuing enabled?
    pub queueing_enabled: bool,
    /// Maximum number of active simultaneous downloads
    pub max_active_downloads: i64,
    /// Maximum number of active simultaneous downloads and uploads
    pub max_active_torrents: i64,
    /// Maximum number of active simultaneous uploads
    pub max_active_uploads: i64,
    /// If true torrents w/o any activity (stalled ones) will not be counted towards `max_active_*` limits
    pub dont_count_slow_torrents: bool,
    /// Download rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_dl_rate_threshold: i64,
    /// Upload rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_ul_rate_threshold: i64,
    /// Seconds a torrent should be inactive before considered "slow"
    pub slow_torrent_inactive_timer: i64,
    /// To add new torrents to the top of the queue by default or not?
    pub add_to_top_of_queue: bool,
    /// Default setting for allowing new torrents to start automatically.
    ///
    /// - True = don't start downloading automatically.
    /// - False = Start downloading automatically.
    pub add_stopped_enabled: bool,

    // ========== Seed Limits ==========
    /// Show an action be taken once the torrent ratio is achieved?
    pub max_ratio_enabled: bool,
    /// THe ratio to achieve to take an action.
    pub max_ratio: f64,
    /// Should an action be taken once the torrent has been seeding for a certain amount of time?
    pub max_seeding_time_enabled: bool,
    /// Number of minutes to seed a torrent before an action is taken
    ///
    /// -1 = disabled (will also set `max_seeding_time_enabled` to false)
    pub max_seeding_time: i64,
    /// Should an action be taken once the torrent has been inactive (during seeding) for a certain amount of time?
    pub max_inactive_seeding_time_enabled: bool,
    /// Number of minutes for the torrent to be inactive (during seeding) before an action is taken.
    ///
    /// -1 = disabled (will also set `max_inactive_seeding_time_enabled` to false)
    pub max_inactive_seeding_time: i64,
    /// Action performed when a torrent reaches a ratio / seed limit.
    ///
    /// See: `max_ratio`, `max_seeding_time` and `max_inactive_seeding_time`
    ///
    /// The selected action in `max_ratio_act` is executed when either condition is met:
    /// - If `max_ratio_enabled` is true and the torrent's ratio reaches or exceeds `max_ratio`.
    /// - If `max_seeding_time_enabled` is true and the torrent has been seeding for at least `max_seeding_time` minutes.
    /// - If `max_inactive_seeding_time_enabled` is true and the torrent has been inactive (during seeding) for at least `max_inactive_seeding_time` minutes.
    ///
    /// If any are enabled, the action occurs when either condition is satisfied.
    pub max_ratio_act: SeedLimitActions,

    // ========== Connection Settings ==========
    /// Port for incoming connections
    pub listen_port: u16,
    /// Is UPnP/NAT-PMP enabled?
    pub upnp: bool,
    /// True if the port is randomly selected
    ///
    /// NOTE: This is marked as deprecated in the src file
    /// [Github referanse](https://github.com/qbittorrent/qBittorrent/blob/4f94eac235cefa8b83489cb3135dad87fcbed1e3/src/webui/api/appcontroller.cpp#L228)
    #[deprecated(note = "This field is deprecated upstream; retained here for compatibility.")]
    pub random_port: bool,
    /// Maximum global number of simultaneous connections
    ///
    /// `-1` means disabled
    #[serde(rename = "max_connec")]
    pub max_connections: i64,
    /// Maximum number of simultaneous connections per torrent
    ///
    /// `-1` means disabled
    #[serde(rename = "max_connec_per_torrent")]
    pub max_connections_per_torrent: i64,
    /// Maximum number of upload slots
    ///
    /// `-1` means disabled
    pub max_uploads: i64,
    /// Maximum number of upload slots per torrent
    ///
    /// `-1` means disabled
    pub max_uploads_per_torrent: i64,

    // ========== Speed Limits ==========
    /// Global download speed limit in KiB/s; 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub dl_limit: u64,
    /// Global upload speed limit in KiB/s; 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub up_limit: u64,
    /// Alternative global download speed limit in KiB/s. 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub alt_dl_limit: u64,
    /// Alternative global upload speed limit in KiB/s. 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub alt_up_limit: u64,

    // ========== Speed Limit Scheduler ==========
    /// Should alternative limits be applied according to the schedule
    pub scheduler_enabled: bool,
    /// Scheduler starting hour
    pub schedule_from_hour: i8,
    /// Scheduler starting minute
    pub schedule_from_min: i8,
    /// Scheduler ending hour
    pub schedule_to_hour: i8,
    /// Scheduler ending minute
    pub schedule_to_min: i8,
    /// Days on which the schedule is applied.
    pub scheduler_days: SchedulerTime,

    // ========== BitTorrent Protocol ==========
    /// Bittorrent Protocol to use (see list of possible values below)
    pub bittorrent_protocol: BittorrentProtocol,
    /// Should `dl_limit` be applied to uTP connections?
    ///
    /// Note: qbittorrent built against libtorrent version `0.16.x` and higher is required for this setting.
    pub limit_utp_rate: bool,
    /// Should `dl_limit` be applied to estimated TCP overhead? (e.g. service data such as packet headers)
    pub limit_tcp_overhead: bool,
    /// Should `dl_limit` be applied to peers on the LAN?
    pub limit_lan_peers: bool,
    /// μTP-TCP mixed mode algorithm (see list of possible values below)
    pub utp_tcp_mixed_mode: UtpTcpMixedMode,

    // ========== Peer Discovery ==========
    // More info (can't work out where to place this): https://www.reddit.com/r/torrents/comments/jmcmx1/comment/gauf8kn/
    /// Is DHT (Decentrialized Network) enabled?
    ///
    /// See https://superuser.com/a/592244 for more info.
    pub dht: bool,
    /// Is PeX (Peer Exchange) enabled?
    pub pex: bool,
    /// Is LSD (Local Peer Discovery) enabled?
    pub lsd: bool,

    // ========== Encryption & Privacy ==========
    /// State of encryption for file transfer.
    pub encryption: Encryption,
    /// Is the user anonymous?
    ///
    /// WARNING: This doesn't grant enough protection on its own. See https://github.com/qbittorrent/qBittorrent/wiki/Anonymous-Mode for more information.
    ///
    /// Note: qbittorrent built against libtorrent version `0.16.x` and higher is required for this setting.
    pub anonymous_mode: bool,

    // ========== Proxy Settings ==========
    /// The protocol to use for the proxy server
    pub proxy_type: ProxyType,
    /// Proxy IP address or domain name
    pub proxy_ip: String,
    /// Proxy port
    pub proxy_port: u16,
    /// Should the proxyy be used for bittorrent purposes?
    pub proxy_bittorrent: bool,
    /// Should the proxyy be used for peer and web seed connections?
    ///
    /// Note: requires `proxy_bittorrent`
    pub proxy_peer_connections: bool,
    /// Should the proxyy be used for RSS purposes?
    pub proxy_rss: bool,
    /// Should the proxyy be used for General purposes?
    pub proxy_misc: bool,
    /// Should the proxyy be used for Hostname lookup?
    pub proxy_hostname_lookup: bool,
    /// Does the proxy require authentication?
    ///
    /// Note: This does not apply when ProxyType is SOCKS4
    pub proxy_auth_enabled: bool,
    /// Username for proxy authentication
    pub proxy_username: String,
    /// Password for proxy authentication
    pub proxy_password: String,

    // ========== IP Filtering ==========
    /// Should external IPs be filtered?
    pub ip_filter_enabled: bool,
    /// Path to IP filter file (.dat, .p2p, .p2b files are supported); path is separated by slashes
    pub ip_filter_path: String,
    /// Is the IP filter also applied to trackers?
    pub ip_filter_trackers: bool,
    /// List of banned IPs. Separated by new lines (`\n`)
    #[serde(rename = "banned_IPs")]
    pub banned_ips: String,

    // ========== WebUI Settings ==========
    /// Semicolon-separated list of domains to accept when performing Host header validation. Accepts: '*'
    ///
    /// Requires: `web_ui_host_header_validation_enabled` to be true.
    pub web_ui_domain_list: String,
    /// IP address to use for the WebUI. Accepts: '*'
    pub web_ui_address: String,
    /// WebUI port
    pub web_ui_port: u16,
    /// Use UPnP for port forwarding from the router
    pub web_ui_upnp: bool,
    /// WebUI username
    pub web_ui_username: String,
    /// For API ≥ v2.3.0: Plaintext WebUI password. This field is write-only and cannot be read back.
    ///
    /// The password is used exclusively for setting or updating the WebUI password.
    pub web_ui_password: Option<String>,
    /// True if WebUI CSRF protection is enabled
    pub web_ui_csrf_protection_enabled: bool,
    /// True if WebUI clickjacking protection is enabled
    pub web_ui_clickjacking_protection_enabled: bool,
    /// True if WebUI cookie Secure flag is enabled (requires `use_https`)
    pub web_ui_secure_cookie_enabled: bool,
    /// Maximum number of authentication failures before WebUI access ban
    pub web_ui_max_auth_fail_count: i64,
    /// WebUI access ban duration in seconds
    pub web_ui_ban_duration: i64,
    /// Seconds until WebUI is automatically signed off
    pub web_ui_session_timeout: i64,
    /// Is WebUI Host header validated?
    pub web_ui_host_header_validation_enabled: bool,
    /// True if authentication challenge for loopback address (127.0.0.1) should be disabled
    pub bypass_local_auth: bool,
    /// True if webui authentication should be bypassed for clients whose ip resides within (at least) one of the subnets on the whitelist
    pub bypass_auth_subnet_whitelist_enabled: bool,
    /// (White)list of ipv4/ipv6 subnets for which webui authentication should be bypassed; list entries are separated by commas
    pub bypass_auth_subnet_whitelist: String,
    /// Are using reverse proxies allowed?
    pub web_ui_reverse_proxy_enabled: bool,
    /// List of trusted proxies to access the webui. Separated by `;`
    pub web_ui_reverse_proxies_list: String,
    /// Should an alternative web ui be used?
    ///
    /// NOTE: This is not the same as a theme (`.qbttheme`)
    pub alternative_webui_enabled: bool,
    /// File path to the alternative WebUI
    pub alternative_webui_path: String,
    /// Does the server use HTTPS?
    pub use_https: bool,
    /// For API ≥ v2.0.1: Path to SSL keyfile
    pub web_ui_https_key_path: String,
    /// For API ≥ v2.0.1: Path to SSL certificate
    ///
    /// See https://httpd.apache.org/docs/current/ssl/ssl_faq.html#aboutcerts for information on certificates.
    pub web_ui_https_cert_path: String,
    /// For API ≥ v2.5.1: Enable custom http headers
    pub web_ui_use_custom_http_headers_enabled: bool,
    /// For API ≥ v2.5.1: List of custom http headers.
    ///
    /// Format: `Key: Value`. Separated by a new line
    pub web_ui_custom_http_headers: String,

    // ========== Dynamic DNS ==========
    /// Should the server DNS be updated dynamically?
    pub dyndns_enabled: bool,
    /// The DNS service that is in use.
    pub dyndns_service: DyndnsService,
    /// Username for DDNS service
    pub dyndns_username: String,
    /// Password for DDNS service
    pub dyndns_password: String,
    /// Your DDNS domain name
    pub dyndns_domain: String,

    // ========== RSS Settings ==========
    /// Enable processing of RSS feeds (Also enables fetching them, etc)
    pub rss_processing_enabled: bool,
    /// How long (in minutes) before the feeds are refreshed?
    pub rss_refresh_interval: i64,
    /// How long (in seconds) should be waited before a fetch request from the same host?
    pub rss_fetch_delay: i64,
    /// Maximum number of articles stored per feed.
    pub rss_max_articles_per_feed: u32,
    /// Enable auto-downloading of torrents from the RSS feeds
    pub rss_auto_downloading_enabled: bool,
    /// For API ≥ v2.5.1: Enable downloading of repack/proper Episodes
    pub rss_download_repack_proper_episodes: bool,
    /// For API ≥ v2.5.1: List of RSS Smart Episode Filters. Separated by a new line (`\n`)
    pub rss_smart_episode_filters: String,

    // ========== Tracker Settings ==========
    /// Enable automatic adding of trackers to new torrents
    pub add_trackers_enabled: bool,
    /// List of trackers to add to new torrent. Separated by a new line (`\n`)
    pub add_trackers: String,
    /// Enables automatic adding of trackers (from URL) to a new torrent
    pub add_trackers_from_url_enabled: bool,
    /// The URL to get the trackers from
    pub add_trackers_url: String,
    /// Read-only list of trackers automatiaclly updated from provided url in `add_trackers_url`. Separated by new line (`\n`)
    pub add_trackers_url_list: String,
    /// Timeout in seconds for a stopped announce request to trackers
    ///
    /// If the value is set to 0, the connections to trackers with the stopped event are suppressed.
    pub stop_tracker_timeout: i64,
    /// The IP address passed along to trackers. Requires qbittorrent restart
    ///
    /// More information: https://www.libtorrent.org/reference-Settings.html#announce_ip
    pub announce_ip: String,
    /// The port reported to trackers. 0 uses `listening_port`.
    pub announce_port: u16,
    /// Tells all trackers when either the IP or Port of our client changes.
    pub reannounce_when_address_changed: bool,
    /// Always announce to all tiers.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#announce_to_all_tiers for more information
    pub announce_to_all_tiers: bool,
    /// Always announce to all trackers in a tier.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#announce_to_all_trackers for more information
    pub announce_to_all_trackers: bool,
    /// Limits the number of concurrent HTTP tracker announces.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#max_concurrent_http_announces for more information.
    pub max_concurrent_http_announces: i64,

    // ========== Advanced Settings ==========
    /// Enables LibTorrent `piece_extent_affinity` setting.
    ///
    /// See https://libtorrent.org/single-page-ref.html#piece_extent_affinity for more information.
    pub enable_piece_extent_affinity: bool,
    /// Number of asynchronous I/O threads
    pub async_io_threads: u16,
    /// Keep x number of blocks outstanding to allow for faster re-checks at cost of memory.
    /// Value in MiB.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#checking_mem_usage for more information.
    pub checking_memory_use: u32,
    /// IP Address to bind to. Empty String means All addresses
    pub current_interface_address: String,
    /// Network Interface used
    pub current_network_interface: String,
    /// The name of the network interface used.
    pub current_interface_name: String,
    /// Disk cache used in MiB
    ///
    /// Only supported in LibTorrent < 2.0
    pub disk_cache: i64,
    /// Disk cache expiry interval in seconds
    ///
    /// Only supported in LibTorrent < 2.0
    pub disk_cache_ttl: i64,
    /// Is the OS allowed to cache read data from files?
    pub disk_io_read_mode: DiskRead,
    /// Is the OS allowed to cache write data to files?
    pub disk_io_write_mode: DiskWrite,
    /// Configure how libtorrent should perform disk I/O for reading and writing
    /// torrent data.
    ///
    /// See: https://www.libtorrent.org/single-page-ref.html#default-disk-io-constructor
    pub disk_io_type: DiskIOType,
    /// Maximum number of bytes that can wait in the I/O thread queue.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#max_queued_disk_bytes for more information.
    pub disk_queue_size: u64,
    /// Number of threads to use for piece hash verification
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#hashing_threads for more information.
    pub hashing_threads: u64,

    /// Enable qbittorrent to become a tracker.
    ///
    /// See https://github.com/qbittorrent/qBittorrent/wiki/How-to-use-qBittorrent-as-a-tracker for more information.
    pub enable_embedded_tracker: bool,
    /// The port used for the embedded tracker.
    pub embedded_tracker_port: u16,
    /// Enables the embedded tracker to use port forwarding.
    pub embedded_tracker_port_forwarding: bool,

    /// Enable coalesce read/writes
    ///
    /// Requires LibTorrent < 2.0.0
    pub enable_coalesce_read_write: bool,
    /// Allows multiple connections from the same IP address.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#allow_multiple_connections_per_ip for more information.
    pub enable_multi_connections_from_same_ip: bool,
    /// Don't make network requests to peers who ports are < 1024
    ///
    /// See https://libtorrent.org/single-page-ref.html#no_connect_privileged_ports for more information
    pub block_peers_on_privileged_ports: bool,
    /// Should Server-side request forgery (SSRF) be mitigated?
    pub ssrf_mitigation: bool,
    /// Makes the certificate of trackers and web seeds validated against the system certificate.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#validate_https_trackers for more information.
    pub validate_https_tracker_certificate: bool,
    /// Allows trackers/web seeds with an internationalised domain name.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#allow_idna for more information.
    pub idn_support_enabled: bool,

    /// Enable sending out a message with recent read pieces of a torrent in order to create a bias.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#suggest_mode for more information.
    pub enable_upload_suggestions: bool,
    /// The maximum number of files this session will keep open.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#file_pool_size for more information.
    pub file_pool_size: i64,
    /// Maximal outgoing port (0: Disabled)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#outgoing_port for more information
    pub outgoing_ports_max: u16,
    /// Minimal outgoing port (0: Disabled)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#outgoing_port for more information
    pub outgoing_ports_min: u16,
    /// Recheck the torrent upon the torrent being completed.
    pub recheck_completed_torrents: bool,
    /// True resolves peer countries
    pub resolve_peer_countries: bool,
    /// How often the `fastresume` file is saved (in minutes). 0 = disabled
    pub save_resume_data_interval: u64,
    /// How often the `statistics` file is saved (in minutes). 0 = disabled
    pub save_statistics_interval: u64,
    /// Send buffer low watermark in KiB. The minimum send buffer target size (includes bytes pending read from disk); for snappy seeding set this high enough to fit a few blocks.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_low_watermark for more information
    pub send_buffer_low_watermark: i64,
    /// Send buffer watermark in KiB. If the send buffer has fewer bytes than this value, another block will be read onto it; setting it too small hurts upload capacity, too large wastes memory.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_watermark for more information
    pub send_buffer_watermark: i64,
    /// Send buffer watermark factor in percent. The current upload rate to a peer is multiplied by this percentage to derive the watermark (clamped to send_buffer_watermark); higher values can improve throughput but may waste RAM.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_watermark_factor for more information
    pub send_buffer_watermark_factor: i64,
    /// Number of outstanding incoming connections to queue whilst not actively waiting for one to be accepted
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#listen_queue_size for more information
    pub socket_backlog_size: i64,
    /// Specify the buffer size on receiving peer sockets. 0 = system default.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_socket_buffer_size for more information
    pub socket_send_buffer_size: u64,
    /// Specify the buffer size on sending peer sockets. 0 = system default.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_socket_buffer_size for more information
    pub socket_receive_buffer_size: u64,

    /// Controls the bahviour of unchocking. How peers are selected
    ///
    /// Read more: https://transfercloud.io/blog/2024/02/26/what-is-torrent-chokin/
    pub upload_choking_algorithm: UploadChokingAlgorithm,
    /// Specify which algorithm to use to determine how many peers to unchoke.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#choking_algorithm for more information
    pub upload_slots_behavior: UploadSlotsBehavior,
    /// upnp lease duration specified in seconds (0: Permanent lease)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#upnp_lease_duration for more information
    pub upnp_lease_duration: u32,
    /// Specify the max number of nested lists/dictionaries in the data structure
    ///
    /// See https://www.libtorrent.org/reference-Bdecoding.html#bdecode() for more information
    pub bdecode_depth_limit: u64,
    /// The maximum number of tokens to be parsed from the buffer.
    ///
    /// See https://www.libtorrent.org/reference-Bdecoding.html#bdecode() for more information
    pub bdecode_token_limit: u64,
    /// Determinds the DSCP field in the IP header
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_dscp for more information
    ///
    /// Note: Qbittorrent uses the old version of this setting name.
    #[serde(rename = "peer_tos")]
    pub peer_dscp: u64,
    /// Percentage of peers to disconnect every turnover.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    pub peer_turnover: u64,
    /// The limit of the maximum limit before turnover starts
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    pub peer_turnover_cutoff: u64,
    /// How often the turnover occurs
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    pub peer_turnover_interval: u64,
    /// Affects certification validation and non-torrent activities.
    pub ignore_ssl_errors: bool,
    /// Should torrents use SSL connections
    pub ssl_enabled: bool,
    /// The port for SSL Torrents to connect to.
    pub ssl_listen_port: u16,
    /// What type of storage should be used to save the Fastresume files.
    pub resume_data_storage_type: FastResumeType,
    /// CSV of IP port-pairs added to the DHT Node if enabled
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#dht_bootstrap_nodes for more information
    pub dht_bootstrap_nodes: String,
    /// Maximum number of outstanding requests to send to a peer.
    ///
    /// See ttps://www.libtorrent.org/reference-Settings.html#max_out_request_queue for more information.
    pub request_queue_size: u64,

    // ========== File Log Settings ==========
    /// Enable storing logs to disk
    pub file_log_enabled: bool,
    /// The folder to store logs to
    pub file_log_path: String,
    /// Enable backing up log files when the file gets too big.
    pub file_log_backup_enabled: bool,
    /// How big should the log file be before being backed up (in KiB)
    pub file_log_max_size: u64,
    /// Should old logs be deleted?
    pub file_log_delete_old: u64,
    /// How old does the log need to be before being auto deleted?
    /// See `file_log_age_type`
    pub file_log_age: u64,
    /// The type of age the log needs to be before being deleted.
    pub file_log_age_type: FileAge,
    /// Log performance warnings as well as everything else.
    pub performance_warning: bool,

    // ========== I2P Settings ==========
    /// Is I2P (Invisible Internet Project) networking enabled?
    ///
    /// NOTE: This is experimental!
    pub i2p_enabled: bool,
    /// I2P SAM bridge address
    pub i2p_address: String,
    /// I2P SAM bridge port
    pub i2p_port: u16,
    /// Should I2P mixed mode be enabled? (allows both I2P and regular connections)
    pub i2p_mixed_mode: bool,
    /// Length of inbound I2P tunnels (number of hops)
    pub i2p_inbound_length: u64,
    /// Number of inbound I2P tunnels to create
    pub i2p_inbound_quantity: u64,
    /// Length of outbound I2P tunnels (number of hops)
    pub i2p_outbound_length: u64,
    /// Number of outbound I2P tunnels to create
    pub i2p_outbound_quantity: u64,
}

/// How the torrent content is laied out.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum ContentLayout {
    /// Does whatever the client says to do, which by default is Subfolder
    #[default]
    Original,
    /// In cases of batches, will create a separate subfolder automatically of the batch name.
    /// Example: `Save_path/Torrent_name/Torrent_files`
    Subfolder,
    /// In cases of batches, will just place them all in the save_path.
    /// Example: `Save_path/Torrent_files`
    NoSubfolder,
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
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum StopCondition {
    /// Don't stop and go straight to downloading
    #[default]
    None,
    /// Stop after receiving the metadata
    MetadataReceived,
    /// Stop after checking the files.
    FilesChecked,
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

/// Whether the `.torrent` file should be deleted after the torrent is added.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum AutoDeleteMode {
    /// Never delete the `.torrent` file.
    #[default]
    Never,
    /// Only delete the `.torrent` file if the torrent is added.
    IfAdded,
    /// Always delete the `.torrent` file.
    Always,
}

/// What to do when removing content files upon removing a torrent.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum TorrentDeletion {
    /// Erase from disk permanatly
    #[default]
    Delete,
    /// Attempts to move to Trash/Wastebin if possible.
    MoveToTrash,
}

impl std::fmt::Display for TorrentDeletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delete => write!(f, "Delete"),
            Self::MoveToTrash => write!(f, "MoveToTrash"),
        }
    }
}

/// Where to save the torrent if it's appears in the specified folder.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ScanDir {
    /// The folder that is monitored.
    MonitoredFolder,
    /// The default save path according to client settings.
    #[default]
    DefaultSavePath,
    /// A user-specified path.
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

/// What action should be taken when the seeding limit is reached?
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum SeedLimitActions {
    #[default]
    /// Stop the torrent upon the limit being reached
    StopTorrent = 0,
    /// Remove the torrent upon the limit being reached
    RemoveTorrent = 1,
    /// Remove the torrent and files upon the limit being reached
    RemoveTorrentFiles = 2,
    /// Make the torrent use the super seeding algorithm upon the limit being reached.
    TorrentSuperSeeding = 3,
}

/// Bittorrent protocols
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum BittorrentProtocol {
    /// To use both TCP and UTP
    TcpUtp = 0,
    #[default]
    /// To just use TCP
    Tcp = 1,
    /// To just use UTP
    Utp = 2,
}

/// Days on which the alternative speed limit schedule is applied.
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum SchedulerTime {
    /// Every day
    #[default]
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
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum Encryption {
    #[default]
    /// Allows encryption for file transfer.
    Allow = 0,
    /// Requires encryption for file transfer.
    Require = 1,
    /// Disables encryption for file transfer.
    Disable = 2,
}

/// The proxy protocol to use.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum ProxyType {
    /// Use no proxy at all
    #[default]
    None,
    /// Use HTTP Protocol
    Http,
    /// Use SOCKS5 protocol
    Socks5,
    /// Use SOCKS4 protocol
    Socks4,
}

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProxyType::None => "None",
                ProxyType::Http => "HTTP",
                ProxyType::Socks5 => "SOCKS5",
                ProxyType::Socks4 => "SOCKS4",
            }
        )
    }
}

/// Dyndns servcice types
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum DyndnsService {
    #[default]
    /// Uses DYN: https://account.dyn.com/
    Dydns = 0,
    /// Uses NO-IP: https://www.noip.com/
    Noip = 1,
}

/// Upload choking algorithm
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    #[default]
    /// Rotate unchoked peers in a round-robin fashion, giving each peer a fair chance to upload.
    RoundRobin = 0,
    /// Prefer peers that currently offer the fastest upload throughput to maximise overall upload performance.
    FastestUpload = 1,
    /// Use anti-leech heuristics to deprioritize peers that do not contribute, favouring peers that upload data back.
    AntiLeech = 2,
}

/// Algorithm to use for unchoking peers.
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    #[default]
    /// Unchokes a fixed amount of seeders
    Fixed = 0,
    /// Opens up slots based on the upload rate achieved to peers.
    UploadRate = 1,
}

/// μTP / TCP mixed-mode algorithm selection.
///
/// This setting controls how the client mixes uTP and TCP connections when both
/// protocols are available. It determines the preference or distribution of
/// connection attempts between the two transport protocols.
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    /// Prefer TCP connections when both TCP and uTP are available.
    ///
    /// When this mode is selected, the client will favour establishing TCP
    /// connections over uTP ones whenever possible.
    #[default]
    PreferTcp = 0,
    /// Distribute connections proportionally based on peer capabilities.
    ///
    /// In this mode the client attempts to balance or proportion connections
    /// between TCP and uTP according to peer availability and characteristics,
    /// rather than strictly preferring one protocol.
    PeerProportional = 1,
}

/// Struct containing information about an individual cookie.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
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

/// The type of results to get back whilst doing `get_directory_contents`
///
/// Hidden files are not included.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DirMode {
    /// Only return directories
    Dirs,
    /// Only return files
    Files,
    /// Returns everything
    #[default]
    All,
}

impl Display for DirMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DirMode::Dirs => "dirs",
                DirMode::Files => "files",
                DirMode::All => "all",
            }
        )
    }
}

/// Is the OS allowed to cache read data from files?
///
/// See https://www.libtorrent.org/reference-Settings.html#disk_io_read_mode for more information.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DiskRead {
    /// Allow the OS to cache read data.
    #[default]
    Enable,
    /// Don't Allow the OS to cache read data.
    Disable,
}

/// Is the OS allowed to cache write data to files?
///
/// See https://www.libtorrent.org/reference-Settings.html#disk_io_write_mode for more information.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DiskWrite {
    /// Allow the OS to cache write data.
    #[default]
    Enable,
    /// Don't Allow the OS to cache write data.
    Disable,
    /// FLushes pieces to disk as they complete validation.
    ///
    /// Requires LibTorrent >= 2.0.6
    WriteThrough,
}

/// Disk I/O constructor selection.
///
/// See: https://www.libtorrent.org/single-page-ref.html#default-disk-io-constructor
///
/// Enum values sourced from VueTorrent. Choose how libtorrent should perform
/// disk I/O for reading and writing torrent data.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DiskIOType {
    /// Use the library's default behaviour: memory-mapped I/O when available,
    /// otherwise fall back to POSIX-based I/O.
    #[default]
    Default,
    /// Use memory-mapped files (mmap) for disk I/O. This can improve performance
    /// by mapping file contents directly into memory.
    MemoryMappedFiles,
    /// Use POSIX-compliant file I/O methods. This variant selects a POSIX-style
    /// approach (e.g., pread/pwrite semantics) for compatibility on POSIX systems.
    PosixComplaint,
    /// Use single pread/pwrite operations for reads and writes.
    /// This is a more basic I/O method that performs single-shot read/write calls.
    SinglePReadWrite,
}

/// The type of age
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FileAge {
    /// After X days
    Day,
    /// After X months
    #[default]
    Month,
    /// After X years
    Year,
}

/// The file structure to use for the fastresume file.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FastResumeType {
    /// Use the "legacy" file format
    #[default]
    Files,
    /// Use the experimental SQLite Database format.
    SQLite,
}

impl std::fmt::Display for FastResumeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Files => "Legacy",
                Self::SQLite => "SQLite",
            }
        )
    }
}
