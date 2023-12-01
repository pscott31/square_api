
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCreatedWebhookEventContextMerge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_customer_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_customer_id: Option<String>,
}
impl std::fmt::Display for CustomerCreatedWebhookEventContextMerge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}