
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyPromotion};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListLoyaltyPromotionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_promotions: Option<Vec<LoyaltyPromotion>>,
}
impl std::fmt::Display for ListLoyaltyPromotionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}