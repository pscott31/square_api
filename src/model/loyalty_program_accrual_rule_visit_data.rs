
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramAccrualRuleVisitData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_money: Option<Money>,
    pub tax_mode: String,
}
impl std::fmt::Display for LoyaltyProgramAccrualRuleVisitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}