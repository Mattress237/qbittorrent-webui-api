use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{collections::HashMap, fmt::Display};

use crate::utilities::{deserializers, serializers};

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

/// Preferences response data object.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Builder)]
pub struct Preferences {
    // ======================================
    // ============== Behavior ==============
    // ======================================
    /// Currently selected language (e.g. en_GB for English)
    pub locale: String,

    /// Log performance warnings as well as everything else.
    pub performance_warning: bool,
    /// Should the client external IP be shown in the status bar?
    pub status_bar_external_ip: bool,
    /// Show a confirmation message before deleting a torrent. Does not apply to the API
    pub confirm_torrent_deletion: bool,
    /// Allow using of sub-categories. Sub-categories are made by adding `/` between the parent and child.
    // [qBittorrent #23585](https://github.com/qbittorrent/qBittorrent/pull/23585)
    #[cfg(feature = "qBittorrent-5_1")]
    pub use_subcategories: bool,

    // ========== File Log Settings ==========
    /// Enable storing logs to disk
    pub file_log_enabled: bool,
    /// The folder to store logs to
    pub file_log_path: String,
    /// Enable backing up log files when the file gets too big.
    pub file_log_backup_enabled: bool,
    /// How big should the log file be before being backed up (in KiB)
    pub file_log_max_size: i32,
    /// Should old logs be deleted?
    pub file_log_delete_old: bool,
    /// How old does the log need to be before being auto deleted?
    /// See `file_log_age_type`
    pub file_log_age: i32,
    /// The type of age the log needs to be before being deleted.
    pub file_log_age_type: FileAge,

    /// To delete content files alongside the torrent. A "cache" setting
    ///
    /// NOTE: In the webui, this setting is only visible by checking the icon next to the `Also remove content files`
    /// checkbox upon deleting a file.
    pub delete_torrent_content_files: bool,

    // ======================================
    // ============= Downloads ==============
    // ======================================

