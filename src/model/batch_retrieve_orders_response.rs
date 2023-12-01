
use serde::{Serialize, Deserialize};
use super::{Error, Order};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRetrieveOrdersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
}
impl std::fmt::Display for BatchRetrieveOrdersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}