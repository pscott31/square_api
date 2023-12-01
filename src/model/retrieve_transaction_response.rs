
use serde::{Serialize, Deserialize};
use super::{Error, Transaction};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveTransactionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Transaction>,
}
impl std::fmt::Display for RetrieveTransactionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}