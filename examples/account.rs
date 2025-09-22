use binance_api::auth::BinanceAuth;
use binance_api::client::BinanceClient;
use binance_api::config::BinanceConfig;

#[tokio::main]
async fn main() {
    let auth = BinanceAuth::ApiKeys {
        api_key: "api_key".to_string(),
        secret_key: "api_secret".to_string(),
    };
    let config = BinanceConfig::default();

    let client = BinanceClient::new(auth, config);

    let account = client.get_account().await.unwrap();
    println!("{:#?}", account);
}
