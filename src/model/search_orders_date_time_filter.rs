
use serde::{Serialize, Deserialize};
use super::TimeRange;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchOrdersDateTimeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<TimeRange>,
}
impl std::fmt::Display for SearchOrdersDateTimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}