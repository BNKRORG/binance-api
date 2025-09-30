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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_account_information() {
        let json = r#"{
    "makerCommission": 15,
    "takerCommission": 15,
    "buyerCommission": 0,
    "sellerCommission": 0,
    "canTrade": true,
    "canWithdraw": true,
    "canDeposit": true,
    "updateTime": 123456789,
    "accountType": "SPOT",
    "balances": [{
            "asset": "BTC",
            "free": "4723846.89208129",
            "locked": "0.00000000"
        },
        {
            "asset": "LTC",
            "free": "4763368.68006011",
            "locked": "0.00000000"
        }
    ],
    "permissions": [
        "SPOT"
    ]
}"#;

        let account: AccountInformation = serde_json::from_str(json).unwrap();

        assert_eq!(account.maker_commission, 15.0);
        assert_eq!(account.taker_commission, 15.0);
        assert_eq!(account.buyer_commission, 0.0);
        assert_eq!(account.seller_commission, 0.0);
        assert_eq!(account.can_trade, true);
        assert_eq!(account.can_withdraw, true);
        assert_eq!(account.can_deposit, true);
        assert_eq!(
            account.balances,
            vec![
                Balance {
                    asset: "BTC".to_string(),
                    free: "4723846.89208129".to_string(),
                    locked: "0.00000000".to_string(),
                },
                Balance {
                    asset: "LTC".to_string(),
                    free: "4763368.68006011".to_string(),
                    locked: "0.00000000".to_string(),
                }
            ]
        );
    }
}
