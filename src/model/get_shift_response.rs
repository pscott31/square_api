
use serde::{Serialize, Deserialize};
use super::{Error, Shift};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetShiftResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Shift>,
}
impl std::fmt::Display for GetShiftResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}