
use serde::{Serialize, Deserialize};
use super::{Error, TeamMemberBookingProfile};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListTeamMemberBookingProfilesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_booking_profiles: Option<Vec<TeamMemberBookingProfile>>,
}
impl std::fmt::Display for ListTeamMemberBookingProfilesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}