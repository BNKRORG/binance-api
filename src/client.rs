use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::api::API;
use crate::error::Result;

#[derive(Clone)]
pub struct BinanceClient {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: Client,
}

impl BinanceClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        Self {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client: Client::builder().pool_idle_timeout(None).build().unwrap(),
        }
    }

    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    pub async fn get_signed<T>(&self, endpoint: API, request: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.sign_request(endpoint, request);
        let headers = self.build_headers(true)?;
        let client = &self.inner_client;
        let response = client.get(url.as_str()).headers(headers).send().await?;

        self.handler(response).await
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: String,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, Some(request));
        let client = &self.inner_client;

        let headers = self.build_headers(true)?;
        let response = client.post(url.as_str()).headers(headers).send().await?;

        self.handler(response).await
    }

    pub async fn delete_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let headers = self.build_headers(true)?;
        let client = &self.inner_client;
        let response = client.delete(url.as_str()).headers(headers).send().await?;

        self.handler(response).await
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send().await?;

        self.handler(response).await
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: API) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn put<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;

        let headers = self.build_headers(true)?;
        let response = client
            .put(url.as_str())
            .headers(headers)
            .body(data)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()
            .await?;

        self.handler(response).await
    }

    // Request must be signed
    fn sign_request(&self, endpoint: API, request: Option<String>) -> String {
        if let Some(request) = request {
            let mut signed_key =
                Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
            signed_key.update(request.as_bytes());
            let signature = hex_encode(signed_key.finalize().into_bytes());
            let request_body: String = format!("{}&signature={}", request, signature);
            format!("{}{}?{}", self.host, String::from(endpoint), request_body)
        } else {
            let signed_key = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
            let signature = hex_encode(signed_key.finalize().into_bytes());
            let request_body: String = format!("&signature={}", signature);
            format!("{}{}?{}", self.host, String::from(endpoint), request_body)
        }
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
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
            HeaderValue::from_str(self.api_key.as_str())?,
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
}
