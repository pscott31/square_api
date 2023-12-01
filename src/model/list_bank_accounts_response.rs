
use serde::{Serialize, Deserialize};
use super::{BankAccount, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListBankAccountsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_accounts: Option<Vec<BankAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for ListBankAccountsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}