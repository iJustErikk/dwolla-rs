#![allow(unused_imports)]
use chrono::NaiveDate;
use dwolla::{Client, types::{CatalogResponse, ListCustomersStatus, CreateCustomer, FailedCreateCustomerResponse}};
use progenitor_client::{Error, ResponseValue};
#[tokio::main]
async fn main() {
    let client = Client::from_env().await;
    let response = client.create_customer(Some("123"), &CreateCustomer{address1: Some(String::from("315 Utica Ave.")), address2: None, business_classification: None, business_name: None, business_type: None, city: Some(String::from("New York City")), correlation_id: None, date_of_birth: Some(NaiveDate::from_ymd_opt(1996, 11, 2).unwrap()), doing_business_as: None, ein: None, email: Some(String::from("testemail@nowhere.com")), first_name: String::from("Family"), ip_address: None, last_name: String::from("Crab"), phone: Some(String::from("7184841488")), postal_code: Some(String::from("11203")), ssn: Some(String::from("XXXX")), state: Some(String::from("NY")), type_: Some(String::from("personal")), website: None}).await;
    match response {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(Error::ErrorResponse(res)) => {
            let err = res.into_inner();
            match err {
                FailedCreateCustomerResponse::Validation(e) => {
                    println!("{:#?}", e.embedded["errors"]);
                },
                FailedCreateCustomerResponse::Empty(e) => {
                    println!("{:#?}", e);

                }
            }
        },
        Err(Error::CommunicationError(_)) => {
            // should retry with backoff and jitter
        },
        _ => {
            // something was wrong with serializing. this should be incredibly rare if there was not a breaking API change.
        }
    }
}