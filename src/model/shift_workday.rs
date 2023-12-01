
use serde::{Serialize, Deserialize};
use super::DateRange;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShiftWorkday {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<DateRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_shifts_by: Option<String>,
}
impl std::fmt::Display for ShiftWorkday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}