use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateCustomerFundingSourceRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub links: Option<serde_json::Value>,
    pub account_number: String,
    pub bank_account_type: Option<String>,
    pub channels: Option<Vec<String>>,
    pub id: String,
    pub name: Option<String>,
    pub routing_number: String,
    pub type_: Option<String>,
    pub verified: Option<bool>,
}
impl<'a> CreateCustomerFundingSourceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/customers/{id}/funding-sources", id = self.id));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.links {
            r = r.json(json!({ "_links" : unwrapped }));
        }
        r = r.json(json!({ "accountNumber" : self.account_number }));
        if let Some(ref unwrapped) = self.bank_account_type {
            r = r.json(json!({ "bankAccountType" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.channels {
            r = r.json(json!({ "channels" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.json(json!({ "name" : unwrapped }));
        }
        r = r.json(json!({ "routingNumber" : self.routing_number }));
        if let Some(ref unwrapped) = self.type_ {
            r = r.json(json!({ "type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.verified {
            r = r.json(json!({ "verified" : unwrapped }));
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
    pub fn bank_account_type(mut self, bank_account_type: &str) -> Self {
        self.bank_account_type = Some(bank_account_type.to_owned());
        self
    }
    pub fn channels(
        mut self,
        channels: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .channels = Some(
            channels.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
    pub fn verified(mut self, verified: bool) -> Self {
        self.verified = Some(verified);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateCustomerFundingSourceRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}