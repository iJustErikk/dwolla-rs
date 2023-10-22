use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetAccountTransfersRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub correlation_id: Option<String>,
    pub end_amount: Option<String>,
    pub end_date: Option<String>,
    pub id: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub start_amount: Option<String>,
    pub start_date: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetAccountTransfersRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/accounts/{id}/transfers", id = self.id));
        if let Some(ref unwrapped) = self.correlation_id {
            r = r.query("correlationId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_amount {
            r = r.query("endAmount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.query("endDate", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_amount {
            r = r.query("startAmount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.query("startDate", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_owned());
        self
    }
    pub fn end_amount(mut self, end_amount: &str) -> Self {
        self.end_amount = Some(end_amount.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn start_amount(mut self, start_amount: &str) -> Self {
        self.start_amount = Some(start_amount.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetAccountTransfersRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}