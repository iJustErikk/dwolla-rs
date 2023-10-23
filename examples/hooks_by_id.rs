#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let id = "your id";
    let response = client
        .hooks_by_id(id)
        .end_date("your end date")
        .limit(1)
        .offset(1)
        .start_date("your start date")
        .await
        .unwrap();
    println!("{:#?}", response);
}