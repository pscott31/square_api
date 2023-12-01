
use serde::{Serialize, Deserialize};
use super::OrderCreated;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderCreatedObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_created: Option<OrderCreated>,
}
impl std::fmt::Display for OrderCreatedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}