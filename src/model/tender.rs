
use serde::{Serialize, Deserialize};
use super::{AdditionalRecipient, Money, TenderCardDetails, TenderCashDetails};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<Vec<AdditionalRecipient>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_details: Option<TenderCardDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_details: Option<TenderCashDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Tender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}