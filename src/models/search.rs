use serde::{Deserialize, Serialize};

/// Information about a search job.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Search {
    /// ID of the search job
    pub id: u64,
    /// Current status of the search job, indicating whether it is still running or has stopped.
    pub status: SearchStatus,
    /// Total number of results found. This number may continue to increase if the status is `Running`.
    pub total: u64,
}

/// The status of the search job
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum SearchStatus {
    /// The search job active looking for results
    Running,
    /// The search job has finished / failed / user stopped.
    #[default]
    Stopped,
}

/// Results of the provided search id.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchResult {
    /// List of `SearchResultItem`.
    pub results: Vec<SearchResultItem>,
    /// Current status of the search job, indicating whether it is still running or has stopped.
    pub status: SearchStatus,
    /// Total number of results found. This number may continue to increase if the status is `Running`.
    pub total: u64,
}

/// An individual item that has been found.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchResultItem {
    /// URL pointing to the torrent's description page on the source site.
    #[serde(rename = "descrLink")]
    pub descr_link: String,
    /// Name of the file associated with the torrent.
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// Size of the file in bytes.
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    /// URL for downloading the torrent, either as a `.torrent` file or a magnet link.
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    /// Number of leechers currently downloading the torrent.
    #[serde(rename = "nbLeechers")]
    pub leechers: u64,
    /// Number of seeders currently uploading the torrent.
    #[serde(rename = "nbSeeders")]
    pub seeders: u64,
    /// URL of the torrent site where the file is hosted.
    #[serde(rename = "siteUrl")]
    pub site_url: String,
}

/// Information about a specific plugin used to search.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchPlugin {
    /// Whether the plugin is enabled.
    pub enabled: bool,
    /// Full name of the plugin.
    #[serde(rename = "fullName")]
    pub full_name: String,
    /// Short name of the plugin.
    pub name: String,
    /// List of supported categories.
    #[serde(rename = "supportedCategories")]
    pub categories: Vec<SearchCategory>,
    /// URL of the torrent site
    pub url: String,
    /// Installed version of the plugin
    pub version: String,
}

/// Information about the category the search plugin comes under.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchCategory {
    /// Identifier for the category (e.g., "all", "books", "tv").
    pub id: String,
    /// Human-readable name of the category.
    pub name: String,
}
