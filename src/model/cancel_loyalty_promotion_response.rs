
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyPromotion};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelLoyaltyPromotionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_promotion: Option<LoyaltyPromotion>,
}
impl std::fmt::Display for CancelLoyaltyPromotionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}