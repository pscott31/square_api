
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyReward};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveLoyaltyRewardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<LoyaltyReward>,
}
impl std::fmt::Display for RetrieveLoyaltyRewardResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}