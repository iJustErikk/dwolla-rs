#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let amount = Amount {
        currency: "your currency".to_owned(),
        value: "your value".to_owned(),
    };
    let id = "your id";
    let response = client
        .create_label(amount, id)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}