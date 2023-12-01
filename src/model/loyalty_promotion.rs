
use serde::{Serialize, Deserialize};
use super::{
    LoyaltyPromotionAvailableTimeData, LoyaltyPromotionIncentive,
    LoyaltyPromotionTriggerLimit, Money,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotion {
    pub available_time: LoyaltyPromotionAvailableTimeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub incentive: LoyaltyPromotionIncentive,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend_amount_money: Option<Money>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_limit: Option<LoyaltyPromotionTriggerLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl std::fmt::Display for LoyaltyPromotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}