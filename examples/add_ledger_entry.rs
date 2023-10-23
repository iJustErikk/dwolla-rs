#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let amount = Amount {
        currency: "your currency".to_owned(),
        value: "your value".to_owned(),
    };
    let id = "your id";
    let response = client
        .add_ledger_entry(amount, id)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}