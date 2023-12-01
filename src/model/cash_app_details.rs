
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CashAppDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_cashtag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_full_name: Option<String>,
}
impl std::fmt::Display for CashAppDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}