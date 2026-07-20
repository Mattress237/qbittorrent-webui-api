use serde::{Deserialize, Serialize};

/// Information about a search job.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Search {
    /// ID of the search job
    pub id: i32,
    /// Current status of the search job, indicating whether it is still running
    /// or has stopped.
    pub status: SearchStatus,
    /// Total number of results found. This number may continue to increase if
    /// the status is `Running`.
    // technically it is a u64 in the source code, but we use i32 to be
    // consistent with other similar models that use i32 for total counts
    pub total: i32,
}

/// The status of the search job
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum SearchStatus {
    /// The search job active looking for results
    #[serde(rename = "Running")]
    Running,
    /// The search job has finished / failed / user stopped.
    #[default]
    #[serde(rename = "Stopped")]
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
    pub total: i32,
}

/// An individual item that has been found.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchResultItem {
    /// Name of the file associated with the torrent.
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// URL for downloading the torrent, either as a `.torrent` file or a magnet link.
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    /// Size of the file in bytes.
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    /// Number of leechers currently downloading the torrent.
    #[serde(rename = "nbLeechers")]
    pub leechers: i64,
    /// Number of seeders currently uploading the torrent.
    #[serde(rename = "nbSeeders")]
    pub seeders: i64,
    /// Name of the engine that found this torrent.
    #[serde(rename = "engineName")]
    pub engine_name: String,
    /// URL of the torrent site where the file is hosted.
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    /// URL pointing to the torrent's description page on the source site.
    #[serde(rename = "descrLink")]
    pub descr_link: String,
    /// Public data associated with the torrent.
    #[serde(rename = "pubData")]
    pub pub_data: i64,
}

/// Information about a specific plugin used to search.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchPlugin {
    /// Short name of the plugin.
    pub name: String,
    /// Installed version of the plugin
    pub version: String,
    /// Full name of the plugin.
    #[serde(rename = "fullName")]
    pub full_name: String,
    /// URL of the torrent site
    pub url: String,
    /// List of supported categories.
    #[serde(rename = "supportedCategories")]
    pub categories: Vec<SearchCategory>,
    /// Whether the plugin is enabled.
    pub enabled: bool,
}

