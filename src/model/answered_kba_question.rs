
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnsweredKbaQuestion {
    #[serde(rename = "answerId")]
    pub answer_id: String,
    #[serde(rename = "questionId")]
    pub question_id: String,
}
impl std::fmt::Display for AnsweredKbaQuestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}