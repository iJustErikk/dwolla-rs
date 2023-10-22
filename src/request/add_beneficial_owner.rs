use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AddBeneficialOwnerRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub address: Address,
    pub date_of_birth: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub passport: Option<Passport>,
    pub ssn: Option<String>,
}
impl<'a> AddBeneficialOwnerRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/customers/{id}/beneficial-owners", id = self.id));
        r = r.json(json!({ "address" : self.address }));
        r = r.json(json!({ "dateOfBirth" : self.date_of_birth }));
        r = r.json(json!({ "firstName" : self.first_name }));
        r = r.json(json!({ "lastName" : self.last_name }));
        if let Some(ref unwrapped) = self.passport {
            r = r.json(json!({ "passport" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ssn {
            r = r.json(json!({ "ssn" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
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
pub struct AddBeneficialOwnerRequired<'a> {
    pub address: Address,
    pub date_of_birth: &'a str,
    pub first_name: &'a str,
    pub id: &'a str,
    pub last_name: &'a str,
}
impl<'a> AddBeneficialOwnerRequired<'a> {}
impl<'a> ::std::future::IntoFuture for AddBeneficialOwnerRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}