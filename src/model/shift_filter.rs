
use serde::{Serialize, Deserialize};
use super::{ShiftWorkday, TimeRange};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShiftFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workday: Option<ShiftWorkday>,
}
impl std::fmt::Display for ShiftFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}