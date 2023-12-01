
use serde::{Serialize, Deserialize};
use super::GiftCardCustomerLinkedWebhookObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCardCustomerLinkedWebhookData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<GiftCardCustomerLinkedWebhookObject>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for GiftCardCustomerLinkedWebhookData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}