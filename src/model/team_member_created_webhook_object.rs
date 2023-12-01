
use serde::{Serialize, Deserialize};
use super::TeamMember;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamMemberCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member: Option<TeamMember>,
}
impl std::fmt::Display for TeamMemberCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}