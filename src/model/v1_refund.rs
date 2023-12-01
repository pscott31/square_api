
use serde::{Serialize, Deserialize};
use super::{V1Money, V1PaymentSurcharge, V1PaymentTax};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1Refund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_exchange: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_additive_tax: Option<Vec<V1PaymentTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_additive_tax_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_discount_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_inclusive_tax: Option<Vec<V1PaymentTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_inclusive_tax_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_processing_fee_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_surcharge_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_surcharges: Option<Vec<V1PaymentSurcharge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_tax_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_tip_money: Option<V1Money>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for V1Refund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}