
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchOrdersFulfillmentFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_states: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_types: Option<Vec<String>>,
}
impl std::fmt::Display for SearchOrdersFulfillmentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}