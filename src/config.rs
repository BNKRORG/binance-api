//! Binance configuration

use std::time::Duration;

use url::Url;

/// SPOT API endpoint (mainnet)
pub const SPOT_MAINNET: &str = "https://api.binance.com";
/// SPOT API endpoint (mainnet)
pub const SPOT_TESTNET: &str = "https://testnet.binance.vision";
const DEFAULT_RECV_WINDOW: u64 = 5000;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// Binance configuration
#[derive(Debug, Clone)]
pub struct BinanceConfig {
    /// REST API endpoint
    pub rest_api_endpoint: Url,
    /// Recv window
    pub recv_window: u64,
    /// Request timeout
    pub timeout: Duration,
}

impl Default for BinanceConfig {
    fn default() -> Self {
        Self {
            rest_api_endpoint: Url::parse(SPOT_MAINNET).expect("Invalid rest API endpoint"),
            recv_window: DEFAULT_RECV_WINDOW,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl BinanceConfig {
    /// Testnet configs
    #[inline]
    pub fn testnet() -> Self {
        let endpoint: Url = Url::parse(SPOT_TESTNET).expect("Invalid rest API endpoint");
        Self::default().set_rest_api_endpoint(endpoint)
    }

    /// Set REST API endpoint
    #[inline]
    pub fn set_rest_api_endpoint(mut self, rest_api_endpoint: Url) -> Self {
        self.rest_api_endpoint = rest_api_endpoint;
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
