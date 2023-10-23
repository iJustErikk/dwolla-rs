#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let amount1 = Amount {
        currency: "your currency".to_owned(),
        value: "your value".to_owned(),
    };
    let amount2 = Amount {
        currency: "your currency".to_owned(),
        value: "your value".to_owned(),
    };
    let id = "your id";
    let response = client
        .initiate_micro_deposits(amount1, amount2, id)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}