
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderMoneyAmounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_charge_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
}
impl std::fmt::Display for OrderMoneyAmounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}