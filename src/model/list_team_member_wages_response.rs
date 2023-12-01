
use serde::{Serialize, Deserialize};
use super::{Error, TeamMemberWage};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListTeamMemberWagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_wages: Option<Vec<TeamMemberWage>>,
}
impl std::fmt::Display for ListTeamMemberWagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}