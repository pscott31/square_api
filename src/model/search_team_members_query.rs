
use serde::{Serialize, Deserialize};
use super::SearchTeamMembersFilter;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchTeamMembersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SearchTeamMembersFilter>,
}
impl std::fmt::Display for SearchTeamMembersQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}