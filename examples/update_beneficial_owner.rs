#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let id = "your id";
    let response = client
        .update_beneficial_owner(id)
        .idempotency_key("your idempotency key")
        .address(Address {
            address1: "your address 1".to_owned(),
            address2: Some("your address 2".to_owned()),
            address3: Some("your address 3".to_owned()),
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: Some("your postal code".to_owned()),
            state_province_region: "your state province region".to_owned(),
        })
        .date_of_birth(chrono::Utc::now().date_naive())
        .first_name("your first name")
        .last_name("your last name")
        .passport(Passport {
            country: "your country".to_owned(),
            number: "your number".to_owned(),
        })
        .ssn("your ssn")
        .await
        .unwrap();
    println!("{:#?}", response);
}