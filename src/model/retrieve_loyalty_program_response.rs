
use serde::{Serialize, Deserialize};
use super::{Error, LoyaltyProgram};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveLoyaltyProgramResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<LoyaltyProgram>,
}
impl std::fmt::Display for RetrieveLoyaltyProgramResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}