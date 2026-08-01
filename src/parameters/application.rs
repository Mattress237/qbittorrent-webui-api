use std::collections::HashMap;

use derive_builder::Builder;

use crate::{
    models::{
        AutoDeleteMode, BittorrentProtocol, ContentLayout, DiskIOType, DiskRead, DiskWrite,
        DyndnsService, Encryption, FastResumeType, FileAge, ProxyType, ScanDir, SchedulerTime,
        SeedLimitActions, StopCondition, TorrentDeletion, UploadChokingAlgorithm,
        UploadSlotsBehavior, UtpTcpMixedMode,
    },
    utilities::{deserializers, serializers},
};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, Builder)]
pub struct Preferences {
    // ======================================
    // ============== Behavior ==============
    // ======================================
    /// Currently selected language (e.g. en_GB for English)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub locale: Option<String>,

    /// Log performance warnings as well as everything else.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub performance_warning: Option<bool>,
    /// Should the client external IP be shown in the status bar?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub status_bar_external_ip: Option<bool>,
    /// Show a confirmation message before deleting a torrent. Does not apply to the API
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub confirm_torrent_deletion: Option<bool>,
    /// Allow using of sub-categories. Sub-categories are made by adding `/` between the parent and child.
    // [qBittorrent #23585](https://github.com/qbittorrent/qBittorrent/pull/23585)
    #[cfg(feature = "qBittorrent-5_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub use_subcategories: Option<bool>,

    // ========== File Log Settings ==========
    /// Enable storing logs to disk
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_enabled: Option<bool>,
    /// The folder to store logs to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_path: Option<String>,
    /// Enable backing up log files when the file gets too big.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_backup_enabled: Option<bool>,
    /// How big should the log file be before being backed up (in KiB)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_max_size: Option<i32>,
    /// Should old logs be deleted?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_delete_old: Option<bool>,
    /// How old does the log need to be before being auto deleted?
    /// See `file_log_age_type`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_age: Option<i32>,
    /// The type of age the log needs to be before being deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_log_age_type: Option<FileAge>,

    /// To delete content files alongside the torrent. A "cache" setting
    ///
    /// NOTE: In the webui, this setting is only visible by checking the icon next to the `Also remove content files`
    /// checkbox upon deleting a file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub delete_torrent_content_files: Option<bool>,

    // ======================================
    // ============= Downloads ==============
    // ======================================

    // ======== When adding a torrent =======
    /// The default layout of the torrent content.
    #[serde(
        deserialize_with = "crate::models::option_string_to_content_layout",
        serialize_with = "crate::models::option_content_layout_to_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_content_layout: Option<ContentLayout>,
    /// To add new torrents to the top of the queue by default or not?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_to_top_of_queue: Option<bool>,
    /// Default setting for allowing new torrents to start automatically.
    ///
    /// - True = don't start downloading automatically.
    /// - False = Start downloading automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_stopped_enabled: Option<bool>,
    /// When does the torrent stop
    #[serde(deserialize_with = "crate::models::option_string_to_stop_condition")]
    #[serde(serialize_with = "crate::models::option_stop_condition_to_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_stop_condition: Option<StopCondition>,
    /// If the torrent exists, do we merge trackers with it or fail to add the torrent altogether?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub merge_trackers: Option<bool>,
    /// When (and should) the `.torrent` file be deleted after added.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub auto_delete_mode: Option<AutoDeleteMode>,
    /// Should disk space be pre-allocated for all files?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub preallocate_all: Option<bool>,
    /// Should `.!qb` be added to incomplete files?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub incomplete_files_ext: Option<bool>,
    /// Should unchecked files be added to the `.unwanted` folder?
    ///
    /// See https://github.com/qbittorrent/qBittorrent/issues/13531 for an argument on the subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub use_unwanted_folder: Option<bool>,

    // ========= Saving Management ==========
    /// Should `Automatic Torrent Mangament` be enabled for new torrents by default?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub auto_tmm_enabled: Option<bool>,
    /// Should the torrent be relocated or switched to manual mode when category is changed?
    ///
    /// True = Relocated, False = Manual Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_changed_tmm_enabled: Option<bool>,
    /// Should the affected torrents be relocated or switched to manual mode when the default save/incomplete path is changed?
    ///
    /// True = Relocated, False = Manual Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub save_path_changed_tmm_enabled: Option<bool>,
    /// Should the affected torrents be relocated or switched to manual mode when it's category save path has changed?
    ///
    /// True = Relocated, False = Manual Mode
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub category_changed_tmm_enabled: Option<bool>,
    /// Default save path for torrents
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub save_path: Option<String>,
    /// Should another path be used for incomplete torrents?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub temp_path_enabled: Option<bool>,
    /// The path to use for incomplete torrents.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub temp_path: Option<String>,
    /// Use the path specified by the category even if the torrent is in manual mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub use_category_paths_in_manual_mode: Option<bool>,

    // ========== .torrent files backup management ==========
    /// Path to copy `.torrent` files to.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(not(feature = "qBittorrent-5_3"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub export_dir: Option<String>,
    /// Path to copy `.torrent` files of completed downloads to.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(not(feature = "qBittorrent-5_3"))]
    #[serde(rename = "export_dir_fin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub export_dir_finished: Option<String>,
    /// Enable backup of `.torrent` files.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_files_backup_enabled: Option<bool>,
    /// Path to backup `.torrent` files to.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_files_backup_dir: Option<String>,
    /// Enable backup of `.torrent` files of completed downloads.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_files_finished_backup_dir_enabled: Option<bool>,
    /// Path to backup `.torrent` files of completed downloads to.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_files_finished_backup_dir: Option<String>,
    /// Remove backup when removing the torrent.
    // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub remove_torrent_file_backup: Option<bool>,

    /// Directories to scan for `.torrent` files. `ScanDir` enum is used to
    /// overwrite the default save path of adding torrents.
    ///
    /// NOTE: This is marked as deprecated in the qBittorrent source code. It
    /// might be removed or replaced in future versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub scan_dirs: Option<HashMap<String, ScanDir>>,

    // ========== Excluded file names ==========
    /// Is the filename blacklist enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub excluded_file_names_enabled: Option<bool>,
    /// Blacklist filter file names from being downloaded from the torrent.
    ///
    /// Files matching any of this list will have the priority set to `Do Not Download` by default. (newline separator)
    ///
    /// The follow wildcards can be used:
    /// - *: Any character
    /// - ?: Any Single character
    /// - [...]: Sets of characters
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub excluded_file_names: Option<Vec<String>>,

