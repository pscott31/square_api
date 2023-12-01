
use serde::{Serialize, Deserialize};
use super::{
    LoyaltyProgramAccrualRule, LoyaltyProgramExpirationPolicy, LoyaltyProgramRewardTier,
    LoyaltyProgramTerminology,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgram {
    pub accrual_rules: Vec<LoyaltyProgramAccrualRule>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_policy: Option<LoyaltyProgramExpirationPolicy>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    pub reward_tiers: Vec<LoyaltyProgramRewardTier>,
    pub status: String,
    pub terminology: LoyaltyProgramTerminology,
    pub updated_at: String,
}
impl std::fmt::Display for LoyaltyProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}