    // ======== When adding a torrent =======
    /// The default layout of the torrent content.
    #[serde(
        deserialize_with = "string_to_content_layout",
        serialize_with = "content_layout_to_string"
    )]
    pub torrent_content_layout: ContentLayout,
    /// To add new torrents to the top of the queue by default or not?
    pub add_to_top_of_queue: bool,
    /// Default setting for allowing new torrents to start automatically.
    ///
    /// - True = don't start downloading automatically.
    /// - False = Start downloading automatically.
    pub add_stopped_enabled: bool,
    /// When does the torrent stop
    #[serde(deserialize_with = "string_to_stop_condition")]
    #[serde(serialize_with = "stop_condition_to_string")]
    pub torrent_stop_condition: StopCondition,
    /// If the torrent exists, do we merge trackers with it or fail to add the torrent altogether?
    pub merge_trackers: bool,
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

    // ========= Saving Management ==========
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
    /// Default save path for torrents
    pub save_path: String,
    /// Should another path be used for incomplete torrents?
    pub temp_path_enabled: bool,
    /// The path to use for incomplete torrents.
    pub temp_path: String,
    /// Use the path specified by the category even if the torrent is in manual mode.
    pub use_category_paths_in_manual_mode: bool,
    /// Path to copy `.torrent` files to.
    pub export_dir: String,
    /// Path to copy `.torrent` files of completed downloads to.
    pub export_dir_fin: String,

    /// Directories to scan for `.torrent` files. `ScanDir` enum is used to
    /// overwrite the default save path of adding torrents.
    ///
    /// NOTE: This is marked as deprecated in the qBittorrent source code. It
    /// might be removed or replaced in future versions.
    pub scan_dirs: HashMap<String, ScanDir>,
    // ========== Excluded file names ==========
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
    #[serde(deserialize_with = "deserializers::string_to_vec_newline_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_newline_separated")]
    pub excluded_file_names: Vec<String>,

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
    /// Encryption type to use for SMTP notification emails.
    // [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
    #[cfg(feature = "qBittorrent-5_3")]
    pub mail_notification_encryption_type: SMTPEncryptionType,
    /// Does the smtp server require a secure connection (SSL)?
    // [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
    #[cfg(not(feature = "qBittorrent-5_3"))]
    pub mail_notification_ssl_enabled: bool,
    /// Does the smtp server require authentication?
    pub mail_notification_auth_enabled: bool,
    /// Username for smtp authentication
    pub mail_notification_username: String,
    /// Password for smtp authentication
    pub mail_notification_password: String,

    // ========== External Programs ==========
    /// Should an external program be run after a torrent has been added?
    pub autorun_on_torrent_added_enabled: bool,
    /// Program path/name/argumets to run if `autorun_on_torrent_added_enabled` is enabled.
    ///
    /// See `autorun_program` for the supported parameters, tips and examples.
    pub autorun_on_torrent_added_program: String,
    /// Enables Mark-of-the-web. Tells external programs that this file is potentiall unsafe.
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

    // =====================================
    // ============ Connection =============
    // =====================================
    /// Port for incoming connections
    pub listen_port: u16,
    /// Should torrents use SSL connections
    pub ssl_enabled: bool,
    /// The port for SSL Torrents to connect to.
    pub ssl_listen_port: u16,
    /// True if the port is randomly selected
    ///
    /// NOTE: This is marked as deprecated in the src file
    /// [Github referanse](https://github.com/qbittorrent/qBittorrent/blob/4f94eac235cefa8b83489cb3135dad87fcbed1e3/src/webui/api/appcontroller.cpp#L228)
    #[deprecated(note = "This field is deprecated upstream; retained here for compatibility.")]
    pub random_port: bool,
    /// Is UPnP/NAT-PMP enabled?
    pub upnp: bool,
    /// Maximum global number of simultaneous connections
    ///
    /// `-1` means disabled
    #[serde(rename = "max_connec")]
    pub max_connections: i32,
    /// Maximum number of simultaneous connections per torrent
    ///
    /// `-1` means disabled
    #[serde(rename = "max_connec_per_torrent")]
    pub max_connections_per_torrent: i32,
    /// Maximum number of upload slots
    ///
    /// `-1` means disabled
    pub max_uploads: i32,
    /// Maximum number of upload slots per torrent
    ///
    /// `-1` means disabled
    pub max_uploads_per_torrent: i32,

    // ============ I2P Settings =============
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
    /// Number of inbound I2P tunnels to create
    pub i2p_inbound_quantity: i32,
    /// Number of outbound I2P tunnels to create
    pub i2p_outbound_quantity: i32,
    /// Length of inbound I2P tunnels (number of hops)
    pub i2p_inbound_length: i32,
    /// Length of outbound I2P tunnels (number of hops)
    pub i2p_outbound_length: i32,

    // ========== Proxy Settings ==========
    /// The protocol to use for the proxy server
    #[serde(deserialize_with = "string_to_proxy_type")]
    #[serde(serialize_with = "proxy_type_to_string")]
    pub proxy_type: ProxyType,
    /// Proxy IP address or domain name
    pub proxy_ip: String,
    /// Proxy port
    pub proxy_port: u16,
    /// Does the proxy require authentication?
    ///
    /// Note: This does not apply when ProxyType is SOCKS4
    pub proxy_auth_enabled: bool,
    /// Username for proxy authentication
    pub proxy_username: String,
    /// Password for proxy authentication
    pub proxy_password: String,
    /// Should the proxyy be used for Hostname lookup?
    pub proxy_hostname_lookup: bool,
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

    // ========== IP Filtering ==========
    /// Should external IPs be filtered?
    pub ip_filter_enabled: bool,
    /// Path to IP filter file (.dat, .p2p, .p2b files are supported); path is separated by slashes
    pub ip_filter_path: String,
    /// Is the IP filter also applied to trackers?
    pub ip_filter_trackers: bool,
    /// List of banned IPs. Separated by new lines (`\n`)
    #[serde(rename = "banned_IPs")]
    #[serde(deserialize_with = "deserializers::string_to_vec_newline_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_newline_separated")]
    pub banned_ips: Vec<String>,

    // ===================================
    // ========= Speed Settings ==========
    // ===================================
    //
    // ====== Global Rate Limits =========
    /// Global download speed limit in KiB/s; 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub dl_limit: i32,
    /// Global upload speed limit in KiB/s; 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub up_limit: i32,
    /// Alternative global download speed limit in KiB/s. 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub alt_dl_limit: i32,
    /// Alternative global upload speed limit in KiB/s. 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    pub alt_up_limit: i32,
    /// Bittorrent Protocol to use (see list of possible values below)
    pub bittorrent_protocol: BittorrentProtocol,
    /// Should `dl_limit` be applied to uTP connections?
    pub limit_utp_rate: bool,
    /// Should `dl_limit` be applied to estimated TCP overhead? (e.g. service
    /// data such as packet headers)
    pub limit_tcp_overhead: bool,
    /// Should `dl_limit` be applied to peers on the LAN?
    pub limit_lan_peers: bool,

    // ========== Scheduling ==========
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

    // ======================================
    // ======== BitTorrent Settings =========
    // ======================================
    //
    // ========== Privacy Settings ==========
    // More info (can't work out where to place this): https://www.reddit.com/r/torrents/comments/jmcmx1/comment/gauf8kn/
    /// Is DHT (Decentrialized Network) enabled?
    ///
    /// See https://superuser.com/a/592244 for more info.
    pub dht: bool,
    /// Is PeX (Peer Exchange) enabled?
    pub pex: bool,
    /// Is LSD (Local Peer Discovery) enabled?
    pub lsd: bool,
    /// State of encryption for file transfer.
    pub encryption: Encryption,
    /// Is the user anonymous?
    ///
    /// WARNING: This doesn't grant enough protection on its own.
    /// See https://github.com/qbittorrent/qBittorrent/wiki/Anonymous-Mode for more information.
    pub anonymous_mode: bool,

    // ========== Queue Management ==========
    /// How many torrents can be actively checking at one time.
    pub max_active_checking_torrents: i32,
    /// Is torrent queuing enabled?
    pub queueing_enabled: bool,
    /// Maximum number of active simultaneous downloads
    pub max_active_downloads: i32,
    /// Maximum number of active simultaneous downloads and uploads
    pub max_active_torrents: i32,
    /// Maximum number of active simultaneous uploads
    pub max_active_uploads: i32,
    /// If true torrents w/o any activity (stalled ones) will not be counted towards `max_active_*` limits
    pub dont_count_slow_torrents: bool,
    /// Download rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_dl_rate_threshold: i32,
    /// Upload rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_ul_rate_threshold: i32,
    /// Seconds a torrent should be inactive before considered "slow"
    pub slow_torrent_inactive_timer: i32,

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
    pub max_seeding_time: i32,
    /// Should an action be taken once the torrent has been inactive (during
    /// seeding) for a certain amount of time?
    pub max_inactive_seeding_time_enabled: bool,
    /// Number of minutes for the torrent to be inactive (during seeding) before an action is taken.
    ///
    /// -1 = disabled (will also set `max_inactive_seeding_time_enabled` to false)
    pub max_inactive_seeding_time: i32,
    /// The mode for share limits.
    // [qBittorrent #24043](https://github.com/qbittorrent/qBittorrent/pull/24043)
    #[cfg(feature = "qBittorrent-5_3")]
    pub share_limits_mode: SeedLimitMode,
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

    // ============ Add Tracker  ============
    /// Enable automatic adding of trackers to new torrents
    pub add_trackers_enabled: bool,
    /// List of trackers to add to new torrent. Separated by a new line (`\n`)
    pub add_trackers: String,
    /// Enables automatic adding of trackers (from URL) to a new torrent
    pub add_trackers_from_url_enabled: bool,
    /// The URL to get the trackers from
    pub add_trackers_url: String,
    /// Read-only list of trackers automatiaclly updated from provided url in `add_trackers_url`.
    #[serde(deserialize_with = "deserializers::string_to_vec_newline_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_newline_separated")]
    pub add_trackers_url_list: Vec<String>,

    // ==================================
    // ============= Web UI =============
    // ==================================
    //
    // =========== HTTP Server ==========
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
    /// Does the server use HTTPS?
    pub use_https: bool,
    /// For API ≥ v2.0.1: Path to SSL certificate
    ///
    /// See https://httpd.apache.org/docs/current/ssl/ssl_faq.html#aboutcerts for information on certificates.
    pub web_ui_https_cert_path: String,
    /// For API ≥ v2.0.1: Path to SSL keyfile
    pub web_ui_https_key_path: String,

    // ========== WebUI Authentication ==========
    /// WebUI username
    pub web_ui_username: String,
    /// For API ≥ v2.3.0: Plaintext WebUI password. This field is write-only and cannot be read back.
    ///
    /// The password is used exclusively for setting or updating the WebUI password.
    #[serde(skip_serializing_if = "Option::is_none")] // Needed to acoid overwriting password
    pub web_ui_password: Option<String>,
    /// True if authentication challenge for loopback address (127.0.0.1) should be disabled
    pub bypass_local_auth: bool,
    /// True if webui authentication should be bypassed for clients whose ip resides within (at least) one of the subnets on the whitelist
    pub bypass_auth_subnet_whitelist_enabled: bool,
    /// (White)list of ipv4/ipv6 subnets for which webui authentication should be bypassed;
    #[serde(deserialize_with = "deserializers::string_to_vec_newline_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_newline_separated")]
    pub bypass_auth_subnet_whitelist: Vec<String>,
    /// Maximum number of authentication failures before WebUI access ban
    pub web_ui_max_auth_fail_count: i32,
    /// WebUI access ban duration in seconds
    pub web_ui_ban_duration: i32,
    /// Seconds until WebUI is automatically signed off
    pub web_ui_session_timeout: i32,

    // ========== API Key ==========
    /// API key for WebUI authentication
    // [qBittorrent #23212](https://github.com/qbittorrent/qBittorrent/pull/23212)
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub web_ui_api_key: String,

    // ========== Alternative WebUI ==========
    /// Should an alternative web ui be used?
    ///
    /// NOTE: This is not the same as a theme (`.qbttheme`)
    pub alternative_webui_enabled: bool,
    /// File path to the alternative WebUI
    pub alternative_webui_path: String,

    // ============== Security =============
    /// True if WebUI clickjacking protection is enabled
    pub web_ui_clickjacking_protection_enabled: bool,
    /// True if WebUI CSRF protection is enabled
    pub web_ui_csrf_protection_enabled: bool,
    /// True if WebUI cookie Secure flag is enabled (requires `use_https`)
    pub web_ui_secure_cookie_enabled: bool,
    /// Is WebUI Host header validated?
    pub web_ui_host_header_validation_enabled: bool,

    // ========== Custom HTTP Headers ==========
    /// For API ≥ v2.5.1: Enable custom http headers
    pub web_ui_use_custom_http_headers_enabled: bool,
    /// For API ≥ v2.5.1: List of custom http headers.
    ///
    /// Format: `Key: Value`.
    #[serde(deserialize_with = "deserializers::string_to_vec_newline_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_newline_separated")]
    pub web_ui_custom_http_headers: Vec<String>, //

    // ========== Reverse Proxy ==========
    /// Are using reverse proxies allowed?
    pub web_ui_reverse_proxy_enabled: bool,
    /// List of trusted proxies to access the webui. Separated by `;`
    #[serde(deserialize_with = "deserializers::string_to_vec_semicolon_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_semicolon_separated")]
    pub web_ui_reverse_proxies_list: Vec<String>,

    // =========== Dynamic DNS ==========
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

    // =================================
    // ========= RSS Settings ==========
    // =================================
    /// How long (in minutes) before the feeds are refreshed?
    pub rss_refresh_interval: i32,
    /// How long (in seconds) should be waited before a fetch request from the same host?
    pub rss_fetch_delay: i64,
    /// Maximum number of articles stored per feed.
    pub rss_max_articles_per_feed: i32,
    /// Enable processing of RSS feeds (Also enables fetching them, etc)
    pub rss_processing_enabled: bool,
    /// Enable auto-downloading of torrents from the RSS feeds
    pub rss_auto_downloading_enabled: bool,
    /// Enable downloading of repack/proper Episodes
    pub rss_download_repack_proper_episodes: bool,
    /// List of RSS Smart Episode Filters.
    #[serde(deserialize_with = "deserializers::string_to_vec_newline_separated")]
    #[serde(serialize_with = "serializers::vec_to_string_newline_separated")]
    pub rss_smart_episode_filters: Vec<String>,

    // =================================
    // ======= Advanced Settings =======
    // =================================
    //
    // ===== Preferences Settings ======
    /// What type of storage should be used to save the Fastresume files.
    #[serde(
        deserialize_with = "string_to_fast_resume_type",
        serialize_with = "fast_resume_type_to_string"
    )]
    pub resume_data_storage_type: FastResumeType,
    /// What to do with removing torrents.
    #[serde(
        deserialize_with = "string_to_torrent_deletion",
        serialize_with = "torrent_deletion_to_string"
    )]
    pub torrent_content_remove_option: TorrentDeletion,
    /// Memory usage limit of Physical RAM in MiB
    pub memory_working_set_limit: i32,
    /// Network Interface used
    pub current_network_interface: String,
    /// The name of the network interface used.
    pub current_interface_name: String,
    /// IP Address to bind to. Empty String means All addresses
    pub current_interface_address: String,
    /// How often the `fastresume` file is saved (in minutes). 0 = disabled
    pub save_resume_data_interval: i32,
    /// How often the `statistics` file is saved (in minutes). 0 = disabled
    pub save_statistics_interval: i32,
    /// The size limit of `.torrent` files
    pub torrent_file_size_limit: i64,
    /// Show a confirmation message before rechecking a torrent. Does not apply to the API
    pub confirm_torrent_recheck: bool,
    /// Recheck the torrent upon the torrent being completed.
    pub recheck_completed_torrents: bool,
    /// Customise the name of the app instance
    pub app_instance_name: String,
    /// How often should the UI refresh to get new updates? (in ms)
    pub refresh_interval: i32,
    /// Resolve peer host names
    // [qBittorrent #23708](https://github.com/qbittorrent/qBittorrent/pull/23708)
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub resolve_peer_host_names: bool,
    /// True resolves peer countries
    pub resolve_peer_countries: bool,
    /// Tells all trackers when either the IP or Port of our client changes.
    pub reannounce_when_address_changed: bool,
    /// Enable qbittorrent to become a tracker.
    ///
    /// See https://github.com/qbittorrent/qBittorrent/wiki/How-to-use-qBittorrent-as-a-tracker for more information.
    pub enable_embedded_tracker: bool,
    /// The port used for the embedded tracker.
    pub embedded_tracker_port: u16,
    /// Enables the embedded tracker to use port forwarding.
    pub embedded_tracker_port_forwarding: bool,
    ///
    /// Windows (MOTW) and Mac (quarantine) only
    pub mark_of_the_web: bool,
    /// Affects certification validation and non-torrent activities.
    pub ignore_ssl_errors: bool,
    /// Python executable path. For use in stuff like search engine plugins which require python.
    ///
    /// Will attempt to find and use a system wide one if nothing is specified.
    pub python_executable_path: String,

    // ===================================
    // ======= Libtorrent Settings =======
    // ===================================
    /// Specify the max number of nested lists/dictionaries in the data structure
    ///
    /// See https://www.libtorrent.org/reference-Bdecoding.html#bdecode() for more information
    pub bdecode_depth_limit: i32,
    /// The maximum number of tokens to be parsed from the buffer.
    ///
    /// See https://www.libtorrent.org/reference-Bdecoding.html#bdecode() for more information
    pub bdecode_token_limit: i32,
    /// Number of asynchronous I/O threads
    pub async_io_threads: i32,
    /// Number of threads to use for piece hash verification
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#hashing_threads for more information.
    pub hashing_threads: i32,
    /// The maximum number of files this session will keep open.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#file_pool_size for more information.
    pub file_pool_size: i32,
    /// Keep x number of blocks outstanding to allow for faster re-checks at cost of memory.
    /// Value in MiB.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#checking_mem_usage for more information.
    pub checking_memory_use: i32,
    /// Disk cache used in MiB
    ///
    /// Only supported in LibTorrent < 2.0
    pub disk_cache: i32,
    /// Disk cache expiry interval in seconds
    ///
    /// Only supported in LibTorrent < 2.0
    pub disk_cache_ttl: i32,
    /// Maximum number of bytes that can wait in the I/O thread queue.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#max_queued_disk_bytes for more information.
    pub disk_queue_size: i64,
    /// Configure how libtorrent should perform disk I/O for reading and writing
    /// torrent data.
    ///
    /// See: https://www.libtorrent.org/single-page-ref.html#default-disk-io-constructor
    pub disk_io_type: DiskIOType,
    /// Is the OS allowed to cache read data from files?
    pub disk_io_read_mode: DiskRead,
    /// Is the OS allowed to cache write data to files?
    pub disk_io_write_mode: DiskWrite,
    /// Enable coalesce read/writes
    ///
    /// Requires LibTorrent < 2.0.0
    pub enable_coalesce_read_write: bool,
    /// Enables LibTorrent `piece_extent_affinity` setting.
    ///
    /// See https://libtorrent.org/single-page-ref.html#piece_extent_affinity for more information.
    pub enable_piece_extent_affinity: bool,
    /// Enable sending out a message with recent read pieces of a torrent in order to create a bias.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#suggest_mode for more information.
    pub enable_upload_suggestions: bool,
    /// Send buffer watermark in KiB. If the send buffer has fewer bytes than this value, another block will be read onto it; setting it too small hurts upload capacity, too large wastes memory.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_watermark for more information
    pub send_buffer_watermark: i32,
    /// Send buffer low watermark in KiB. The minimum send buffer target size (includes bytes pending read from disk); for snappy seeding set this high enough to fit a few blocks.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_low_watermark for more information
    pub send_buffer_low_watermark: i32,
    /// Send buffer watermark factor in percent. The current upload rate to a peer is multiplied by this percentage to derive the watermark (clamped to send_buffer_watermark); higher values can improve throughput but may waste RAM.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_watermark_factor for more information
    pub send_buffer_watermark_factor: i64,
    /// Number of connection attempts made per second.
    ///
    /// If number < 0, a default of 200 will be made.
    pub connection_speed: i32,
    /// Allow outgoing connections when seeding
    // [qBittorrent #24158](https://github.com/qbittorrent/qBittorrent/pull/24158)
    #[cfg(feature = "qBittorrent-5_3")]
    pub seeding_outgoing_connections: bool,
    /// Specify the buffer size on receiving peer sockets. 0 = system default.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_socket_buffer_size for more information
    pub socket_send_buffer_size: i32,
    /// Specify the buffer size on sending peer sockets. 0 = system default.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_socket_buffer_size for more information
    pub socket_receive_buffer_size: i32,
    /// Number of outstanding incoming connections to queue whilst not actively waiting for one to be accepted
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#listen_queue_size for more information
    pub socket_backlog_size: i32,
    /// Minimal outgoing port (0: Disabled)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#outgoing_port for more information
    pub outgoing_ports_min: i32,
    /// Maximal outgoing port (0: Disabled)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#outgoing_port for more information
    pub outgoing_ports_max: i32,
    /// upnp lease duration specified in seconds (0: Permanent lease)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#upnp_lease_duration for more information
    pub upnp_lease_duration: i32,
    /// Determinds the DSCP field in the IP header
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_dscp for more information
    #[serde(rename = "peer_tos")]
    pub peer_dscp: i32,
    /// μTP-TCP mixed mode algorithm (see list of possible values below)
    pub utp_tcp_mixed_mode: UtpTcpMixedMode,
    /// Hostname resolver cache TTL
    // [qBittorrent #24158](https://github.com/qbittorrent/qBittorrent/pull/24158)
    #[cfg(not(feature = "qBittorrent-5_1"))]
    pub hostname_cache_ttl: i32,
    /// Allows trackers/web seeds with an internationalised domain name.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#allow_idna for more information.
    pub idn_support_enabled: bool,
    /// Allows multiple connections from the same IP address.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#allow_multiple_connections_per_ip for more information.
    pub enable_multi_connections_from_same_ip: bool,
    /// Makes the certificate of trackers and web seeds validated against the system certificate.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#validate_https_trackers for more information.
    pub validate_https_tracker_certificate: bool,
    /// Should Server-side request forgery (SSRF) be mitigated?
    pub ssrf_mitigation: bool,
    /// Don't make network requests to peers who ports are < 1024
    ///
    /// See https://libtorrent.org/single-page-ref.html#no_connect_privileged_ports for more information
    pub block_peers_on_privileged_ports: bool,
    /// Specify which algorithm to use to determine how many peers to unchoke.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#choking_algorithm for more information
    pub upload_slots_behavior: UploadSlotsBehavior,
    /// Controls the bahviour of unchocking. How peers are selected
    ///
    /// Read more: https://transfercloud.io/blog/2024/02/26/what-is-torrent-chokin/
    pub upload_choking_algorithm: UploadChokingAlgorithm,
    /// Always announce to all trackers in a tier.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#announce_to_all_trackers for more information
    pub announce_to_all_trackers: bool,
    /// Always announce to all tiers.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#announce_to_all_tiers for more information
    pub announce_to_all_tiers: bool,
    /// The IP address passed along to trackers. Requires qbittorrent restart
    ///
    /// More information: https://www.libtorrent.org/reference-Settings.html#announce_ip
    pub announce_ip: String,
    /// The port reported to trackers. 0 uses `listening_port`.
    pub announce_port: u16,
    /// Limits the number of concurrent HTTP tracker announces.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#max_concurrent_http_announces for more information.
    pub max_concurrent_http_announces: i32,
    /// Timeout in seconds for a stopped announce request to trackers
    ///
    /// If the value is set to 0, the connections to trackers with the stopped event are suppressed.
    pub stop_tracker_timeout: i32,
    /// Percentage of peers to disconnect every turnover.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    pub peer_turnover: i32,
    /// The limit of the maximum limit before turnover starts
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    pub peer_turnover_cutoff: i32,
    /// How often the turnover occurs
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    pub peer_turnover_interval: i32,
    /// Maximum number of outstanding requests to send to a peer.
    ///
    /// See ttps://www.libtorrent.org/reference-Settings.html#max_out_request_queue for more information.
    pub request_queue_size: i32,
    /// CSV of IP port-pairs added to the DHT Node if enabled
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#dht_bootstrap_nodes for more information
    pub dht_bootstrap_nodes: String,
}