    // ========== Email Notifications ==========
    /// Should email notifications be sent after a download is finished?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_enabled: Option<bool>,
    /// e-mail where notifications should originate from
    ///
    /// Client Default: qBittorrent_notification@example.com
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_sender: Option<String>,
    /// e-mail to send notifications to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_email: Option<String>,
    /// smtp server for e-mail notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_smtp: Option<String>,
    /// Encryption type to use for SMTP notification emails.
    // [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_encryption_type: Option<SMTPEncryptionType>,
    /// Does the smtp server require a secure connection (SSL)?
    // [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
    #[cfg(not(feature = "qBittorrent-5_3"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_ssl_enabled: Option<bool>,
    /// Does the smtp server require authentication?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_auth_enabled: Option<bool>,
    /// Username for smtp authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_username: Option<String>,
    /// Password for smtp authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mail_notification_password: Option<String>,

    // ========== External Programs ==========
    /// Should an external program be run after a torrent has been added?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub autorun_on_torrent_added_enabled: Option<bool>,
    /// Program path/name/argumets to run if `autorun_on_torrent_added_enabled` is enabled.
    ///
    /// See `autorun_program` for the supported parameters, tips and examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub autorun_on_torrent_added_program: Option<String>,
    /// Enables Mark-of-the-web. Tells external programs that this file is potentially unsafe.
    /// Should an external program be run after a torrent has completed?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub autorun_enabled: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub autorun_program: Option<String>,

    // =====================================
    // ============ Connection =============
    // =====================================
    /// Port for incoming connections
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub listen_port: Option<u16>,
    /// Should torrents use SSL connections
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ssl_enabled: Option<bool>,
    /// The port for SSL Torrents to connect to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ssl_listen_port: Option<u16>,
    /// True if the port is randomly selected
    ///
    /// NOTE: This is marked as deprecated in the src file
    /// [Github referanse](https://github.com/qbittorrent/qBittorrent/blob/4f94eac235cefa8b83489cb3135dad87fcbed1e3/src/webui/api/appcontroller.cpp#L228)
    #[deprecated(note = "This field is deprecated upstream; retained here for compatibility.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub random_port: Option<bool>,
    /// Is UPnP/NAT-PMP enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub upnp: Option<bool>,
    /// Maximum global number of simultaneous connections
    ///
    /// `-1` means disabled
    #[serde(rename = "max_connec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_connections: Option<i32>,
    /// Maximum number of simultaneous connections per torrent
    ///
    /// `-1` means disabled
    #[serde(rename = "max_connec_per_torrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_connections_per_torrent: Option<i32>,
    /// Maximum number of upload slots
    ///
    /// `-1` means disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_uploads: Option<i32>,
    /// Maximum number of upload slots per torrent
    ///
    /// `-1` means disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_uploads_per_torrent: Option<i32>,

    // ============ I2P Settings =============
    /// Is I2P (Invisible Internet Project) networking enabled?
    ///
    /// NOTE: This is experimental!
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_enabled: Option<bool>,
    /// I2P SAM bridge address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_address: Option<String>,
    /// I2P SAM bridge port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_port: Option<u16>,
    /// Should I2P mixed mode be enabled? (allows both I2P and regular connections)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_mixed_mode: Option<bool>,
    /// Number of inbound I2P tunnels to create
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_inbound_quantity: Option<i32>,
    /// Number of outbound I2P tunnels to create
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_outbound_quantity: Option<i32>,
    /// Length of inbound I2P tunnels (number of hops)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_inbound_length: Option<i32>,
    /// Length of outbound I2P tunnels (number of hops)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub i2p_outbound_length: Option<i32>,

    // ========== Proxy Settings ==========
    /// The protocol to use for the proxy server
    #[serde(deserialize_with = "crate::models::string_to_option_proxy_type")]
    #[serde(serialize_with = "crate::models::option_proxy_type_to_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_type: Option<ProxyType>,
    /// Proxy IP address or domain name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_ip: Option<String>,
    /// Proxy port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_port: Option<u16>,
    /// Does the proxy require authentication?
    ///
    /// Note: This does not apply when ProxyType is SOCKS4
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_auth_enabled: Option<bool>,
    /// Username for proxy authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_username: Option<String>,
    /// Password for proxy authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_password: Option<String>,
    /// Should the proxyy be used for Hostname lookup?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_hostname_lookup: Option<bool>,
    /// Should the proxyy be used for bittorrent purposes?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_bittorrent: Option<bool>,
    /// Should the proxyy be used for peer and web seed connections?
    ///
    /// Note: requires `proxy_bittorrent`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_peer_connections: Option<bool>,
    /// Should the proxyy be used for RSS purposes?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_rss: Option<bool>,
    /// Should the proxyy be used for General purposes?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub proxy_misc: Option<bool>,

    // ========== IP Filtering ==========
    /// Should external IPs be filtered?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ip_filter_enabled: Option<bool>,
    /// Path to IP filter file (.dat, .p2p, .p2b files are supported); path is separated by slashes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ip_filter_path: Option<String>,
    /// Is the IP filter also applied to trackers?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ip_filter_trackers: Option<bool>,
    /// List of banned IPs. Separated by new lines (`\n`)
    #[serde(rename = "banned_IPs")]
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub banned_ips: Option<Vec<String>>,

    // ===================================
    // ========= Speed Settings ==========
    // ===================================
    //
    // ====== Global Rate Limits =========
    /// Global download speed limit in KiB/s; 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dl_limit: Option<i32>,
    /// Global upload speed limit in KiB/s; 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub up_limit: Option<i32>,
    /// Alternative global download speed limit in KiB/s. 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub alt_dl_limit: Option<i32>,
    /// Alternative global upload speed limit in KiB/s. 0 means unlimited
    ///
    /// Note: Value is in Bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub alt_up_limit: Option<i32>,
    /// Bittorrent Protocol to use (see list of possible values below)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub bittorrent_protocol: Option<BittorrentProtocol>,
    /// Should `dl_limit` be applied to uTP connections?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub limit_utp_rate: Option<bool>,
    /// Should `dl_limit` be applied to estimated TCP overhead? (e.g. service
    /// data such as packet headers)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub limit_tcp_overhead: Option<bool>,
    /// Should `dl_limit` be applied to peers on the LAN?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub limit_lan_peers: Option<bool>,

