#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus}};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let response = client.list_customers(Some(1), None, Some("email=erikaww%40umich.edu"), Some(ListCustomersStatus::Verified)).await.unwrap();
    println!("{:#?}", response);
}