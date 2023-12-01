
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemVariationLocationOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_alert_threshold: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_alert_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_out: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_out_valid_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_inventory: Option<bool>,
}
impl std::fmt::Display for ItemVariationLocationOverrides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}