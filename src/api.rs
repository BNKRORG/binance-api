use crate::account::Account;
use crate::client::BinanceClient;
use crate::config::{Config, SPOT_MAINNET, SPOT_TESTNET};
use crate::general::General;
use crate::savings::Savings;

#[allow(clippy::all)]
pub enum API {
    Spot(Spot),
    Savings(Sapi),
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    AvgPrice,
    Ticker24hr,
    Price,
    BookTicker,
    Order,
    OrderTest,
    OpenOrders,
    AllOrders,
    Oco,
    OrderList,
    AllOrderList,
    OpenOrderList,
    Account,
    MyTrades,
    UserDataStream,
}

pub enum Sapi {
    AllCoins,
    AssetDetail,
    DepositAddress,
    SpotFuturesTransfer,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(route) => match route {
                Spot::Ping => "/api/v3/ping",
                Spot::Time => "/api/v3/time",
                Spot::ExchangeInfo => "/api/v3/exchangeInfo",
                Spot::Depth => "/api/v3/depth",
                Spot::Trades => "/api/v3/trades",
                Spot::HistoricalTrades => "/api/v3/historicalTrades",
                Spot::AggTrades => "/api/v3/aggTrades",
                Spot::Klines => "/api/v3/klines",
                Spot::AvgPrice => "/api/v3/avgPrice",
                Spot::Ticker24hr => "/api/v3/ticker/24hr",
                Spot::Price => "/api/v3/ticker/price",
                Spot::BookTicker => "/api/v3/ticker/bookTicker",
                Spot::Order => "/api/v3/order",
                Spot::OrderTest => "/api/v3/order/test",
                Spot::OpenOrders => "/api/v3/openOrders",
                Spot::AllOrders => "/api/v3/allOrders",
                Spot::Oco => "/api/v3/order/oco",
                Spot::OrderList => "/api/v3/orderList",
                Spot::AllOrderList => "/api/v3/allOrderList",
                Spot::OpenOrderList => "/api/v3/openOrderList",
                Spot::Account => "/api/v3/account",
                Spot::MyTrades => "/api/v3/myTrades",
                Spot::UserDataStream => "/api/v3/userDataStream",
            },
            API::Savings(route) => match route {
                Sapi::AllCoins => "/sapi/v1/capital/config/getall",
                Sapi::AssetDetail => "/sapi/v1/asset/assetDetail",
                Sapi::DepositAddress => "/sapi/v1/capital/deposit/address",
                Sapi::SpotFuturesTransfer => "/sapi/v1/futures/transfer",
            },
        })
    }
}

pub trait Binance: Sized {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> Self;

    fn set_testnet(&mut self, testnet: bool);
}

impl Binance for General {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> General {
        General {
            client: BinanceClient::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}

impl Binance for Account {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> Self {
        Self {
            client: BinanceClient::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}

impl Binance for Savings {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> Self {
        Self {
            client: BinanceClient::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}
