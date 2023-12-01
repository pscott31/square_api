
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalRecipient {
    pub amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receivable_id: Option<String>,
}
impl std::fmt::Display for AdditionalRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}