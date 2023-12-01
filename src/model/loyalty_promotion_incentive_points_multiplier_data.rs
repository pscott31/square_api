
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyPromotionIncentivePointsMultiplierData {
    pub points_multiplier: i64,
}
impl std::fmt::Display for LoyaltyPromotionIncentivePointsMultiplierData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}