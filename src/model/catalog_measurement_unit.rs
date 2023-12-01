
use serde::{Serialize, Deserialize};
use super::MeasurementUnit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogMeasurementUnit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<MeasurementUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
}
impl std::fmt::Display for CatalogMeasurementUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}