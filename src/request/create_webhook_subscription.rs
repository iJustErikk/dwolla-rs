use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateWebhookSubscriptionRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub secret: String,
    pub url: String,
}
impl<'a> CreateWebhookSubscriptionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.post("/webhook-subscriptions");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        r = r.json(json!({ "secret" : self.secret }));
        r = r.json(json!({ "url" : self.url }));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateWebhookSubscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}