    // ========== Scheduling ==========
    /// Should alternative limits be applied according to the schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub scheduler_enabled: Option<bool>,
    /// Scheduler starting hour
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub schedule_from_hour: Option<i8>,
    /// Scheduler starting minute
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub schedule_from_min: Option<i8>,
    /// Scheduler ending hour
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub schedule_to_hour: Option<i8>,
    /// Scheduler ending minute
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub schedule_to_min: Option<i8>,
    /// Days on which the schedule is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub scheduler_days: Option<SchedulerTime>,

    // ======================================
    // ======== BitTorrent Settings =========
    // ======================================
    //
    // ========== Privacy Settings ==========
    // More info (can't work out where to place this): https://www.reddit.com/r/torrents/comments/jmcmx1/comment/gauf8kn/
    /// Is DHT (Decentrialized Network) enabled?
    ///
    /// See https://superuser.com/a/592244 for more info.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dht: Option<bool>,
    /// Is PeX (Peer Exchange) enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub pex: Option<bool>,
    /// Is LSD (Local Peer Discovery) enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub lsd: Option<bool>,
    /// State of encryption for file transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub encryption: Option<Encryption>,
    /// Is the user anonymous?
    ///
    /// WARNING: This doesn't grant enough protection on its own.
    /// See https://github.com/qbittorrent/qBittorrent/wiki/Anonymous-Mode for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub anonymous_mode: Option<bool>,

    // ========== Queue Management ==========
    /// How many torrents can be actively checking at one time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_active_checking_torrents: Option<i32>,
    /// Is torrent queuing enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub queueing_enabled: Option<bool>,
    /// Maximum number of active simultaneous downloads
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_active_downloads: Option<i32>,
    /// Maximum number of active simultaneous downloads and uploads
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_active_torrents: Option<i32>,
    /// Maximum number of active simultaneous uploads
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_active_uploads: Option<i32>,
    /// If true torrents w/o any activity (stalled ones) will not be counted towards `max_active_*` limits
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dont_count_slow_torrents: Option<bool>,
    /// Download rate in KiB/s for a torrent to be considered "slow"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub slow_torrent_dl_rate_threshold: Option<i32>,
    /// Upload rate in KiB/s for a torrent to be considered "slow"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub slow_torrent_ul_rate_threshold: Option<i32>,
    /// Seconds a torrent should be inactive before considered "slow"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub slow_torrent_inactive_timer: Option<i32>,

    // ========== Seed Limits ==========
    /// Show an action be taken once the torrent ratio is achieved?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_ratio_enabled: Option<bool>,
    /// THe ratio to achieve to take an action.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_ratio: Option<f64>,
    /// Should an action be taken once the torrent has been seeding for a certain amount of time?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_seeding_time_enabled: Option<bool>,
    /// Number of minutes to seed a torrent before an action is taken
    ///
    /// -1 = disabled (will also set `max_seeding_time_enabled` to false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_seeding_time: Option<i32>,
    /// Should an action be taken once the torrent has been inactive (during
    /// seeding) for a certain amount of time?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_inactive_seeding_time_enabled: Option<bool>,
    /// Number of minutes for the torrent to be inactive (during seeding) before an action is taken.
    ///
    /// -1 = disabled (will also set `max_inactive_seeding_time_enabled` to false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_inactive_seeding_time: Option<i32>,
    /// The mode for share limits.
    // [qBittorrent #24043](https://github.com/qbittorrent/qBittorrent/pull/24043)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub share_limits_mode: Option<SeedLimitMode>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_ratio_act: Option<SeedLimitActions>,

    // ============ Add Tracker  ============
    /// Enable automatic adding of trackers to new torrents
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_trackers_enabled: Option<bool>,
    /// List of trackers to add to new torrent. Separated by a new line (`\n`)
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_trackers: Option<Vec<String>>,
    /// Enables automatic adding of trackers (from URL) to a new torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_trackers_from_url_enabled: Option<bool>,
    /// The URL to get the trackers from
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_trackers_url: Option<String>,
    /// Read-only list of trackers automatiaclly updated from provided url in `add_trackers_url`.
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub add_trackers_url_list: Option<Vec<String>>,

    // ==================================
    // ============= Web UI =============
    // ==================================
    //
    // =========== HTTP Server ==========
    /// Semicolon-separated list of domains to accept when performing Host header validation. Accepts: '*'
    ///
    /// Requires: `web_ui_host_header_validation_enabled` to be true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_domain_list: Option<String>,
    /// IP address to use for the WebUI. Accepts: '*'
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_address: Option<String>,
    /// WebUI port
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_port: Option<u16>,
    /// Use UPnP for port forwarding from the router
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_upnp: Option<bool>,
    /// Does the server use HTTPS?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub use_https: Option<bool>,
    /// For API ≥ v2.0.1: Path to SSL certificate
    ///
    /// See https://httpd.apache.org/docs/current/ssl/ssl_faq.html#aboutcerts for information on certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_https_cert_path: Option<String>,
    /// For API ≥ v2.0.1: Path to SSL keyfile
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_https_key_path: Option<String>,

    // ========== WebUI Authentication ==========
    /// WebUI username
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_username: Option<String>,
    /// For API ≥ v2.3.0: Plaintext WebUI password. This field is write-only and cannot be read back.
    ///
    /// The password is used exclusively for setting or updating the WebUI password.
    #[serde(skip_serializing_if = "Option::is_none")] // Needed to acoid overwriting password
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_password: Option<String>,
    /// True if authentication challenge for loopback address (127.0.0.1) should be disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub bypass_local_auth: Option<bool>,
    /// True if webui authentication should be bypassed for clients whose ip resides within (at least) one of the subnets on the whitelist
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub bypass_auth_subnet_whitelist_enabled: Option<bool>,
    /// (White)list of ipv4/ipv6 subnets for which webui authentication should be bypassed;
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub bypass_auth_subnet_whitelist: Option<Vec<String>>,
    /// Maximum number of authentication failures before WebUI access ban
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_max_auth_fail_count: Option<i32>,
    /// WebUI access ban duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_ban_duration: Option<i32>,
    /// Seconds until WebUI is automatically signed off
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_session_timeout: Option<i32>,
    // TODO: verison, text, feature flag, etc...
    #[cfg(feature = "qBittorrent-5_3")]
    // https://github.com/qbittorrent/qBittorrent/pull/24720
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_sessions_count_limit: Option<i32>,

    // ========== API Key ==========
    /// API key for WebUI authentication
    // [qBittorrent #23212](https://github.com/qbittorrent/qBittorrent/pull/23212)
    #[cfg(not(feature = "qBittorrent-5_1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_api_key: Option<String>,

