
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventTypeMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_introduced: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_status: Option<String>,
}
impl std::fmt::Display for EventTypeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}