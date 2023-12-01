
use serde::{Serialize, Deserialize};
use super::{
    LoyaltyEventAccumulatePoints, LoyaltyEventAccumulatePromotionPoints,
    LoyaltyEventAdjustPoints, LoyaltyEventCreateReward, LoyaltyEventDeleteReward,
    LoyaltyEventExpirePoints, LoyaltyEventOther, LoyaltyEventRedeemReward,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_points: Option<LoyaltyEventAccumulatePoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_promotion_points: Option<LoyaltyEventAccumulatePromotionPoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_points: Option<LoyaltyEventAdjustPoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_reward: Option<LoyaltyEventCreateReward>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_reward: Option<LoyaltyEventDeleteReward>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_points: Option<LoyaltyEventExpirePoints>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    pub loyalty_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_event: Option<LoyaltyEventOther>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_reward: Option<LoyaltyEventRedeemReward>,
    pub source: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for LoyaltyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}