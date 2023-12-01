
use serde::{Serialize, Deserialize};
use super::{Error, InventoryChange, InventoryCount};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchChangeInventoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<InventoryChange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<Vec<InventoryCount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for BatchChangeInventoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}