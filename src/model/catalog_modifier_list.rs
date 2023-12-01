
use serde::{Serialize, Deserialize};
use super::CatalogObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogModifierList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<CatalogObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_type: Option<String>,
}
impl std::fmt::Display for CatalogModifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}