use reqwest::header::{self};

use crate::{Credentials, LoginState, error::Error};

impl super::Api {
    /// Create a new API instance and login to qbittorrent with the provided credentials.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `credentials` - The credentials to use for authentication.
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
    /// }
    /// ```
    pub async fn new_login(url: &str, credentials: Credentials) -> Result<Self, Error> {
        let mut api = Self::new(url)?;

        *api.state.write().await = LoginState::NotLoggedIn {
            credentials: credentials.clone(),
        };

        api.login(false).await?;

        Ok(api)
    }

    /// Create a new API instance and login to the service with the provided username and password.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::Api;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Api::new_login_username_password("http://127.0.0.1/", "username", "password")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn new_login_username_password(
        url: &str,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self, Error> {
        let credentials = Credentials::new(username, password);

        Self::new_login(url, credentials).await
    }

    /// Login to the service.
    ///
    /// This method allows you to login using the credentials stored in the API state.
    /// If the user is already logged in, it will check the validity of the session.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#login)
    ///
    /// # Arguments
    /// * `credentials` - The credentials to use for authentication.
    /// * `force` - If true, forces a login even if already logged in.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let mut client = Api::new_login("http://127.0.0.1/", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     // relogin the client
    ///     client.login(true).await.unwrap();
    /// }
    /// ```
    pub async fn login(&mut self, force: bool) -> Result<(), Error> {
        // check if already login (aka cookie set)
        if self.state.read().await.as_cookie().is_some() && !force {
            // test if the cookie is valid by calling the version api
            if self.version().await.is_ok() {
                return Ok(());
            }
        }

        if let Some(cred) = self.state.read().await.as_credentials() {
            if cred.is_empty() {
                return Err(Error::AuthFailed(format!(
                    "Credential filed is empty and missing values: {}",
                    cred
                )));
            }
        } else {
            return Err(Error::AuthFailed("Credentials are not set".to_string()));
        }

        let res = self
            ._post("auth/login")
            .await?
            .header(header::REFERER, self.base_url.read().await.to_string())
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(
                self.state
                    .read()
                    .await
                    .as_credentials()
                    .unwrap()
                    .to_string(),
            )
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Error::AuthFailed(format!(
                "Login endpoint returned no success status code: {}",
                res.status()
            )));
        }

        let sid = res.headers().get(header::SET_COOKIE);
        if sid.is_none() {
            return Err(Error::AuthFailed(
                "Missing set-cookie header from response".to_string(),
            ));
        }

        let mut state = self.state.write().await;
        *state = LoginState::LoggedIn {
            credentials: state.as_credentials().unwrap().clone(),
            cookie_sid: sid
                .unwrap()
                .to_str()
                .map_err(|e| {
                    Error::AuthFailed(format!("Failed to pars SID cookie to str. err: {}", e))
                })?
                .split(';')
                .next()
                .ok_or(Error::AuthFailed("Failed to parse SID cookie".to_string()))?
                .trim_start_matches("SID=")
                .to_string(),
        };

        Ok(())
    }

    /// Login to the service by providing an existing valid SID (Session identifier) cookie.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `sid_cookie` - The session ID cookie for authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::Api;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Api::new_from_cookie("http://127.0.0.1/", "cookie")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn new_from_cookie(url: &str, sid_cookie: impl Into<&str>) -> Result<Self, Error> {
        let mut api = Self::new(url)?;

        api.set_sid_cookie(sid_cookie).await?;

        let test_result = api.version().await;

        if test_result.is_err() {
            return Err(Error::AuthFailed(
                "Failed to login with provided SID cookie".to_string(),
            ));
        }

        Ok(api)
    }

    /// Logout the client instance
    ///
    /// This will clear the current session and remove the SID cookie.
    ///
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#logout)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qbit::{Api, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new("username", "password");
    ///     let client = Api::new_login("http://127.0.0.1", credentials)
    ///         .await
    ///         .unwrap();
    ///
    ///     client.logout().await.unwrap();
    /// }
    /// ```
    pub async fn logout(&self) -> Result<(), Error> {
        self._post("auth/logout")
            .await?
            .send()
            .await?
            .error_for_status()?;

        let mut state = self.state.write().await;
        *state = LoginState::NotLoggedIn {
            credentials: state.as_credentials().unwrap().clone(),
        };

        Ok(())
    }
}
