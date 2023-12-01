
use serde::{Serialize, Deserialize};
use super::{BankAccount, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBankAccountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for GetBankAccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}