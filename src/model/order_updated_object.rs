
use serde::{Serialize, Deserialize};
use super::OrderUpdated;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderUpdatedObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_updated: Option<OrderUpdated>,
}
impl std::fmt::Display for OrderUpdatedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}