
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderLineItemDiscount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderLineItemDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}