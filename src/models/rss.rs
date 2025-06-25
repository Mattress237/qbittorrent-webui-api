use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RssFeed {
    Item(String),
    Folder(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct RssFeedCollection {
    pub feeds: HashMap<String, RssFeed>,
}

#[derive(Debug, Deserialize, Serialize)]
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
