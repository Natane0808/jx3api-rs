use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum Jx3ApiError {
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),
    ApiError(i32, String),
}

impl Display for Jx3ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Jx3ApiError::HttpError(e) => write!(f, "HTTP error: {}", e),
            Jx3ApiError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            Jx3ApiError::ApiError(code, msg) => {
                write!(f, "API request failed: code={}, message={}", code, msg)
            }
        }
    }
}

impl Error for Jx3ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Jx3ApiError::HttpError(e) => Some(e),
            Jx3ApiError::JsonError(e) => Some(e),
            Jx3ApiError::ApiError(_code, _msg) => None,
        }
    }
}

impl From<reqwest::Error> for Jx3ApiError {
    fn from(error: reqwest::Error) -> Self {
        Jx3ApiError::HttpError(error)
    }
}

impl From<serde_json::Error> for Jx3ApiError {
    fn from(error: serde_json::Error) -> Self {
        Jx3ApiError::JsonError(error)
    }
}
