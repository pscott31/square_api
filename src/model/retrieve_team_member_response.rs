
use serde::{Serialize, Deserialize};
use super::{Error, TeamMember};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveTeamMemberResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member: Option<TeamMember>,
}
impl std::fmt::Display for RetrieveTeamMemberResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}