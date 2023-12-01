
use serde::{Serialize, Deserialize};
use super::PaymentRefund;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<PaymentRefund>,
}
impl std::fmt::Display for RefundCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}