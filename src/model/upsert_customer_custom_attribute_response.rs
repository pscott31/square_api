
use serde::{Serialize, Deserialize};
use super::{CustomAttribute, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpsertCustomerCustomAttributeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute: Option<CustomAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for UpsertCustomerCustomAttributeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}