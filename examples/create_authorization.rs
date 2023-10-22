#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let response = client.create_authorization().await.unwrap();
    println!("{:#?}", response);
}