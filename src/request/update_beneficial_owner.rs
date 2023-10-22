use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateBeneficialOwnerRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub address: Option<Address>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub first_name: Option<String>,
    pub id: String,
    pub last_name: Option<String>,
    pub passport: Option<Passport>,
    pub ssn: Option<String>,
}
impl<'a> UpdateBeneficialOwnerRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/beneficial-owners/{id}", id = self.id));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.address {
            r = r.json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.date_of_birth {
            r = r.json(json!({ "dateOfBirth" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.first_name {
            r = r.json(json!({ "firstName" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.json(json!({ "lastName" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.passport {
            r = r.json(json!({ "passport" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ssn {
            r = r.json(json!({ "ssn" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }
    pub fn date_of_birth(mut self, date_of_birth: chrono::NaiveDate) -> Self {
        self.date_of_birth = Some(date_of_birth);
        self
    }
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }
    pub fn passport(mut self, passport: Passport) -> Self {
        self.passport = Some(passport);
        self
    }
    pub fn ssn(mut self, ssn: &str) -> Self {
        self.ssn = Some(ssn.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateBeneficialOwnerRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}