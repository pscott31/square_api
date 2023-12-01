
use serde::{Serialize, Deserialize};
use super::CustomerDeletedWebhookEventContextMerge;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerDeletedWebhookEventContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<CustomerDeletedWebhookEventContextMerge>,
}
impl std::fmt::Display for CustomerDeletedWebhookEventContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}