//! Binance configuration

use std::time::Duration;

use url::Url;

use crate::constant::{DEFAULT_RECV_WINDOW, DEFAULT_TIMEOUT, SPOT_MAINNET, SPOT_TESTNET};

/// Binance endpoint
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinanceEndpoint {
    /// Mainnet (international)
    #[default]
    Mainnet,
    /// Testnet (international)
    Testnet,
}

impl BinanceEndpoint {
    pub(crate) fn url(&self) -> Url {
        match self {
            Self::Mainnet => Url::parse(SPOT_MAINNET).expect("Invalid rest API endpoint"),
            Self::Testnet => Url::parse(SPOT_TESTNET).expect("Invalid rest API endpoint"),
        }
    }
}

/// Binance configuration
#[derive(Debug, Clone)]
pub struct BinanceConfig {
    /// Endpoint
    pub endpoint: BinanceEndpoint,
    /// Recv window
    pub recv_window: u64,
    /// Request timeout
    pub timeout: Duration,
}

impl Default for BinanceConfig {
    fn default() -> Self {
        Self {
            endpoint: BinanceEndpoint::default(),
            recv_window: DEFAULT_RECV_WINDOW,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl BinanceConfig {
    /// Set endpoint
    #[inline]
    pub fn endpoint(mut self, endpoint: BinanceEndpoint) -> Self {
        self.endpoint = endpoint;
        self
    }

    /// Set recv window
    #[inline]
    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    /// Set timeout
    #[inline]
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
