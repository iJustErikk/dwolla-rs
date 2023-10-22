#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let id = "your id";
    let response = client.get_customer_funding_sources(id).removed(true).await.unwrap();
    println!("{:#?}", response);
}