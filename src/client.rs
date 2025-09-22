use std::collections::BTreeMap;
use std::fmt;

use hmac::{Hmac, Mac};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::api::{BinanceApi, Spot};
use crate::auth::BinanceAuth;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::model::{AccountInformation, Balance};
use crate::util::build_signed_request;

/// Binance Client
#[derive(Clone)]
pub struct BinanceClient {
    client: Client,
    host: String,
    auth: BinanceAuth,
    recv_window: u64,
}

impl fmt::Debug for BinanceClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinanceClient")
            .field("host", &self.host)
            .finish()
    }
}

impl BinanceClient {
    pub fn new(auth: BinanceAuth, config: Config) -> Self {
        Self {
            client: Client::builder()
                .timeout(config.timeout)
                .build()
                .expect("Failed to create reqwest client"),
            host: config.rest_api_endpoint,
            auth,
            recv_window: config.recv_window,
        }
    }

    fn sign_request(&self, endpoint: BinanceApi, request: Option<String>) -> Result<String, Error> {
        let secret_key: &str = self.auth.get_api_secret_key()?;

        let mut signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();

        if let Some(request) = &request {
            signed_key.update(request.as_bytes());
        }

        let signature = hex::encode(signed_key.finalize().into_bytes());

        let request_body: String = match request {
            Some(request) => format!("{request}&signature={signature}"),
            None => format!("signature={signature}"),
        };

        Ok(format!("{}{endpoint}?{request_body}", self.host))
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let api_key: &str = self.auth.get_api_key()?;

        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-api"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key)?,
        );

        Ok(custom_headers)
    }

    async fn handle_http_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response: Response = response.error_for_status()?;
        Ok(response.json().await?)
    }

    async fn get_signed<T>(&self, endpoint: BinanceApi, request: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.sign_request(endpoint, request)?;
        let headers = self.build_headers(true)?;
        let response = self
            .client
            .get(url.as_str())
            .headers(headers)
            .send()
            .await?;

        self.handle_http_response(response).await
    }

    // Account Information
    pub async fn get_account(&self) -> Result<AccountInformation> {
        // Build signed request
        let request: String = build_signed_request(BTreeMap::new(), self.recv_window)?;

        // Get signed request
        self.get_signed(BinanceApi::Spot(Spot::Account), Some(request))
            .await
    }

    // Balance for a single Asset
    pub async fn get_balance<S>(&self, asset: S) -> Result<Balance>
    where
        S: AsRef<str>,
    {
        let asset: &str = asset.as_ref();

        let account: AccountInformation = self.get_account().await?;

        // Find the balance for the given asset
        for balance in account.balances.into_iter() {
            if balance.asset == asset {
                return Ok(balance);
            }
        }

        Err(Error::AssetNotFound)
    }
}
