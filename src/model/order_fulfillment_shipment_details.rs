
use serde::{Serialize, Deserialize};
use super::OrderFulfillmentRecipient;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFulfillmentShipmentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_shipped_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaged_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<OrderFulfillmentRecipient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipped_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
impl std::fmt::Display for OrderFulfillmentShipmentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}