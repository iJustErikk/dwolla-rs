use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateMassPaymentRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub links: serde_json::Value,
    pub ach_details: Option<serde_json::Value>,
    pub correlation_id: Option<String>,
    pub items: Vec<MassPaymentItemRequestBody>,
    pub metadata: Option<serde_json::Value>,
    pub status: Option<String>,
}
impl<'a> CreateMassPaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.post("/mass-payments");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        r = r.json(json!({ "_links" : self.links }));
        if let Some(ref unwrapped) = self.ach_details {
            r = r.json(json!({ "achDetails" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.correlation_id {
            r = r.json(json!({ "correlationId" : unwrapped }));
        }
        r = r.json(json!({ "items" : self.items }));
        if let Some(ref unwrapped) = self.metadata {
            r = r.json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.json(json!({ "status" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn ach_details(mut self, ach_details: serde_json::Value) -> Self {
        self.ach_details = Some(ach_details);
        self
    }
    pub fn correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateMassPaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}