
use serde::{Serialize, Deserialize};
use super::TerminalRefund;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalRefundCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<TerminalRefund>,
}
impl std::fmt::Display for TerminalRefundCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}