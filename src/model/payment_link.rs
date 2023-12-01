
use serde::{Serialize, Deserialize};
use super::{CheckoutOptions, PrePopulatedData};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_options: Option<CheckoutOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_populated_data: Option<PrePopulatedData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub version: i64,
}
impl std::fmt::Display for PaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}