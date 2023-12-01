
use serde::{Serialize, Deserialize};
use super::{
    LoyaltyEventDateTimeFilter, LoyaltyEventLocationFilter,
    LoyaltyEventLoyaltyAccountFilter, LoyaltyEventOrderFilter, LoyaltyEventTypeFilter,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyEventFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_filter: Option<LoyaltyEventDateTimeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_filter: Option<LoyaltyEventLocationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_account_filter: Option<LoyaltyEventLoyaltyAccountFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<LoyaltyEventOrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<LoyaltyEventTypeFilter>,
}
impl std::fmt::Display for LoyaltyEventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}