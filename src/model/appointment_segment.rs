
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppointmentSegment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_team_member: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermission_minutes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_variation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_variation_version: Option<i64>,
    pub team_member_id: String,
}
impl std::fmt::Display for AppointmentSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}