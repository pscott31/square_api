
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramExpirationPolicy {
    pub expiration_duration: String,
}
impl std::fmt::Display for LoyaltyProgramExpirationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}