
use serde::{Serialize, Deserialize};
use super::{
    Money, OrderLineItemAppliedDiscount, OrderLineItemAppliedTax, OrderLineItemModifier,
    OrderLineItemPricingBlocklists, OrderQuantityUnit,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_discounts: Option<Vec<OrderLineItemAppliedDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_sales_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<OrderLineItemModifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_blocklists: Option<OrderLineItemPricingBlocklists>,
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<OrderQuantityUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_total_price_money: Option<Money>,
}
impl std::fmt::Display for OrderLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}