/// Whether the `.torrent` file should be deleted after the torrent is added.
#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Default, PartialEq)]
pub enum AutoDeleteMode {
    /// Never delete the `.torrent` file.
    #[default]
    Never = 0,
    /// Only delete the `.torrent` file if the torrent is added.
    IfAdded = 1,
    /// Always delete the `.torrent` file.
    Always = 2,
}

/// The encryption type to use for SMTP notifications.
// [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
#[cfg(feature = "qBittorrent-5_3")]
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
pub enum SMTPEncryptionType {
    None = 0,
    STARTTLS = 1,
    #[default]
    SMTPS = 2,
}

/// The mode for share limits.
// [qBittorrent #24043](https://github.com/qbittorrent/qBittorrent/pull/24043)
#[cfg(feature = "qBittorrent-5_3")]
#[repr(i8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
pub enum SeedLimitMode {
    /// Use the default mode
    #[default]
    Default = -1, // special value

    /// Match any of the share limits
    MatchAny = 0,
    /// Match all of the share limits
    MatchAll = 1,
}

/// What action should be taken when the seeding limit is reached?
#[repr(i8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
pub enum SeedLimitActions {
    /// Use the default action
    #[default]
    Default = -1, // special value
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
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
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
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
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
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
pub enum Encryption {
    #[default]
    /// Allows encryption for file transfer.
    Allow = 0,
    /// Requires encryption for file transfer.
    Require = 1,
    /// Disables encryption for file transfer.
    Disable = 2,
}

/// Dyndns servcice types
#[repr(i8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
pub enum DyndnsService {
    /// No dynamic DNS service is selected.
    None = -1,
    #[default]
    /// Uses DYN: https://account.dyn.com/
    Dydns = 0,
    /// Uses NO-IP: https://www.noip.com/
    Noip = 1,
}

/// Upload choking algorithm
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
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
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
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
#[repr(u8)]
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Default, PartialEq)]
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

