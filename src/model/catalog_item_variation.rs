
use serde::{Serialize, Deserialize};
use super::{
    CatalogItemOptionValueForItemVariation, CatalogStockConversion,
    ItemVariationLocationOverrides, Money,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogItemVariation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_for_booking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_alert_threshold: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_alert_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_option_values: Option<Vec<CatalogItemOptionValueForItemVariation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_overrides: Option<Vec<ItemVariationLocationOverrides>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_unit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sellable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stockable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stockable_conversion: Option<CatalogStockConversion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_inventory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}
impl std::fmt::Display for CatalogItemVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}