/// Information about the category the search plugin comes under.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct SearchCategory {
    /// Identifier for the category (e.g., "all", "books", "tv").
    pub id: String,
    /// Human-readable name of the category.
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod deserialization {
        use super::*;
        use serde_json::json;

        #[test]
        fn search_from_json() {
            let data = json!({ "id": 1, "status": "Running", "total": 100 });
            let search: Search = serde_json::from_value(data).unwrap();
            assert_eq!(search.id, 1);
            assert_eq!(search.status, SearchStatus::Running);
            assert_eq!(search.total, 100);
        }

        #[test]
        fn search_status_from_string() {
            let data = json!({ "id": 1, "status": "Running", "total": 1 });
            let status: Search = serde_json::from_value(data).unwrap();
            assert_eq!(status.status, SearchStatus::Running);

            let data = json!({ "id": 1, "status": "Stopped", "total": 1 });
            let status: Search = serde_json::from_value(data).unwrap();
            assert_eq!(status.status, SearchStatus::Stopped);
        }

        #[test]
        fn search_result_from_json() {
            let data = json!({
                "status": "Running",
                "total": 100,
                "results": [
                    {
                        "fileName": "test.torrent",
                        "fileUrl": "http://example.com/test.torrent",
                        "fileSize": 100,
                        "nbLeechers": 0,
                        "nbSeeders": 0,
                        "engineName": "test",
                        "siteUrl": "http://example.com",
                        "descrLink": "http://example.com",
                        "pubData": 0
                    }
                ]
            });
            let result: SearchResult = serde_json::from_value(data).unwrap();
            assert_eq!(result.status, SearchStatus::Running);
            assert_eq!(result.total, 100);
            assert_eq!(result.results.len(), 1);
        }

        #[test]
        fn search_result_item_from_json() {
            let data = json!({
                "fileName": "test.torrent",
                "fileUrl": "http://example.com/test.torrent",
                "fileSize": 100,
                "nbLeechers": 0,
                "nbSeeders": 0,
                "engineName": "test",
                "siteUrl": "http://example.com",
                "descrLink": "http://example.com",
                "pubData": 0
            });
            let item: SearchResultItem = serde_json::from_value(data).unwrap();
            assert_eq!(item.file_name, "test.torrent");
            assert_eq!(item.file_url, "http://example.com/test.torrent");
            assert_eq!(item.file_size, 100);
        }

        #[test]
        fn search_plugin_from_json() {
            let json = json!({
                "name": "test",
                "version": "1.0.0",
                "fullName": "Test Plugin",
                "url": "http://example.com",
                "supportedCategories": [],
                "enabled": true
            });
            let plugin: SearchPlugin = serde_json::from_value(json).unwrap();
            assert_eq!(plugin.name, "test");
            assert_eq!(plugin.version, "1.0.0");
            assert_eq!(plugin.full_name, "Test Plugin");
            assert_eq!(plugin.url, "http://example.com");
            assert!(plugin.categories.is_empty());
            assert!(plugin.enabled);
        }

        #[test]
        fn search_category_from_json() {
            let json = json!({ "name": "test1", "id": "test2" });
            let category: SearchCategory = serde_json::from_value(json).unwrap();
            assert_eq!(category.name, "test1");
            assert_eq!(category.id, "test2");
        }
    }

    mod serialization {
        use super::*;

        #[test]
        fn search_to_json() {
            let search = Search {
                id: 1,
                status: SearchStatus::Running,
                total: 100,
            };
            let serialized = serde_json::to_string(&search).unwrap();
            assert_eq!(serialized, r#"{"id":1,"status":"Running","total":100}"#);
        }

        #[test]
        fn search_status_to_json() {
            let search = SearchStatus::Running;
            let serialized = serde_json::to_string(&search).unwrap();
            assert_eq!(serialized, r#""Running""#);
        }

        #[test]
        fn search_result_to_json() {
            let result = SearchResult {
                results: vec![],
                status: SearchStatus::Running,
                total: 100,
            };
            let serialized = serde_json::to_string(&result).unwrap();
            assert_eq!(
                serialized,
                r#"{"results":[],"status":"Running","total":100}"#
            );
        }

        #[test]
        fn search_result_item_to_json() {
            let item = SearchResultItem {
                file_name: "test.torrent".to_string(),
                file_url: "http://example.com/test.torrent".to_string(),
                file_size: 100,
                leechers: 0,
                seeders: 0,
                engine_name: "test".to_string(),
                site_url: "http://example.com".to_string(),
                descr_link: "http://example.com".to_string(),
                pub_data: 0,
            };
            let serialized = serde_json::to_string(&item).unwrap();
            assert_eq!(
                serialized,
                r#"{"fileName":"test.torrent","fileUrl":"http://example.com/test.torrent","fileSize":100,"nbLeechers":0,"nbSeeders":0,"engineName":"test","siteUrl":"http://example.com","descrLink":"http://example.com","pubData":0}"#
            );
        }

        #[test]
        fn search_plugin_to_json() {
            let plugin = SearchPlugin {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                full_name: "Test Plugin".to_string(),
                url: "http://example.com".to_string(),
                categories: vec![],
                enabled: true,
            };
            let serialized = serde_json::to_string(&plugin).unwrap();
            assert_eq!(
                serialized,
                r#"{"name":"test","version":"1.0.0","fullName":"Test Plugin","url":"http://example.com","supportedCategories":[],"enabled":true}"#
            );
        }

        #[test]
        fn search_category_to_json() {
            let category = SearchCategory {
                name: "test1".to_string(),
                id: "test2".to_string(),
            };
            let serialized = serde_json::to_string(&category).unwrap();
            assert_eq!(serialized, r#"{"id":"test2","name":"test1"}"#);
        }
    }
}
