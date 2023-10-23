#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let response = client.list_business_classifications().await.unwrap();
    println!("{:#?}", response);
}