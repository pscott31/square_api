
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BreakType {
    pub break_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub expected_duration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub is_paid: bool,
    pub location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for BreakType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}