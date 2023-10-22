
use serde::{Serialize, Deserialize};
use super::Amount;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocateLabelRequired {
    pub amount: Amount,
    #[serde(rename = "fromLabelId")]
    pub from_label_id: String,
    #[serde(rename = "partnerId")]
    pub partner_id: String,
    #[serde(rename = "toLabelId")]
    pub to_label_id: String,
}
impl std::fmt::Display for ReallocateLabelRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}