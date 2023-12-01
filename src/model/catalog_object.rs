
use serde::{Serialize, Deserialize};
use super::{
    CatalogCategory, CatalogCustomAttributeDefinition, CatalogDiscount, CatalogImage,
    CatalogItem, CatalogItemOption, CatalogItemOptionValue, CatalogItemVariation,
    CatalogMeasurementUnit, CatalogModifier, CatalogModifierList, CatalogPricingRule,
    CatalogProductSet, CatalogQuickAmountsSettings, CatalogSubscriptionPlan, CatalogTax,
    CatalogTimePeriod, CatalogV1Id,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_at_location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_v1_ids: Option<Vec<CatalogV1Id>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_data: Option<CatalogCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_values: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_data: Option<CatalogDiscount>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<CatalogImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_data: Option<CatalogItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_option_data: Option<CatalogItemOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_option_value_data: Option<CatalogItemOptionValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_variation_data: Option<CatalogItemVariation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_unit_data: Option<CatalogMeasurementUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_data: Option<CatalogModifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_list_data: Option<CatalogModifierList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_at_all_locations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_at_location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_rule_data: Option<CatalogPricingRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_set_data: Option<CatalogProductSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_amounts_settings_data: Option<CatalogQuickAmountsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_data: Option<CatalogTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period_data: Option<CatalogTimePeriod>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for CatalogObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}