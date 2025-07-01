mod client;
mod error;

/// Data object models.
pub mod models;
/// Parameter objects.
pub mod parameters;

pub use client::Api;
pub use error::Error;

/// Login state
///
/// Represents the authentication state of a user in the system.
///
/// Inspired by the design from [George-Miao qbit repo](https://github.com/George-Miao/qbit) -
/// [Commit](https://github.com/George-Miao/qbit/commit/fe1240c05b4d3feeafb327e8ba7f0eeba97735c5#diff-b1a35a68f14e696205874893c07fd24fdb88882b47c23cc0e0c80a30c7d53759R28)
#[derive(Clone)]
pub enum LoginState {
    LoggedIn {
        credentials: Credentials,
        cookie_sid: String,
    },
    NotLoggedIn {
        credentials: Credentials,
    },
    CookieProvidet {
        cookie_sid: String,
    },
    Unknown,
}

impl LoginState {
    fn as_cookie(&self) -> Option<String> {
        match self {
            Self::LoggedIn { cookie_sid, .. } => Some(cookie_sid.to_string()),
            Self::NotLoggedIn { .. } => None,
            Self::CookieProvidet { cookie_sid } => Some(cookie_sid.to_string()),
            Self::Unknown => None,
        }
    }

    fn as_credentials(&self) -> Option<&Credentials> {
        let creds = match self {
            Self::LoggedIn { credentials, .. } => Some(credentials),
            Self::NotLoggedIn { credentials } => Some(credentials),
            Self::CookieProvidet { .. } => return None,
            Self::Unknown => return None,
        };

        if creds.unwrap().is_empty() {
            return None;
        }

        creds
    }

    fn add_cookie(&self, cookie: &str) -> Self {
        match self {
            Self::LoggedIn { credentials, .. } => Self::LoggedIn {
                cookie_sid: cookie.to_string(),
                credentials: credentials.clone(),
            },
            Self::NotLoggedIn { credentials } => Self::LoggedIn {
                cookie_sid: cookie.to_string(),
                credentials: credentials.clone(),
            },
            Self::CookieProvidet { .. } => Self::CookieProvidet {
                cookie_sid: cookie.to_string(),
            },
            Self::Unknown => Self::CookieProvidet {
                cookie_sid: cookie.to_string(),
            },
        }
    }
}

/// The `Credentials` struct represents a user's login credentials.
#[derive(Clone)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    fn is_empty(&self) -> bool {
        self.username.is_empty() && self.password.is_empty()
    }
}

impl ToString for Credentials {
    fn to_string(&self) -> String {
        format!("username={}&password={}", self.username, self.password)
    }
}
