
use serde::{Serialize, Deserialize};
use super::GiftCard;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCardCustomerLinkedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card: Option<GiftCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_customer_id: Option<String>,
}
impl std::fmt::Display for GiftCardCustomerLinkedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}