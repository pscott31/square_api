
use serde::{Serialize, Deserialize};
use super::{BankAccount, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBankAccountByV1IdResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for GetBankAccountByV1IdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}