
use serde::{Serialize, Deserialize};
use super::Order;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
impl std::fmt::Display for CreateOrderRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}