
use serde::{Serialize, Deserialize};
use super::GiftCard;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCardCustomerUnlinkedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card: Option<GiftCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlinked_customer_id: Option<String>,
}
impl std::fmt::Display for GiftCardCustomerUnlinkedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}