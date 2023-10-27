#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus}};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let id = "id";
    let response = client.get_customer_funding_sources(id, Some(false)).await.unwrap();
    println!("{:#?}", response);
}