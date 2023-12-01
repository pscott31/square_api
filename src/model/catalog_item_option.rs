
use serde::{Serialize, Deserialize};
use super::CatalogObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogItemOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<CatalogObject>>,
}
impl std::fmt::Display for CatalogItemOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}