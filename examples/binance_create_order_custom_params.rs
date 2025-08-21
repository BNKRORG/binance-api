use std::collections::BTreeMap;
use binance::api::*;
use binance::account::*;

fn main() {
    create_order_with_params()
}

fn create_order_with_params() {
    let api_key: &str = "YOUR_API_KEY";
    let secret_key: &str = "YOUR_SECRET_KEY";

    let mut account: Account = Binance::new(Some(api_key.into()), Some(secret_key.into()));

    // account.set_verbose(true); // Uncomment to enable verbose logging

    // account.set_testnet(true); // Use testnet for testing if needed

    // use this to provide custom parameters to the request not supported by the types directly
    let mut custom_params: BTreeMap<String, String> = BTreeMap::new();
    custom_params.insert("customParam".into(), "customValue".into());
    custom_params.insert("customParam2".into(), "customValue2".into());

    let order = account.custom_order_with_params(
        "BNBUSDT",
        0.1,
        300.0,
        None,
        OrderSide::Buy,
        OrderType::Limit,
        TimeInForce::GTC,
        None,
        custom_params,
    );

    println!("Order response: {:?}", order);
}
