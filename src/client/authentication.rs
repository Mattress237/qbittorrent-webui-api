use reqwest::header::{self};

use crate::{Credentials, LoginState, error::Error};

impl super::Api {
    /// Create a new API instance and login to the service.
    ///
    /// This method allows you to create a new API instance and login using the provided credentials.s
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `credentials` - The credentials to use for authentication.
    pub async fn new_login(url: &str, credentials: Credentials) -> Result<Self, Error> {
        let mut api = Self::new(url)?;

        *api.state.write().await = LoginState::NotLoggedIn {
            credentials: credentials.clone(),
        };

        api.login(false).await?;

        Ok(api)
    }

    /// Create a new API instance and login to the service with username and password.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
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
    /// # Arguments
    /// * `credentials` - The credentials to use for authentication.
    /// * `force` - If true, forces a login even if already logged in.
    pub async fn login(&mut self, force: bool) -> Result<(), Error> {
        // check if already login (aka cookie set)
        if self.state.read().await.as_cookie().is_some() && !force {
            // test if the cookie is valid by calling the version api
            if self.version().await.unwrap() != "Forbidden" {
                println!("login");
                return Ok(());
            }
        }

        if let Some(cred) = self.state.read().await.as_credentials() {
            if cred.is_empty() {
                return Err(Error::AuthFailed(
                    format!(
                        "Credential filed is empty and missing values: {}",
                        cred.to_string()
                    )
                    .to_string(),
                ));
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
                    Error::AuthFailed(format!(
                        "Failed to pars SID cookie to str. err: {}",
                        e.to_string()
                    ))
                })?
                .split(';')
                .next()
                .ok_or(Error::AuthFailed("Failed to parse SID cookie".to_string()))?
                .trim_start_matches("SID=")
                .to_string(),
        };

        Ok(())
    }

    /// Login to the service.
    ///
    /// This method allows you to create a new API instance and login using an existing SID cookie.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `sid_cookie` - The session ID cookie for authentication.
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
    pub async fn logout(&self) -> Result<(), Error> {
        self._post("auth/logout").await?.send().await?;

        let mut state = self.state.write().await;
        *state = LoginState::NotLoggedIn {
            credentials: state.as_credentials().unwrap().clone(),
        };

        Ok(())
    }
}