    // ========== Alternative WebUI ==========
    /// Should an alternative web ui be used?
    ///
    /// NOTE: This is not the same as a theme (`.qbttheme`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub alternative_webui_enabled: Option<bool>,
    /// File path to the alternative WebUI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub alternative_webui_path: Option<String>,

    // ============== Security =============
    /// True if WebUI clickjacking protection is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_clickjacking_protection_enabled: Option<bool>,
    /// True if WebUI CSRF protection is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_csrf_protection_enabled: Option<bool>,
    /// True if WebUI cookie Secure flag is enabled (requires `use_https`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_secure_cookie_enabled: Option<bool>,
    /// Is WebUI Host header validated?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_host_header_validation_enabled: Option<bool>,

    // ========== Custom HTTP Headers ==========
    /// For API ≥ v2.5.1: Enable custom http headers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_use_custom_http_headers_enabled: Option<bool>,
    /// For API ≥ v2.5.1: List of custom http headers.
    ///
    /// Format: `Key: Value`.
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_custom_http_headers: Option<Vec<String>>,

    // ========== Reverse Proxy ==========
    /// Are using reverse proxies allowed?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_reverse_proxy_enabled: Option<bool>,
    /// List of trusted proxies to access the webui. Separated by `;`
    #[serde(deserialize_with = "deserializers::string_to_option_vec_semicolon_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_semicolon_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub web_ui_reverse_proxies_list: Option<Vec<String>>,

    // =========== Dynamic DNS ==========
    /// Should the server DNS be updated dynamically?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dyndns_enabled: Option<bool>,
    /// The DNS service that is in use.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dyndns_service: Option<DyndnsService>,
    /// Username for DDNS service
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dyndns_username: Option<String>,
    /// Password for DDNS service
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dyndns_password: Option<String>,
    /// Your DDNS domain name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dyndns_domain: Option<String>,

    // =================================
    // ========= RSS Settings ==========
    // =================================
    /// How long (in minutes) before the feeds are refreshed?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_refresh_interval: Option<i32>,
    /// How long (in seconds) should be waited before a fetch request from the same host?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_fetch_delay: Option<i64>,
    /// Maximum number of articles stored per feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_max_articles_per_feed: Option<i32>,
    /// Enable processing of RSS feeds (Also enables fetching them, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_processing_enabled: Option<bool>,
    /// Enable auto-downloading of torrents from the RSS feeds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_auto_downloading_enabled: Option<bool>,
    /// Enable downloading of repack/proper Episodes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_download_repack_proper_episodes: Option<bool>,
    /// List of RSS Smart Episode Filters.
    #[serde(deserialize_with = "deserializers::string_to_option_vec_newline_separated")]
    #[serde(serialize_with = "serializers::option_vec_to_string_newline_separated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub rss_smart_episode_filters: Option<Vec<String>>,

    // =================================
    // ======= Advanced Settings =======
    // =================================
    //
    // ===== Preferences Settings ======
    /// What type of storage should be used to save the Fastresume files.
    #[serde(
        deserialize_with = "crate::models::string_to_option_fast_resume_type",
        serialize_with = "crate::models::option_fast_resume_type_to_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub resume_data_storage_type: Option<FastResumeType>,
    /// What to do with removing torrents.
    #[serde(
        deserialize_with = "crate::models::string_to_option_torrent_deletion",
        serialize_with = "crate::models::option_torrent_deletion_to_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_content_remove_option: Option<TorrentDeletion>,
    /// Memory usage limit of Physical RAM in MiB
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub memory_working_set_limit: Option<i32>,
    /// Network Interface used
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub current_network_interface: Option<String>,
    /// The name of the network interface used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub current_interface_name: Option<String>,
    /// IP Address to bind to. Empty String means All addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub current_interface_address: Option<String>,
    /// How often the `fastresume` file is saved (in minutes). 0 = disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub save_resume_data_interval: Option<i32>,
    /// How often the `statistics` file is saved (in minutes). 0 = disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub save_statistics_interval: Option<i32>,
    /// The size limit of `.torrent` files
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub torrent_file_size_limit: Option<i64>,
    /// Show a confirmation message before rechecking a torrent. Does not apply to the API
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub confirm_torrent_recheck: Option<bool>,
    /// Recheck the torrent upon the torrent being completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub recheck_completed_torrents: Option<bool>,
    /// Customise the name of the app instance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub app_instance_name: Option<String>,
    /// How often should the UI refresh to get new updates? (in ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub refresh_interval: Option<i32>,
    /// Resolve peer host names
    // [qBittorrent #23708](https://github.com/qbittorrent/qBittorrent/pull/23708)
    #[cfg(not(feature = "qBittorrent-5_1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub resolve_peer_host_names: Option<bool>,
    /// True resolves peer countries
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub resolve_peer_countries: Option<bool>,
    /// Tells all trackers when either the IP or Port of our client changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub reannounce_when_address_changed: Option<bool>,

    /// Enable qbittorrent to become a tracker.
    ///
    /// See https://github.com/qbittorrent/qBittorrent/wiki/How-to-use-qBittorrent-as-a-tracker for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub enable_embedded_tracker: Option<bool>,
    /// The port used for the embedded tracker.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub embedded_tracker_port: Option<u16>,
    /// Enables the embedded tracker to use port forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub embedded_tracker_port_forwarding: Option<bool>,
    ///
    /// Windows (MOTW) and Mac (quarantine) only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub mark_of_the_web: Option<bool>,
    /// Affects certification validation and non-torrent activities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ignore_ssl_errors: Option<bool>,
    /// Python executable path. For use in stuff like search engine plugins which require python.
    ///
    /// Will attempt to find and use a system wide one if nothing is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub python_executable_path: Option<String>,

