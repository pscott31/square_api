
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchLoyaltyRewardsRequestLoyaltyRewardQuery {
    pub loyalty_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for SearchLoyaltyRewardsRequestLoyaltyRewardQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}