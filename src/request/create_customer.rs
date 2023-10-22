use serde_json::json;
use crate::model::*;
use crate::DwollaClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateCustomerRequest<'a> {
    pub(crate) http_client: &'a DwollaClient,
    pub idempotency_key: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub business_classification: Option<String>,
    pub business_name: Option<String>,
    pub business_type: Option<String>,
    pub city: Option<String>,
    pub correlation_id: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub doing_business_as: Option<String>,
    pub ein: Option<String>,
    pub email: Option<String>,
    pub first_name: String,
    pub ip_address: Option<String>,
    pub last_name: String,
    pub phone: Option<String>,
    pub postal_code: Option<String>,
    pub ssn: Option<String>,
    pub state: Option<String>,
    pub type_: Option<String>,
    pub website: Option<String>,
}
impl<'a> CreateCustomerRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self.http_client.client.post("/customers");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.header("Idempotency-Key", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.address1 {
            r = r.json(json!({ "address1" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.json(json!({ "address2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.business_classification {
            r = r.json(json!({ "businessClassification" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.business_name {
            r = r.json(json!({ "businessName" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.business_type {
            r = r.json(json!({ "businessType" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.correlation_id {
            r = r.json(json!({ "correlationId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.date_of_birth {
            r = r.json(json!({ "dateOfBirth" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.doing_business_as {
            r = r.json(json!({ "doingBusinessAs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ein {
            r = r.json(json!({ "ein" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email {
            r = r.json(json!({ "email" : unwrapped }));
        }
        r = r.json(json!({ "firstName" : self.first_name }));
        if let Some(ref unwrapped) = self.ip_address {
            r = r.json(json!({ "ipAddress" : unwrapped }));
        }
        r = r.json(json!({ "lastName" : self.last_name }));
        if let Some(ref unwrapped) = self.phone {
            r = r.json(json!({ "phone" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.postal_code {
            r = r.json(json!({ "postalCode" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ssn {
            r = r.json(json!({ "ssn" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.json(json!({ "type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.website {
            r = r.json(json!({ "website" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn address1(mut self, address1: &str) -> Self {
        self.address1 = Some(address1.to_owned());
        self
    }
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn business_classification(mut self, business_classification: &str) -> Self {
        self.business_classification = Some(business_classification.to_owned());
        self
    }
    pub fn business_name(mut self, business_name: &str) -> Self {
        self.business_name = Some(business_name.to_owned());
        self
    }
    pub fn business_type(mut self, business_type: &str) -> Self {
        self.business_type = Some(business_type.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_owned());
        self
    }
    pub fn date_of_birth(mut self, date_of_birth: chrono::NaiveDate) -> Self {
        self.date_of_birth = Some(date_of_birth);
        self
    }
    pub fn doing_business_as(mut self, doing_business_as: &str) -> Self {
        self.doing_business_as = Some(doing_business_as.to_owned());
        self
    }
    pub fn ein(mut self, ein: &str) -> Self {
        self.ein = Some(ein.to_owned());
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn ip_address(mut self, ip_address: &str) -> Self {
        self.ip_address = Some(ip_address.to_owned());
        self
    }
    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.to_owned());
        self
    }
    pub fn postal_code(mut self, postal_code: &str) -> Self {
        self.postal_code = Some(postal_code.to_owned());
        self
    }
    pub fn ssn(mut self, ssn: &str) -> Self {
        self.ssn = Some(ssn.to_owned());
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
    pub fn website(mut self, website: &str) -> Self {
        self.website = Some(website.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateCustomerRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}