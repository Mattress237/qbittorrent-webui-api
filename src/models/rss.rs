use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

use crate::models::ShareLimitAction;

/// This module defines structures for representing RSS feeds collections.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum RssFeedCollection {
    /// Represents a full RSS feed object containing detailed information about the feed.
    Feed(RssFeed),
    /// Represents a folder containing multiple RSS feeds.
    Folder(HashMap<String, RssFeedCollection>),
    /// Represents a short base object with minimal information about the feed.
    FeedBase(RssFeedBase),
}

/// Represents a base RSS feed object with minimal information.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RssFeedBase {
    /// Unique identifier for the RSS feed.
    uid: String,
    /// URL of the RSS feed.
    url: String,
}

/// Represents a detailed RSS feed object containing full information.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RssFeed {
    /// Unique identifier for the RSS feed.
    pub uid: String,
    /// Title of the RSS feed.
    pub title: String,
    /// URL of the RSS feed.
    pub url: String,
    /// The last build date of the RSS feed.
    #[serde(rename = "lastBuildDate")]
    pub last_build_date: String,
    /// Indicates whether the RSS feed has encountered an error.
    #[serde(rename = "hasError")]
    pub has_error: bool,
    /// Indicates whether the RSS feed is currently loading.
    #[serde(rename = "isLoading")]
    pub is_loading: bool,
    /// List of articles associated with the RSS feed.
    pub articles: Vec<RssArticle>,
}

/// Represents an article within an RSS feed.
///
// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/rss/rss_article.h
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RssArticle {
    /// Identifier for the article.
    pub id: String,
    /// Publication date of the article.
    pub date: String,
    /// Title of the article.
    pub title: String,
    /// Author of the article.
    #[serde(default = "default_is_empty_string")]
    pub author: String,
    /// Description of the article.
    pub description: String,
    /// URL of the torrent associated with the article.
    #[serde(rename = "torrentURL")]
    #[serde(default = "default_is_empty_string")]
    pub torrent_url: String,
    /// Link to the article.
    #[serde(default = "default_is_empty_string")]
    pub link: String,
    /// Whether the article has been read.
    #[serde(rename = "isRead")]
    #[serde(default = "default_is_false")]
    pub is_read: bool,
}

fn default_is_false() -> bool {
    false
}

fn default_is_empty_string() -> String {
    String::new()
}

/// Information about a specific rule to use within rss feeds.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RssRule {
    /// Whether the rule is enabled
    pub enabled: bool,
    /// The priority of the rule.
    pub priority: i32,
    /// Enable regex mode in "mustContain" and "mustNotContain"
    #[serde(rename = "useRegex")]
    pub use_regex: bool,
    /// The substring that the torrent name must contain
    #[serde(rename = "mustContain")]
    pub must_contain: String,
    /// The substring that the torrent name must not contain
    #[serde(rename = "mustNotContain")]
    pub must_not_contain: String,
    /// Episode filter definition
    #[serde(rename = "episodeFilter")]
    pub episode_filter: String,
    /// The feed URLs the rule applied to
    #[serde(rename = "affectedFeeds")]
    pub affected_feeds: Vec<String>,
    /// The rule last match time
    #[serde(rename = "lastMatch")]
    pub last_match: String,
    /// Ignore sunsequent rule matches
    #[serde(rename = "ignoreDays")]
    pub ignore_days: i32,
    /// Enable smart episode filter
    #[serde(rename = "smartFilter")]
    pub smart_filter: bool,
    /// The list of episode IDs already matched by smart filter
    #[serde(rename = "previouslyMatchedEpisodes")]
    pub previously_matched_episodes: Vec<String>,
    // ==== DEPRECATED ====
    // keeping it here just in case something breaks
    // server still sends the deprecated fields but they are handled by
    // the torrentParams object
    //
    // /// Add matched torrent in paused mode
    // #[serde(rename = "addPaused")]
    // pub add_paused: bool,
    // /// Torrent content layout
    // #[serde(rename = "torrentContentLayout")]
    // pub content_layout: String,
    // /// Save torrent to the given directory
    // #[serde(rename = "savePath")]
    // pub save_path: String,
    // /// Assign category to the torrent
    // #[serde(rename = "assignedCategory")]
    // pub assigned_category: String,
    // ==== END DEPRECATED ====
    #[serde(rename = "torrentParams")]
    pub torrent_params: AddTorrentParams,
}

// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/addtorrentparams.h
// Serialize function https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/addtorrentparams.cpp#L147
// share limit fields https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/sharelimits.h
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AddTorrentParams {
    pub category: String,
    pub tags: Vec<String>,
    pub save_path: String,
    pub download_path: String,
    pub operating_mode: OperatingMode,
    pub skip_checking: bool,
    pub upload_limit: i32,
    pub download_limit: i32,
    pub ratio_limit: f64,
    pub seeding_time_limit: i32,
    pub inactive_seeding_time_limit: i32,
    pub share_limit_action: ShareLimitAction,
    #[cfg(feature = "qBittorrent-5_3")]
    pub share_limit_mode: ShareLimitMode,
    pub ssl_certificate: String,
    pub ssl_private_key: String,
    pub ssl_dh_params: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OperatingMode {
    #[default]
    #[serde(rename = "Forced")]
    Forced,
    #[serde(rename = "AutoManaged")]
    AutoManaged,
}

impl Display for OperatingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatingMode::Forced => write!(f, "Forced"),
            OperatingMode::AutoManaged => write!(f, "AutoManaged"),
        }
    }
}

