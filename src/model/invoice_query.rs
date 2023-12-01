
use serde::{Serialize, Deserialize};
use super::{InvoiceFilter, InvoiceSort};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceQuery {
    pub filter: InvoiceFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<InvoiceSort>,
}
impl std::fmt::Display for InvoiceQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}