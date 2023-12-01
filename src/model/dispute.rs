
use serde::{Serialize, Deserialize};
use super::{DisputedPayment, Money};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dispute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_dispute_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputed_payment: Option<DisputedPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for Dispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}