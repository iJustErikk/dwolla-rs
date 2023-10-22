#![allow(unused_imports)]
use dwolla::DwollaClient;
use dwolla::model::*;
#[tokio::main]
async fn main() {
    let client = DwollaClient::from_env();
    let answers = vec![
        AnsweredKbaQuestion { answer_id : "your answer id".to_owned(), question_id :
        "your question id".to_owned() }
    ];
    let id = "your id";
    let response = client
        .answer_kba_questions(answers, id)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}