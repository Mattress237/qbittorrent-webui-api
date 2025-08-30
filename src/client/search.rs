use reqwest::multipart;

use crate::{
    error::Error,
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let id = client.search_start("Ubuntu 18.04", "legittorrents", "all")
    ///         .await
    ///         .unwrap();
    ///
    ///        println!("id: {}", id);
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.search_stop(1337).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let status = client.search_status(Some(1337))
    ///         .await
    ///         .unwrap();
    ///
    ///     for search in status {
    ///         println!("{:?}", search);
    ///     }
    /// }
    /// ```
    pub async fn search_status(&self, id: Option<u64>) -> Result<Vec<Search>, Error> {
        let mut query = vec![];
        if let Some(id) = id {
            query.push(("id", id));
        }

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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let searchs = client.search_results(1337, 10, None)
    ///         .await
    ///         .unwrap();
    ///
    ///     println!("{:#?}", searchs);
    ///
    /// }
    /// ```
    pub async fn search_results(
        &self,
        id: u64,
        limit: u64,
        offset: Option<i64>,
    ) -> Result<SearchResult, Error> {
        let mut query = vec![];
        query.push(("id", id.to_string()));
        query.push(("limit", limit.to_string()));
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }

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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.search_delete(1337).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let plugins = client.search_plugins()
    ///         .await
    ///         .unwrap();
    ///
    ///     for plugin in plugins {
    ///         println!("{:?}", plugin);
    ///     }
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.search_install_plugin(vec!["plugin"]).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.search_uninstall_plugin(vec!["plugin"]).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.search_enable_plugin(vec!["plugin"], true).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("url", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.search_update_plugin().await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn search_update_plugin(&self) -> Result<(), Error> {
        self._post("search/updatePlugins")
            .await?
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
