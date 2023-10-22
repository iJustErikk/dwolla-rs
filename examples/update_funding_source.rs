#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let id = "your id";
    let name = "your name";
    let response = client
        .update_funding_source(id, name)
        .idempotency_key("your idempotency key")
        .links(serde_json::json!({}))
        .account_number("your account number")
        .bank_account_type("your bank account type")
        .routing_number("your routing number")
        .await
        .unwrap();
    println!("{:#?}", response);
}