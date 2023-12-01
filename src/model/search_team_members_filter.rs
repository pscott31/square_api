
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchTeamMembersFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for SearchTeamMembersFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}