#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let id = "your id";
    let token_type = "your token type";
    let response = client.get_account_iav_token(id, token_type).await.unwrap();
    println!("{:#?}", response);
}