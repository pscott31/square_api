
use serde::{Serialize, Deserialize};
use super::{Error, WebhookSubscription};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListWebhookSubscriptionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<WebhookSubscription>>,
}
impl std::fmt::Display for ListWebhookSubscriptionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}