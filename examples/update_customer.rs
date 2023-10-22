#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let id = "your id";
    let response = client
        .update_customer(id)
        .idempotency_key("your idempotency key")
        .address1("your address 1")
        .address2("your address 2")
        .business_classification("your business classification")
        .business_name("your business name")
        .business_type("your business type")
        .city("your city")
        .controller(serde_json::json!({}))
        .correlation_id("your correlation id")
        .date_of_birth(chrono::Utc::now().date_naive())
        .doing_business_as("your doing business as")
        .ein("your ein")
        .email("your email")
        .first_name("your first name")
        .ip_address("your ip address")
        .last_name("your last name")
        .phone("your phone")
        .postal_code("your postal code")
        .ssn("your ssn")
        .state("your state")
        .status("your status")
        .type_("your type")
        .website("your website")
        .await
        .unwrap();
    println!("{:#?}", response);
}