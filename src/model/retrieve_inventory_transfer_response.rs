
use serde::{Serialize, Deserialize};
use super::{Error, InventoryTransfer};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveInventoryTransferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<InventoryTransfer>,
}
impl std::fmt::Display for RetrieveInventoryTransferResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}