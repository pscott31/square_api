
use serde::{Serialize, Deserialize};
use super::MeasurementUnit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandardUnitDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<MeasurementUnit>,
}
impl std::fmt::Display for StandardUnitDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}