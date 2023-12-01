
use serde::{Serialize, Deserialize};
use super::CustomerCustomAttributeFilter;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCustomAttributeFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CustomerCustomAttributeFilter>>,
}
impl std::fmt::Display for CustomerCustomAttributeFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}