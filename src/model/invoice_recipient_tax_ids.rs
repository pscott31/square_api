
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceRecipientTaxIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_vat: Option<String>,
}
impl std::fmt::Display for InvoiceRecipientTaxIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}