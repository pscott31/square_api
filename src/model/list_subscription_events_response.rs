
use serde::{Serialize, Deserialize};
use super::{Error, SubscriptionEvent};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListSubscriptionEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_events: Option<Vec<SubscriptionEvent>>,
}
impl std::fmt::Display for ListSubscriptionEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}