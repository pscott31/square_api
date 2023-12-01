
use serde::{Serialize, Deserialize};
use super::Address;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrePopulatedData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_phone_number: Option<String>,
}
impl std::fmt::Display for PrePopulatedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}