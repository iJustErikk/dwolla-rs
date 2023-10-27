#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus, TransferRequestBody, Amount}};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let source_id = "5050d874-8eaf-4e1a-9144-1d6f129fc568";
    let dest_id = "50d4d571-0327-494c-ac97-5904d23ea1a1";
    let amount  = Amount {
        value: 0.5f64.to_string(),
        currency: "USD".to_string()
    };
    let links = client.get_transfer_links(source_id, dest_id);

    let response = client.create_transfer(Some("123"), &TransferRequestBody{ach_details: None, amount, clearing: None, correlation_id: None, fees: None, links, metadata: None}).await.unwrap();
    println!("{}", response);
}