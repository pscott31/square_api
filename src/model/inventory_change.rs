
use serde::{Serialize, Deserialize};
use super::{
    CatalogMeasurementUnit, InventoryAdjustment, InventoryPhysicalCount,
    InventoryTransfer,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<InventoryAdjustment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<CatalogMeasurementUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_unit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_count: Option<InventoryPhysicalCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<InventoryTransfer>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for InventoryChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}