
use serde::{Serialize, Deserialize};
use super::{
    OrderFulfillmentFulfillmentEntry, OrderFulfillmentPickupDetails,
    OrderFulfillmentShipmentDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFulfillment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<OrderFulfillmentFulfillmentEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_details: Option<OrderFulfillmentPickupDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_details: Option<OrderFulfillmentShipmentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}