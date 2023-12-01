
use serde::{Serialize, Deserialize};
use super::TerminalCheckout;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalCheckoutCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout: Option<TerminalCheckout>,
}
impl std::fmt::Display for TerminalCheckoutCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}