use thiserror::Error;
use url::ParseError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    Url(#[from] ParseError),
    #[error(transparent)]
    Timestamp(#[from] std::time::SystemTimeError),
    #[error("Asset not found")]
    AssetNotFound,
    #[error("Failed to get timestamp")]
    FailedToGetTimestamp,
    #[error("API keys not available")]
    ApiKeysNotAvailable,
}
