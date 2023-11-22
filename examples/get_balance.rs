#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus}};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let id = "7508dea4-1d80-4c01-b973-5e4171628c28";
    let response = client.get_balance(id).await.unwrap();
    println!("{:#?}", response);
}