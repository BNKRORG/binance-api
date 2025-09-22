use serde::Deserialize;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    Timestamp(#[from] std::time::SystemTimeError),
    #[error("Asset not found")]
    AssetNotFound,
    #[error("Failed to get timestamp")]
    FailedToGetTimestamp,
    #[error("API keys not available")]
    ApiKeysNotAvailable,
}
