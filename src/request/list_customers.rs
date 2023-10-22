use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCustomersRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
}
impl<'a> ListCustomersRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.get("/customers");
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.search {
            r = r.query("search", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCustomersRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}