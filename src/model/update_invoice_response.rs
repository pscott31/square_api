
use serde::{Serialize, Deserialize};
use super::{Error, Invoice};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInvoiceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
}
impl std::fmt::Display for UpdateInvoiceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}