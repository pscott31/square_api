
use serde::{Serialize, Deserialize};
use super::{Money, OrderLineItemAppliedTax};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderReturnServiceCharge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_service_charge_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderReturnServiceCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}