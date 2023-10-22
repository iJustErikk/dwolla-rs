
use serde::{Serialize, Deserialize};
use super::Amount;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassPaymentItemRequestBody {
    #[serde(rename = "_links")]
    pub links: serde_json::Value,
    #[serde(rename = "achDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_details: Option<serde_json::Value>,
    pub amount: Amount,
    #[serde(rename = "correlationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl std::fmt::Display for MassPaymentItemRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}