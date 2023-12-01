
use serde::{Serialize, Deserialize};
use super::Invoice;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceCanceledWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
}
impl std::fmt::Display for InvoiceCanceledWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}