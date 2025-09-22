//! Binance models

use serde::{Deserialize, Serialize};

/// Account information
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    /// Maker commission rate
    pub maker_commission: f32,
    /// Taker commission rate
    pub taker_commission: f32,
    /// Buyer commission rate
    pub buyer_commission: f32,
    /// Seller commission rate
    pub seller_commission: f32,
    /// Can trade
    pub can_trade: bool,
    /// Can withdraw
    pub can_withdraw: bool,
    /// Can deposit
    pub can_deposit: bool,
    /// Balances
    pub balances: Vec<Balance>,
}

/// Balance
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Asset
    pub asset: String,
    /// Free balance
    // TODO: use f64?
    pub free: String,
    /// Locked balance
    // TODO: use f64?
    pub locked: String,
}
