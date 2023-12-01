
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionPhase {
    pub cadence: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_price_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for SubscriptionPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}