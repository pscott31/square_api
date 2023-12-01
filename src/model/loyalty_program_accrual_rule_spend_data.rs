
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleSpendData {
    pub amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_category_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_item_variation_ids: Option<Vec<String>>,
    pub tax_mode: String,
}
impl std::fmt::Display for LoyaltyProgramAccrualRuleSpendData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}