use std::collections::HashMap;

use reqwest::multipart;

use crate::{
    error::Error,
    insert_optional,
    models::{Search, SearchPlugin, SearchResult},
};

impl super::Api {
    /// Start search
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#start-search)
    ///
    /// # Arguments
    /// * `pattern` - Pattern to search for (e.g. "Ubuntu 18.04")
    /// * `plugins` - Plugins to use for searching (e.g. "legittorrents"). Supports
    ///   multiple plugins separated by `|`. Also supports `all` and `enabled`
    /// * `category` - Categories to limit your search to (e.g. "legittorrents").
    ///   Available categories depend on the specified plugins. Also supports `all`
    ///
    pub async fn search_start(
        &self,
        pattern: &str,
        plugins: &str,
        category: &str,
    ) -> Result<u64, Error> {
        let form = multipart::Form::new()
            .text("pattern", pattern.to_string())
            .text("plugins", plugins.to_string())
            .text("category", category.to_string());

        let json: serde_json::Value = self
            ._post("search/start")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let id = json["id"].as_u64().ok_or_else(|| {
            Error::InvalidResponse("Missing or invalid 'id' in response".to_string())
        })?;

        Ok(id)
    }

    /// Stop search
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#start-search)
    ///
    /// # Arguments
    /// * `id` - ID of the search job
    ///
    pub async fn search_stop(&self, id: u64) -> Result<(), Error> {
        let form = multipart::Form::new().text("id", id.to_string());

        self._post("search/stop")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get search status
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-search-status)
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the search job. If `None`, all search jobs are returned
    ///
    pub async fn search_status(&self, id: Option<u64>) -> Result<Vec<Search>, Error> {
        let mut query = HashMap::new();
        insert_optional!(query, "id", id, |v: u64| v.to_string());

        let searches = self
            ._get("search/status")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Search>>()
            .await?;

        Ok(searches)
    }

    /// Get search results
    ///
    /// This function retrieves search results for a given search job.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-search-results)
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the search job.
    /// * `limit` - The maximum number of results to return. A value of `0` indicates no limit.
    /// * `offset` - The starting point for results. If negative, counts backwards (e.g., `-2` retrieves the 2 most recent results).
    ///
    pub async fn search_results(
        &self,
        id: u64,
        limit: u64,
        offset: Option<i64>,
    ) -> Result<SearchResult, Error> {
        let mut query = HashMap::new();
        query.insert("id", id.to_string());
        query.insert("limit", limit.to_string());
        insert_optional!(query, "offset", offset, |v: i64| v.to_string());

        let searches = self
            ._get("search/results")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<SearchResult>()
            .await?;

        Ok(searches)
    }

    /// Delete search
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#delete-search)
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the search job.
    ///
    pub async fn search_delete(&self, id: u64) -> Result<(), Error> {
        let form = multipart::Form::new().text("id", id.to_string());

        self._post("search/delete")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get search plugins
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-search-plugins)
    ///
    pub async fn search_plugins(&self) -> Result<Vec<SearchPlugin>, Error> {
        let plugins = self
            ._get("search/plugins")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<SearchPlugin>>()
            .await?;

        Ok(plugins)
    }

    /// Install search plugin
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#install-search-plugin)
    ///
    /// # Arguments
    /// * `sources` - List of Url and file path of the plugin to install.
    ///
    pub async fn search_install_plugin(&self, sources: Vec<&str>) -> Result<(), Error> {
        let form = multipart::Form::new().text("sources", sources.join("|"));

        self._post("search/installPlugin")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Uninstall search plugin
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#uninstall-search-plugin)
    ///
    /// # Arguments
    /// * `names` - List of names for torrents to uninstall.
    ///
    pub async fn search_uninstall_plugin(&self, names: Vec<&str>) -> Result<(), Error> {
        let form = multipart::Form::new().text("names", names.join("|"));

        self._post("search/uninstallPlugin")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Enable search plugin
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#enable-search-plugin)
    ///
    /// # Arguments
    /// * `names` - List of names for torrents to enable.
    ///
    pub async fn search_enable_plugin(&self, names: Vec<&str>, enable: bool) -> Result<(), Error> {
        let form = multipart::Form::new()
            .text("names", names.join("|"))
            .text("enable", enable.to_string());

        self._post("search/enablePlugin")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Update search plugins
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#update-search-plugins)
    ///
    pub async fn search_update_plugin(&self) -> Result<(), Error> {
        self._post("search/updatePlugins")
            .await?
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
