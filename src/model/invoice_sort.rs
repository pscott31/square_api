
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceSort {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
impl std::fmt::Display for InvoiceSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}