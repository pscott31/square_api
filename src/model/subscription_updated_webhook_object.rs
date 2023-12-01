
use serde::{Serialize, Deserialize};
use super::Subscription;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}
impl std::fmt::Display for SubscriptionUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}