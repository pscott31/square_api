
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchOrdersCustomerFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
}
impl std::fmt::Display for SearchOrdersCustomerFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}