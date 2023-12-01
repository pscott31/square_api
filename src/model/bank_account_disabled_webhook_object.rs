
use serde::{Serialize, Deserialize};
use super::BankAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccountDisabledWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
}
impl std::fmt::Display for BankAccountDisabledWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}