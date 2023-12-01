
use serde::{Serialize, Deserialize};
use super::LoyaltyAccountMapping;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchLoyaltyAccountsRequestLoyaltyAccountQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<LoyaltyAccountMapping>>,
}
impl std::fmt::Display for SearchLoyaltyAccountsRequestLoyaltyAccountQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}