    // ===================================
    // ======= Libtorrent Settings =======
    // ===================================
    /// Specify the max number of nested lists/dictionaries in the data structure
    ///
    /// See https://www.libtorrent.org/reference-Bdecoding.html#bdecode() for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub bdecode_depth_limit: Option<i32>,
    /// The maximum number of tokens to be parsed from the buffer.
    ///
    /// See https://www.libtorrent.org/reference-Bdecoding.html#bdecode() for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub bdecode_token_limit: Option<i32>,
    /// Number of asynchronous I/O threads
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub async_io_threads: Option<i32>,
    /// Number of threads to use for piece hash verification
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#hashing_threads for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub hashing_threads: Option<i32>,
    /// The maximum number of files this session will keep open.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#file_pool_size for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub file_pool_size: Option<i32>,
    /// Keep x number of blocks outstanding to allow for faster re-checks at cost of memory.
    /// Value in MiB.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#checking_mem_usage for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub checking_memory_use: Option<i32>,
    /// Disk cache used in MiB
    ///
    /// Only supported in LibTorrent < 2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub disk_cache: Option<i32>,
    /// Disk cache expiry interval in seconds
    ///
    /// Only supported in LibTorrent < 2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub disk_cache_ttl: Option<i32>,
    /// Maximum number of bytes that can wait in the I/O thread queue.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#max_queued_disk_bytes for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub disk_queue_size: Option<i64>,
    /// Configure how libtorrent should perform disk I/O for reading and writing
    /// torrent data.
    ///
    /// See: https://www.libtorrent.org/single-page-ref.html#default-disk-io-constructor
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub disk_io_type: Option<DiskIOType>,
    /// Is the OS allowed to cache read data from files?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub disk_io_read_mode: Option<DiskRead>,
    /// Is the OS allowed to cache write data to files?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub disk_io_write_mode: Option<DiskWrite>,
    /// Enable coalesce read/writes
    ///
    /// Requires LibTorrent < 2.0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub enable_coalesce_read_write: Option<bool>,
    /// Enables LibTorrent `piece_extent_affinity` setting.
    ///
    /// See https://libtorrent.org/single-page-ref.html#piece_extent_affinity for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub enable_piece_extent_affinity: Option<bool>,
    /// Enable sending out a message with recent read pieces of a torrent in order to create a bias.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#suggest_mode for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub enable_upload_suggestions: Option<bool>,
    /// Send buffer watermark in KiB. If the send buffer has fewer bytes than this value, another block will be read onto it; setting it too small hurts upload capacity, too large wastes memory.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_watermark for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub send_buffer_watermark: Option<i32>,
    /// Send buffer low watermark in KiB. The minimum send buffer target size (includes bytes pending read from disk); for snappy seeding set this high enough to fit a few blocks.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_low_watermark for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub send_buffer_low_watermark: Option<i32>,
    /// Send buffer watermark factor in percent. The current upload rate to a peer is multiplied by this percentage to derive the watermark (clamped to send_buffer_watermark); higher values can improve throughput but may waste RAM.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_buffer_watermark_factor for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub send_buffer_watermark_factor: Option<i64>,
    /// Number of connection attempts made per second.
    ///
    /// If number < 0, a default of 200 will be made.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub connection_speed: Option<i32>,
    /// Allow outgoing connections when seeding
    // [qBittorrent #24158](https://github.com/qbittorrent/qBittorrent/pull/24158)
    #[cfg(feature = "qBittorrent-5_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub seeding_outgoing_connections: Option<bool>,
    /// Specify the buffer size on receiving peer sockets. 0 = system default.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_socket_buffer_size for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub socket_send_buffer_size: Option<i32>,
    /// Specify the buffer size on sending peer sockets. 0 = system default.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#send_socket_buffer_size for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub socket_receive_buffer_size: Option<i32>,
    /// Number of outstanding incoming connections to queue whilst not actively waiting for one to be accepted
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#listen_queue_size for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub socket_backlog_size: Option<i32>,
    /// Minimal outgoing port (0: Disabled)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#outgoing_port for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub outgoing_ports_min: Option<i32>,
    /// Maximal outgoing port (0: Disabled)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#outgoing_port for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub outgoing_ports_max: Option<i32>,
    /// upnp lease duration specified in seconds (0: Permanent lease)
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#upnp_lease_duration for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub upnp_lease_duration: Option<i32>,
    /// Determinds the DSCP field in the IP header
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_dscp for more information
    #[serde(rename = "peer_tos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub peer_dscp: Option<i32>,
    /// μTP-TCP mixed mode algorithm (see list of possible values below)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub utp_tcp_mixed_mode: Option<UtpTcpMixedMode>,
    /// Hostname resolver cache TTL
    // [qBittorrent #24158](https://github.com/qbittorrent/qBittorrent/pull/24158)
    #[cfg(not(feature = "qBittorrent-5_1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub hostname_cache_ttl: Option<i32>,
    /// Allows trackers/web seeds with an internationalised domain name.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#allow_idna for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub idn_support_enabled: Option<bool>,
    /// Allows multiple connections from the same IP address.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#allow_multiple_connections_per_ip for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub enable_multi_connections_from_same_ip: Option<bool>,
    // TODO: verison, text, feature flag, etc...
    #[cfg(feature = "qBittorrent-5_3")]
    // https://github.com/qbittorrent/qBittorrent/pull/24684
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub enable_multi_connections_from_same_peer_id: Option<bool>,
    /// Makes the certificate of trackers and web seeds validated against the system certificate.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#validate_https_trackers for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub validate_https_tracker_certificate: Option<bool>,
    /// Should Server-side request forgery (SSRF) be mitigated?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub ssrf_mitigation: Option<bool>,
    /// Don't make network requests to peers who ports are < 1024
    ///
    /// See https://libtorrent.org/single-page-ref.html#no_connect_privileged_ports for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub block_peers_on_privileged_ports: Option<bool>,
    /// Specify which algorithm to use to determine how many peers to unchoke.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#choking_algorithm for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub upload_slots_behavior: Option<UploadSlotsBehavior>,
    /// Controls the bahviour of unchocking. How peers are selected
    ///
    /// Read more: https://transfercloud.io/blog/2024/02/26/what-is-torrent-chokin/
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub upload_choking_algorithm: Option<UploadChokingAlgorithm>,
    /// Always announce to all trackers in a tier.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#announce_to_all_trackers for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub announce_to_all_trackers: Option<bool>,
    /// Always announce to all tiers.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#announce_to_all_tiers for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub announce_to_all_tiers: Option<bool>,
    /// The IP address passed along to trackers. Requires qbittorrent restart
    ///
    /// More information: https://www.libtorrent.org/reference-Settings.html#announce_ip
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub announce_ip: Option<String>,
    /// The port reported to trackers. 0 uses `listening_port`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub announce_port: Option<u16>,
    /// Limits the number of concurrent HTTP tracker announces.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#max_concurrent_http_announces for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub max_concurrent_http_announces: Option<i32>,
    /// Timeout in seconds for a stopped announce request to trackers
    ///
    /// If the value is set to 0, the connections to trackers with the stopped event are suppressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub stop_tracker_timeout: Option<i32>,
    /// Percentage of peers to disconnect every turnover.
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub peer_turnover: Option<i32>,
    /// The limit of the maximum limit before turnover starts
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub peer_turnover_cutoff: Option<i32>,
    /// How often the turnover occurs
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#peer_turnover for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub peer_turnover_interval: Option<i32>,
    /// Maximum number of outstanding requests to send to a peer.
    ///
    /// See ttps://www.libtorrent.org/reference-Settings.html#max_out_request_queue for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub request_queue_size: Option<i32>,
    /// CSV of IP port-pairs added to the DHT Node if enabled
    ///
    /// See https://www.libtorrent.org/reference-Settings.html#dht_bootstrap_nodes for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default = None)]
    pub dht_bootstrap_nodes: Option<String>,
}

