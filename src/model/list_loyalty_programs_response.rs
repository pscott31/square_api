
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyProgram};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListLoyaltyProgramsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<LoyaltyProgram>>,
}
impl std::fmt::Display for ListLoyaltyProgramsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}