use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ReallocateLabelRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub amount: Amount,
    pub from_label_id: String,
    pub partner_id: String,
    pub to_label_id: String,
}
impl<'a> ReallocateLabelRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.post("/label-reallocations");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "fromLabelId" : self.from_label_id }));
        r = r.json(json!({ "partnerId" : self.partner_id }));
        r = r.json(json!({ "toLabelId" : self.to_label_id }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
pub struct ReallocateLabelRequired<'a> {
    pub amount: Amount,
    pub from_label_id: &'a str,
    pub partner_id: &'a str,
    pub to_label_id: &'a str,
}
impl<'a> ReallocateLabelRequired<'a> {}
impl<'a> ::std::future::IntoFuture for ReallocateLabelRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}