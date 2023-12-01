
use serde::{Serialize, Deserialize};
use super::Payment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
}
impl std::fmt::Display for PaymentUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}