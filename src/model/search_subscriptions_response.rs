
use serde::{Serialize, Deserialize};
use super::{Error, Subscription};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchSubscriptionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}
impl std::fmt::Display for SearchSubscriptionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}