
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramAccrualRuleCategoryData {
    pub category_id: String,
}
impl std::fmt::Display for LoyaltyProgramAccrualRuleCategoryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}