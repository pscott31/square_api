
use serde::{Serialize, Deserialize};
use super::{Error, WebhookSubscription};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWebhookSubscriptionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<WebhookSubscription>,
}
impl std::fmt::Display for UpdateWebhookSubscriptionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}