//! Binance API

#![forbid(unsafe_code)]
#![warn(clippy::large_futures)]
#![warn(rustdoc::bare_urls)]

mod api;
pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod model;
mod util;
