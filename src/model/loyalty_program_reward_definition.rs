
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramRewardDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_ids: Option<Vec<String>>,
    pub discount_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_discount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_discount: Option<String>,
    pub scope: String,
}
impl std::fmt::Display for LoyaltyProgramRewardDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}