
use serde::{Serialize, Deserialize};
use super::{Money, ProcessingFee};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRefund {
    pub amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_fee: Option<Vec<ProcessingFee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl std::fmt::Display for PaymentRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}