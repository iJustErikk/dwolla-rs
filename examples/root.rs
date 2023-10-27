#![allow(unused_imports)]
use dwolla::{Client, types::CatalogResponse};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let response = client.root().await.unwrap();
    println!("{:#?}", response);
}