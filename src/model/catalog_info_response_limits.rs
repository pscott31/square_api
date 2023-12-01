
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogInfoResponseLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_delete_max_object_ids: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_retrieve_max_object_ids: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_upsert_max_objects_per_batch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_upsert_max_total_objects: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_max_page_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_item_modifier_lists_max_item_ids: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_item_modifier_lists_max_modifier_lists_to_disable: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_item_modifier_lists_max_modifier_lists_to_enable: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_item_taxes_max_item_ids: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_item_taxes_max_taxes_to_disable: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_item_taxes_max_taxes_to_enable: Option<i64>,
}
impl std::fmt::Display for CatalogInfoResponseLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}