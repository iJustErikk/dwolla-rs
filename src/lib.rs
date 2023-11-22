use std::collections::HashMap;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
use reqwest::header::{self, CONTENT_TYPE, ACCEPT, LOCATION};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

use types::{CatalogResponse, CustomerListResponse, Customer, FailedCreateCustomerResponse, FailedValidationCreateCustomerResponse, FundingSourceBalance, TransferListResponse, HalLink, FundingSourceListResponse, OAuthResponse, FundingSource};
pub mod types {
    use chrono::{NaiveDate, DateTime, Utc};
    use reqwest::Response;
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Debug)]
    pub enum FailedCreateCustomerResponse {
        Validation(FailedValidationCreateCustomerResponse),
        Empty(Response),
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FailedValidationCreateCustomerResponse {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccountFundingSourcesToken {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub token: String,
    }

    impl From<&AccountFundingSourcesToken> for AccountFundingSourcesToken {
        fn from(value: &AccountFundingSourcesToken) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddLabelLedgerEntryRequest {
        pub amount: Amount,
    }

    impl From<&AddLabelLedgerEntryRequest> for AddLabelLedgerEntryRequest {
        fn from(value: &AddLabelLedgerEntryRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Address {
        pub address1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address3: Option<String>,
        pub city: String,
        pub country: String,
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub postal_code: Option<String>,
        #[serde(rename = "stateProvinceRegion")]
        pub state_province_region: String,
    }

    impl From<&Address> for Address {
        fn from(value: &Address) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Amount {
        pub currency: String,
        pub value: String,
    }

    impl From<&Amount> for Amount {
        fn from(value: &Amount) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Answer {
        pub id: String,
        pub text: String,
    }

    impl From<&Answer> for Answer {
        fn from(value: &Answer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AnswerKbaQuestionsRequest {
        pub answers: Vec<AnsweredKbaQuestion>,
    }

    impl From<&AnswerKbaQuestionsRequest> for AnswerKbaQuestionsRequest {
        fn from(value: &AnswerKbaQuestionsRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AnswerKbaQuestionsResponse {
        #[serde(rename = "_embedded")]
        pub embedded: std::collections::HashMap<String, f64>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(rename = "verificationStatus")]
        pub verification_status: String,
    }

    impl From<&AnswerKbaQuestionsResponse> for AnswerKbaQuestionsResponse {
        fn from(value: &AnswerKbaQuestionsResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AnsweredKbaQuestion {
        #[serde(rename = "answerId")]
        pub answer_id: String,
        #[serde(rename = "questionId")]
        pub question_id: String,
    }

    impl From<&AnsweredKbaQuestion> for AnsweredKbaQuestion {
        fn from(value: &AnsweredKbaQuestion) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ApplicationEvent {
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(rename = "resourceId")]
        pub resource_id: String,
        pub topic: String,
    }

    impl From<&ApplicationEvent> for ApplicationEvent {
        fn from(value: &ApplicationEvent) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BusinessClassification {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub name: String,
    }

    impl From<&BusinessClassification> for BusinessClassification {
        fn from(value: &BusinessClassification) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BusinessClassificationListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&BusinessClassificationListResponse> for BusinessClassificationListResponse {
        fn from(value: &BusinessClassificationListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CatalogResponse {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&CatalogResponse> for CatalogResponse {
        fn from(value: &CatalogResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CertifyRequest {
        pub status: String,
    }

    impl From<&CertifyRequest> for CertifyRequest {
        fn from(value: &CertifyRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateCustomer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address2: Option<String>,
        #[serde(
            rename = "businessClassification",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_classification: Option<String>,
        #[serde(
            rename = "businessName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_name: Option<String>,
        #[serde(
            rename = "businessType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub date_of_birth: Option<NaiveDate>,
        #[serde(
            rename = "doingBusinessAs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub doing_business_as: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ein: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub postal_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssn: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub website: Option<String>,
    }

    impl From<&CreateCustomer> for CreateCustomer {
        fn from(value: &CreateCustomer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateCustomerLabelRequest {
        pub amount: Amount,
    }

    impl From<&CreateCustomerLabelRequest> for CreateCustomerLabelRequest {
        fn from(value: &CreateCustomerLabelRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateFundingSourceWithPlaidRequest {
        #[serde(rename = "name")]
        pub funding_source_name: String,
        #[serde(rename = "plaidToken")]
        pub plaid_token: String
    }

    impl From<&CreateFundingSourceWithPlaidRequest> for CreateFundingSourceWithPlaidRequest {
        fn from(value: &CreateFundingSourceWithPlaidRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateFundingSourceRequest {
        #[serde(rename = "accountNumber")]
        pub account_number: String,
        #[serde(
            rename = "bankAccountType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bank_account_type: Option<String>,
        ///
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub channels: Vec<String>,
        #[serde(
            rename = "_links",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub links: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "routingNumber")]
        pub routing_number: String,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub verified: Option<bool>,
    }

    impl From<&CreateFundingSourceRequest> for CreateFundingSourceRequest {
        fn from(value: &CreateFundingSourceRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateOwnerRequest {
        pub address: Address,
        #[serde(rename = "dateOfBirth")]
        pub date_of_birth: String,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub passport: Option<Passport>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssn: Option<String>,
    }

    impl From<&CreateOwnerRequest> for CreateOwnerRequest {
        fn from(value: &CreateOwnerRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateWebhook {
        pub secret: String,
        pub url: String,
    }

    impl From<&CreateWebhook> for CreateWebhook {
        fn from(value: &CreateWebhook) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Customer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address2: Option<String>,
        #[serde(
            rename = "businessName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub controller: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        pub created: String,
        #[serde(
            rename = "doingBusinessAs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub doing_business_as: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "firstName")]
        pub first_name: String,
        pub id: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub postal_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        pub status: String,
        #[serde(rename = "type")]
        pub type_: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub website: Option<String>,
    }

    impl From<&Customer> for Customer {
        fn from(value: &Customer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CustomerFundingSourcesToken {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub token: String,
    }

    impl From<&CustomerFundingSourcesToken> for CustomerFundingSourcesToken {
        fn from(value: &CustomerFundingSourcesToken) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CustomerList {
        pub customers: Vec<Customer>
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CustomerListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: CustomerList,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&CustomerListResponse> for CustomerListResponse {
        fn from(value: &CustomerListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Document {
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "failureReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub failure_reason: Option<String>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub status: String,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&Document> for Document {
        fn from(value: &Document) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DocumentListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&DocumentListResponse> for DocumentListResponse {
        fn from(value: &DocumentListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&EventListResponse> for EventListResponse {
        fn from(value: &EventListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeesBySourceResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
        ///
        pub transactions: Vec<Transfer>,
    }

    impl From<&FeesBySourceResponse> for FeesBySourceResponse {
        fn from(value: &FeesBySourceResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FullAccountInfo {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: FullAccountInfoLinks,
        pub name: String,
        #[serde(rename = "timezoneOffset")]
        pub timezone_offset: i32,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&FullAccountInfo> for FullAccountInfo {
        fn from(value: &FullAccountInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FullAccountInfoLinks {
        #[serde(rename = "", default, skip_serializing_if = "Option::is_none")]
        pub x: Option<String>,
        #[serde(flatten)]
        pub extra: std::collections::HashMap<String, HalLink>,
    }

    impl From<&FullAccountInfoLinks> for FullAccountInfoLinks {
        fn from(value: &FullAccountInfoLinks) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FundingSource {
        #[serde(
            rename = "bankAccountType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bank_account_type: Option<String>,
        #[serde(rename = "bankName", default, skip_serializing_if = "Option::is_none")]
        pub bank_name: Option<String>,
        ///
        pub channels: Vec<String>,
        pub created: DateTime<Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fingerprint: Option<String>,
        #[serde(
            rename = "iavAccountHolders",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub iav_account_holders: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub name: String,
        pub removed: bool,
        pub status: String,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&FundingSource> for FundingSource {
        fn from(value: &FundingSource) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Balance {
        pub value: String,
        pub currency: String
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FundingSourceBalance {
        pub balance: Balance,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_updated: Option<String>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
    }

    impl From<&FundingSourceBalance> for FundingSourceBalance {
        fn from(value: &FundingSourceBalance) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FundingSourceList {
        #[serde(rename = "funding-sources")]
        pub funding_sources: Vec<FundingSource>
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FundingSourceListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: FundingSourceList,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&FundingSourceListResponse> for FundingSourceListResponse {
        fn from(value: &FundingSourceListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum GetAccountTransfersStatus {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "processed")]
        Processed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "cancelled")]
        Cancelled,
    }

    impl From<&GetAccountTransfersStatus> for GetAccountTransfersStatus {
        fn from(value: &GetAccountTransfersStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for GetAccountTransfersStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Pending => "pending".to_string(),
                Self::Processed => "processed".to_string(),
                Self::Failed => "failed".to_string(),
                Self::Cancelled => "cancelled".to_string(),
            }
        }
    }

    impl std::str::FromStr for GetAccountTransfersStatus {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "pending" => Ok(Self::Pending),
                "processed" => Ok(Self::Processed),
                "failed" => Ok(Self::Failed),
                "cancelled" => Ok(Self::Cancelled),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for GetAccountTransfersStatus {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for GetAccountTransfersStatus {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for GetAccountTransfersStatus {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum GetCustomerTransfersStatus {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "processed")]
        Processed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "cancelled")]
        Cancelled,
    }

    impl From<&GetCustomerTransfersStatus> for GetCustomerTransfersStatus {
        fn from(value: &GetCustomerTransfersStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for GetCustomerTransfersStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Pending => "pending".to_string(),
                Self::Processed => "processed".to_string(),
                Self::Failed => "failed".to_string(),
                Self::Cancelled => "cancelled".to_string(),
            }
        }
    }

    impl std::str::FromStr for GetCustomerTransfersStatus {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "pending" => Ok(Self::Pending),
                "processed" => Ok(Self::Processed),
                "failed" => Ok(Self::Failed),
                "cancelled" => Ok(Self::Cancelled),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for GetCustomerTransfersStatus {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for GetCustomerTransfersStatus {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for GetCustomerTransfersStatus {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum GetMassPaymentItemsStatus {
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "success")]
        Success,
    }

    impl From<&GetMassPaymentItemsStatus> for GetMassPaymentItemsStatus {
        fn from(value: &GetMassPaymentItemsStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for GetMassPaymentItemsStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Failed => "failed".to_string(),
                Self::Pending => "pending".to_string(),
                Self::Success => "success".to_string(),
            }
        }
    }

    impl std::str::FromStr for GetMassPaymentItemsStatus {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "failed" => Ok(Self::Failed),
                "pending" => Ok(Self::Pending),
                "success" => Ok(Self::Success),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for GetMassPaymentItemsStatus {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for GetMassPaymentItemsStatus {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for GetMassPaymentItemsStatus {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct HalLink {
        pub href: String,
        #[serde(
            rename = "resource-type",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resource_type: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&HalLink> for HalLink {
        fn from(value: &HalLink) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IavToken {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub token: String,
    }

    impl From<&IavToken> for IavToken {
        fn from(value: &IavToken) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IavTokenRequest {
        #[serde(rename = "tokenType")]
        pub token_type: String,
    }

    impl From<&IavTokenRequest> for IavTokenRequest {
        fn from(value: &IavTokenRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Label {
        pub amount: Amount,
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&Label> for Label {
        fn from(value: &Label) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LabelListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&LabelListResponse> for LabelListResponse {
        fn from(value: &LabelListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LabelReallocation {
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&LabelReallocation> for LabelReallocation {
        fn from(value: &LabelReallocation) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LabelReallocationRequest {
        pub amount: Amount,
        #[serde(rename = "fromLabelId")]
        pub from_label_id: String,
        #[serde(rename = "partnerId")]
        pub partner_id: String,
        #[serde(rename = "toLabelId")]
        pub to_label_id: String,
    }

    impl From<&LabelReallocationRequest> for LabelReallocationRequest {
        fn from(value: &LabelReallocationRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LedgerEntry {
        pub amount: Amount,
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&LedgerEntry> for LedgerEntry {
        fn from(value: &LedgerEntry) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LedgerEntryListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&LedgerEntryListResponse> for LedgerEntryListResponse {
        fn from(value: &LedgerEntryListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum ListCustomersStatus {
        #[serde(rename = "unverified")]
        Unverified,
        #[serde(rename = "retry")]
        Retry,
        #[serde(rename = "document")]
        Document,
        #[serde(rename = "verified")]
        Verified,
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "deactivated")]
        Deactivated,
    }

    impl From<&ListCustomersStatus> for ListCustomersStatus {
        fn from(value: &ListCustomersStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for ListCustomersStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Unverified => "unverified".to_string(),
                Self::Retry => "retry".to_string(),
                Self::Document => "document".to_string(),
                Self::Verified => "verified".to_string(),
                Self::Suspended => "suspended".to_string(),
                Self::Deactivated => "deactivated".to_string(),
            }
        }
    }

    impl std::str::FromStr for ListCustomersStatus {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "unverified" => Ok(Self::Unverified),
                "retry" => Ok(Self::Retry),
                "document" => Ok(Self::Document),
                "verified" => Ok(Self::Verified),
                "suspended" => Ok(Self::Suspended),
                "deactivated" => Ok(Self::Deactivated),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ListCustomersStatus {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ListCustomersStatus {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ListCustomersStatus {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MassPayment {
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub metadata: serde_json::Map<String, serde_json::Value>,
        pub status: String,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub total: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "totalFees",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub total_fees: serde_json::Map<String, serde_json::Value>,
    }

    impl From<&MassPayment> for MassPayment {
        fn from(value: &MassPayment) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MassPaymentItem {
        pub amount: Amount,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub metadata: serde_json::Map<String, serde_json::Value>,
        pub status: String,
    }

    impl From<&MassPaymentItem> for MassPaymentItem {
        fn from(value: &MassPaymentItem) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MassPaymentItemListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&MassPaymentItemListResponse> for MassPaymentItemListResponse {
        fn from(value: &MassPaymentItemListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MassPaymentItemRequestBody {
        #[serde(
            rename = "achDetails",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub ach_details: serde_json::Map<String, serde_json::Value>,
        pub amount: Amount,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        #[serde(rename = "_links")]
        pub links: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub metadata: serde_json::Map<String, serde_json::Value>,
    }

    impl From<&MassPaymentItemRequestBody> for MassPaymentItemRequestBody {
        fn from(value: &MassPaymentItemRequestBody) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MassPaymentListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&MassPaymentListResponse> for MassPaymentListResponse {
        fn from(value: &MassPaymentListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MassPaymentRequestBody {
        #[serde(
            rename = "achDetails",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub ach_details: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        ///
        pub items: Vec<MassPaymentItemRequestBody>,
        #[serde(rename = "_links")]
        pub links: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub metadata: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
    }

    impl From<&MassPaymentRequestBody> for MassPaymentRequestBody {
        fn from(value: &MassPaymentRequestBody) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MicroDeposits {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&MicroDeposits> for MicroDeposits {
        fn from(value: &MicroDeposits) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MicroDepositsInitiated {
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub failure: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub status: String,
    }

    impl From<&MicroDepositsInitiated> for MicroDepositsInitiated {
        fn from(value: &MicroDepositsInitiated) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Notification {
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "fromEmail")]
        pub from_email: String,
        #[serde(rename = "fromName")]
        pub from_name: String,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "replyToEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reply_to_email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sent: Option<String>,
        pub status: String,
        #[serde(
            rename = "statusReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_reason: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subject: Option<String>,
        pub template: String,
        #[serde(rename = "toEmail")]
        pub to_email: String,
        #[serde(rename = "toName", default, skip_serializing_if = "Option::is_none")]
        pub to_name: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&Notification> for Notification {
        fn from(value: &Notification) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NotificationListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&NotificationListResponse> for NotificationListResponse {
        fn from(value: &NotificationListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OAuthResponse {
        pub access_token: String,
        pub expires_in: f64,
        pub token_type: String,
    }

    impl From<&OAuthResponse> for OAuthResponse {
        fn from(value: &OAuthResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Owner {
        pub address: Address,
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub embedded: std::collections::HashMap<String, f64>,
        #[serde(rename = "firstName")]
        pub first_name: String,
        pub id: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(rename = "verificationStatus")]
        pub verification_status: String,
    }

    impl From<&Owner> for Owner {
        fn from(value: &Owner) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Ownership {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub embedded: std::collections::HashMap<String, f64>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub status: String,
    }

    impl From<&Ownership> for Ownership {
        fn from(value: &Ownership) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Passport {
        pub country: String,
        pub number: String,
    }

    impl From<&Passport> for Passport {
        fn from(value: &Passport) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProcessResult {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: serde_json::Map<String, serde_json::Value>,
        pub total: i32,
    }

    impl From<&ProcessResult> for ProcessResult {
        fn from(value: &ProcessResult) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Question {
        pub answers: Vec<Answer>,
        pub id: String,
        pub text: String,
    }

    impl From<&Question> for Question {
        fn from(value: &Question) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Session {
        #[serde(rename = "_embedded")]
        pub embedded: std::collections::HashMap<String, f64>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub questions: Vec<Question>,
    }

    impl From<&Session> for Session {
        fn from(value: &Session) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Transfer {
        #[serde(
            rename = "achDetails",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub ach_details: serde_json::Map<String, serde_json::Value>,
        pub amount: Amount,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub clearing: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(
            rename = "individualAchId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub individual_ach_id: Option<String>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub metadata: serde_json::Map<String, serde_json::Value>,
        pub status: String,
    }

    impl From<&Transfer> for Transfer {
        fn from(value: &Transfer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransferFailure {
        pub code: String,
        pub created: String,
        pub description: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub explanation: Option<String>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
    }

    impl From<&TransferFailure> for TransferFailure {
        fn from(value: &TransferFailure) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransferList {
        pub transfers: Vec<Transfer>
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransferListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: TransferList,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&TransferListResponse> for TransferListResponse {
        fn from(value: &TransferListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransferRequestBody {
        #[serde(
            rename = "achDetails",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ach_details: Option<serde_json::Map<String, serde_json::Value>>,
        pub amount: Amount,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clearing: Option<serde_json::Map<String, serde_json::Value>>,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fees: Option<serde_json::Map<String, serde_json::Value>>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    }

    impl From<&TransferRequestBody> for TransferRequestBody {
        fn from(value: &TransferRequestBody) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateBankRequest {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub account_number: Option<String>,
        #[serde(
            rename = "bankAccountType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bank_account_type: Option<String>,
        #[serde(
            rename = "_links",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub links: serde_json::Map<String, serde_json::Value>,
        pub name: String,
        #[serde(
            rename = "routingNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub routing_number: Option<String>,
    }

    impl From<&UpdateBankRequest> for UpdateBankRequest {
        fn from(value: &UpdateBankRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateCustomer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address2: Option<String>,
        #[serde(
            rename = "businessClassification",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_classification: Option<String>,
        #[serde(
            rename = "businessName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_name: Option<String>,
        #[serde(
            rename = "businessType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub business_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub controller: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "correlationId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub correlation_id: Option<String>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub date_of_birth: Option<NaiveDate>,
        #[serde(
            rename = "doingBusinessAs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub doing_business_as: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ein: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub postal_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssn: Option<UpdateCustomerSsn>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub website: Option<String>,
    }

    impl From<&UpdateCustomer> for UpdateCustomer {
        fn from(value: &UpdateCustomer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct UpdateCustomerSsn(String);
    impl std::ops::Deref for UpdateCustomerSsn {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<UpdateCustomerSsn> for String {
        fn from(value: UpdateCustomerSsn) -> Self {
            value.0
        }
    }

    impl From<&UpdateCustomerSsn> for UpdateCustomerSsn {
        fn from(value: &UpdateCustomerSsn) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for UpdateCustomerSsn {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if regress::Regex::new("^\\d{3}-\\d{2}-\\d{4}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^\\d{3}-\\d{2}-\\d{4}$\"");
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for UpdateCustomerSsn {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for UpdateCustomerSsn {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for UpdateCustomerSsn {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for UpdateCustomerSsn {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateJobRequestBody {
        pub status: String,
    }

    impl From<&UpdateJobRequestBody> for UpdateJobRequestBody {
        fn from(value: &UpdateJobRequestBody) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateOwnerRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub date_of_birth: Option<NaiveDate>,
        #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub passport: Option<Passport>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssn: Option<String>,
    }

    impl From<&UpdateOwnerRequest> for UpdateOwnerRequest {
        fn from(value: &UpdateOwnerRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateSubscription {
        pub paused: bool,
    }

    impl From<&UpdateSubscription> for UpdateSubscription {
        fn from(value: &UpdateSubscription) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateTransfer {
        pub status: String,
    }

    impl From<&UpdateTransfer> for UpdateTransfer {
        fn from(value: &UpdateTransfer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VerifyMicroDepositsRequest {
        pub amount1: Amount,
        pub amount2: Amount,
    }

    impl From<&VerifyMicroDepositsRequest> for VerifyMicroDepositsRequest {
        fn from(value: &VerifyMicroDepositsRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Webhook {
        #[serde(rename = "accountId")]
        pub account_id: serde_json::Map<String, serde_json::Value>,
        ///
        pub attempts: Vec<WebhookAttempt>,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "eventId")]
        pub event_id: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        #[serde(rename = "subscriptionId")]
        pub subscription_id: serde_json::Map<String, serde_json::Value>,
        pub topic: String,
    }

    impl From<&Webhook> for Webhook {
        fn from(value: &Webhook) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookAttempt {
        pub id: String,
        pub request: WebhookHttpRequest,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub response: serde_json::Map<String, serde_json::Value>,
    }

    impl From<&WebhookAttempt> for WebhookAttempt {
        fn from(value: &WebhookAttempt) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookEventListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&WebhookEventListResponse> for WebhookEventListResponse {
        fn from(value: &WebhookEventListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookHeader {
        pub name: String,
        pub value: String,
    }

    impl From<&WebhookHeader> for WebhookHeader {
        fn from(value: &WebhookHeader) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookHttpRequest {
        pub body: String,
        ///
        pub headers: Vec<WebhookHeader>,
        pub timestamp: String,
        pub url: String,
    }

    impl From<&WebhookHttpRequest> for WebhookHttpRequest {
        fn from(value: &WebhookHttpRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&WebhookListResponse> for WebhookListResponse {
        fn from(value: &WebhookListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookRetry {
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub timestamp: String,
    }

    impl From<&WebhookRetry> for WebhookRetry {
        fn from(value: &WebhookRetry) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookRetryRequestListResponse {
        #[serde(rename = "_embedded")]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub total: i32,
    }

    impl From<&WebhookRetryRequestListResponse> for WebhookRetryRequestListResponse {
        fn from(value: &WebhookRetryRequestListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WebhookSubscription {
        pub created: String,
        #[serde(
            rename = "_embedded",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub embedded: serde_json::Map<String, serde_json::Value>,
        pub id: String,
        #[serde(rename = "_links")]
        pub links: std::collections::HashMap<String, HalLink>,
        pub paused: bool,
        pub url: String,
    }

    impl From<&WebhookSubscription> for WebhookSubscription {
        fn from(value: &WebhookSubscription) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Dwolla API
///
///Dwolla API Documentation
///
///https://www.dwolla.com/legal/tos/
///
///Version: 2.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

async fn get_access_token(base_url: &str) -> String {
    let client_id = std::env::var("DWOLLA_KEY").expect("Missing environment variable DWOLLA_KEY").trim_end().to_string();
    let client_secret = std::env::var("DWOLLA_SECRET").expect("Missing environment variable DWOLLA_SECRET").trim_end().to_string();
    let params = [
        (String::from("client_id"), client_id),
        (String::from("client_secret"), client_secret),
        (String::from("grant_type"), String::from("client_credentials")),
    ];

    let client = reqwest::Client::new();
    client.post(&format!("{}/token", base_url))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await.unwrap().json::<OAuthResponse>().await.unwrap().access_token
}
impl Client {
    /// Create a new client from environment variables.
    /// 
    /// Requires DWOLLA_URL, DWOLLA_KEY and DWOLLA_SECRET all to be set.
    /// 
    /// This creates an access token each time. Later, this functionality will be exposed, so an access_token can be stored in a database and provided later.
    /// 
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub async fn from_env() -> Self {
        let mut baseurl = std::env::var("DWOLLA_URL")
        .expect("Missing environment variable DWOLLA_URL");
        baseurl = baseurl.trim_end_matches('/').to_string();
        baseurl = baseurl.trim_end().to_string();
        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(&format!("Bearer {}", get_access_token(&baseurl).await)).unwrap());
        headers.insert(ACCEPT, header::HeaderValue::from_static("application/vnd.dwolla.v1.hal+json"));
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
                .default_headers(headers)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new().default_headers(headers);
        Self::new_with_client(&baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "2.0"
    }
}

impl Client {
    ///root
    ///
    ///Entry point to the Dwolla API.
    ///
    ///Sends a `GET` request to `/`
    pub async fn root<'a>(&'a self) -> Result<CatalogResponse, Error<()>> {
        let url = format!("{}/", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => {
                let catalog_response: CatalogResponse = response.json().await?;
                Ok(catalog_response)
            },
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///id
    ///
    ///Get account info by id.
    ///
    ///Sends a `GET` request to `/accounts/{id}`
    ///
    ///Arguments:
    /// - `id`: Account ID to get info for.
    pub async fn get_account<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/accounts/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getAccountFundingSources
    ///
    ///Get an account's funding sources.
    ///
    ///Sends a `GET` request to `/accounts/{id}/funding-sources`
    ///
    ///Arguments:
    /// - `id`: Account id to get funding sources for.
    /// - `removed`: Filter funding sources by this value.
    pub async fn get_account_funding_sources<'a>(
        &'a self,
        id: &'a str,
        removed: Option<bool>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/accounts/{}/funding-sources",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &removed {
            query.push(("removed", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createFundingSourcesToken
    ///
    ///Create an token that is capable of adding a financial institution for
    /// the given Dwolla Account.
    ///
    ///Sends a `POST` request to `/accounts/{id}/funding-sources-token`
    ///
    ///Arguments:
    /// - `id`: Account ID to create token for.
    pub async fn create_funding_sources_token<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/accounts/{}/funding-sources-token",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getAccountIavToken
    ///
    ///Get iav token for account.
    ///
    ///Sends a `POST` request to `/accounts/{id}/iav-token`
    ///
    ///Arguments:
    /// - `id`: ID of account.
    /// - `body`: IAV Token options
    pub async fn get_account_iav_token<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::IavTokenRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/accounts/{}/iav-token",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getByAccount
    ///
    ///List mass payments for an account.
    ///
    ///Sends a `GET` request to `/accounts/{id}/mass-payments`
    ///
    ///Arguments:
    /// - `id`: Account ID
    /// - `correlation_id`: Correlation ID to search by.
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    pub async fn get_account_mass_payments<'a>(
        &'a self,
        id: &'a str,
        correlation_id: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i32>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/accounts/{}/mass-payments",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &correlation_id {
            query.push(("correlationId", v.to_string()));
        }

        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    

    ///getAccountTransfers
    ///
    ///List and search transfers for an account.
    ///
    ///Sends a `GET` request to `/accounts/{id}/transfers`
    ///
    ///Arguments:
    /// - `id`: Account id to get transfers for.
    /// - `correlation_id`: A correlationId value specified on a transfer or
    ///   mass payment item.
    /// - `end_amount`: Only include transfers with an amount equal to or less
    ///   than endAmount.
    /// - `end_date`: Only include transfers created before this date. ISO-8601
    ///   format is YYYY-MM-DD.
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    /// - `start_amount`: Only include transfers with an amount equal to or
    ///   greater than startAmount.
    /// - `start_date`: Only include transfers created after this date. ISO-8601
    ///   format is YYYY-MM-DD.
    /// - `status`: What status to filter by.
    pub async fn get_account_transfers<'a>(
        &'a self,
        id: &'a str,
        correlation_id: Option<&'a str>,
        end_amount: Option<&'a str>,
        end_date: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i32>,
        start_amount: Option<&'a str>,
        start_date: Option<&'a str>,
        status: Option<types::GetAccountTransfersStatus>,
    ) -> Result<TransferListResponse, Error<()>> {
        let url = format!(
            "{}/accounts/{}/transfers",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &correlation_id {
            query.push(("correlationId", v.to_string()));
        }

        if let Some(v) = &end_amount {
            query.push(("endAmount", v.to_string()));
        }

        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }

        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        if let Some(v) = &start_amount {
            query.push(("startAmount", v.to_string()));
        }

        if let Some(v) = &start_date {
            query.push(("startDate", v.to_string()));
        }

        if let Some(v) = &status {
            query.push(("status", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => {
                Ok(response.json::<TransferListResponse>().await.unwrap())
            },
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getById
    ///
    ///Get an beneficial owner by ID
    ///
    ///Sends a `GET` request to `/beneficial-owners/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of beneficial owner.
    pub async fn get_beneficial_owner<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/beneficial-owners/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update
    ///
    ///Update a beneficial owner.
    ///
    ///Sends a `POST` request to `/beneficial-owners/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of beneficial owner.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Owner to update.
    pub async fn update_beneficial_owner<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::UpdateOwnerRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/beneficial-owners/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getBeneficiaOwnerDocuments
    ///
    ///Get documents uploaded for owner.
    ///
    ///Sends a `GET` request to `/beneficial-owners/{id}/documents`
    ///
    ///Arguments:
    /// - `id`: ID of beneficial owner.
    pub async fn get_beneficia_owner_documents<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/beneficial-owners/{}/documents",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///list
    ///
    ///Get a list business classifications.
    ///
    ///Sends a `GET` request to `/business-classifications`
    pub async fn list_business_classifications<'a>(
        &'a self,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/business-classifications", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getBusinessClassification
    ///
    ///Get a business classification with a list of industry classifications.
    ///
    ///Sends a `GET` request to `/business-classifications/{id}`
    ///
    ///Arguments:
    /// - `id`: Id of business classification to get.
    pub async fn get_business_classification<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/business-classifications/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///list
    ///
    ///Get a list of customers.
    ///
    ///Sends a `GET` request to `/customers`
    ///
    ///Arguments:
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    /// - `search`: Search term.
    /// - `email`: Email filter.
    pub async fn list_customers<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i32>,
        email: Option<&'a str>,
        status: Option<types::ListCustomersStatus>,
    ) -> Result<CustomerListResponse, Error<()>> {
        let url = format!("{}/customers", self.baseurl,);
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        if let Some(v) = &email {
            query.push(("email", v.to_string()));
        }

        if let Some(v) = &status {
            query.push(("status", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let url = request.url().as_ref().to_string();
        println!("{}", url);
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => {
                let catalog_response: CustomerListResponse = response.json().await?;
                Ok(catalog_response)
            },
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create
    ///
    ///Create a new customer.
    ///
    ///Sends a `POST` request to `/customers`
    /// 
    /// If successful, this returns the ID of the created customer
    ///
    ///Arguments:
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Customer to create.
    pub async fn create_customer<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateCustomer,
    ) -> Result<String, Error<FailedCreateCustomerResponse>> {
        let url = format!("{}/customers", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        let status = response.status();
        let headers = response.headers().to_owned();
        match status.as_u16() {
            201u16 => {
                let location = headers.get(LOCATION).unwrap().to_str().unwrap().to_string();
                let last_slash = location.rfind('/').unwrap();
                Ok(location[last_slash + 1..].to_string())
            },
            400u16 => {
                let err_response: FailedValidationCreateCustomerResponse = response.json().await?;
                // TODO: can we avoid this clone?
                Err(Error::ErrorResponse(ResponseValue::new(FailedCreateCustomerResponse::Validation(err_response), status, headers)))
            },
            403u16 => Err(Error::ErrorResponse(ResponseValue::new(FailedCreateCustomerResponse::Empty(response), status, headers))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::new(FailedCreateCustomerResponse::Empty(response), status, headers))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getCustomer
    ///
    ///Get a customer by id
    ///
    ///Sends a `GET` request to `/customers/{id}`
    ///
    ///Arguments:
    /// - `id`: Id of customer to get.
    pub async fn get_customer<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<Customer, Error<()>> {
        let url = format!(
            "{}/customers/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => {
                Ok(response.json::<Customer>().await?)
            },
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///updateCustomer
    ///
    ///Update customer record. Personal customer records are re-verified upon
    /// update.
    ///
    ///Sends a `POST` request to `/customers/{id}`
    ///
    ///Arguments:
    /// - `id`: Id of customer to get.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Customer to update.
    pub async fn update_customer<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::UpdateCustomer,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///addBeneficialOwner
    ///
    ///Add a beneficial owner
    ///
    ///Sends a `POST` request to `/customers/{id}/beneficial-owners`
    ///
    ///Arguments:
    /// - `id`: Customer id to add owner for for.
    /// - `body`: Beneficial owner to create.
    pub async fn add_beneficial_owner<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::CreateOwnerRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/beneficial-owners",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getOwnershipStatus
    ///
    ///Get a customer's ownership certification status.
    ///
    ///Sends a `GET` request to `/customers/{id}/beneficial-ownership`
    ///
    ///Arguments:
    /// - `id`: Customer id for beneficial ownership certification.
    pub async fn get_ownership_status<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/beneficial-ownership",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///changeOwnershipStatus
    ///
    ///Change ownership certification status.
    ///
    ///Sends a `POST` request to `/customers/{id}/beneficial-ownership`
    ///
    ///Arguments:
    /// - `id`: Customer id for beneficial ownership certification.
    /// - `body`: Status of ownership
    pub async fn change_ownership_status<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::CertifyRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/customers/{}/beneficial-ownership",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getCustomerDocuments
    ///
    ///Get documents uploaded for customer.
    ///
    ///Sends a `GET` request to `/customers/{id}/documents`
    ///
    ///Arguments:
    /// - `id`: ID of a customer.
    pub async fn get_customer_documents<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/documents",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getCustomerFundingSources
    ///
    ///Get a customer's funding sources.
    ///
    ///Sends a `GET` request to `/customers/{id}/funding-sources`
    ///
    ///Arguments:
    /// - `id`: ID for a Customer.
    /// - `removed`: Filter funding sources by this value.
    pub async fn get_customer_funding_sources<'a>(
        &'a self,
        id: &'a str,
        removed: Option<bool>,
    ) -> Result<FundingSourceListResponse, Error<()>> {
        let url = format!(
            "{}/customers/{}/funding-sources",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &removed {
            query.push(("removed", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(response.json::<FundingSourceListResponse>().await.unwrap()),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createCustomerFundingSource
    ///
    ///Create a new funding source.
    ///
    ///Sends a `POST` request to `/customers/{id}/funding-sources`
    ///
    ///Arguments:
    /// - `id`: ID for a Customer.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Funding source to create.
    pub async fn create_customer_funding_source<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateFundingSourceRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/funding-sources",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createCustomerFundingSource
    ///
    ///Create a new funding source with plaid.
    ///
    ///Sends a `POST` request to `/customers/{id}/funding-sources`
    /// 
    /// If successful, this returns the id of the created funding source.
    ///
    ///Arguments:
    /// - `id`: ID for a Customer.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Funding source to create.
    pub async fn create_customer_funding_source_with_plaid<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateFundingSourceWithPlaidRequest,
    ) -> Result<String, Error<()>> {
        let url = format!(
            "{}/customers/{}/funding-sources",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        
        let result = self.client.execute(request).await;
        let response = result?;
        let headers = response.headers().clone();
        match response.status().as_u16() {
            201u16 => {
                let location = headers.get(LOCATION).unwrap().to_str().unwrap().to_string();
                let last_slash = location.rfind('/').unwrap();
                Ok(location[last_slash + 1..].to_string())
            },
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createFundingSourcesTokenForCustomer
    ///
    ///Create an token that is capable of adding a financial institution for
    /// the given customer.
    ///
    ///Sends a `POST` request to `/customers/{id}/funding-sources-token`
    ///
    ///Arguments:
    /// - `id`: ID of customer.
    pub async fn create_customer_funding_sources_token<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/funding-sources-token",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getCustomerIavToken
    ///
    ///Get iav token for customer.
    ///
    ///Sends a `POST` request to `/customers/{id}/iav-token`
    ///
    ///Arguments:
    /// - `id`: ID of customer.
    pub async fn get_customer_iav_token<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/iav-token",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getLabelsForCustomer
    ///
    ///Get labels for customer.
    ///
    ///Sends a `GET` request to `/customers/{id}/labels`
    ///
    ///Arguments:
    /// - `id`: Customer ID.
    pub async fn get_labels_for_customer<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/labels",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createLabel
    ///
    ///Create a label.
    ///
    ///Sends a `POST` request to `/customers/{id}/labels`
    ///
    ///Arguments:
    /// - `id`: Customer ID.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Label to create.
    pub async fn create_label<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateCustomerLabelRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/labels",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getByCustomer
    ///
    ///Get a customer's mass payments.
    ///
    ///Sends a `GET` request to `/customers/{id}/mass-payments`
    ///
    ///Arguments:
    /// - `id`: Customer ID
    /// - `correlation_id`: Correlation ID to search by.
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    pub async fn get_customer_mass_payments<'a>(
        &'a self,
        id: &'a str,
        correlation_id: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i32>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/mass-payments",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &correlation_id {
            query.push(("correlationId", v.to_string()));
        }

        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getNotifications
    ///
    ///Get a customer's notifications.
    ///
    ///Sends a `GET` request to `/customers/{id}/notifications`
    ///
    ///Arguments:
    /// - `id`: Customer id to get notifications for.
    pub async fn get_customer_notifications<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/customers/{}/notifications",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getCustomerTransfers
    ///
    ///Get a customer's transfers.
    ///
    ///Sends a `GET` request to `/customers/{id}/transfers`
    ///
    ///Arguments:
    /// - `id`: Customer id to get transfers for.
    /// - `correlation_id`: A correlationId value specified on a transfer or
    ///   mass payment item.
    /// - `end_amount`: Only include transfers with an amount equal to or less
    ///   than endAmount.
    /// - `end_date`: Only include transfers created before this date. ISO-8601
    ///   format is YYYY-MM-DD.
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    /// - `start_amount`: Only include transfers with an amount equal to or
    ///   greater than startAmount.
    /// - `start_date`: Only include transfers created after this date. ISO-8601
    ///   format is YYYY-MM-DD.
    /// - `status`: What status to filter by.
    pub async fn get_customer_transfers<'a>(
        &'a self,
        id: &'a str,
        correlation_id: Option<&'a str>,
        end_amount: Option<&'a str>,
        end_date: Option<&'a str>,
        limit: Option<i32>,
        offset: Option<i32>,
        start_amount: Option<&'a str>,
        start_date: Option<&'a str>,
        status: Option<types::GetCustomerTransfersStatus>,
    ) -> Result<TransferListResponse, Error<()>> {
        let url = format!(
            "{}/customers/{}/transfers",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &correlation_id {
            query.push(("correlationId", v.to_string()));
        }

        if let Some(v) = &end_amount {
            query.push(("endAmount", v.to_string()));
        }

        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }

        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        if let Some(v) = &start_amount {
            query.push(("startAmount", v.to_string()));
        }

        if let Some(v) = &start_date {
            query.push(("startDate", v.to_string()));
        }

        if let Some(v) = &status {
            query.push(("status", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => {
                Ok(response.json::<TransferListResponse>().await.unwrap())
            },
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getDocument
    ///
    ///Get a document by id
    ///
    ///Sends a `GET` request to `/documents/{id}`
    ///
    ///Arguments:
    /// - `id`: Id of document to get.
    pub async fn get_document<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/documents/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///events
    ///
    ///List events.
    ///
    ///Sends a `GET` request to `/events`
    ///
    ///Arguments:
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    pub async fn list_events<'a>(
        &'a self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/events", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///id
    ///
    ///Get an event by id.
    ///
    ///Sends a `GET` request to `/events/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of application event to get.
    pub async fn get_event<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/events/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createFundingSource
    ///
    ///Create a new funding source.
    ///
    ///Sends a `POST` request to `/funding-sources`
    ///
    ///Arguments:
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Funding source to create.
    pub async fn create_account_funding_source<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateFundingSourceRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/funding-sources", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///id
    ///
    ///Get a funding source by id.
    ///
    ///Sends a `GET` request to `/funding-sources/{id}`
    ///
    ///Arguments:
    /// - `id`: Funding source ID.
    pub async fn get_funding_source<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<FundingSource, Error<()>> {
        let url = format!(
            "{}/funding-sources/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(response.json::<FundingSource>().await?),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update
    ///
    ///Update a funding source.
    ///
    ///Sends a `POST` request to `/funding-sources/{id}`
    ///
    ///Arguments:
    /// - `id`: Funding source ID.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: request body to update a funding source
    pub async fn update_funding_source<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::UpdateBankRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/funding-sources/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getBalance
    ///
    ///Get the balance of a funding source.
    ///
    ///Sends a `GET` request to `/funding-sources/{id}/balance`
    ///
    ///Arguments:
    /// - `id`: Funding source ID to get the balance for.
    pub async fn get_balance<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<f64, Error<()>> {
        let url = format!(
            "{}/funding-sources/{}/balance",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => {
                let balance:FundingSourceBalance  = response.json().await.unwrap();
                // TODO: this assumes USD
                // can address this when progenitor works with dwolla's spec
                Ok(balance.balance.value.as_str().parse::<f64>().unwrap())
            },
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///verifyMicroDepositsExist
    ///
    ///Verify pending verifications exist.
    ///
    ///Sends a `GET` request to `/funding-sources/{id}/micro-deposits`
    ///
    ///Arguments:
    /// - `id`: Funding source ID.
    pub async fn get_micro_deposits<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/funding-sources/{}/micro-deposits",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///microDeposits
    ///
    ///Initiate or verify micro deposits for bank verification.
    ///
    ///Sends a `POST` request to `/funding-sources/{id}/micro-deposits`
    ///
    ///Arguments:
    /// - `id`: Funding source ID.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Optional micro deposit amounts for verification
    pub async fn initiate_micro_deposits<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::VerifyMicroDepositsRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/funding-sources/{}/micro-deposits",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a KBA session by id
    ///
    ///Retrieve KBA Questions
    ///
    ///Sends a `GET` request to `/kba/{id}`
    ///
    ///Arguments:
    /// - `id`: Id of KBA session.
    pub async fn get_kba_session<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/kba/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Attempt to answer KBA questions
    ///
    ///Answer KBA question set
    ///
    ///Sends a `POST` request to `/kba/{id}`
    ///
    ///Arguments:
    /// - `id`: Id of KBA session.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: KBA answers
    pub async fn answer_kba_questions<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::AnswerKbaQuestionsRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/kba/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///byId
    ///
    ///Get a label by id.
    ///
    ///Sends a `GET` request to `/labels/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of a Label.
    pub async fn get_label<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/labels/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///removeLabel
    ///
    ///Remove a label.
    ///
    ///Sends a `DELETE` request to `/labels/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of a Label.
    pub async fn remove_label<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/labels/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getLedgerEntriesForLabel
    ///
    ///Get ledger entries by label id.
    ///
    ///Sends a `GET` request to `/labels/{id}/ledger-entries`
    ///
    ///Arguments:
    /// - `id`: ID of label to for label ledger entries.
    pub async fn get_ledger_entries_for_label<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/labels/{}/ledger-entries",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///addLedgerEntry
    ///
    ///Add ledger entry.
    ///
    ///Sends a `POST` request to `/labels/{id}/ledger-entries`
    ///
    ///Arguments:
    /// - `id`: ID of label to for label ledger entries.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Ledger entry to add.
    pub async fn add_ledger_entry<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::AddLabelLedgerEntryRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/labels/{}/ledger-entries",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///reallocateLabel
    ///
    ///Reallocate two labels.
    ///
    ///Sends a `POST` request to `/label-reallocations`
    ///
    ///Arguments:
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Labels to reallocate.
    pub async fn reallocate_label<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::LabelReallocationRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/label-reallocations", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getLabelReallocation
    ///
    ///Get label reallocation.
    ///
    ///Sends a `GET` request to `/label-reallocations/{id}`
    ///
    ///Arguments:
    /// - `id`: Label reallocation ID
    pub async fn get_label_reallocation<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/label-reallocations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getLabelLedgerEntry
    ///
    ///Get a ledger entry by id.
    ///
    ///Sends a `GET` request to `/ledger-entries/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of ledger entry to get.
    pub async fn by_id3<'a>(&'a self, id: &'a str) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/ledger-entries/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///byId
    ///
    ///Get a mass payment item by id.
    ///
    ///Sends a `GET` request to `/mass-payment-items/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of mass payment item to get.
    pub async fn get_mass_payment_item<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/mass-payment-items/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create
    ///
    ///Create a new mass payment.
    ///
    ///Sends a `POST` request to `/mass-payments`
    ///
    ///Arguments:
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Mass payment request.
    pub async fn create_mass_payment<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::MassPaymentRequestBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/mass-payments", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///byId
    ///
    ///Get a mass payment by id.
    ///
    ///Sends a `GET` request to `/mass-payments/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of mass payment.
    pub async fn get_mass_payment<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/mass-payments/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update
    ///
    ///Update a mass-payment.
    ///
    ///Sends a `POST` request to `/mass-payments/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of mass payment.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Mass-payment to update.
    pub async fn update_mass_payment<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::UpdateJobRequestBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/mass-payments/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getMassPaymentItems
    ///
    ///Get a mass payment's items.
    ///
    ///Sends a `GET` request to `/mass-payments/{id}/items`
    ///
    ///Arguments:
    /// - `id`: Mass payment id to get items for.
    /// - `correlation_id`: Correlation ID to search by.
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    /// - `status`: What status to filter by.
    pub async fn get_mass_payment_items<'a>(
        &'a self,
        id: &'a str,
        correlation_id: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i32>,
        status: Option<types::GetMassPaymentItemsStatus>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/mass-payments/{}/items",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &correlation_id {
            query.push(("correlationId", v.to_string()));
        }

        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        if let Some(v) = &status {
            query.push(("status", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///byId
    ///
    ///Get a notification by id.
    ///
    ///Sends a `GET` request to `/notifications/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of notification to get.
    pub async fn get_notification<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/notifications/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///createAuthorization
    ///
    ///Create a new on-demand authorization.
    ///
    ///Sends a `POST` request to `/on-demand-authorizations`
    pub async fn create_authorization<'a>(
        &'a self,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/on-demand-authorizations", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///simulations
    ///
    ///Simulate ach processing
    ///
    ///Sends a `POST` request to `/sandbox-simulations`
    pub async fn sandbox_simulations<'a>(&'a self) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/sandbox-simulations", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create
    ///
    ///Create a new transfer.
    ///
    ///Sends a `POST` request to `/transfers`
    /// 
    /// If successful, this returns the ID of the created transfer.
    ///
    ///Arguments:
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Transfer request.
    pub async fn create_transfer<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::TransferRequestBody,
    ) -> Result<String, Error<()>> {
        let url = format!("{}/transfers", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        println!("{:#?}", serde_json::to_string(body).unwrap());

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => {
                Ok(response.headers().get(LOCATION).unwrap().to_str().unwrap().to_string())
            }
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///byId
    ///
    ///Get a transfer by id.
    ///
    ///Sends a `GET` request to `/transfers/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of transfer.
    pub async fn get_transfer<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/transfers/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update
    ///
    ///Update a transfer.
    ///
    ///Sends a `POST` request to `/transfers/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of transfer.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Transfer to update.
    pub async fn update_transfer<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::UpdateTransfer,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/transfers/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///failureById
    ///
    ///Get a bank transfer failure by transfer id.
    ///
    ///Sends a `GET` request to `/transfers/{id}/failure`
    ///
    ///Arguments:
    /// - `id`: ID of failed bank transfer to get.
    pub async fn get_transfer_failure<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/transfers/{}/failure",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getFeesBySource
    ///
    ///Get a transfer's fees.
    ///
    ///Sends a `GET` request to `/transfers/{id}/fees`
    ///
    ///Arguments:
    /// - `id`: Transfer id to get fees for.
    pub async fn get_fees_by_source<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/transfers/{}/fees",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///list
    ///
    ///Get the list of webhooks.
    ///
    ///Sends a `GET` request to `/webhook-subscriptions`
    pub async fn list_webhoook_subscriptions<'a>(
        &'a self,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/webhook-subscriptions", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create
    ///
    ///Create a new webhook subscription.
    ///
    ///Sends a `POST` request to `/webhook-subscriptions`
    ///
    ///Arguments:
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Webhook subscription to create.
    pub async fn create_webhook_subscription<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateWebhook,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/webhook-subscriptions", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///id
    ///
    ///Get a webhook subscription by id.
    ///
    ///Sends a `GET` request to `/webhook-subscriptions/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of webhook subscription.
    pub async fn get_webhook_subscription<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/webhook-subscriptions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///updateSubscription
    ///
    ///Update a subscription by id.
    ///
    ///Sends a `POST` request to `/webhook-subscriptions/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of webhook subscription.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    /// - `body`: Details to update.
    pub async fn update_webhook_subscription<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
        body: &'a types::UpdateSubscription,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/webhook-subscriptions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deleteById
    ///
    ///Delete a webhook subscription by id.
    ///
    ///Sends a `DELETE` request to `/webhook-subscriptions/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of webhook subscription.
    pub async fn delete_by_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/webhook-subscriptions/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///getWebhooksForWebhoookSubscription
    ///
    ///Get webhooks by subscription id.
    ///
    ///Sends a `GET` request to `/webhook-subscriptions/{id}/webhooks`
    ///
    ///Arguments:
    /// - `id`: ID of webhook to get.
    /// - `end_date`: Only include transfers created before this date. ISO-8601
    ///   format is YYYY-MM-DD.
    /// - `limit`: How many results to return.
    /// - `offset`: How many results to skip.
    /// - `start_date`: Only include transfers created after this date. ISO-8601
    ///   format is YYYY-MM-DD.
    pub async fn hooks_by_id<'a>(
        &'a self,
        id: &'a str,
        end_date: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i32>,
        start_date: Option<&'a str>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/webhook-subscriptions/{}/webhooks",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }

        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }

        if let Some(v) = &offset {
            query.push(("offset", v.to_string()));
        }

        if let Some(v) = &start_date {
            query.push(("startDate", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///id
    ///
    ///Get a webhook by id.
    ///
    ///Sends a `GET` request to `/webhooks/{id}`
    ///
    ///Arguments:
    /// - `id`: ID of webhook to get.
    pub async fn get_webhook<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/webhooks/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///retriesById
    ///
    ///Get retries requested by webhook id.
    ///
    ///Sends a `GET` request to `/webhooks/{id}/retries`
    ///
    ///Arguments:
    /// - `id`: ID of webhook.
    pub async fn retries_by_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/webhooks/{}/retries",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///retryWebhook
    ///
    ///Retry a webhook by id.
    ///
    ///Sends a `POST` request to `/webhooks/{id}/retries`
    ///
    ///Arguments:
    /// - `id`: ID of webhook.
    /// - `idempotency_key`: Unique key used to prevent duplication over a short
    ///   period of time.
    pub async fn retry_webhook<'a>(
        &'a self,
        id: &'a str,
        idempotency_key: Option<&'a str>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/webhooks/{}/retries",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = idempotency_key {
            header_map.append("Idempotency-Key", HeaderValue::try_from(v)?);
        }

        let request = self.client.post(url).headers(header_map).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    pub fn get_transfer_links(&self, source_id: &str, dest_id: &str) -> HashMap<String, HalLink> {
        let mut res = HashMap::new();
        let source = format!("{}/funding-sources/{}", self.baseurl, source_id);
        let dest = format!("{}/funding-sources/{}", self.baseurl, dest_id);
        res.insert(String::from("source"), HalLink{href: source, resource_type: None, type_: None});
        res.insert(String::from("destination"), HalLink{href: dest, resource_type: None, type_: None});
        res
    }
}

pub mod prelude {
    pub use super::Client;
}

