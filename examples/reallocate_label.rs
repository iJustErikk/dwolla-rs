#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
use dwolla::request::ReallocateLabelRequired;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env().await;
    let args = ReallocateLabelRequired {
        amount: Amount {
            currency: "your currency".to_owned(),
            value: "your value".to_owned(),
        },
        from_label_id: "your from label id",
        partner_id: "your partner id",
        to_label_id: "your to label id",
    };
    let response = client
        .reallocate_label(args)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}