
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeRequestAdditionalRecipient {
    pub amount_money: Money,
    pub description: String,
    pub location_id: String,
}
impl std::fmt::Display for ChargeRequestAdditionalRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}