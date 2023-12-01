
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyEvent};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedeemLoyaltyRewardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<LoyaltyEvent>,
}
impl std::fmt::Display for RedeemLoyaltyRewardResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}