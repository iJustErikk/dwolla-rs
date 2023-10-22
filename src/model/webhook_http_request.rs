
use serde::{Serialize, Deserialize};
use super::WebhookHeader;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookHttpRequest {
    pub body: String,
    pub headers: Vec<WebhookHeader>,
    pub timestamp: String,
    pub url: String,
}
impl std::fmt::Display for WebhookHttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}