
use serde::{Serialize, Deserialize};
use super::{BreakType, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListBreakTypesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub break_types: Option<Vec<BreakType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for ListBreakTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}