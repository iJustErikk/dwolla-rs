#![allow(unused_imports)]
use dwolla::{Client, types::{CatalogResponse, CreateFundingSourceWithPlaidRequest}};
#[tokio::main]
async fn main() {
    // get customer by id, create funding source using processor token
    let client = Client::from_env().await;
    let customer_id = "id";
    let plaid_token = "plaid";
    let response = client.create_customer_funding_source_with_plaid(customer_id, Some("123"), &CreateFundingSourceWithPlaidRequest{plaid_token: String::from(plaid_token), funding_source_name: String::from("Funding Source Name")}).await.unwrap();
    println!("{}", response);
}