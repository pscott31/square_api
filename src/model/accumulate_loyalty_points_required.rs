
use serde::{Serialize, Deserialize};
use super::LoyaltyEventAccumulatePoints;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccumulateLoyaltyPointsRequired {
    pub account_id: String,
    pub accumulate_points: LoyaltyEventAccumulatePoints,
    pub idempotency_key: String,
    pub location_id: String,
}
impl std::fmt::Display for AccumulateLoyaltyPointsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}