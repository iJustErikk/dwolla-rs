#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
use dwolla::request::AddBeneficialOwnerRequired;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let args = AddBeneficialOwnerRequired {
        address: Address {
            address1: "your address 1".to_owned(),
            address2: Some("your address 2".to_owned()),
            address3: Some("your address 3".to_owned()),
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: Some("your postal code".to_owned()),
            state_province_region: "your state province region".to_owned(),
        },
        date_of_birth: "your date of birth",
        first_name: "your first name",
        id: "your id",
        last_name: "your last name",
    };
    let response = client
        .add_beneficial_owner(args)
        .passport(Passport {
            country: "your country".to_owned(),
            number: "your number".to_owned(),
        })
        .ssn("your ssn")
        .await
        .unwrap();
    println!("{:#?}", response);
}