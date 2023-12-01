
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryAdjustmentGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_adjustment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_state: Option<String>,
}
impl std::fmt::Display for InventoryAdjustmentGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}