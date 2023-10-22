#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let id = "your id";
    let response = client
        .get_account_transfers(id)
        .correlation_id("your correlation id")
        .end_amount("your end amount")
        .end_date("your end date")
        .limit(1)
        .offset(1)
        .start_amount("your start amount")
        .start_date("your start date")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}