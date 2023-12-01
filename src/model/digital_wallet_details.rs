
use serde::{Serialize, Deserialize};
use super::CashAppDetails;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigitalWalletDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_app_details: Option<CashAppDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for DigitalWalletDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}