use std::collections::BTreeMap;
use std::fmt::Display;

use crate::api::{API, Spot};
use crate::client::BinanceClient;
use crate::error::{Error, Result};
use crate::model::{AccountInformation, Balance};
use crate::util::build_signed_request;

#[derive(Debug, Clone)]
pub struct Account {
    pub client: BinanceClient,
    pub recv_window: u64,
}

pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
}

impl OrderType {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderType::Limit),
            2 => Some(OrderType::Market),
            3 => Some(OrderType::StopLossLimit),
            _ => None,
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
        }
    }
}

pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderSide::Buy),
            2 => Some(OrderSide::Sell),
            _ => None,
        }
    }
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

#[allow(clippy::all)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

impl TimeInForce {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(TimeInForce::GTC),
            2 => Some(TimeInForce::IOC),
            3 => Some(TimeInForce::FOK),
            _ => None,
        }
    }
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
        }
    }
}

impl Account {
    // Account Information
    pub async fn get_account(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Account), Some(request))
            .await
    }

    // Balance for a single Asset
    pub async fn get_balance<S>(&self, asset: S) -> Result<Balance>
    where
        S: Into<String>,
    {
        match self.get_account().await {
            Ok(account) => {
                let cmp_asset = asset.into();
                for balance in account.balances {
                    if balance.asset == cmp_asset {
                        return Ok(balance);
                    }
                }

                Err(Error::AssetNotFound)
            }
            Err(e) => Err(e),
        }
    }
}
