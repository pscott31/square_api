
use serde::{Serialize, Deserialize};
use super::MeasurementUnitCustom;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeasurementUnit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit: Option<MeasurementUnitCustom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
}
impl std::fmt::Display for MeasurementUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}