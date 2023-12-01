
use serde::{Serialize, Deserialize};
use super::BookingUpdatedWebhookObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookingUpdatedWebhookData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<BookingUpdatedWebhookObject>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for BookingUpdatedWebhookData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}