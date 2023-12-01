
use serde::{Serialize, Deserialize};
use super::{InvoicePaymentReminder, Money};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoicePaymentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount_requested_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_requested: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<InvoicePaymentReminder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_adjustment_included_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_completed_amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for InvoicePaymentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}