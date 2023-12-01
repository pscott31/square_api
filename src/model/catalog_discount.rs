
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogDiscount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_tax_basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_required: Option<bool>,
}
impl std::fmt::Display for CatalogDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}