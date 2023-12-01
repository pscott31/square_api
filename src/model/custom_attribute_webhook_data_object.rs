
use serde::{Serialize, Deserialize};
use super::CustomAttribute;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomAttributeWebhookDataObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute: Option<CustomAttribute>,
}
impl std::fmt::Display for CustomAttributeWebhookDataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}