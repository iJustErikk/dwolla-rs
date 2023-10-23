#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let account_number = "your account number";
    let routing_number = "your routing number";
    let response = client
        .create_account_funding_source(account_number, routing_number)
        .idempotency_key("your idempotency key")
        .links(serde_json::json!({}))
        .bank_account_type("your bank account type")
        .channels(&["your channels"])
        .name("your name")
        .type_("your type")
        .verified(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}