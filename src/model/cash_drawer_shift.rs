
use serde::{Serialize, Deserialize};
use super::{CashDrawerDevice, Money};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CashDrawerShift {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_paid_in_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_paid_out_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_payment_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_refunds_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_cash_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<CashDrawerDevice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_cash_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_cash_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for CashDrawerShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}