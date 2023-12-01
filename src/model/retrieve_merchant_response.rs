
use serde::{Serialize, Deserialize};
use super::{Error, Merchant};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveMerchantResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<Merchant>,
}
impl std::fmt::Display for RetrieveMerchantResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}