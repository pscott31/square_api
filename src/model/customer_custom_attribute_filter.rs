
use serde::{Serialize, Deserialize};
use super::{CustomerCustomAttributeFilterValue, TimeRange};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCustomAttributeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CustomerCustomAttributeFilterValue>,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<TimeRange>,
}
impl std::fmt::Display for CustomerCustomAttributeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}