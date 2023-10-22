
use serde::{Serialize, Deserialize};
use super::WebhookHttpRequest;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAttempt {
    pub id: String,
    pub request: WebhookHttpRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,
}
impl std::fmt::Display for WebhookAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}