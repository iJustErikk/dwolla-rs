#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let id = "your id";
    let response = client
        .get_mass_payment_items(id)
        .correlation_id("your correlation id")
        .limit(1)
        .offset(1)
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}