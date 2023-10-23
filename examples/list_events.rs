#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let response = client.list_events().limit(1).offset(1).await.unwrap();
    println!("{:#?}", response);
}