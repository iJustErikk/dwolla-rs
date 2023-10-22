#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let id = "your id";
    let response = client.get_beneficia_owner_documents(id).await.unwrap();
    println!("{:#?}", response);
}