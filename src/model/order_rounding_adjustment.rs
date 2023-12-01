
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderRoundingAdjustment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderRoundingAdjustment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}