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

    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    async fn get_signed<T>(&self, endpoint: BinanceApi, request: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.sign_request(endpoint, request)?;
        let headers = self.build_headers(true)?;
        let client = &self.client;
        let response = client.get(url.as_str()).headers(headers).send().await?;

        self.handler(response).await
    }

    // async fn post_signed<T: DeserializeOwned>(
    //     &self,
    //     endpoint: BinanceApi,
    //     request: String,
    // ) -> Result<T> {
    //     let url = self.sign_request(endpoint, Some(request));
    //     let client = &self.client;
    //
    //     let headers = self.build_headers(true)?;
    //     let response = client.post(url.as_str()).headers(headers).send().await?;
    //
    //     self.handler(response).await
    // }

    // async fn delete_signed<T: DeserializeOwned>(
    //     &self,
    //     endpoint: BinanceApi,
    //     request: Option<String>,
    // ) -> Result<T> {
    //     let url = self.sign_request(endpoint, request);
    //     let headers = self.build_headers(true)?;
    //     let client = &self.client;
    //     let response = client.delete(url.as_str()).headers(headers).send().await?;
    //
    //     self.handler(response).await
    // }
    //
    // async fn get<T: DeserializeOwned>(
    //     &self,
    //     endpoint: BinanceApi,
    //     request: Option<String>,
    // ) -> Result<T> {
    //     let mut url: String = format!("{}{}", self.host, String::from(endpoint));
    //     if let Some(request) = request {
    //         if !request.is_empty() {
    //             url.push_str(format!("?{}", request).as_str());
    //         }
    //     }
    //
    //     let client = &self.client;
    //     let response = client.get(url.as_str()).send().await?;
    //
    //     self.handler(response).await
    // }
    //
    // async fn post<T: DeserializeOwned>(&self, endpoint: BinanceApi) -> Result<T> {
    //     let url: String = format!("{}{}", self.host, String::from(endpoint));
    //
    //     let client = &self.client;
    //     let response = client
    //         .post(url.as_str())
    //         .headers(self.build_headers(false)?)
    //         .send()
    //         .await?;
    //
    //     self.handler(response).await
    // }
    //
    // async fn put<T: DeserializeOwned>(&self, endpoint: BinanceApi, listen_key: &str) -> Result<T> {
    //     let url: String = format!("{}{}", self.host, String::from(endpoint));
    //     let data: String = format!("listenKey={}", listen_key);
    //
    //     let client = &self.client;
    //
    //     let headers = self.build_headers(true)?;
    //     let response = client
    //         .put(url.as_str())
    //         .headers(headers)
    //         .body(data)
    //         .send()
    //         .await?;
    //
    //     self.handler(response).await
    // }
    //
    // async fn delete<T: DeserializeOwned>(&self, endpoint: BinanceApi, listen_key: &str) -> Result<T> {
    //     let url: String = format!("{}{}", self.host, String::from(endpoint));
    //     let data: String = format!("listenKey={}", listen_key);
    //
    //     let client = &self.client;
    //     let response = client
    //         .delete(url.as_str())
    //         .headers(self.build_headers(false)?)
    //         .body(data)
    //         .send()
    //         .await?;
    //
    //     self.handler(response).await
    // }

    // Request must be signed
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

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
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

    async fn handler<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response: Response = response.error_for_status()?;
        Ok(response.json().await?)
    }

    // Account Information
    pub async fn get_account(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.get_signed(BinanceApi::Spot(Spot::Account), Some(request))
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
