#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus}};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let response = client.list_customers(Some(1), None, Some("erikaww@umich.edu"), Some(ListCustomersStatus::Suspended)).await.unwrap();
    println!("{:#?}", response);
}