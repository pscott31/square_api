
use serde::{Serialize, Deserialize};
use super::{Error, InventoryPhysicalCount};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveInventoryPhysicalCountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<InventoryPhysicalCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for RetrieveInventoryPhysicalCountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}