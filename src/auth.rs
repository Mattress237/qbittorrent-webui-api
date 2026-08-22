/// Login state
///
/// Represents the authentication state of a user in the system.
///
/// Inspired by the design from [George-Miao qbit repo](https://github.com/George-Miao/qbit) -
/// [Commit](https://github.com/George-Miao/qbit/commit/fe1240c05b4d3feeafb327e8ba7f0eeba97735c5#diff-b1a35a68f14e696205874893c07fd24fdb88882b47c23cc0e0c80a30c7d53759R28)
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LoginState {
    /// The user is logged in.
    LoggedIn {
        /// Credentials used to log in.
        credentials: Credentials,
        /// Session cookie used to log in.
        cookie_sid: String,
    },
    /// The user is not logged in but we have credenitals.
    NotLoggedIn {
        /// Credentials used to log in
        credentials: Credentials,
    },
    /// The user is not logged in but we have the cookie to log them in.
    CookieProvided {
        /// Session cookie used to log in.
        cookie_sid: String,
    },
    /// We don't know if the user is logged in or not.
    #[default]
    Unknown,
}

impl LoginState {
    pub fn as_cookie(&self) -> Option<String> {
        match self {
            Self::LoggedIn { cookie_sid, .. } => Some(cookie_sid.to_string()),
            Self::NotLoggedIn { .. } => None,
            Self::CookieProvided { cookie_sid } => Some(cookie_sid.to_string()),
            Self::Unknown => None,
        }
    }

    pub fn as_credentials(&self) -> Option<&Credentials> {
        match self {
            Self::LoggedIn { credentials, .. } => Some(credentials),
            Self::NotLoggedIn { credentials } => Some(credentials),
            Self::CookieProvided { .. } => return None,
            Self::Unknown => return None,
        }
    }

    pub fn add_cookie(&self, cookie: &str) -> Self {
        match self {
            Self::LoggedIn { credentials, .. } => Self::LoggedIn {
                cookie_sid: cookie.to_string(),
                credentials: credentials.clone(),
            },
            Self::NotLoggedIn { credentials } => Self::LoggedIn {
                cookie_sid: cookie.to_string(),
                credentials: credentials.clone(),
            },
            Self::CookieProvided { .. } => Self::CookieProvided {
                cookie_sid: cookie.to_string(),
            },
            Self::Unknown => Self::CookieProvided {
                cookie_sid: cookie.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Credentials {
    Login(String, String),
    APIKey(String),
}
