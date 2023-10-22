
use serde::{Serialize, Deserialize};
use super::WebhookAttempt;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(rename = "_embedded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<serde_json::Value>,
    #[serde(rename = "_links")]
    pub links: serde_json::Value,
    #[serde(rename = "accountId")]
    pub account_id: serde_json::Value,
    pub attempts: Vec<WebhookAttempt>,
    #[serde(rename = "eventId")]
    pub event_id: serde_json::Value,
    pub id: String,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: serde_json::Value,
    pub topic: String,
}
impl std::fmt::Display for Webhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}