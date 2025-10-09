use std::time::Duration;

pub(crate) const SPOT_MAINNET: &str = "https://api.binance.com";
pub(crate) const SPOT_MAINNET_US: &str = "https://api.binance.us";
pub(crate) const SPOT_TESTNET: &str = "https://testnet.binance.vision";

pub(crate) const DEFAULT_RECV_WINDOW: u64 = 5000;
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(25);
