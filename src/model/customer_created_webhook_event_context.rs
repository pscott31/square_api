
use serde::{Serialize, Deserialize};
use super::CustomerCreatedWebhookEventContextMerge;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCreatedWebhookEventContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<CustomerCreatedWebhookEventContextMerge>,
}
impl std::fmt::Display for CustomerCreatedWebhookEventContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}