/// Is the OS allowed to cache read data from files?
///
/// See https://www.libtorrent.org/reference-Settings.html#disk_io_read_mode for more information.
#[repr(u8)]
#[derive(
    Debug, Default, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub enum DiskRead {
    /// Don't Allow the OS to cache read data.
    Disable = 0,
    /// Allow the OS to cache read data.
    #[default]
    Enable = 1,
}

/// Is the OS allowed to cache write data to files?
///
/// See https://www.libtorrent.org/reference-Settings.html#disk_io_write_mode for more information.
#[repr(u8)]
#[derive(
    Debug, Default, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub enum DiskWrite {
    /// Don't Allow the OS to cache write data.
    Disable = 0,
    /// Allow the OS to cache write data.
    #[default]
    Enable = 1,
    /// FLushes pieces to disk as they complete validation.
    WriteThrough = 2,
}

/// Disk I/O constructor selection.
///
/// See: https://www.libtorrent.org/single-page-ref.html#default-disk-io-constructor
///
/// Enum values sourced from VueTorrent. Choose how libtorrent should perform
/// disk I/O for reading and writing torrent data.
#[repr(u8)]
#[derive(
    Debug, Default, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub enum DiskIOType {
    /// Use the library's default behaviour: memory-mapped I/O when available,
    /// otherwise fall back to POSIX-based I/O.
    #[default]
    Default = 0,
    /// Use memory-mapped files (mmap) for disk I/O. This can improve performance
    /// by mapping file contents directly into memory.
    MemoryMappedFiles = 1,
    /// Use POSIX-compliant file I/O methods. This variant selects a POSIX-style
    /// approach (e.g., pread/pwrite semantics) for compatibility on POSIX systems.
    PosixComplaint = 2,
    /// Use single pread/pwrite operations for reads and writes.
    /// This is a more basic I/O method that performs single-shot read/write calls.
    SinglePReadWrite = 3,
    /// Use pread/pwrite operations for reads and writes.
    PreadPwrite = 4,
}

