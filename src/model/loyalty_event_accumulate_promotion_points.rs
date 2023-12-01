
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyEventAccumulatePromotionPoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_promotion_id: Option<String>,
    pub order_id: String,
    pub points: i64,
}
impl std::fmt::Display for LoyaltyEventAccumulatePromotionPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}