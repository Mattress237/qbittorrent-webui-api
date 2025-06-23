use reqwest::multipart;

use crate::error::Error;

impl super::Api {
    /// Add RSS folder
    ///
    /// # Arguments
    ///
    /// * `path` - Full path of added folder (e.g. "The Pirate Bay\Top100")
    pub async fn rss_add_folder(&self, path: &str) -> Result<(), Error> {
        let url = self._build_url("api/v2/rss/addFolder").await?;

        let mut form = multipart::Form::new();
        form = form.text("path", path.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }
}
