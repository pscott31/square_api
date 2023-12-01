
use serde::{Serialize, Deserialize};
use super::{CustomAttributeDefinition, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCustomerCustomAttributeDefinitionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definitions: Option<Vec<CustomAttributeDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for ListCustomerCustomAttributeDefinitionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}