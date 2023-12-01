
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Break {
    pub break_type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    pub expected_duration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub is_paid: bool,
    pub name: String,
    pub start_at: String,
}
impl std::fmt::Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}