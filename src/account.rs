use std::collections::BTreeMap;

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
