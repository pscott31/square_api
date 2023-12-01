
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramTerminology {
    pub one: String,
    pub other: String,
}
impl std::fmt::Display for LoyaltyProgramTerminology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}