#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let links = serde_json::json!({});
    let items = vec![
        MassPaymentItemRequestBody { links : serde_json::json!({}), ach_details :
        Some(serde_json::json!({})), amount : Amount { currency : "your currency"
        .to_owned(), value : "your value".to_owned() }, correlation_id :
        Some("your correlation id".to_owned()), metadata : Some(serde_json::json!({})) }
    ];
    let response = client
        .create_mass_payment(links, items)
        .idempotency_key("your idempotency key")
        .ach_details(serde_json::json!({}))
        .correlation_id("your correlation id")
        .metadata(serde_json::json!({}))
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}