
use serde::{Serialize, Deserialize};
use super::TimeRange;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalRefundQueryFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for TerminalRefundQueryFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}