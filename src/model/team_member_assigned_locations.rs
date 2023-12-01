
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamMemberAssignedLocations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
}
impl std::fmt::Display for TeamMemberAssignedLocations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}