
use serde::{Serialize, Deserialize};
use super::CustomAttributeDefinition;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomAttributeDefinitionWebhookDataObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition: Option<CustomAttributeDefinition>,
}
impl std::fmt::Display for CustomAttributeDefinitionWebhookDataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}