/// The type of age
#[repr(i8)]
#[derive(
    Debug, Default, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub enum FileAge {
    /// After X days
    Day = 0,
    /// After X months
    #[default]
    Month = 1,
    /// After X years
    Year = 2,
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

pub fn string_to_content_layout<'de, D>(deserializer: D) -> Result<ContentLayout, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::String(s) => match s.as_str() {
            "Original" => Ok(ContentLayout::Original),
            "Subfolder" => Ok(ContentLayout::Subfolder),
            "NoSubfolder" => Ok(ContentLayout::NoSubfolder),
            _ => Err(serde::de::Error::custom(format!(
                "invalid content layout: {}",
                s
            ))),
        },
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for content layout: {:?}",
            v
        ))),
    }
}

pub fn content_layout_to_string<S>(value: &ContentLayout, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
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

pub fn string_to_stop_condition<'de, D>(deserializer: D) -> Result<StopCondition, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::String(s) => match s.as_str() {
            "None" => Ok(StopCondition::None),
            "MetadataReceived" => Ok(StopCondition::MetadataReceived),
            "FilesChecked" => Ok(StopCondition::FilesChecked),
            _ => Err(serde::de::Error::custom(format!(
                "invalid stop condition: {}",
                s
            ))),
        },
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for stop condition: {:?}",
            v
        ))),
    }
}

