
use serde::{Serialize, Deserialize};
use super::{CatalogObjectReference, LoyaltyProgramRewardDefinition};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramRewardTier {
    pub created_at: String,
    pub definition: LoyaltyProgramRewardDefinition,
    pub id: String,
    pub name: String,
    pub points: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_rule_reference: Option<CatalogObjectReference>,
}
impl std::fmt::Display for LoyaltyProgramRewardTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}