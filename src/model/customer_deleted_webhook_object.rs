
use serde::{Serialize, Deserialize};
use super::{Customer, CustomerDeletedWebhookEventContext};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerDeletedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_context: Option<CustomerDeletedWebhookEventContext>,
}
impl std::fmt::Display for CustomerDeletedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}