pub fn stop_condition_to_string<S>(value: &StopCondition, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
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

pub fn string_to_torrent_deletion<'de, D>(deserializer: D) -> Result<TorrentDeletion, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::String(s) => match s.as_str() {
            "Delete" => Ok(TorrentDeletion::Delete),
            "MoveToTrash" => Ok(TorrentDeletion::MoveToTrash),
            _ => Err(serde::de::Error::custom(format!(
                "invalid torrent deletion: {}",
                s
            ))),
        },
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for torrent deletion: {:?}",
            v
        ))),
    }
}

pub fn torrent_deletion_to_string<S>(
    value: &TorrentDeletion,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
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
            Helper::Int(x) => Err(serde::de::Error::custom(format!(
                "unexpected value, expected 0, 1, or a string (path): {}",
                x
            ))),
        }
    }
}

impl Serialize for ScanDir {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ScanDir::MonitoredFolder => serializer.serialize_i8(0),
            ScanDir::DefaultSavePath => serializer.serialize_i8(1),
            ScanDir::OtherPath(s) => serializer.serialize_str(s),
        }
    }
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

pub fn string_to_proxy_type<'de, D>(deserializer: D) -> Result<ProxyType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::String(s) => match s.as_str() {
            "None" => Ok(ProxyType::None),
            "HTTP" => Ok(ProxyType::Http),
            "SOCKS5" => Ok(ProxyType::Socks5),
            "SOCKS4" => Ok(ProxyType::Socks4),
            _ => Err(serde::de::Error::custom(format!(
                "invalid proxy type: {}",
                s
            ))),
        },
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for proxy type: {:?}",
            v
        ))),
    }
}

pub fn proxy_type_to_string<S>(value: &ProxyType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
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

pub fn deserialize_dir_mode<'de, D>(deserializer: D) -> Result<DirMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::Number(n) => {
            let n = n.as_i64().unwrap_or(2);
            match n {
                0 => Ok(DirMode::Dirs),
                1 => Ok(DirMode::Files),
                2 => Ok(DirMode::All),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid dir mode number: {}",
                    n
                ))),
            }
        }
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for dir mode: {:?}",
            v
        ))),
    }
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

pub fn string_to_fast_resume_type<'de, D>(deserializer: D) -> Result<FastResumeType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;

    match v {
        JsonValue::String(s) => match s.as_str() {
            "Legacy" => Ok(FastResumeType::Files),
            "SQLite" => Ok(FastResumeType::SQLite),
            _ => Err(serde::de::Error::custom(format!(
                "invalid fast resume type: {}",
                s
            ))),
        },
        _ => Err(serde::de::Error::custom(format!(
            "unexpected type for fast resume type: {:?}",
            v
        ))),
    }
}

pub fn fast_resume_type_to_string<S>(
    value: &FastResumeType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
