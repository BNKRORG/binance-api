//! Binance client

use std::collections::BTreeMap;
use std::fmt;

use hmac::{Hmac, Mac};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use sha2::Sha256;
use url::Url;

use crate::api::{BinanceApi, Spot};
use crate::auth::BinanceAuth;
use crate::config::BinanceConfig;
use crate::error::Error;
use crate::response::{AccountInformation, Balance};
use crate::util::build_signed_request;

/// Binance client
#[derive(Clone)]
pub struct BinanceClient {
    client: Client,
    host: Url,
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
    /// Construct new binance client
    pub fn new(auth: BinanceAuth, config: BinanceConfig) -> Self {
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

    fn sign_request(&self, endpoint: BinanceApi, request: Option<String>) -> Result<Url, Error> {
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

        // Build URL endpoint
        let mut url: Url = self.host.join(endpoint.as_str())?;

        // Add query parameters
        url.set_query(Some(&request_body));

        Ok(url)
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap, Error> {
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

    async fn handle_http_response<T>(&self, response: Response) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response: Response = response.error_for_status()?;
        Ok(response.json().await?)
    }

    async fn get_signed<T>(&self, endpoint: BinanceApi, request: Option<String>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url = self.sign_request(endpoint, request)?;
        let headers = self.build_headers(true)?;
        let response = self.client.get(url).headers(headers).send().await?;

        self.handle_http_response(response).await
    }

    /// Get account information
    pub async fn get_account(&self) -> Result<AccountInformation, Error> {
        // Build signed request
        let request: String = build_signed_request(BTreeMap::new(), self.recv_window)?;

        // Get signed request
        self.get_signed(BinanceApi::Spot(Spot::Account), Some(request))
            .await
    }

    /// Get balance for a single asset
    pub async fn get_balance<S>(&self, asset: S) -> Result<Balance, Error>
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