// Convert model Preferences -> parameters::Preferences
impl From<&crate::models::Preferences> for Preferences {
    fn from(p: &crate::models::Preferences) -> Self {
        let p = p.clone();
        Preferences {
            locale: Some(p.locale),
            performance_warning: Some(p.performance_warning),
            status_bar_external_ip: Some(p.status_bar_external_ip),
            confirm_torrent_deletion: Some(p.confirm_torrent_deletion),
            // [qBittorrent #23585](https://github.com/qbittorrent/qBittorrent/pull/23585)
            #[cfg(feature = "qBittorrent-5_1")]
            use_subcategories: Some(p.use_subcategories),

            file_log_enabled: Some(p.file_log_enabled),
            file_log_path: Some(p.file_log_path),
            file_log_backup_enabled: Some(p.file_log_backup_enabled),
            file_log_max_size: Some(p.file_log_max_size),
            file_log_delete_old: Some(p.file_log_delete_old),
            file_log_age: Some(p.file_log_age),
            file_log_age_type: Some(p.file_log_age_type),

            delete_torrent_content_files: Some(p.delete_torrent_content_files),

            torrent_content_layout: Some(p.torrent_content_layout),
            add_to_top_of_queue: Some(p.add_to_top_of_queue),
            add_stopped_enabled: Some(p.add_stopped_enabled),
            torrent_stop_condition: Some(p.torrent_stop_condition),
            merge_trackers: Some(p.merge_trackers),
            auto_delete_mode: Some(p.auto_delete_mode),
            preallocate_all: Some(p.preallocate_all),
            incomplete_files_ext: Some(p.incomplete_files_ext),
            use_unwanted_folder: Some(p.use_unwanted_folder),
            auto_tmm_enabled: Some(p.auto_tmm_enabled),
            torrent_changed_tmm_enabled: Some(p.torrent_changed_tmm_enabled),
            save_path_changed_tmm_enabled: Some(p.save_path_changed_tmm_enabled),
            category_changed_tmm_enabled: Some(p.category_changed_tmm_enabled),
            save_path: Some(p.save_path),
            temp_path_enabled: Some(p.temp_path_enabled),
            temp_path: Some(p.temp_path),
            use_category_paths_in_manual_mode: Some(p.use_category_paths_in_manual_mode),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(not(feature = "qBittorrent-5_3"))]
            export_dir: Some(p.export_dir),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(not(feature = "qBittorrent-5_3"))]
            export_dir_finished: Some(p.export_dir_finished),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(feature = "qBittorrent-5_3")]
            torrent_files_backup_enabled: Some(p.torrent_files_backup_enabled),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(feature = "qBittorrent-5_3")]
            torrent_files_backup_dir: Some(p.torrent_files_backup_dir),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(feature = "qBittorrent-5_3")]
            torrent_files_finished_backup_dir_enabled: Some(
                p.torrent_files_finished_backup_dir_enabled,
            ),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(feature = "qBittorrent-5_3")]
            torrent_files_finished_backup_dir: Some(p.torrent_files_finished_backup_dir),
            // [pr 24641](https://github.com/qbittorrent/qBittorrent/pull/24641)
            #[cfg(feature = "qBittorrent-5_3")]
            remove_torrent_file_backup: Some(p.remove_torrent_file_backup),

            scan_dirs: Some(p.scan_dirs),

            excluded_file_names_enabled: Some(p.excluded_file_names_enabled),
            excluded_file_names: Some(p.excluded_file_names),

            mail_notification_enabled: Some(p.mail_notification_enabled),
            mail_notification_sender: Some(p.mail_notification_sender),
            mail_notification_email: Some(p.mail_notification_email),
            mail_notification_smtp: Some(p.mail_notification_smtp),
            // [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
            #[cfg(feature = "qBittorrent-5_3")]
            mail_notification_encryption_type: Some(p.mail_notification_encryption_type),
            // [qBittorrent #23838](https://github.com/qbittorrent/qBittorrent/pull/23838)
            #[cfg(not(feature = "qBittorrent-5_3"))]
            mail_notification_ssl_enabled: Some(p.mail_notification_ssl_enabled),
            mail_notification_auth_enabled: Some(p.mail_notification_auth_enabled),
            mail_notification_username: Some(p.mail_notification_username),
            mail_notification_password: Some(p.mail_notification_password),

            autorun_on_torrent_added_enabled: Some(p.autorun_on_torrent_added_enabled),
            autorun_on_torrent_added_program: Some(p.autorun_on_torrent_added_program),
            autorun_enabled: Some(p.autorun_enabled),
            autorun_program: Some(p.autorun_program),

            listen_port: Some(p.listen_port),
            ssl_enabled: Some(p.ssl_enabled),
            ssl_listen_port: Some(p.ssl_listen_port),
            #[allow(deprecated)]
            random_port: Some(p.random_port),
            upnp: Some(p.upnp),
            max_connections: Some(p.max_connections),
            max_connections_per_torrent: Some(p.max_connections_per_torrent),
            max_uploads: Some(p.max_uploads),
            max_uploads_per_torrent: Some(p.max_uploads_per_torrent),

            i2p_enabled: Some(p.i2p_enabled),
            i2p_address: Some(p.i2p_address),
            i2p_port: Some(p.i2p_port),
            i2p_mixed_mode: Some(p.i2p_mixed_mode),
            i2p_inbound_quantity: Some(p.i2p_inbound_quantity),
            i2p_outbound_quantity: Some(p.i2p_outbound_quantity),
            i2p_inbound_length: Some(p.i2p_inbound_length),
            i2p_outbound_length: Some(p.i2p_outbound_length),

            proxy_type: Some(p.proxy_type),
            proxy_ip: Some(p.proxy_ip),
            proxy_port: Some(p.proxy_port),
            proxy_auth_enabled: Some(p.proxy_auth_enabled),
            proxy_username: Some(p.proxy_username),
            proxy_password: Some(p.proxy_password),
            proxy_hostname_lookup: Some(p.proxy_hostname_lookup),
            proxy_bittorrent: Some(p.proxy_bittorrent),
            proxy_peer_connections: Some(p.proxy_peer_connections),
            proxy_rss: Some(p.proxy_rss),
            proxy_misc: Some(p.proxy_misc),

            ip_filter_enabled: Some(p.ip_filter_enabled),
            ip_filter_path: Some(p.ip_filter_path),
            ip_filter_trackers: Some(p.ip_filter_trackers),
            banned_ips: Some(p.banned_ips),

            dl_limit: Some(p.dl_limit),
            up_limit: Some(p.up_limit),
            alt_dl_limit: Some(p.alt_dl_limit),
            alt_up_limit: Some(p.alt_up_limit),
            bittorrent_protocol: Some(p.bittorrent_protocol),
            limit_utp_rate: Some(p.limit_utp_rate),
            limit_tcp_overhead: Some(p.limit_tcp_overhead),
            limit_lan_peers: Some(p.limit_lan_peers),

            scheduler_enabled: Some(p.scheduler_enabled),
            schedule_from_hour: Some(p.schedule_from_hour),
            schedule_from_min: Some(p.schedule_from_min),
            schedule_to_hour: Some(p.schedule_to_hour),
            schedule_to_min: Some(p.schedule_to_min),
            scheduler_days: Some(p.scheduler_days),

            dht: Some(p.dht),
            pex: Some(p.pex),
            lsd: Some(p.lsd),
            encryption: Some(p.encryption),
            anonymous_mode: Some(p.anonymous_mode),

            max_active_checking_torrents: Some(p.max_active_checking_torrents),
            queueing_enabled: Some(p.queueing_enabled),
            max_active_downloads: Some(p.max_active_downloads),
            max_active_torrents: Some(p.max_active_torrents),
            max_active_uploads: Some(p.max_active_uploads),
            dont_count_slow_torrents: Some(p.dont_count_slow_torrents),
            slow_torrent_dl_rate_threshold: Some(p.slow_torrent_dl_rate_threshold),
            slow_torrent_ul_rate_threshold: Some(p.slow_torrent_ul_rate_threshold),
            slow_torrent_inactive_timer: Some(p.slow_torrent_inactive_timer),

            max_ratio_enabled: Some(p.max_ratio_enabled),
            max_ratio: Some(p.max_ratio),
            max_seeding_time_enabled: Some(p.max_seeding_time_enabled),
            max_seeding_time: Some(p.max_seeding_time),
            max_inactive_seeding_time_enabled: Some(p.max_inactive_seeding_time_enabled),
            max_inactive_seeding_time: Some(p.max_inactive_seeding_time),
            // [qBittorrent #24043](https://github.com/qbittorrent/qBittorrent/pull/24043)
            #[cfg(feature = "qBittorrent-5_3")]
            share_limits_mode: Some(p.share_limits_mode),
            max_ratio_act: Some(p.max_ratio_act),

            add_trackers_enabled: Some(p.add_trackers_enabled),
            add_trackers: Some(p.add_trackers),
            add_trackers_from_url_enabled: Some(p.add_trackers_from_url_enabled),
            add_trackers_url: Some(p.add_trackers_url),
            add_trackers_url_list: Some(p.add_trackers_url_list),

            web_ui_domain_list: Some(p.web_ui_domain_list),
            web_ui_address: Some(p.web_ui_address),
            web_ui_port: Some(p.web_ui_port),
            web_ui_upnp: Some(p.web_ui_upnp),
            use_https: Some(p.use_https),
            web_ui_https_cert_path: Some(p.web_ui_https_cert_path),
            web_ui_https_key_path: Some(p.web_ui_https_key_path),

            web_ui_username: Some(p.web_ui_username),
            web_ui_password: p.web_ui_password,
            bypass_local_auth: Some(p.bypass_local_auth),
            bypass_auth_subnet_whitelist_enabled: Some(p.bypass_auth_subnet_whitelist_enabled),
            bypass_auth_subnet_whitelist: Some(p.bypass_auth_subnet_whitelist),
            web_ui_max_auth_fail_count: Some(p.web_ui_max_auth_fail_count),
            web_ui_ban_duration: Some(p.web_ui_ban_duration),
            web_ui_session_timeout: Some(p.web_ui_session_timeout),
            // https://github.com/qbittorrent/qBittorrent/pull/24720
            #[cfg(feature = "qBittorrent-5_3")]
            web_ui_sessions_count_limit: Some(p.web_ui_sessions_count_limit),

            // [qBittorrent #23212](https://github.com/qbittorrent/qBittorrent/pull/23212)
            #[cfg(not(feature = "qBittorrent-5_1"))]
            web_ui_api_key: Some(p.web_ui_api_key),

            alternative_webui_enabled: Some(p.alternative_webui_enabled),
            alternative_webui_path: Some(p.alternative_webui_path),

            web_ui_clickjacking_protection_enabled: Some(p.web_ui_clickjacking_protection_enabled),
            web_ui_csrf_protection_enabled: Some(p.web_ui_csrf_protection_enabled),
            web_ui_secure_cookie_enabled: Some(p.web_ui_secure_cookie_enabled),
            web_ui_host_header_validation_enabled: Some(p.web_ui_host_header_validation_enabled),

            web_ui_use_custom_http_headers_enabled: Some(p.web_ui_use_custom_http_headers_enabled),
            web_ui_custom_http_headers: Some(p.web_ui_custom_http_headers),

            web_ui_reverse_proxy_enabled: Some(p.web_ui_reverse_proxy_enabled),
            web_ui_reverse_proxies_list: Some(p.web_ui_reverse_proxies_list),

            dyndns_enabled: Some(p.dyndns_enabled),
            dyndns_service: Some(p.dyndns_service),
            dyndns_username: Some(p.dyndns_username),
            dyndns_password: Some(p.dyndns_password),
            dyndns_domain: Some(p.dyndns_domain),

            rss_refresh_interval: Some(p.rss_refresh_interval),
            rss_fetch_delay: Some(p.rss_fetch_delay),
            rss_max_articles_per_feed: Some(p.rss_max_articles_per_feed),
            rss_processing_enabled: Some(p.rss_processing_enabled),
            rss_auto_downloading_enabled: Some(p.rss_auto_downloading_enabled),
            rss_download_repack_proper_episodes: Some(p.rss_download_repack_proper_episodes),
            rss_smart_episode_filters: Some(p.rss_smart_episode_filters),

            resume_data_storage_type: Some(p.resume_data_storage_type),
            torrent_content_remove_option: Some(p.torrent_content_remove_option),
            memory_working_set_limit: Some(p.memory_working_set_limit),
            current_network_interface: Some(p.current_network_interface),
            current_interface_name: Some(p.current_interface_name),
            current_interface_address: Some(p.current_interface_address),
            save_resume_data_interval: Some(p.save_resume_data_interval),
            save_statistics_interval: Some(p.save_statistics_interval),
            torrent_file_size_limit: Some(p.torrent_file_size_limit),
            confirm_torrent_recheck: Some(p.confirm_torrent_recheck),
            recheck_completed_torrents: Some(p.recheck_completed_torrents),
            app_instance_name: Some(p.app_instance_name),
            refresh_interval: Some(p.refresh_interval),
            // [qBittorrent #23708](https://github.com/qbittorrent/qBittorrent/pull/23708)
            #[cfg(not(feature = "qBittorrent-5_1"))]
            resolve_peer_host_names: Some(p.resolve_peer_host_names),
            resolve_peer_countries: Some(p.resolve_peer_countries),
            reannounce_when_address_changed: Some(p.reannounce_when_address_changed),

            enable_embedded_tracker: Some(p.enable_embedded_tracker),
            embedded_tracker_port: Some(p.embedded_tracker_port),
            embedded_tracker_port_forwarding: Some(p.embedded_tracker_port_forwarding),
            mark_of_the_web: Some(p.mark_of_the_web),
            ignore_ssl_errors: Some(p.ignore_ssl_errors),
            python_executable_path: Some(p.python_executable_path),

            bdecode_depth_limit: Some(p.bdecode_depth_limit),
            bdecode_token_limit: Some(p.bdecode_token_limit),
            async_io_threads: Some(p.async_io_threads),
            hashing_threads: Some(p.hashing_threads),
            file_pool_size: Some(p.file_pool_size),
            checking_memory_use: Some(p.checking_memory_use),
            disk_cache: Some(p.disk_cache),
            disk_cache_ttl: Some(p.disk_cache_ttl),
            disk_queue_size: Some(p.disk_queue_size),
            disk_io_type: Some(p.disk_io_type),
            disk_io_read_mode: Some(p.disk_io_read_mode),
            disk_io_write_mode: Some(p.disk_io_write_mode),
            enable_coalesce_read_write: Some(p.enable_coalesce_read_write),
            enable_piece_extent_affinity: Some(p.enable_piece_extent_affinity),
            enable_upload_suggestions: Some(p.enable_upload_suggestions),
            send_buffer_watermark: Some(p.send_buffer_watermark),
            send_buffer_low_watermark: Some(p.send_buffer_low_watermark),
            send_buffer_watermark_factor: Some(p.send_buffer_watermark_factor),
            connection_speed: Some(p.connection_speed),
            // [qBittorrent #24158](https://github.com/qbittorrent/qBittorrent/pull/24158)
            #[cfg(feature = "qBittorrent-5_3")]
            seeding_outgoing_connections: Some(p.seeding_outgoing_connections),
            socket_send_buffer_size: Some(p.socket_send_buffer_size),
            socket_receive_buffer_size: Some(p.socket_receive_buffer_size),
            socket_backlog_size: Some(p.socket_backlog_size),
            outgoing_ports_min: Some(p.outgoing_ports_min),
            outgoing_ports_max: Some(p.outgoing_ports_max),
            upnp_lease_duration: Some(p.upnp_lease_duration),
            peer_dscp: Some(p.peer_dscp),
            utp_tcp_mixed_mode: Some(p.utp_tcp_mixed_mode),
            // [qBittorrent #24158](https://github.com/qbittorrent/qBittorrent/pull/24158)
            #[cfg(not(feature = "qBittorrent-5_1"))]
            hostname_cache_ttl: Some(p.hostname_cache_ttl),
            idn_support_enabled: Some(p.idn_support_enabled),
            enable_multi_connections_from_same_ip: Some(p.enable_multi_connections_from_same_ip),
            #[cfg(feature = "qBittorrent-5_3")]
            // https://github.com/qbittorrent/qBittorrent/pull/24684
            enable_multi_connections_from_same_peer_id: Some(p.enable_multi_connections_from_same_peer_id),
            validate_https_tracker_certificate: Some(p.validate_https_tracker_certificate),
            ssrf_mitigation: Some(p.ssrf_mitigation),
            block_peers_on_privileged_ports: Some(p.block_peers_on_privileged_ports),
            upload_slots_behavior: Some(p.upload_slots_behavior),
            upload_choking_algorithm: Some(p.upload_choking_algorithm),
            announce_to_all_trackers: Some(p.announce_to_all_trackers),
            announce_to_all_tiers: Some(p.announce_to_all_tiers),
            announce_ip: Some(p.announce_ip),
            announce_port: Some(p.announce_port),
            max_concurrent_http_announces: Some(p.max_concurrent_http_announces),
            stop_tracker_timeout: Some(p.stop_tracker_timeout),
            peer_turnover: Some(p.peer_turnover),
            peer_turnover_cutoff: Some(p.peer_turnover_cutoff),
            peer_turnover_interval: Some(p.peer_turnover_interval),
            request_queue_size: Some(p.request_queue_size),
            dht_bootstrap_nodes: Some(p.dht_bootstrap_nodes),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serialize_from_empty() {
        let json = "{}".to_string();
        let pref: Preferences = PreferencesBuilder::default().build().unwrap();

        assert_eq!(serde_json::to_string(&pref).unwrap(), json);
    }

    #[test]
    fn test_serialize_withe_locale() {
        let json = "{\"locale\":\"dddd\"}".to_string();
        let pref: Preferences = PreferencesBuilder::default()
            .locale("dddd")
            .build()
            .unwrap();

        assert_eq!(serde_json::to_string(&pref).unwrap(), json);
    }

    #[test]
    fn test_serialize_with_multiple_fields() {
        let pref: Preferences = PreferencesBuilder::default()
            .locale("dddd")
            .alt_dl_limit(445)
            .add_trackers(vec!["test1".to_string(), "test2".to_string()])
            .build()
            .unwrap();

        let json = serde_json::to_string(&pref).unwrap();

        assert!(json.contains("\"locale\":\"dddd\""));
        assert!(json.contains("\"alt_dl_limit\":445"));
        assert!(json.contains("\"add_trackers\":\"test1\\ntest2\""));
    }

    #[test]
    fn can_create_preferences_from_model_preferences() {
        let mut mod_pref = crate::models::Preferences::default();
        mod_pref.locale = "test".to_string();

        let pref: Preferences = Preferences::from(&mod_pref);
        assert_eq!(pref.locale, Some("test".to_string()));
    }
}