#[cfg(feature = "qBittorrent-5_3")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[repr(i8)]
pub enum ShareLimitMode {
    #[default]
    #[serde(rename = "Default")]
    Default = -1,
    #[serde(rename = "MatchAny")]
    MatchAny = 0,
    #[serde(rename = "MatchAll")]
    MatchAll = 1,
}

#[cfg(feature = "qBittorrent-5_3")]
impl Display for ShareLimitMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShareLimitMode::Default => write!(f, "Default"),
            ShareLimitMode::MatchAny => write!(f, "MatchAny"),
            ShareLimitMode::MatchAll => write!(f, "MatchAll"),
        }
    }
}

// https://github.com/qbittorrent/qBittorrent/blob/master/src/base/bittorrent/downloadpriority.h
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i8)]
pub enum DownloadPriority {
    Mixed = -1,
    Ignore = 0,
    Normal = 1,
    High = 6,
    Maximum = 7,
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    mod deserialization {
        use super::*;

        use serde_json::json;

        #[test]
        fn add_torrent_params() {
            #[allow(unused_mut)]
            let mut params = json!({
                "category": "cat1",
                "content_layout": "Subfolder",
                "download_limit": -1,
                "download_path": "",
                "inactive_seeding_time_limit": -2,
                "operating_mode": "AutoManaged",
                "ratio_limit": -2,
                "save_path": "dsdas",
                "seeding_time_limit": -2,
                "share_limit_action": "Default",
                "skip_checking": false,
                "ssl_certificate": "",
                "ssl_dh_params": "",
                "ssl_private_key": "",
                "stopped": true,
                "tags": ["tag1"],
                "upload_limit": -1,
                "use_auto_tmm": false
            });

            #[cfg(feature = "qBittorrent-5_3")]
            params
                .as_object_mut()
                .unwrap()
                .insert("share_limit_mode".to_string(), json!("Default"));

            let result = serde_json::from_value::<AddTorrentParams>(params);
            assert!(result.is_ok());
            let res: AddTorrentParams = result.unwrap();

            assert_eq!(res.category, "cat1");
            assert_eq!(res.tags, vec!["tag1"]);
            assert_eq!(res.save_path, "dsdas");
            assert_eq!(res.upload_limit, -1);
            assert_eq!(res.download_limit, -1);
            assert_eq!(res.ratio_limit, -2.0);
            assert_eq!(res.operating_mode, OperatingMode::AutoManaged);
            assert_eq!(res.share_limit_action, ShareLimitAction::Default);
            #[cfg(feature = "qBittorrent-5_3")]
            assert_eq!(res.share_limit_mode, ShareLimitMode::Default);
        }

        #[test]
        fn rss_rule() {
            #[allow(unused_mut)]
            let mut params = json!({
                "addPaused": true,
                "affectedFeeds": ["https://feeds.bbci.co.uk/news/rss.xml"],
                "assignedCategory": "cat1",
                "enabled": true,
                "episodeFilter": "",
                "ignoreDays": 4,
                "lastMatch": "",
                "mustContain": "",
                "mustNotContain": "",
                "previouslyMatchedEpisodes": [],
                "priority": 0,
                "savePath": "/place/to/save",
                "smartFilter": false,
                "torrentContentLayout": "Subfolder",
                "torrentParams": {
                    "category": "cat1",
                    "content_layout": "Subfolder",
                    "download_limit": -1,
                    "download_path": "",
                    "inactive_seeding_time_limit": -2,
                    "operating_mode": "AutoManaged",
                    "ratio_limit": -2,
                    "save_path": "/place/to/save",
                    "seeding_time_limit": -2,
                    "share_limit_action": "Default",
                    "skip_checking": false,
                    "ssl_certificate": "",
                    "ssl_dh_params": "",
                    "ssl_private_key": "",
                    "stopped": true,
                    "tags": ["tag1"],
                    "upload_limit": -1,
                    "use_auto_tmm": false
                },
                "useRegex": false
            });

            #[cfg(feature = "qBittorrent-5_3")]
            params.as_object_mut().unwrap().insert(
                "torrentParams.share_limit_mode".to_string(),
                json!("Default"),
            );

            let result = serde_json::from_value::<RssRule>(params);
            assert!(result.is_ok());
            let res: RssRule = result.unwrap();
            assert_eq!(
                res.torrent_params.operating_mode,
                OperatingMode::AutoManaged
            );
            assert_eq!(res.priority, 0);
            assert_eq!(res.torrent_params.category, "cat1");
            assert_eq!(res.must_contain, "");
            assert_eq!(res.must_not_contain, "");
            assert_eq!(res.smart_filter, false);

            #[cfg(feature = "qBittorrent-5_3")]
            assert_eq!(res.torrent_params.share_limit_mode, ShareLimitMode::Default);
        }
    }
}
