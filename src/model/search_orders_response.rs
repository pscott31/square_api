
use serde::{Serialize, Deserialize};
use super::{Error, Order, OrderEntry};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchOrdersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_entries: Option<Vec<OrderEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
}
impl std::fmt::Display for SearchOrdersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}