
use serde::{Serialize, Deserialize};
use super::CustomAttributeDefinitionWebhookData;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCustomAttributeDefinitionDeletedWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CustomAttributeDefinitionWebhookData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CustomerCustomAttributeDefinitionDeletedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}