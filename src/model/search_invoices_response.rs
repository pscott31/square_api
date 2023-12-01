
use serde::{Serialize, Deserialize};
use super::{Error, Invoice};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchInvoicesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<Vec<Invoice>>,
}
impl std::fmt::Display for SearchInvoicesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}