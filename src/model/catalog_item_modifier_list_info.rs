
use serde::{Serialize, Deserialize};
use super::CatalogModifierOverride;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogItemModifierListInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_modifiers: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_selected_modifiers: Option<i64>,
    pub modifier_list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_overrides: Option<Vec<CatalogModifierOverride>>,
}
impl std::fmt::Display for CatalogItemModifierListInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}