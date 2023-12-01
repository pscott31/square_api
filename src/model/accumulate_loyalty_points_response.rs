
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyEvent};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccumulateLoyaltyPointsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<LoyaltyEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<LoyaltyEvent>>,
}
impl std::fmt::Display for AccumulateLoyaltyPointsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}