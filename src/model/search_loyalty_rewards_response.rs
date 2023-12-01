
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyReward};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchLoyaltyRewardsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<LoyaltyReward>>,
}
impl std::fmt::Display for SearchLoyaltyRewardsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}