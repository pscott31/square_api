
use serde::{Serialize, Deserialize};
use super::AppointmentSegment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Availability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment_segments: Option<Vec<AppointmentSegment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
}
impl std::fmt::Display for Availability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}