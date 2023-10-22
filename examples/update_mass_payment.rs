#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let id = "your id";
    let status = "your status";
    let response = client
        .update_mass_payment(id, status)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}