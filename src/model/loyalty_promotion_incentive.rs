
use serde::{Serialize, Deserialize};
use super::{
    LoyaltyPromotionIncentivePointsAdditionData,
    LoyaltyPromotionIncentivePointsMultiplierData,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyPromotionIncentive {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_addition_data: Option<LoyaltyPromotionIncentivePointsAdditionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_multiplier_data: Option<LoyaltyPromotionIncentivePointsMultiplierData>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for LoyaltyPromotionIncentive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}