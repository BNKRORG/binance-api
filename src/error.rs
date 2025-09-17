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
    IO(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),
    // #[error(transparent)]
    // UrlParserError(#[from] url::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Timestamp(#[from] std::time::SystemTimeError),
    #[error("{0:?}")]
    Binance(BinanceContentError),
    #[error("{1} at {0} is missing")]
    KlineValueMissing(usize, &'static str),
    #[error("Symbol not found")]
    SymbolNotFound,
    #[error("Asset not found")]
    AssetNotFound,
    #[error("Failed to get timestamp")]
    FailedToGetTimestamp,
}
