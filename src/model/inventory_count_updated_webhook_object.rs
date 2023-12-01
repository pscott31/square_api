
use serde::{Serialize, Deserialize};
use super::InventoryCount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryCountUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_counts: Option<Vec<InventoryCount>>,
}
impl std::fmt::Display for InventoryCountUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}