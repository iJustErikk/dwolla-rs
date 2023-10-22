use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateFundingSourceRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub links: Option<serde_json::Value>,
    pub account_number: Option<String>,
    pub bank_account_type: Option<String>,
    pub id: String,
    pub name: String,
    pub routing_number: Option<String>,
}
impl<'a> UpdateFundingSourceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/funding-sources/{id}", id = self.id));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.links {
            r = r.json(json!({ "_links" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_number {
            r = r.json(json!({ "accountNumber" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bank_account_type {
            r = r.json(json!({ "bankAccountType" : unwrapped }));
        }
        r = r.json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.routing_number {
            r = r.json(json!({ "routingNumber" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn links(mut self, links: serde_json::Value) -> Self {
        self.links = Some(links);
        self
    }
    pub fn account_number(mut self, account_number: &str) -> Self {
        self.account_number = Some(account_number.to_owned());
        self
    }
    pub fn bank_account_type(mut self, bank_account_type: &str) -> Self {
        self.bank_account_type = Some(bank_account_type.to_owned());
        self
    }
    pub fn routing_number(mut self, routing_number: &str) -> Self {
        self.routing_number = Some(routing_number.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateFundingSourceRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}