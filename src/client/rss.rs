use std::collections::HashMap;

use reqwest::multipart;

use crate::{
    error::Error,
    models::{RssFeedCollection, RssRule},
};

impl super::Api {
    /// Add RSS folder
    ///
    /// # Arguments
    ///
    /// * `path` - Full path of added folder. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    pub async fn rss_add_folder(&self, path: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("path", path.to_string());

        self._post("api/v2/rss/addFolder")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Add RSS feed
    ///
    /// # Arguments
    /// * `feed_url` - URL of RSS feed (e.g. "http://thepiratebay.org/rss//top100/200")
    /// * `path` - Full path of added feed. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    pub async fn rss_add_feed(&self, feed_url: &str, path: Option<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("url", feed_url.to_string());
        if let Some(path) = path {
            form = form.text("path", path.to_string());
        } else {
            form = form.text("path", "".to_string());
        }

        self._post("api/v2/rss/addFeed")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Remove RSS item
    ///
    /// Removes folder or feed.
    ///
    /// # Arguments
    /// * `path` - Full path of removed item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    pub async fn rss_remove_item(&self, path: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("path", path.to_string());

        self._post("api/v2/rss/removeItem")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Move RSS item
    ///
    /// Moves/renames folder or feed.
    ///
    /// # Arguments
    /// * `item_path` - Current full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    /// * `dest_path` - New full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay")
    pub async fn rss_move_item(&self, item_path: &str, dest_path: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("itemPath", item_path.to_string());
        form = form.text("destPath", dest_path.to_string());

        self._post("api/v2/rss/moveItem")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Get all RSS items
    ///
    /// # Arguments
    ///
    /// * `withData` - True if you need current feed articles
    ///
    /// # Returns
    /// A `HashMap` where the keys are feed names and the values are `RssFeedCollection` objects.
    /// The `RssFeedCollection` contains detailed information about each RSS feed, including its
    /// articles if `withData` is set to true. `RssFeedCollection` can have nested `RssFeedCollection`
    pub async fn rss_items(
        &self,
        with_data: bool,
    ) -> Result<HashMap<String, RssFeedCollection>, Error> {
        let mut query = vec![];
        query.push(("withData", with_data));

        let feed = self
            ._get("api/v2/rss/items")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<HashMap<String, RssFeedCollection>>()
            .await?;

        Ok(feed)
    }

    /// Mark as read
    ///
    /// If `article_id` is set only the article is marked as read otherwise the whole
    /// feed is going to be marked as read.
    ///
    /// # Arguments
    /// * `path` - Current full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    /// * `article_id` - ID of article
    pub async fn rss_mark_as_read(&self, path: &str, article_id: Option<u64>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("path", path.to_string());
        if let Some(article_id) = article_id {
            form = form.text("articleId", article_id.to_string());
        }

        self._post("api/v2/rss/markAsRead")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Refresh RSS item
    ///
    /// Refreshes folder or feed.
    ///
    /// # Arguments
    /// * `item_path` - Current full path of item. Use `\\` instead of `/` as the delimiter. (e.g. "The Pirate Bay\\Top100")
    pub async fn rss_refresh_item(&self, item_path: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("itemPath", item_path.to_string());

        self._post("api/v2/rss/refreshItem")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Set RSS rule
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Punisher")
    /// * `def` - rule definition
    pub async fn rss_set_rule(&self, name: &str, def: RssRule) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("ruleName", name.to_string());
        form = form.text("ruleDef", serde_json::to_string(&def)?);

        self._post("api/v2/rss/setRule")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Rename RSS rule
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Punisher")
    /// * `new_name` - New rule name (e.g. "The Punisher")
    pub async fn rss_rename_rule(&self, name: &str, new_name: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("ruleName", name.to_string());
        form = form.text("newRuleName", new_name.to_string());

        self._post("api/v2/rss/renameRule")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Remove RSS rule
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Punisher")
    pub async fn rss_remove_rule(&self, name: &str) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("ruleName", name.to_string());

        self._post("api/v2/rss/removeRule")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Get all RSS rules
    pub async fn rss_rules(&self) -> Result<HashMap<String, RssRule>, Error> {
        let url = self._build_url("api/v2/rss/rules").await?;

        let rules = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<HashMap<String, RssRule>>()
            .await?;

        Ok(rules)
    }

    /// Get all RSS rules articles
    ///
    /// # Arguments
    /// * `name` - Rule name (e.g. "Linux")
    pub async fn rss_rules_articles(
        &self,
        name: &str,
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        let mut query = vec![];
        query.push(("ruleName", name));

        let articles = self
            ._get("api/v2/rss/matchingArticles")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<HashMap<String, Vec<String>>>()
            .await?;

        Ok(articles)
    }
}
