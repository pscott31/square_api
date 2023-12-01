
use serde::{Serialize, Deserialize};
use super::{Error, EventTypeMetadata};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListWebhookEventTypesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<EventTypeMetadata>>,
}
impl std::fmt::Display for ListWebhookEventTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}