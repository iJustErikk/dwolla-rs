#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let response = client
        .list_customers()
        .limit(1)
        .offset(1)
        .search("your search")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}