
use serde::{Serialize, Deserialize};
use super::{Money, SubscriptionAction, SubscriptionSource};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<SubscriptionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charged_through_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_until_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_override_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SubscriptionSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}