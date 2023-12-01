
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentDetails {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for ExternalPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}