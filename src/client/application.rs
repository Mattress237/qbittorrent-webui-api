use std::collections::HashMap;

use reqwest::multipart;

use crate::{
    error::Error,
    models::{self, BuildInfo, Cookie, DirMode},
    parameters,
};

impl super::Api {
    /// Get Qbittorrent application version
    ///
    /// The response is a string with the application version, e.g. `v5.1.0`
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-application-version)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let version = client.version().await.unwrap();
    ///
    ///     println!("{}", version);
    /// }
    /// ```
    pub async fn version(&self) -> Result<String, Error> {
        let version = self
            ._get("app/version")
            .await?
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(version)
    }

    /// Get WebAPI version
    ///
    /// The response is a string with the WebAPI version, e.g. `2.11.4`
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-api-version)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let version = client.webapi_version().await.unwrap();
    ///
    ///     println!("{}", version);
    /// }
    /// ```
    pub async fn webapi_version(&self) -> Result<String, Error> {
        let version = self
            ._get("app/webapiVersion")
            .await?
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(version)
    }

    /// Get build info
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-build-info)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let build = client.build_info().await.unwrap();
    ///
    ///     println!("{:#?}", build);
    /// }
    /// ```
    pub async fn build_info(&self) -> Result<BuildInfo, Error> {
        let build_info = self
            ._get("app/buildInfo")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<BuildInfo>()
            .await?;

        Ok(build_info)
    }

    /// Shutdown Qbittorent application
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#shutdown-application)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     client.shutdown().await.unwrap();
    /// }
    /// ```
    pub async fn shutdown(&self) -> Result<(), Error> {
        self._post("app/shutdown")
            .await?
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get application preferences
    ///
    /// Returns struct with several fields representing the application's settings.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-application-preferences)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let preferences = client.preferences().await.unwrap();
    ///
    ///     println!("{:#?}", preferences);
    /// }
    /// ```
    pub async fn preferences(&self) -> Result<models::Preferences, Error> {
        let preferences = self
            ._get("app/preferences")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<models::Preferences>()
            .await?;

        Ok(preferences)
    }

    /// Set application preferences
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-application-preferences)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::parameters::PreferencesBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let preferences = PreferencesBuilder::default()
    ///         .locale("en")
    ///         .build()
    ///         .unwrap();
    ///
    ///     let resulte = client.set_preferences(preferences).await;
    ///
    ///     assert!(resulte.is_ok());
    /// }
    /// ```
    pub async fn set_preferences(&self, preferences: parameters::Preferences) -> Result<(), Error> {
        let form = multipart::Form::new().text("json", serde_json::to_string(&preferences)?);

        self._post("app/setPreferences")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Get default save path
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-default-save-path)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let save_path = client.default_save_path().await.unwrap();
    ///
    ///     println!("{}", save_path);
    /// }
    /// ```
    pub async fn default_save_path(&self) -> Result<String, Error> {
        let preferences = self
            ._get("app/defaultSavePath")
            .await?
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(preferences)
    }

    /// Get cookies
    ///
    /// Retrieves cookies used for downloading .torrent files and RSS feeds.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-cookies)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let cookies = client.cookies().await.unwrap();
    ///
    ///     for cookie in cookies {
    ///         println!("{:?}", cookie);
    ///     }
    /// }
    /// ```
    pub async fn cookies(&self) -> Result<Vec<Cookie>, Error> {
        let cookies = self
            ._get("app/cookies")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Cookie>>()
            .await?;

        Ok(cookies)
    }

    /// Set cookies
    ///
    /// Sets the cookies used for downloading .torrent files and RSS feeds.
    ///
    /// This will overwrite all the cookies.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#set-cookies)
    ///
    /// # Arguments
    ///
    /// * `cookies` - A list of cookies to be set.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::models::Cookie;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let cookie = Cookie::default();
    ///     let result = client.set_cookies(vec![cookie]).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn set_cookies(&self, cookies: Vec<Cookie>) -> Result<(), Error> {
        let form = multipart::Form::new().text("cookies", serde_json::to_string(&cookies)?);

        self._post("app/setCookies")
            .await?
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Rotate the API key for the application.
    ///
    /// It rotates the API key for the application and returns the new key.
    /// It will also update the internal state with the new key if `set_state`
    /// is `true`,so that future requests will use the new key. It will
    /// overwrite the old credentials even if it is username/password
    ///
    /// # Examples
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.rotate_api_key(true).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // [pr 23212](https://github.com/qbittorrent/qBittorrent/pull/23212)
    pub async fn rotate_api_key(&self, set_state: bool) -> Result<String, Error> {
        use crate::Credentials;

        #[derive(serde::Deserialize)]
        struct RotateApiKeyResponse {
            #[serde(rename = "apiKey")]
            key: String,
        }

        let response: RotateApiKeyResponse = self
            ._post("app/rotateAPIKey")
            .await?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let key = response.key;

        if set_state {
            self.state.write().await.credentials = Some(Credentials::APIKey(key.clone()));
        }

        Ok(key)
    }

    /// Delete the API key.
    ///
    /// It is recommended to set `set_state` to `true` to clear the credentials
    /// from the state.
    ///
    ///
    /// # Examples
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = client.delete_api_key(true).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    #[cfg(not(feature = "qBittorrent-5_1"))]
    // [pr 23388](https://github.com/qbittorrent/qBittorrent/pull/23388)
    pub async fn delete_api_key(&self, set_state: bool) -> Result<(), Error> {
        self._post("app/deleteAPIKey")
            .await?
            .send()
            .await?
            .error_for_status()?;

        if set_state {
            self.state.write().await.credentials = None;
        }

        Ok(())
    }

    /// List the contents of the directory. (Yes this is an endpoint)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    /// use qbit::models::DirMode;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::Login("username".to_string(), "password".to_string());
    ///     let client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     let contents = client.get_directory_contents("/path/to/some/file", &DirMode::All)
    ///         .await
    ///         .unwrap();
    ///
    ///     for item in contents {
    ///         println!("{}", item);
    ///     }
    /// }
    /// ```
    pub async fn get_directory_contents(
        &self,
        dir: &str,
        mode: &DirMode,
    ) -> Result<Vec<String>, Error> {
        let mut form = HashMap::new();
        form.insert("dirPath", dir.to_string());
        form.insert("mode", mode.to_string());

        Ok(self
            ._post("app/getDirectoryContent")
            .await?
            .form(&form)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<String>>()
            .await?)
    }
}
