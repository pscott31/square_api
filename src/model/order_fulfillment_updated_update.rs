
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFulfillmentUpdatedUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_state: Option<String>,
}
impl std::fmt::Display for OrderFulfillmentUpdatedUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}