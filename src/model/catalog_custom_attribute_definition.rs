
use serde::{Serialize, Deserialize};
use super::{
    CatalogCustomAttributeDefinitionNumberConfig,
    CatalogCustomAttributeDefinitionSelectionConfig,
    CatalogCustomAttributeDefinitionStringConfig, SourceApplication,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogCustomAttributeDefinition {
    pub allowed_object_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_usage_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_config: Option<CatalogCustomAttributeDefinitionNumberConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_config: Option<CatalogCustomAttributeDefinitionSelectionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application: Option<SourceApplication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_config: Option<CatalogCustomAttributeDefinitionStringConfig>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CatalogCustomAttributeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}