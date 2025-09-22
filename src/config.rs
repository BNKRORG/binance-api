pub const SPOT_MAINNET: &str = "https://api.binance.com";
pub const SPOT_TESTNET: &str = "https://testnet.binance.vision";
const DEFAULT_RECV_WINDOW: u64 = 5000;

#[derive(Debug, Clone)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub recv_window: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: SPOT_MAINNET.to_string(),
            recv_window: DEFAULT_RECV_WINDOW,
        }
    }
}

impl Config {
    #[inline]
    pub fn testnet() -> Self {
        Self::default().set_rest_api_endpoint(SPOT_TESTNET)
    }

    #[inline]
    pub fn set_rest_api_endpoint<T>(mut self, rest_api_endpoint: T) -> Self
    where
        T: Into<String>,
    {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    #[inline]
    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}
