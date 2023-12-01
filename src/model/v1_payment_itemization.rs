
use serde::{Serialize, Deserialize};
use super::{
    V1Money, V1PaymentDiscount, V1PaymentItemDetail, V1PaymentModifier, V1PaymentTax,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1PaymentItemization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<V1PaymentDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_sales_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_detail: Option<V1PaymentItemDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_variation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itemization_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<V1PaymentModifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_sales_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_quantity_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<V1PaymentTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_money: Option<V1Money>,
}
impl std::fmt::Display for V1PaymentItemization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}