
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyPromotionIncentivePointsAdditionData {
    pub points_addition: i64,
}
impl std::fmt::Display for LoyaltyPromotionIncentivePointsAdditionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}