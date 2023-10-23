#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let first_name = "your first name";
    let last_name = "your last name";
    let response = client
        .create_customer(first_name, last_name)
        .idempotency_key("your idempotency key")
        .address1("your address 1")
        .address2("your address 2")
        .business_classification("your business classification")
        .business_name("your business name")
        .business_type("your business type")
        .city("your city")
        .correlation_id("your correlation id")
        .date_of_birth(chrono::Utc::now().date_naive())
        .doing_business_as("your doing business as")
        .ein("your ein")
        .email("your email")
        .ip_address("your ip address")
        .phone("your phone")
        .postal_code("your postal code")
        .ssn("your ssn")
        .state("your state")
        .type_("your type")
        .website("your website")
        .await
        .unwrap();
    println!("{:#?}", response);
}