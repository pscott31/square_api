
use serde::{Serialize, Deserialize};
use super::{AchDetails, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccountPaymentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ownership_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_details: Option<AchDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}
impl std::fmt::Display for BankAccountPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}