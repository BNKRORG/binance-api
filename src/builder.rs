//! Binance client builder

use std::time::Duration;

use url::Url;

use crate::auth::BinanceAuth;
use crate::client::BinanceClient;
use crate::constant::{DEFAULT_RECV_WINDOW, DEFAULT_TIMEOUT, SPOT_MAINNET, SPOT_TESTNET};
use crate::error::Error;

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

/// Binance client builder
#[derive(Debug, Clone)]
pub struct BinanceClientBuilder {
    /// Endpoint
    pub endpoint: BinanceEndpoint,
    /// Authentication
    pub auth: BinanceAuth,
    /// Recv window
    pub recv_window: u64,
    /// Request timeout
    pub timeout: Duration,
}

impl Default for BinanceClientBuilder {
    fn default() -> Self {
        Self {
            endpoint: BinanceEndpoint::default(),
            auth: BinanceAuth::default(),
            recv_window: DEFAULT_RECV_WINDOW,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl BinanceClientBuilder {
    /// Set endpoint
    #[inline]
    pub fn endpoint(mut self, endpoint: BinanceEndpoint) -> Self {
        self.endpoint = endpoint;
        self
    }

    /// Set authentication
    #[inline]
    pub fn auth(mut self, auth: BinanceAuth) -> Self {
        self.auth = auth;
        self
    }

    /// Set recv window
    #[inline]
    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    /// Set timeout
    #[inline]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build client
    #[inline]
    pub fn build(self) -> Result<BinanceClient, Error> {
        BinanceClient::from_builder(self)
    }
}
