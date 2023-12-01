
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogPricingRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_products_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_group_ids_any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_products_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_products_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_order_subtotal_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_local_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until_local_time: Option<String>,
}
impl std::fmt::Display for CatalogPricingRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}