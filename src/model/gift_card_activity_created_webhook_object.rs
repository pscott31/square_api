
use serde::{Serialize, Deserialize};
use super::GiftCardActivity;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCardActivityCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_activity: Option<GiftCardActivity>,
}
impl std::fmt::Display for GiftCardActivityCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}