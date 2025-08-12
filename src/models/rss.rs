use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    uid: String,
    /// Title of the RSS feed.
    title: String,
    /// URL of the RSS feed.
    url: String,
    /// The last build date of the RSS feed.
    #[serde(rename = "lastBuildDate")]
    last_build_date: String,
    /// Indicates whether the RSS feed has encountered an error.
    #[serde(rename = "hasError")]
    has_error: bool,
    /// Indicates whether the RSS feed is currently loading.
    #[serde(rename = "isLoading")]
    is_loading: bool,
    /// List of articles associated with the RSS feed.
    articles: Vec<RssArticle>,
}

/// Represents an article within an RSS feed.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RssArticle {
    /// Identifier for the article.
    id: String,
    /// Title of the article.
    title: String,
    /// URL of the torrent associated with the article.
    #[serde(rename = "torrentURL")]
    torrent_url: String,
    /// Link to the article.
    link: String,
    /// Description of the article.
    description: String,
    /// Publication date of the article.
    date: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RssRule {
    /// Whether the rule is enabled
    enabled: bool,
    /// The substring that the torrent name must contain
    #[serde(rename = "mustContain")]
    must_contain: String,
    /// The substring that the torrent name must not contain
    #[serde(rename = "mustNotContain")]
    must_not_contain: String,
    /// Enable regex mode in "mustContain" and "mustNotContain"
    #[serde(rename = "useRegex")]
    use_regex: bool,
    /// Episode filter definition
    #[serde(rename = "episodeFilter")]
    episode_filter: String,
    /// Enable smart episode filter
    #[serde(rename = "smartFilter")]
    smart_filter: bool,
    /// The list of episode IDs already matched by smart filter
    #[serde(rename = "previouslyMatchedEpisodes")]
    previously_matched_episodes: Vec<String>,
    /// The feed URLs the rule applied to
    #[serde(rename = "affectedFeeds")]
    affected_feeds: Vec<String>,
    /// Ignore sunsequent rule matches
    #[serde(rename = "ignoreDays")]
    ignore_days: i64,
    /// The rule last match time
    #[serde(rename = "lastMatch")]
    last_match: String,
    /// Add matched torrent in paused mode
    #[serde(rename = "addPaused")]
    add_paused: bool,
    /// Assign category to the torrent
    #[serde(rename = "assignedCategory")]
    assigned_category: String,
    /// Save torrent to the given directory
    #[serde(rename = "savePath")]
    save_path: String,
}
