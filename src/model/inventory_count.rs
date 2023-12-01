
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryCount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_estimated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for InventoryCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}