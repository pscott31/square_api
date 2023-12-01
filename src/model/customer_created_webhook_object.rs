
use serde::{Serialize, Deserialize};
use super::{Customer, CustomerCreatedWebhookEventContext};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_context: Option<CustomerCreatedWebhookEventContext>,
}
impl std::fmt::Display for CustomerCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}