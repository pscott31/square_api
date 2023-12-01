
use serde::{Serialize, Deserialize};
use super::{ShiftFilter, ShiftSort};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShiftQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ShiftFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ShiftSort>,
}
impl std::fmt::Display for ShiftQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}