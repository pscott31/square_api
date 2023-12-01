
use serde::{Serialize, Deserialize};
use super::CustomerTextFilter;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerAddressFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<CustomerTextFilter>,
}
impl std::fmt::Display for CustomerAddressFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}