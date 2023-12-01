
use serde::{Serialize, Deserialize};
use super::OrderFulfillmentUpdated;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFulfillmentUpdatedObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_fulfillment_updated: Option<OrderFulfillmentUpdated>,
}
impl std::fmt::Display for OrderFulfillmentUpdatedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}