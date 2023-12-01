
use serde::{Serialize, Deserialize};
use super::{
    OrderFulfillmentPickupDetailsCurbsidePickupDetails, OrderFulfillmentRecipient,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFulfillmentPickupDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_complete_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curbside_pickup_details: Option<
        OrderFulfillmentPickupDetailsCurbsidePickupDetails,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_curbside_pickup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picked_up_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_window_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prep_time_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<OrderFulfillmentRecipient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<String>,
}
impl std::fmt::Display for OrderFulfillmentPickupDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}