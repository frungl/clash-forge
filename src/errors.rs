use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(crate::api::errors::Error),
    InvalidTag(String),
    InvalidToken,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(err) => write!(f, "HTTP error: {}", err),
            Error::Json(err) => write!(f, "JSON error: {}", err),
            Error::Api(err) => write!(f, "API error: {}", err),
            Error::InvalidTag(tag) => write!(f, "Invalid tag: {}", tag),
            Error::InvalidToken => write!(f, "Invalid API token"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(err) => Some(err),
            Error::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;