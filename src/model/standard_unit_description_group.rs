
use serde::{Serialize, Deserialize};
use super::StandardUnitDescription;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandardUnitDescriptionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_unit_descriptions: Option<Vec<StandardUnitDescription>>,
}
impl std::fmt::Display for StandardUnitDescriptionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}