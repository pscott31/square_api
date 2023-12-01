
use serde::{Serialize, Deserialize};
use super::CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogCustomAttributeDefinitionSelectionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_selections: Option<
        Vec<CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allowed_selections: Option<i64>,
}
impl std::fmt::Display for CatalogCustomAttributeDefinitionSelectionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}