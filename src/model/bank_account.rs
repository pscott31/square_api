
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccount {
    pub account_number_suffix: String,
    pub account_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    pub country: String,
    pub creditable: bool,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_mandate_reference_id: Option<String>,
    pub debitable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    pub holder_name: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    pub primary_bank_identification_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_bank_identification_number: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for BankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}