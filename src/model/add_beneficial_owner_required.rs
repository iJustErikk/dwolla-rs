
use serde::{Serialize, Deserialize};
use super::Address;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBeneficialOwnerRequired {
    pub address: Address,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    pub id: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
}
impl std::fmt::Display for AddBeneficialOwnerRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}