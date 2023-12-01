
use serde::{Serialize, Deserialize};
use super::{
    CatalogQueryExact, CatalogQueryItemVariationsForItemOptionValues,
    CatalogQueryItemsForItemOptions, CatalogQueryItemsForModifierList,
    CatalogQueryItemsForTax, CatalogQueryPrefix, CatalogQueryRange, CatalogQuerySet,
    CatalogQuerySortedAttribute, CatalogQueryText,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_query: Option<CatalogQueryExact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_variations_for_item_option_values_query: Option<
        CatalogQueryItemVariationsForItemOptionValues,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_for_item_options_query: Option<CatalogQueryItemsForItemOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_for_modifier_list_query: Option<CatalogQueryItemsForModifierList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_for_tax_query: Option<CatalogQueryItemsForTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_query: Option<CatalogQueryPrefix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_query: Option<CatalogQueryRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_query: Option<CatalogQuerySet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorted_attribute_query: Option<CatalogQuerySortedAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_query: Option<CatalogQueryText>,
}
impl std::fmt::Display for CatalogQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}