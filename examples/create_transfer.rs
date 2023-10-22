#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let links = serde_json::json!({});
    let amount = serde_json::json!({});
    let response = client
        .create_transfer(links, amount)
        .idempotency_key("your idempotency key")
        .ach_details(serde_json::json!({}))
        .clearing(serde_json::json!({}))
        .correlation_id("your correlation id")
        .fees(serde_json::json!({}))
        .imad("your imad")
        .metadata(serde_json::json!({}))
        .wire_instructions(serde_json::json!({}))
        .await
        .unwrap();
    println!("{:#?}", response);
}