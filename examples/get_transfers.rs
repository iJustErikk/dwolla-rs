#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus}};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let id = "c9db963a-feac-45b1-be6a-688312f5979c";
    let response = client.get_customer_transfers(id, None, None, None, None, None, None, None, None).await.unwrap();
    println!("{:#?}", response);
}