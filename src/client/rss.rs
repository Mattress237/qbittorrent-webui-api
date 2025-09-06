use std::collections::HashMap;

use reqwest::multipart;

use crate::{
    error::Error,
    models::{RssFeedCollection, RssRule},
};

impl super::Api {
    /// Add RSS folder to help categorise torrents.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-folder)
    ///
    /// # Arguments
    ///
    /// * `path` - Full path of added folder. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_add_folder("folder").await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_add_folder(&self, path: &str) -> Result<(), Error> {
        let form = multipart::Form::new().text("path", path.to_string());

        self._post("rss/addFolder")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Add RSS feed
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#add-feed)
    ///
    /// # Arguments
    /// * `feed_url` - URL of RSS feed (e.g. "http://thepiratebay.org/rss//top100/200")
    /// * `path` - Full path of added feed. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     client.rss_add_folder("folder").await.unwrap();
    ///
    ///     let feed = "http://thepiratebay.org/rss//top100/200";
    ///     let result = client.rss_add_feed(feed, Some("folder")).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    #[allow(rustdoc::bare_urls)]
    pub async fn rss_add_feed(&self, feed_url: &str, path: Option<&str>) -> Result<(), Error> {
        let form = multipart::Form::new()
            .text("url", feed_url.to_string())
            .text("path", path.unwrap_or_default().to_string());

        self._post("rss/addFeed")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Removes folder or rss feed.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#remove-item)
    ///
    /// # Arguments
    /// * `path` - Full path of removed item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_remove_item("remove\\this\\item").await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_remove_item(&self, path: &str) -> Result<(), Error> {
        let form = multipart::Form::new().text("path", path.to_string());

        self._post("rss/removeItem")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Moves/renames folder or feed.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#move-item)
    ///
    /// # Arguments
    /// * `item_path` - Current full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    /// * `dest_path` - New full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_move_item("move\\it", "to\\here").await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_move_item(&self, item_path: &str, dest_path: &str) -> Result<(), Error> {
        let form = multipart::Form::new()
            .text("itemPath", item_path.to_string())
            .text("destPath", dest_path.to_string());

        self._post("rss/moveItem")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get all RSS items
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-all-items)
    ///
    /// # Arguments
    ///
    /// * `withData` - Also return the articles of every feed.
    ///
    /// # Returns
    /// A `HashMap` where the keys are feed names and the values are `RssFeedCollection` objects.
    /// The `RssFeedCollection` contains detailed information about each RSS feed, including its
    /// articles if `withData` is set to true. `RssFeedCollection` can have nested `RssFeedCollection`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let feed = client.rss_items(true).await.unwrap();
    ///
    ///     for item in feed {
    ///         println!("{:?}", item);
    ///     }
    /// }
    /// ```
    pub async fn rss_items(
        &self,
        with_data: bool,
    ) -> Result<HashMap<String, RssFeedCollection>, Error> {
        let query = vec![("withData", with_data)];

        let feed = self
            ._get("rss/items")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<HashMap<String, RssFeedCollection>>()
            .await?;

        Ok(feed)
    }

    /// Mark article / folder / feed as read
    ///
    /// If `article_id` is set only the article is marked as read otherwise the whole
    /// feed is going to be marked as read.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#mark-as-read)
    ///
    /// # Arguments
    /// * `path` - Current full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    /// * `article_id` - ID of article
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_mark_as_read("path\\to\\feed", None).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_mark_as_read(&self, path: &str, article_id: Option<u64>) -> Result<(), Error> {
        let mut form = multipart::Form::new().text("path", path.to_string());
        if let Some(article_id) = article_id {
            form = form.text("articleId", article_id.to_string());
        }

        self._post("rss/markAsRead")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Refreshes folder or feed.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#refresh-item)
    ///
    /// # Arguments
    /// * `item_path` - Current full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_refresh_item("path\\to\\feed").await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_refresh_item(&self, item_path: &str) -> Result<(), Error> {
        let form = multipart::Form::new().text("itemPath", item_path.to_string());

        self._post("rss/refreshItem")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set RSS rule
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-auto-downloading-rule)
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Punisher")
    /// * `def` - rule definition
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::models::RssRule;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let rule = RssRule::default();
    ///     let result = client.rss_set_rule("rule_name", rule).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_set_rule(&self, name: &str, def: RssRule) -> Result<(), Error> {
        let form = multipart::Form::new()
            .text("ruleName", name.to_string())
            .text("ruleDef", serde_json::to_string(&def)?);

        self._post("rss/setRule")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Rename RSS rule
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#rename-auto-downloading-rule)
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Punisher")
    /// * `new_name` - New rule name (e.g. "The Punisher")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_rename_rule("old_name", "new_name").await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_rename_rule(&self, name: &str, new_name: &str) -> Result<(), Error> {
        let form = multipart::Form::new()
            .text("ruleName", name.to_string())
            .text("newRuleName", new_name.to_string());

        self._post("rss/renameRule")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Remove RSS rule
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#remove-auto-downloading-rule)
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Punisher")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rss_remove_rule("rule_name").await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn rss_remove_rule(&self, name: &str) -> Result<(), Error> {
        let form = multipart::Form::new().text("ruleName", name.to_string());

        self._post("rss/removeRule")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get all RSS rules
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-all-auto-downloading-rules)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let rules = client.rss_rules().await.unwrap();
    ///
    ///     for rule in rules {
    ///         println!("{:?}", rule);
    ///     }
    /// }
    /// ```
    pub async fn rss_rules(&self) -> Result<HashMap<String, RssRule>, Error> {
        let rules = self
            ._get("rss/rules")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<HashMap<String, RssRule>>()
            .await?;

        Ok(rules)
    }

    /// Get all articles matching the given rule.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-all-articles-matching-a-rule)
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Linux")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let articles = client.rss_rules_articles("rule_name").await.unwrap();
    ///
    ///     for article in articles {
    ///         println!("{:?}", article);
    ///     }
    /// }
    /// ```
    pub async fn rss_rules_articles(
        &self,
        name: &str,
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        let query = vec![("ruleName", name)];

        let articles = self
            ._get("rss/matchingArticles")
            .await?
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<HashMap<String, Vec<String>>>()
            .await?;

        Ok(articles)
    }
}
