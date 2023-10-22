use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTransferRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub links: serde_json::Value,
    pub ach_details: Option<serde_json::Value>,
    pub amount: serde_json::Value,
    pub clearing: Option<serde_json::Value>,
    pub correlation_id: Option<String>,
    pub fees: Option<serde_json::Value>,
    pub imad: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub wire_instructions: Option<serde_json::Value>,
}
impl<'a> CreateTransferRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.post("/transfers");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        r = r.json(json!({ "_links" : self.links }));
        if let Some(ref unwrapped) = self.ach_details {
            r = r.json(json!({ "achDetails" : unwrapped }));
        }
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.clearing {
            r = r.json(json!({ "clearing" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.correlation_id {
            r = r.json(json!({ "correlationId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fees {
            r = r.json(json!({ "fees" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.imad {
            r = r.json(json!({ "imad" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.wire_instructions {
            r = r.json(json!({ "wireInstructions" : unwrapped }));
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
    pub fn clearing(mut self, clearing: serde_json::Value) -> Self {
        self.clearing = Some(clearing);
        self
    }
    pub fn correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_owned());
        self
    }
    pub fn fees(mut self, fees: serde_json::Value) -> Self {
        self.fees = Some(fees);
        self
    }
    pub fn imad(mut self, imad: &str) -> Self {
        self.imad = Some(imad.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn wire_instructions(mut self, wire_instructions: serde_json::Value) -> Self {
        self.wire_instructions = Some(wire_instructions);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateTransferRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}