
use serde::{Serialize, Deserialize};
use super::{Error, TeamMemberBookingProfile};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveTeamMemberBookingProfileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_booking_profile: Option<TeamMemberBookingProfile>,
}
impl std::fmt::Display for RetrieveTeamMemberBookingProfileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}