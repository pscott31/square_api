
use serde::{Serialize, Deserialize};
use super::{CatalogItemModifierListInfo, CatalogItemOptionForItem, CatalogObject};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_electronically: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_for_pickup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_plaintext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_options: Option<Vec<CatalogItemOptionForItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_modifier_screen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<CatalogObject>>,
}
impl std::fmt::Display for CatalogItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}