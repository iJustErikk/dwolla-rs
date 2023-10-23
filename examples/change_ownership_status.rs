#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let id = "your id";
    let status = "your status";
    let response = client.change_ownership_status(id, status).await.unwrap();
    println!("{:#?}", response);
}