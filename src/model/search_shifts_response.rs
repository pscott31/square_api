
use serde::{Serialize, Deserialize};
use super::{Error, Shift};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchShiftsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shifts: Option<Vec<Shift>>,
}
impl std::fmt::Display for SearchShiftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}