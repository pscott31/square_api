
use serde::{Serialize, Deserialize};
use super::JobAssignment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WageSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_overtime_exempt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_assignments: Option<Vec<JobAssignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for WageSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}