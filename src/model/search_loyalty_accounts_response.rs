
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyAccount};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchLoyaltyAccountsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_accounts: Option<Vec<LoyaltyAccount>>,
}
impl std::fmt::Display for SearchLoyaltyAccountsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}