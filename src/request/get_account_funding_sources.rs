use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetAccountFundingSourcesRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub id: String,
    pub removed: Option<bool>,
}
impl<'a> GetAccountFundingSourcesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/accounts/{id}/funding-sources", id = self.id));
        if let Some(ref unwrapped) = self.removed {
            r = r.query("removed", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn removed(mut self, removed: bool) -> Self {
        self.removed = Some(removed);
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetAccountFundingSourcesRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}