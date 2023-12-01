
use serde::{Serialize, Deserialize};
use super::{Error, InventoryAdjustment};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveInventoryAdjustmentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<InventoryAdjustment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for RetrieveInventoryAdjustmentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}