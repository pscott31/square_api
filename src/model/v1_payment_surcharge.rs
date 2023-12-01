
use serde::{Serialize, Deserialize};
use super::{V1Money, V1PaymentTax};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1PaymentSurcharge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharge_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<V1PaymentTax>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for V1PaymentSurcharge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}