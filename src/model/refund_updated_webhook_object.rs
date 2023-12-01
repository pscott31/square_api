
use serde::{Serialize, Deserialize};
use super::PaymentRefund;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<PaymentRefund>,
}
impl std::fmt::Display for RefundUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}