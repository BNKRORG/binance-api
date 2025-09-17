//! Binance API

#![forbid(unsafe_code)]
#![warn(clippy::large_futures)]
#![warn(rustdoc::bare_urls)]

mod client;
pub mod error;
mod util;

pub mod model;

pub mod account;
pub mod api;
pub mod config;
pub mod general;
pub mod savings;
