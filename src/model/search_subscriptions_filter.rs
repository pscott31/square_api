
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchSubscriptionsFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_names: Option<Vec<String>>,
}
impl std::fmt::Display for SearchSubscriptionsFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}