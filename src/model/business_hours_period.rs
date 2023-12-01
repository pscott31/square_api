
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessHoursPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_local_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_local_time: Option<String>,
}
impl std::fmt::Display for BusinessHoursPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}