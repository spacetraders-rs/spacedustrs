//! Errors that might be returned from space traders
use crate::shared;
use std::fmt;

/// Any error from the space traders client is represented here
#[derive(Debug)]
pub enum SpaceTradersClientError {
    /// If the underlying reqwest driver has an error
    Http(reqwest::Error),
    /// If the SpaceTraders API returns an error
    ApiError(shared::ErrorMessage),
    /// If a request comes back with a 429 or 500 the request will be retried.
    /// This error occurs if there are too many retries
    TooManyRetries,
    /// If a response from the API is unable to be serialized into a known type
    JsonParse(serde_json::Error),
    /// If the SpaceTraders API is in maintenance mode
    ServiceUnavailable,
    /// If the token is invalid for the request being made
    Unauthorized,
}

impl fmt::Display for SpaceTradersClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpaceTradersClientError::ServiceUnavailable => {
                write!(f, "SpaceTraders API is down for maintenance")
            }
            SpaceTradersClientError::Http(ref err) => write!(f, "Http error: {}", err),
            SpaceTradersClientError::JsonParse(ref err) => {
                write!(f, "Error parsing game status response: {}", err)
            }
            SpaceTradersClientError::ApiError(ref err) => write!(f, "Api error: {}", err),
            SpaceTradersClientError::TooManyRetries => write!(f, "Too many retries attempted"),
            SpaceTradersClientError::Unauthorized => write!(f, "Request returned unauthorized"),
        }
    }
}

impl From<reqwest::Error> for SpaceTradersClientError {
    fn from(err: reqwest::Error) -> Self {
        SpaceTradersClientError::Http(err)
    }
}

impl std::error::Error for SpaceTradersClientError {}
