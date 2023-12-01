
use serde::{Serialize, Deserialize};
use super::{CustomAttributeDefinition, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomerCustomAttributeDefinitionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition: Option<CustomAttributeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for UpdateCustomerCustomAttributeDefinitionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}