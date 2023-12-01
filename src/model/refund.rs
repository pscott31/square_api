
use serde::{Serialize, Deserialize};
use super::{AdditionalRecipient, Money};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<Vec<AdditionalRecipient>>,
    pub amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub id: String,
    pub location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_fee_money: Option<Money>,
    pub reason: String,
    pub status: String,
    pub tender_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for Refund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}