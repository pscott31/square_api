
use serde::{Serialize, Deserialize};
use super::{Error, TeamMemberWage};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTeamMemberWageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_wage: Option<TeamMemberWage>,
}
impl std::fmt::Display for GetTeamMemberWageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}